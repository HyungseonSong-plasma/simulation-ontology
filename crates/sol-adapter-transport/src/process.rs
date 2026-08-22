use crate::{
    AdapterTransportMethod, CorrelationError, JsonRpcResponse, ProtocolOutcome, RequestBuildError,
    RequestIdError, RequestIdGenerator, ResponseDecodeError, StdioFrameDecoder, StdioFrameError,
    TransportRequest,
};
use serde_json::Value;
use sol_adapter_protocol::{
    AdapterDescription, ExecutePlanRequest, ExecutePlanResponse, ProtocolFailure,
    ValidatePlanRequest, ValidatePlanResponse,
};
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, ExitStatus, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub const DEFAULT_ADAPTER_RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterProcessCommand {
    program: PathBuf,
    args: Vec<OsString>,
}

impl AdapterProcessCommand {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            program: PathBuf::from(program.as_ref()),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn program(&self) -> &OsStr {
        self.program.as_os_str()
    }

    pub fn args(&self) -> &[OsString] {
        &self.args
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterSessionState {
    Starting,
    Ready,
    Faulted,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterOperationResult<T> {
    Success(T),
    ProtocolFailure(ProtocolFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterProcessExit {
    pub success: bool,
    pub code: Option<i32>,
    pub stderr: Vec<u8>,
}

impl AdapterProcessExit {
    pub const fn is_abnormal(&self) -> bool {
        !self.success
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterSessionErrorKind {
    Spawn,
    SessionNotReady,
    RequestIdentity,
    RequestEnvelope,
    StdinWrite,
    StdinBrokenPipe,
    ResponseTimeout,
    StdoutEof,
    StdoutRead,
    StderrRead,
    Framing,
    ResponseEnvelope,
    Correlation,
    RemoteJsonRpc,
    ProtocolPayload,
    BootstrapProtocolFailure,
    Shutdown,
    ShutdownTimeout,
    ReaderThreadPanicked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterSessionError {
    Spawn(String),
    NotReady,
    RequestId(RequestIdError),
    Request(RequestBuildError),
    Write(String),
    BrokenPipe(String),
    ResponseTimeout,
    StdoutClosed,
    StdoutRead(String),
    StderrRead(String),
    Framing(StdioFrameError),
    Response(ResponseDecodeError),
    Correlation(CorrelationError),
    JsonRpcError {
        code: i32,
        message: String,
        data: Option<Value>,
    },
    ProtocolPayload(String),
    BootstrapProtocolFailure(ProtocolFailure),
    Shutdown(String),
    ShutdownTimeout,
    ReaderThreadPanicked(&'static str),
}

impl AdapterSessionError {
    pub const fn kind(&self) -> AdapterSessionErrorKind {
        match self {
            Self::Spawn(_) => AdapterSessionErrorKind::Spawn,
            Self::NotReady => AdapterSessionErrorKind::SessionNotReady,
            Self::RequestId(_) => AdapterSessionErrorKind::RequestIdentity,
            Self::Request(_) => AdapterSessionErrorKind::RequestEnvelope,
            Self::Write(_) => AdapterSessionErrorKind::StdinWrite,
            Self::BrokenPipe(_) => AdapterSessionErrorKind::StdinBrokenPipe,
            Self::ResponseTimeout => AdapterSessionErrorKind::ResponseTimeout,
            Self::StdoutClosed => AdapterSessionErrorKind::StdoutEof,
            Self::StdoutRead(_) => AdapterSessionErrorKind::StdoutRead,
            Self::StderrRead(_) => AdapterSessionErrorKind::StderrRead,
            Self::Framing(_) => AdapterSessionErrorKind::Framing,
            Self::Response(_) => AdapterSessionErrorKind::ResponseEnvelope,
            Self::Correlation(_) => AdapterSessionErrorKind::Correlation,
            Self::JsonRpcError { .. } => AdapterSessionErrorKind::RemoteJsonRpc,
            Self::ProtocolPayload(_) => AdapterSessionErrorKind::ProtocolPayload,
            Self::BootstrapProtocolFailure(_) => {
                AdapterSessionErrorKind::BootstrapProtocolFailure
            }
            Self::Shutdown(_) => AdapterSessionErrorKind::Shutdown,
            Self::ShutdownTimeout => AdapterSessionErrorKind::ShutdownTimeout,
            Self::ReaderThreadPanicked(_) => AdapterSessionErrorKind::ReaderThreadPanicked,
        }
    }
}

impl Display for AdapterSessionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spawn(detail) => write!(formatter, "could not spawn adapter process: {detail}"),
            Self::NotReady => write!(formatter, "adapter session is not ready"),
            Self::RequestId(error) => Display::fmt(error, formatter),
            Self::Request(error) => Display::fmt(error, formatter),
            Self::Write(detail) => write!(formatter, "could not write adapter request: {detail}"),
            Self::BrokenPipe(detail) => {
                write!(formatter, "adapter stdin closed while writing a request: {detail}")
            }
            Self::ResponseTimeout => write!(formatter, "adapter response timed out"),
            Self::StdoutClosed => write!(formatter, "adapter stdout closed before a response"),
            Self::StdoutRead(detail) => {
                write!(formatter, "could not read adapter stdout: {detail}")
            }
            Self::StderrRead(detail) => {
                write!(formatter, "could not read adapter stderr: {detail}")
            }
            Self::Framing(error) => Display::fmt(error, formatter),
            Self::Response(error) => Display::fmt(error, formatter),
            Self::Correlation(error) => Display::fmt(error, formatter),
            Self::JsonRpcError { code, message, .. } => {
                write!(
                    formatter,
                    "adapter returned JSON-RPC error {code}: {message}"
                )
            }
            Self::ProtocolPayload(detail) => {
                write!(formatter, "invalid Adapter Protocol payload: {detail}")
            }
            Self::BootstrapProtocolFailure(failure) => write!(
                formatter,
                "adapter bootstrap returned ProtocolFailure {}: {}",
                failure.code, failure.detail
            ),
            Self::Shutdown(detail) => write!(formatter, "could not stop adapter process: {detail}"),
            Self::ShutdownTimeout => {
                write!(formatter, "adapter process did not exit after stdin closed")
            }
            Self::ReaderThreadPanicked(stream) => {
                write!(formatter, "adapter {stream} reader thread panicked")
            }
        }
    }
}

impl Error for AdapterSessionError {}

enum StdoutEvent {
    Frame(Result<String, StdioFrameError>),
    ReadError(String),
    Eof,
}

pub struct AdapterProcessSession {
    child: Child,
    stdin: Option<ChildStdin>,
    stdout_events: Receiver<StdoutEvent>,
    stdout_thread: Option<JoinHandle<()>>,
    stderr_thread: Option<JoinHandle<Result<Vec<u8>, String>>>,
    request_ids: RequestIdGenerator,
    response_timeout: Duration,
    state: AdapterSessionState,
    description: Option<AdapterDescription>,
}

impl AdapterProcessSession {
    pub fn spawn(command: AdapterProcessCommand) -> Result<Self, AdapterSessionError> {
        Self::spawn_with_timeout(command, DEFAULT_ADAPTER_RESPONSE_TIMEOUT)
    }

    pub fn spawn_with_timeout(
        command: AdapterProcessCommand,
        response_timeout: Duration,
    ) -> Result<Self, AdapterSessionError> {
        let mut process = Command::new(&command.program);
        process
            .args(&command.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = process
            .spawn()
            .map_err(|error| AdapterSessionError::Spawn(error.to_string()))?;

        let stdin = child
            .stdin
            .take()
            .expect("piped adapter stdin must be available");
        let stdout = child
            .stdout
            .take()
            .expect("piped adapter stdout must be available");
        let stderr = child
            .stderr
            .take()
            .expect("piped adapter stderr must be available");
        let (stdout_events, stdout_thread) = spawn_stdout_reader(stdout);
        let stderr_thread = thread::spawn(move || {
            let mut stderr = stderr;
            let mut bytes = Vec::new();
            stderr
                .read_to_end(&mut bytes)
                .map_err(|error| error.to_string())?;
            Ok(bytes)
        });

        let mut session = Self {
            child,
            stdin: Some(stdin),
            stdout_events,
            stdout_thread: Some(stdout_thread),
            stderr_thread: Some(stderr_thread),
            request_ids: RequestIdGenerator::new(),
            response_timeout,
            state: AdapterSessionState::Starting,
            description: None,
        };

        match session.bootstrap() {
            Ok(description) => {
                session.description = Some(description);
                session.state = AdapterSessionState::Ready;
                Ok(session)
            }
            Err(error) => {
                session.terminate_now();
                Err(error)
            }
        }
    }

    pub const fn state(&self) -> AdapterSessionState {
        self.state
    }

    pub fn description(&self) -> Option<&AdapterDescription> {
        self.description.as_ref()
    }

    pub fn validate_plan(
        &mut self,
        request: &ValidatePlanRequest,
    ) -> Result<AdapterOperationResult<ValidatePlanResponse>, AdapterSessionError> {
        self.require_ready()?;
        let params = protocol_json_value(
            request
                .to_canonical_json()
                .map_err(|error| AdapterSessionError::ProtocolPayload(error.to_string()))?,
        )?;
        match self.exchange(AdapterTransportMethod::ValidatePlan, Some(params))? {
            ProtocolOutcome::Success(payload) => {
                let response = ValidatePlanResponse::from_json(&canonical_protocol_value(payload))
                    .map_err(|error| AdapterSessionError::ProtocolPayload(error.to_string()));
                if response.is_err() {
                    self.state = AdapterSessionState::Faulted;
                }
                response.map(AdapterOperationResult::Success)
            }
            ProtocolOutcome::Failure(failure) => {
                Ok(AdapterOperationResult::ProtocolFailure(failure))
            }
        }
    }

    pub fn execute_plan(
        &mut self,
        request: &ExecutePlanRequest,
    ) -> Result<AdapterOperationResult<ExecutePlanResponse>, AdapterSessionError> {
        self.require_ready()?;
        let params = protocol_json_value(
            request
                .to_canonical_json()
                .map_err(|error| AdapterSessionError::ProtocolPayload(error.to_string()))?,
        )?;
        match self.exchange(AdapterTransportMethod::ExecutePlan, Some(params))? {
            ProtocolOutcome::Success(payload) => {
                let response = ExecutePlanResponse::from_json(&canonical_protocol_value(payload))
                    .map_err(|error| AdapterSessionError::ProtocolPayload(error.to_string()));
                if response.is_err() {
                    self.state = AdapterSessionState::Faulted;
                }
                response.map(AdapterOperationResult::Success)
            }
            ProtocolOutcome::Failure(failure) => {
                Ok(AdapterOperationResult::ProtocolFailure(failure))
            }
        }
    }

    pub fn shutdown(mut self) -> Result<AdapterProcessExit, AdapterSessionError> {
        self.stdin.take();
        let status = self.wait_for_exit()?;
        self.join_stdout_thread()?;
        let stderr = self.join_stderr_thread()?;
        self.state = AdapterSessionState::Stopped;
        Ok(AdapterProcessExit {
            success: status.success(),
            code: status.code(),
            stderr,
        })
    }

    fn bootstrap(&mut self) -> Result<AdapterDescription, AdapterSessionError> {
        match self.exchange(AdapterTransportMethod::DescribeAdapter, None)? {
            ProtocolOutcome::Success(payload) => {
                AdapterDescription::from_json(&canonical_protocol_value(payload))
                    .map_err(|error| AdapterSessionError::ProtocolPayload(error.to_string()))
            }
            ProtocolOutcome::Failure(failure) => {
                Err(AdapterSessionError::BootstrapProtocolFailure(failure))
            }
        }
    }

    fn require_ready(&self) -> Result<(), AdapterSessionError> {
        if self.state == AdapterSessionState::Ready {
            Ok(())
        } else {
            Err(AdapterSessionError::NotReady)
        }
    }

    fn exchange(
        &mut self,
        method: AdapterTransportMethod,
        params: Option<Value>,
    ) -> Result<ProtocolOutcome, AdapterSessionError> {
        let result = self.exchange_once(method, params);
        if result.is_err() && self.state == AdapterSessionState::Ready {
            self.state = AdapterSessionState::Faulted;
        }
        result
    }

    fn exchange_once(
        &mut self,
        method: AdapterTransportMethod,
        params: Option<Value>,
    ) -> Result<ProtocolOutcome, AdapterSessionError> {
        let id = self
            .request_ids
            .allocate()
            .map_err(AdapterSessionError::RequestId)?;
        let request =
            TransportRequest::new(id, method, params).map_err(AdapterSessionError::Request)?;
        let stdin = self.stdin.as_mut().ok_or(AdapterSessionError::NotReady)?;
        stdin
            .write_all(&request.to_stdio_frame())
            .map_err(request_write_error)?;
        stdin
            .flush()
            .map_err(request_write_error)?;

        let frame = match self.stdout_events.recv_timeout(self.response_timeout) {
            Ok(StdoutEvent::Frame(Ok(frame))) => frame,
            Ok(StdoutEvent::Frame(Err(error))) => return Err(AdapterSessionError::Framing(error)),
            Ok(StdoutEvent::ReadError(detail)) => {
                return Err(AdapterSessionError::StdoutRead(detail))
            }
            Ok(StdoutEvent::Eof) => return Err(AdapterSessionError::StdoutClosed),
            Err(RecvTimeoutError::Timeout) => return Err(AdapterSessionError::ResponseTimeout),
            Err(RecvTimeoutError::Disconnected) => return Err(AdapterSessionError::StdoutClosed),
        };

        let response =
            JsonRpcResponse::from_json(&frame, method).map_err(AdapterSessionError::Response)?;
        response
            .correlate(id)
            .map_err(AdapterSessionError::Correlation)?;
        match response {
            JsonRpcResponse::ProtocolResult { outcome, .. } => Ok(outcome),
            JsonRpcResponse::Error(response) => Err(AdapterSessionError::JsonRpcError {
                code: response.error().code(),
                message: response.error().message().to_owned(),
                data: response.error().data().cloned(),
            }),
        }
    }

    fn wait_for_exit(&mut self) -> Result<ExitStatus, AdapterSessionError> {
        let deadline = Instant::now() + self.response_timeout;
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => return Ok(status),
                Ok(None) if Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(10));
                }
                Ok(None) => {
                    let _ = self.child.kill();
                    let _ = self.child.wait();
                    return Err(AdapterSessionError::ShutdownTimeout);
                }
                Err(error) => return Err(AdapterSessionError::Shutdown(error.to_string())),
            }
        }
    }

    fn join_stdout_thread(&mut self) -> Result<(), AdapterSessionError> {
        if let Some(thread) = self.stdout_thread.take() {
            thread
                .join()
                .map_err(|_| AdapterSessionError::ReaderThreadPanicked("stdout"))?;
        }
        Ok(())
    }

    fn join_stderr_thread(&mut self) -> Result<Vec<u8>, AdapterSessionError> {
        let Some(thread) = self.stderr_thread.take() else {
            return Ok(Vec::new());
        };
        thread
            .join()
            .map_err(|_| AdapterSessionError::ReaderThreadPanicked("stderr"))?
            .map_err(AdapterSessionError::StderrRead)
    }

    fn terminate_now(&mut self) {
        self.stdin.take();
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = self.join_stdout_thread();
        let _ = self.join_stderr_thread();
        self.state = AdapterSessionState::Stopped;
    }
}

impl Drop for AdapterProcessSession {
    fn drop(&mut self) {
        if self.state != AdapterSessionState::Stopped {
            self.terminate_now();
        }
    }
}

fn spawn_stdout_reader(
    mut stdout: std::process::ChildStdout,
) -> (Receiver<StdoutEvent>, JoinHandle<()>) {
    let (sender, receiver) = mpsc::channel();
    let thread = thread::spawn(move || {
        let mut decoder = StdioFrameDecoder::new();
        let mut buffer = [0_u8; 8192];
        loop {
            match stdout.read(&mut buffer) {
                Ok(0) => {
                    if let Err(error) = decoder.finish() {
                        let _ = sender.send(StdoutEvent::Frame(Err(error)));
                    }
                    let _ = sender.send(StdoutEvent::Eof);
                    return;
                }
                Ok(count) => {
                    for frame in decoder.push(&buffer[..count]) {
                        if sender.send(StdoutEvent::Frame(frame)).is_err() {
                            return;
                        }
                    }
                }
                Err(error) => {
                    let _ = sender.send(StdoutEvent::ReadError(error.to_string()));
                    return;
                }
            }
        }
    });
    (receiver, thread)
}

fn protocol_json_value(canonical: String) -> Result<Value, AdapterSessionError> {
    serde_json::from_str(&canonical)
        .map_err(|error| AdapterSessionError::ProtocolPayload(error.to_string()))
}

fn request_write_error(error: io::Error) -> AdapterSessionError {
    if error.kind() == io::ErrorKind::BrokenPipe {
        AdapterSessionError::BrokenPipe(error.to_string())
    } else {
        AdapterSessionError::Write(error.to_string())
    }
}

fn canonical_protocol_value(value: Value) -> String {
    serde_json::to_string(&value).expect("JSON Value always serializes")
}

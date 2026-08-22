use sol_adapter_protocol::{ExecutePlanRequest, ValidatePlanRequest};
use sol_adapter_transport::{
    decode_request, AdapterTransportMethod, JsonRpcResponse, RequestDisposition, TransportRequest,
};
use sol_mock_adapter::{MockAdapter, MockProtocolFailureState};
use std::env;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProbeMode {
    MalformedJson,
    InvalidEnvelope,
    HumanStdout,
    InvalidUtf8,
    Eof,
    Crash,
    AbnormalExit,
    BrokenPipe,
    StderrBytes,
    ProtocolFailure,
    DescribeResponseLoss,
    ValidateResponseLoss,
    ExecuteResponseLoss,
    NoReplayObserver,
    ExecuteFailureNone,
    ExecuteFailureAmbiguous,
}

impl ProbeMode {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "malformed-json" => Some(Self::MalformedJson),
            "invalid-envelope" => Some(Self::InvalidEnvelope),
            "human-stdout" => Some(Self::HumanStdout),
            "invalid-utf8" => Some(Self::InvalidUtf8),
            "eof" => Some(Self::Eof),
            "crash" => Some(Self::Crash),
            "abnormal-exit" => Some(Self::AbnormalExit),
            "broken-pipe" => Some(Self::BrokenPipe),
            "stderr-bytes" => Some(Self::StderrBytes),
            "protocol-failure" => Some(Self::ProtocolFailure),
            "describe-response-loss" => Some(Self::DescribeResponseLoss),
            "validate-response-loss" => Some(Self::ValidateResponseLoss),
            "execute-response-loss" => Some(Self::ExecuteResponseLoss),
            "no-replay-observer" => Some(Self::NoReplayObserver),
            "execute-failure-none" => Some(Self::ExecuteFailureNone),
            "execute-failure-ambiguous" => Some(Self::ExecuteFailureAmbiguous),
            _ => None,
        }
    }
}

fn main() {
    let Some(mode) = env::args()
        .nth(1)
        .and_then(|value| ProbeMode::parse(&value))
    else {
        eprintln!("transport error probe requires one recognized mode");
        process::exit(2);
    };

    if let Err(error) = run(mode) {
        eprintln!("transport error probe setup failed: {error}");
        process::exit(2);
    }
}

fn run(mode: ProbeMode) -> Result<(), String> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = BufReader::new(stdin.lock());
    let mut output = stdout.lock();

    let bootstrap = read_request(&mut input)?;
    if bootstrap.method() != AdapterTransportMethod::DescribeAdapter {
        return Err("first request was not describe_adapter".to_owned());
    }
    if mode == ProbeMode::DescribeResponseLoss {
        return Ok(());
    }
    let description = MockAdapter::thermal()
        .describe_adapter_operation()
        .map_err(|failure| failure.detail)?;
    let payload = serde_json::from_str(
        &description
            .to_canonical_json()
            .map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let response = JsonRpcResponse::protocol_success(bootstrap.id(), payload)
        .map_err(|error| error.to_string())?;
    write_frame(&mut output, &response.to_stdio_frame())?;

    match mode {
        ProbeMode::BrokenPipe => {
            eprintln!("probe closed stdin by exiting after bootstrap");
            process::exit(29);
        }
        ProbeMode::StderrBytes => {
            let mut discarded = Vec::new();
            input
                .read_to_end(&mut discarded)
                .map_err(|error| error.to_string())?;
            io::stderr()
                .lock()
                .write_all(&[b'o', b'p', b'a', b'q', b'u', b'e', b':', 0xff, b'\n'])
                .map_err(|error| error.to_string())?;
            return Ok(());
        }
        ProbeMode::NoReplayObserver => {
            let mut unexpected = String::new();
            if input
                .read_line(&mut unexpected)
                .map_err(|error| error.to_string())?
                == 0
            {
                return Ok(());
            }
            eprintln!("unexpected replay after reconnect: {unexpected}");
            process::exit(31);
        }
        _ => {}
    }

    let request = read_request(&mut input)?;
    match mode {
        ProbeMode::MalformedJson => write_frame(&mut output, b"{not-json}\n"),
        ProbeMode::InvalidEnvelope => write_frame(
            &mut output,
            format!(
                "{{\"id\":{},\"jsonrpc\":\"1.0\",\"result\":{{}}}}\n",
                request.id().value()
            )
            .as_bytes(),
        ),
        ProbeMode::HumanStdout => write_frame(
            &mut output,
            b"human diagnostic accidentally sent to stdout\n",
        ),
        ProbeMode::InvalidUtf8 => write_frame(&mut output, &[0xff, b'\n']),
        ProbeMode::Eof => Ok(()),
        ProbeMode::Crash => panic!("intentional transport probe crash"),
        ProbeMode::AbnormalExit => {
            eprintln!("opaque abnormal-exit diagnostic");
            process::exit(23);
        }
        ProbeMode::ProtocolFailure => write_protocol_failure(&mut output, &request),
        ProbeMode::ValidateResponseLoss => {
            require_method(&request, AdapterTransportMethod::ValidatePlan)?;
            eprintln!("validate_plan request received before response loss");
            Ok(())
        }
        ProbeMode::ExecuteResponseLoss => {
            require_method(&request, AdapterTransportMethod::ExecutePlan)?;
            eprintln!("execute_plan request received before response loss");
            Ok(())
        }
        ProbeMode::ExecuteFailureNone => write_execute_protocol_failure(
            &mut output,
            &request,
            MockProtocolFailureState::ExecuteOperationalBeforeSideEffect,
        ),
        ProbeMode::ExecuteFailureAmbiguous => write_execute_protocol_failure(
            &mut output,
            &request,
            MockProtocolFailureState::ExecuteOperationalAmbiguous,
        ),
        ProbeMode::BrokenPipe
        | ProbeMode::StderrBytes
        | ProbeMode::DescribeResponseLoss
        | ProbeMode::NoReplayObserver => {
            unreachable!("early-return probe mode reached response dispatch")
        }
    }
}

fn read_request(input: &mut impl BufRead) -> Result<TransportRequest, String> {
    let mut frame = String::new();
    if input
        .read_line(&mut frame)
        .map_err(|error| error.to_string())?
        == 0
    {
        return Err("stdin reached EOF before the expected request".to_owned());
    }
    let frame = frame.trim_end_matches(&['\r', '\n'][..]);
    match decode_request(frame) {
        RequestDisposition::Dispatch(request) => Ok(request),
        RequestDisposition::Reject(response) => Err(format!(
            "probe received a rejected request: {}",
            response.to_json()
        )),
        RequestDisposition::IgnoreNotification => {
            Err("probe received a notification instead of a request".to_owned())
        }
    }
}

fn write_protocol_failure(
    output: &mut impl Write,
    request: &TransportRequest,
) -> Result<(), String> {
    if request.method() != AdapterTransportMethod::ValidatePlan {
        return Err("protocol-failure mode requires validate_plan".to_owned());
    }
    let params = request
        .params()
        .expect("validate_plan transport request always has params");
    let typed_request = ValidatePlanRequest::from_json(
        &serde_json::to_string(params).expect("JSON Value always serializes"),
    )
    .map_err(|error| error.to_string())?;
    let failure = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::ValidateOperational)
        .validate_plan_operation(&typed_request)
        .expect_err("injected valid operation must produce ProtocolFailure");
    let response = JsonRpcResponse::protocol_failure(request.id(), request.method(), &failure)
        .map_err(|error| error.to_string())?;
    write_frame(output, &response.to_stdio_frame())
}

fn write_execute_protocol_failure(
    output: &mut impl Write,
    request: &TransportRequest,
    state: MockProtocolFailureState,
) -> Result<(), String> {
    require_method(request, AdapterTransportMethod::ExecutePlan)?;
    let params = request
        .params()
        .expect("execute_plan transport request always has params");
    let typed_request = ExecutePlanRequest::from_json(
        &serde_json::to_string(params).expect("JSON Value always serializes"),
    )
    .map_err(|error| error.to_string())?;
    let failure = MockAdapter::thermal()
        .with_protocol_failure_state(state)
        .execute_plan_operation(&typed_request)
        .expect_err("injected execute operation must produce ProtocolFailure");
    let response = JsonRpcResponse::protocol_failure(request.id(), request.method(), &failure)
        .map_err(|error| error.to_string())?;
    write_frame(output, &response.to_stdio_frame())
}

fn require_method(
    request: &TransportRequest,
    expected: AdapterTransportMethod,
) -> Result<(), String> {
    if request.method() == expected {
        Ok(())
    } else {
        Err(format!(
            "probe expected {} but received {}",
            expected.wire_name(),
            request.method().wire_name()
        ))
    }
}

fn write_frame(output: &mut impl Write, bytes: &[u8]) -> Result<(), String> {
    output.write_all(bytes).map_err(|error| error.to_string())?;
    output.flush().map_err(|error| error.to_string())
}

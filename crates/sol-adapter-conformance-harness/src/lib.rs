#![forbid(unsafe_code)]

//! External-process invocation for reusable adapter conformance tooling.
//!
//! This crate composes the M0.5 process transport with the Phase 0 conformance result
//! model. It does not link adapter implementations or backend runtimes into Core.

use sol_adapter_conformance::{HarnessFailure, HarnessFailureKind};
use sol_adapter_protocol::{
    assess_compatibility, AdapterDescription, CompatibilityAssessment, CompatibilitySupport,
    ExecutePlanRequest, ExecutePlanResponse, ProtocolError, ProtocolFailure, ValidatePlanRequest,
    ValidatePlanResponse,
};
use sol_adapter_transport::{
    AdapterOperationResult, AdapterProcessCommand, AdapterProcessExit, AdapterProcessSession,
    AdapterSessionError, AdapterSessionErrorKind, DEFAULT_ADAPTER_RESPONSE_TIMEOUT,
};
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt::{Debug, Display, Formatter};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalAdapterCommand {
    transport: AdapterProcessCommand,
}

impl ExternalAdapterCommand {
    pub fn new(program: impl AsRef<OsStr>) -> Self {
        Self {
            transport: AdapterProcessCommand::new(program),
        }
    }

    pub fn arg(mut self, argument: impl Into<OsString>) -> Self {
        self.transport = self.transport.arg(argument);
        self
    }

    pub fn program(&self) -> &OsStr {
        self.transport.program()
    }

    pub fn args(&self) -> &[OsString] {
        self.transport.args()
    }

    fn into_transport(self) -> AdapterProcessCommand {
        self.transport
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalAdapterHarness {
    response_timeout: Duration,
}

impl ExternalAdapterHarness {
    pub const fn new(response_timeout: Duration) -> Self {
        Self { response_timeout }
    }

    pub const fn response_timeout(&self) -> Duration {
        self.response_timeout
    }

    pub fn launch(
        &self,
        command: ExternalAdapterCommand,
    ) -> Result<ExternalAdapterLaunch, HarnessInvocationError> {
        let transport = match AdapterProcessSession::spawn_with_timeout(
            command.into_transport(),
            self.response_timeout,
        ) {
            Ok(transport) => transport,
            Err(AdapterSessionError::BootstrapProtocolFailure(failure)) => {
                return Ok(ExternalAdapterLaunch::ProtocolFailure(failure));
            }
            Err(error) => {
                let stage = if error.kind() == AdapterSessionErrorKind::Spawn {
                    InvocationStage::ProcessStart
                } else {
                    InvocationStage::Bootstrap
                };
                return Err(HarnessInvocationError::from_session(stage, error));
            }
        };

        let description = transport.description().ok_or_else(|| {
            HarnessInvocationError::runner_invariant(
                InvocationStage::Bootstrap,
                "ready transport session did not retain its adapter description",
            )
        })?;
        let compatibility =
            assess_compatibility(&CompatibilitySupport::current(), &description.bootstrap)
                .map_err(HarnessInvocationError::compatibility)?;

        Ok(ExternalAdapterLaunch::Ready(ExternalAdapterSession {
            transport,
            compatibility,
        }))
    }
}

impl Default for ExternalAdapterHarness {
    fn default() -> Self {
        Self::new(DEFAULT_ADAPTER_RESPONSE_TIMEOUT)
    }
}

pub enum ExternalAdapterLaunch {
    Ready(ExternalAdapterSession),
    ProtocolFailure(ProtocolFailure),
}

impl Debug for ExternalAdapterLaunch {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ready(session) => formatter.debug_tuple("Ready").field(session).finish(),
            Self::ProtocolFailure(failure) => formatter
                .debug_tuple("ProtocolFailure")
                .field(failure)
                .finish(),
        }
    }
}

pub struct ExternalAdapterSession {
    transport: AdapterProcessSession,
    compatibility: CompatibilityAssessment,
}

impl ExternalAdapterSession {
    pub fn description(&self) -> &AdapterDescription {
        self.transport
            .description()
            .expect("ready harness session retains the bootstrapped description")
    }

    pub const fn compatibility(&self) -> &CompatibilityAssessment {
        &self.compatibility
    }

    pub fn validate_plan(
        &mut self,
        request: &ValidatePlanRequest,
    ) -> Result<AdapterProtocolObservation<ValidatePlanResponse>, HarnessInvocationError> {
        self.transport
            .validate_plan(request)
            .map(protocol_observation)
            .map_err(|error| {
                HarnessInvocationError::from_session(InvocationStage::ValidatePlan, error)
            })
    }

    pub fn execute_plan(
        &mut self,
        request: &ExecutePlanRequest,
    ) -> Result<AdapterProtocolObservation<ExecutePlanResponse>, HarnessInvocationError> {
        self.transport
            .execute_plan(request)
            .map(protocol_observation)
            .map_err(|error| {
                HarnessInvocationError::from_session(InvocationStage::ExecutePlan, error)
            })
    }

    pub fn shutdown(self) -> Result<ExternalAdapterExitEvidence, HarnessInvocationError> {
        self.transport
            .shutdown()
            .map(ExternalAdapterExitEvidence::from)
            .map_err(|error| {
                HarnessInvocationError::from_session(InvocationStage::Shutdown, error)
            })
    }
}

impl Debug for ExternalAdapterSession {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ExternalAdapterSession")
            .field("description", &self.description())
            .field("compatibility", &self.compatibility)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterProtocolObservation<T> {
    Success(T),
    ProtocolFailure(ProtocolFailure),
}

fn protocol_observation<T>(
    result: AdapterOperationResult<T>,
) -> AdapterProtocolObservation<T> {
    match result {
        AdapterOperationResult::Success(value) => AdapterProtocolObservation::Success(value),
        AdapterOperationResult::ProtocolFailure(failure) => {
            AdapterProtocolObservation::ProtocolFailure(failure)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalAdapterExitEvidence {
    success: bool,
    code: Option<i32>,
    stderr: Vec<u8>,
}

impl ExternalAdapterExitEvidence {
    pub const fn success(&self) -> bool {
        self.success
    }

    pub const fn is_abnormal(&self) -> bool {
        !self.success
    }

    pub const fn code(&self) -> Option<i32> {
        self.code
    }

    pub fn stderr(&self) -> &[u8] {
        &self.stderr
    }
}

impl From<AdapterProcessExit> for ExternalAdapterExitEvidence {
    fn from(exit: AdapterProcessExit) -> Self {
        Self {
            success: exit.success,
            code: exit.code,
            stderr: exit.stderr,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvocationStage {
    ProcessStart,
    Bootstrap,
    Compatibility,
    ValidatePlan,
    ExecutePlan,
    Shutdown,
}

impl InvocationStage {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProcessStart => "process start",
            Self::Bootstrap => "bootstrap",
            Self::Compatibility => "compatibility assessment",
            Self::ValidatePlan => "validate_plan",
            Self::ExecutePlan => "execute_plan",
            Self::Shutdown => "shutdown",
        }
    }
}

impl Display for InvocationStage {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvocationFailureEvidence {
    Session(AdapterSessionError),
    Compatibility(ProtocolError),
    RunnerInvariant(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessInvocationError {
    stage: InvocationStage,
    evidence: InvocationFailureEvidence,
    report_failure: HarnessFailure,
}

impl HarnessInvocationError {
    pub const fn stage(&self) -> InvocationStage {
        self.stage
    }

    pub const fn evidence(&self) -> &InvocationFailureEvidence {
        &self.evidence
    }

    pub const fn session_error_kind(&self) -> Option<AdapterSessionErrorKind> {
        match &self.evidence {
            InvocationFailureEvidence::Session(error) => Some(error.kind()),
            InvocationFailureEvidence::Compatibility(_)
            | InvocationFailureEvidence::RunnerInvariant(_) => None,
        }
    }

    pub const fn report_failure(&self) -> &HarnessFailure {
        &self.report_failure
    }

    pub fn into_report_failure(self) -> HarnessFailure {
        self.report_failure
    }

    fn from_session(stage: InvocationStage, error: AdapterSessionError) -> Self {
        let report_kind = report_kind_for_session(error.kind());
        let detail = format!("{stage}: {error}");
        Self {
            stage,
            evidence: InvocationFailureEvidence::Session(error),
            report_failure: HarnessFailure::new(report_kind, detail)
                .expect("session errors always produce non-blank harness evidence"),
        }
    }

    fn compatibility(error: ProtocolError) -> Self {
        let stage = InvocationStage::Compatibility;
        let detail = format!("{stage}: {error}");
        Self {
            stage,
            evidence: InvocationFailureEvidence::Compatibility(error),
            report_failure: HarnessFailure::new(HarnessFailureKind::ResultDecoding, detail)
                .expect("compatibility errors always produce non-blank harness evidence"),
        }
    }

    fn runner_invariant(stage: InvocationStage, detail: impl Into<String>) -> Self {
        let detail = detail.into();
        let report_detail = format!("{stage}: {detail}");
        Self {
            stage,
            evidence: InvocationFailureEvidence::RunnerInvariant(detail),
            report_failure: HarnessFailure::new(
                HarnessFailureKind::RunnerInvariant,
                report_detail,
            )
            .expect("runner invariant errors always produce non-blank harness evidence"),
        }
    }
}

impl Display for HarnessInvocationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.evidence {
            InvocationFailureEvidence::Session(error) => {
                write!(formatter, "external adapter {} failed: {error}", self.stage)
            }
            InvocationFailureEvidence::Compatibility(error) => {
                write!(formatter, "external adapter compatibility failed: {error}")
            }
            InvocationFailureEvidence::RunnerInvariant(detail) => {
                write!(formatter, "conformance harness {} invariant failed: {detail}", self.stage)
            }
        }
    }
}

impl Error for HarnessInvocationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.evidence {
            InvocationFailureEvidence::Session(error) => Some(error),
            InvocationFailureEvidence::Compatibility(error) => Some(error),
            InvocationFailureEvidence::RunnerInvariant(_) => None,
        }
    }
}

const fn report_kind_for_session(kind: AdapterSessionErrorKind) -> HarnessFailureKind {
    match kind {
        AdapterSessionErrorKind::Spawn => HarnessFailureKind::AdapterInvocation,
        AdapterSessionErrorKind::ProtocolPayload => HarnessFailureKind::ResultDecoding,
        AdapterSessionErrorKind::SessionNotReady
        | AdapterSessionErrorKind::RequestIdentity
        | AdapterSessionErrorKind::RequestEnvelope
        | AdapterSessionErrorKind::BootstrapProtocolFailure
        | AdapterSessionErrorKind::ReaderThreadPanicked => HarnessFailureKind::RunnerInvariant,
        AdapterSessionErrorKind::StdinWrite
        | AdapterSessionErrorKind::StdinBrokenPipe
        | AdapterSessionErrorKind::ResponseTimeout
        | AdapterSessionErrorKind::StdoutEof
        | AdapterSessionErrorKind::StdoutRead
        | AdapterSessionErrorKind::StderrRead
        | AdapterSessionErrorKind::Framing
        | AdapterSessionErrorKind::ResponseEnvelope
        | AdapterSessionErrorKind::Correlation
        | AdapterSessionErrorKind::RemoteJsonRpc
        | AdapterSessionErrorKind::Shutdown
        | AdapterSessionErrorKind::ShutdownTimeout => HarnessFailureKind::TransportExchange,
    }
}

#[cfg(test)]
mod tests {
    use super::{report_kind_for_session, HarnessFailureKind};
    use sol_adapter_transport::AdapterSessionErrorKind;

    #[test]
    fn transport_error_categories_map_to_stable_report_categories() {
        assert_eq!(
            report_kind_for_session(AdapterSessionErrorKind::Spawn),
            HarnessFailureKind::AdapterInvocation
        );
        assert_eq!(
            report_kind_for_session(AdapterSessionErrorKind::ResponseTimeout),
            HarnessFailureKind::TransportExchange
        );
        assert_eq!(
            report_kind_for_session(AdapterSessionErrorKind::ProtocolPayload),
            HarnessFailureKind::ResultDecoding
        );
        assert_eq!(
            report_kind_for_session(AdapterSessionErrorKind::SessionNotReady),
            HarnessFailureKind::RunnerInvariant
        );
    }
}


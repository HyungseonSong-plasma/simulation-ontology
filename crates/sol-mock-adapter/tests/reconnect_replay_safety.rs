use sol_adapter_protocol::{
    ExecutePlanRequest, SideEffectEvidence, ValidatePlanRequest, FAILURE_OPERATIONAL,
};
use sol_adapter_transport::{
    protocol_failure_authorizes_automatic_replay, response_loss_recovery,
    AdapterOperationResult, AdapterProcessCommand, AdapterProcessSession, AdapterSessionErrorKind,
    AdapterSessionState, AdapterTransportMethod, ReconnectDisposition, ReplayDisposition,
};

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const RECOVERY_PROFILE: &str = include_str!(
    "../../../docs/implementation/m0.5-replay-reconnect-and-ambiguous-execution-safety.md"
);
const RECOVERY_SOURCE: &str = include_str!("../../sol-adapter-transport/src/recovery.rs");

fn probe_command(mode: &str) -> AdapterProcessCommand {
    AdapterProcessCommand::new(env!("CARGO_BIN_EXE_sol-transport-error-probe")).arg(mode)
}

fn worker_command() -> AdapterProcessCommand {
    AdapterProcessCommand::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio"))
}

fn validate_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap()
}

fn execute_request() -> ExecutePlanRequest {
    ExecutePlanRequest::from_json(EXECUTE_REQUEST).unwrap()
}

#[test]
fn response_loss_policy_is_operation_specific_and_reconnect_is_explicit() {
    let describe = response_loss_recovery(AdapterTransportMethod::DescribeAdapter);
    assert_eq!(
        describe.reconnect(),
        ReconnectDisposition::CallerMayOpenNewBootstrappedSession
    );
    assert_eq!(
        describe.replay(),
        ReplayDisposition::CallerMayReissueDescription
    );
    assert_eq!(
        describe.conservative_side_effects(),
        SideEffectEvidence::None
    );

    let validate = response_loss_recovery(AdapterTransportMethod::ValidatePlan);
    assert_eq!(
        validate.replay(),
        ReplayDisposition::CallerMayReissueEquivalentValidation
    );
    assert_eq!(
        validate.conservative_side_effects(),
        SideEffectEvidence::None
    );

    let execute = response_loss_recovery(AdapterTransportMethod::ExecutePlan);
    assert_eq!(
        execute.replay(),
        ReplayDisposition::TransportMustNotReplayExecution
    );
    assert_eq!(
        execute.conservative_side_effects(),
        SideEffectEvidence::MayHaveOccurred
    );
}

#[test]
fn lost_description_requires_a_fresh_bootstrapped_session() {
    let error = match AdapterProcessSession::spawn(probe_command("describe-response-loss")) {
        Ok(session) => {
            drop(session);
            panic!("description response loss unexpectedly produced a ready session");
        }
        Err(error) => error,
    };
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);

    let replacement =
        AdapterProcessSession::spawn(probe_command("no-replay-observer")).unwrap();
    assert_eq!(replacement.state(), AdapterSessionState::Ready);
    let exit = replacement.shutdown().unwrap();
    assert!(exit.success);
    assert!(exit.stderr.is_empty());
}

#[test]
fn equivalent_validation_is_reissued_only_by_an_explicit_caller_action() {
    let mut lost =
        AdapterProcessSession::spawn(probe_command("validate-response-loss")).unwrap();
    let error = lost.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = lost.shutdown().unwrap();
    assert_eq!(
        exit.stderr,
        b"validate_plan request received before response loss\n".to_vec()
    );

    let mut replacement = AdapterProcessSession::spawn(worker_command()).unwrap();
    let result = replacement.validate_plan(&validate_request()).unwrap();
    assert!(matches!(result, AdapterOperationResult::Success(_)));
    assert!(replacement.shutdown().unwrap().success);
}

#[test]
fn lost_execute_response_is_ambiguous_and_never_replayed_on_reconnect() {
    let mut lost =
        AdapterProcessSession::spawn(probe_command("execute-response-loss")).unwrap();
    let error = lost.execute_plan(&execute_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = lost.shutdown().unwrap();
    assert_eq!(
        exit.stderr,
        b"execute_plan request received before response loss\n".to_vec()
    );

    let recovery = response_loss_recovery(AdapterTransportMethod::ExecutePlan);
    assert_eq!(
        recovery.conservative_side_effects(),
        SideEffectEvidence::MayHaveOccurred
    );
    assert_eq!(
        recovery.replay(),
        ReplayDisposition::TransportMustNotReplayExecution
    );

    let replacement =
        AdapterProcessSession::spawn(probe_command("no-replay-observer")).unwrap();
    let exit = replacement.shutdown().unwrap();
    assert!(exit.success);
    assert!(exit.stderr.is_empty());
}

#[test]
fn protocol_side_effect_evidence_is_preserved_but_never_retry_authority() {
    for (mode, expected) in [
        ("execute-failure-none", SideEffectEvidence::None),
        (
            "execute-failure-ambiguous",
            SideEffectEvidence::MayHaveOccurred,
        ),
    ] {
        let mut session = AdapterProcessSession::spawn(probe_command(mode)).unwrap();
        let result = session.execute_plan(&execute_request()).unwrap();
        let AdapterOperationResult::ProtocolFailure(failure) = result else {
            panic!("fault peer did not produce ProtocolFailure");
        };
        assert_eq!(failure.code, FAILURE_OPERATIONAL);
        assert_eq!(failure.side_effects, expected);
        assert!(!protocol_failure_authorizes_automatic_replay(&failure));
        assert_eq!(session.state(), AdapterSessionState::Ready);
        assert!(session.shutdown().unwrap().success);
    }
}

#[test]
fn recovery_profile_contains_no_synthetic_replay_authority() {
    for forbidden in [
        "idempotency_key",
        "deduplication_token",
        "dedup_token",
        "resume_token",
        "retryable",
        "safe_to_retry",
    ] {
        assert!(!RECOVERY_SOURCE.contains(forbidden));
    }

    for required in [
        "response absence -> no side effects",
        "same canonical execute request -> replay-safe",
        "side_effects=none -> automatic retry authority",
        "new process -> continuation of the prior session",
        "new request ID -> a new semantic operation identity",
    ] {
        assert!(
            RECOVERY_PROFILE.contains(required),
            "missing Phase 4 counterexample: {required}"
        );
    }
}

use sol_adapter_protocol::{ExecutePlanRequest, ValidatePlanRequest};
use sol_adapter_transport::{
    AdapterOperationResult, AdapterProcessCommand, AdapterProcessSession, AdapterSessionError,
    AdapterSessionState,
};
use sol_mock_adapter::MockAdapter;

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const PROCESS_BOUNDARY: &str = include_str!(
    "../../../docs/implementation/m0.5-stdio-process-lifecycle-and-adapter-session.md"
);
const TRANSPORT_SOURCE: &str = include_str!("../../sol-adapter-transport/src/process.rs");
const TRANSPORT_CARGO: &str = include_str!("../../sol-adapter-transport/Cargo.toml");
const MOCK_CARGO: &str = include_str!("../Cargo.toml");

fn worker_command() -> AdapterProcessCommand {
    AdapterProcessCommand::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio"))
}

#[test]
fn subprocess_bootstrap_validate_execute_and_shutdown_are_deterministic() {
    let reference = MockAdapter::thermal();
    let mut session = AdapterProcessSession::spawn(worker_command()).unwrap();
    assert_eq!(session.state(), AdapterSessionState::Ready);
    assert_eq!(
        session.description(),
        Some(&reference.describe_adapter_operation().unwrap())
    );

    let validate_request = ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap();
    let expected_validate = reference
        .validate_plan_operation(&validate_request)
        .unwrap();
    assert_eq!(
        session.validate_plan(&validate_request).unwrap(),
        AdapterOperationResult::Success(expected_validate)
    );

    let execute_request = ExecutePlanRequest::from_json(EXECUTE_REQUEST).unwrap();
    let expected_execute = reference.execute_plan_operation(&execute_request).unwrap();
    assert_eq!(
        session.execute_plan(&execute_request).unwrap(),
        AdapterOperationResult::Success(expected_execute)
    );

    let exit = session.shutdown().unwrap();
    assert!(exit.success);
    assert_eq!(exit.code, Some(0));
    assert!(exit.stderr.is_empty());
}

#[test]
fn a_missing_adapter_command_is_a_spawn_error() {
    let command = AdapterProcessCommand::new(format!(
        "sol-adapter-command-that-does-not-exist-{}",
        std::process::id()
    ));
    assert!(matches!(
        AdapterProcessSession::spawn(command),
        Err(AdapterSessionError::Spawn(_))
    ));
}

#[test]
fn process_and_session_state_remain_transport_only() {
    for required in [
        "process state -> Adapter Protocol semantic state",
        "process ID -> SOL semantic identity",
        "startup success -> SOL lifecycle state",
        "response timeout -> automatic retry",
        "session reconnect -> Phase 2 behavior",
        "network transport -> Phase 2 scope",
    ] {
        assert!(
            PROCESS_BOUNDARY.contains(required),
            "missing Phase 2 counterexample: {required}"
        );
    }

    assert!(MOCK_CARGO.contains("sol-adapter-transport"));
    assert!(!TRANSPORT_CARGO.contains("sol-mock-adapter"));

    // A child-process PID may be exposed as local operational evidence, but it
    // must not become a serialized Protocol/Public Contract identity field.
    assert!(TRANSPORT_SOURCE.contains("pub fn process_id(&self) -> u32"));
    for forbidden_payload_marker in ["\"process_id\"", "\"session_id\""] {
        assert!(!TRANSPORT_SOURCE.contains(forbidden_payload_marker));
    }

    for forbidden_dependency in [
        "tokio", "reqwest", "tonic", "zmq", "moose", "comsol", "ansys",
    ] {
        assert!(!TRANSPORT_CARGO.contains(forbidden_dependency));
        assert!(!MOCK_CARGO.contains(forbidden_dependency));
    }
}

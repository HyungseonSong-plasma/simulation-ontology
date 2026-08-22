use sol_adapter_conformance::HarnessFailureKind;
use sol_adapter_conformance_harness::{
    AdapterProtocolObservation, ExternalAdapterCommand, ExternalAdapterHarness,
    ExternalAdapterLaunch, InvocationStage,
};
use sol_adapter_protocol::{
    CompatibilityOutcome, ExecutePlanRequest, FailureCategory, ValidatePlanRequest,
};
use sol_adapter_transport::AdapterSessionErrorKind;
use sol_mock_adapter::MockAdapterProfile;
use std::time::Duration;

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const HARNESS_BOUNDARY: &str =
    include_str!("../../../docs/implementation/m0.6-external-adapter-invocation-harness.md");
const HARNESS_CARGO: &str = include_str!("../../sol-adapter-conformance-harness/Cargo.toml");
const HARNESS_SOURCE: &str = include_str!("../../sol-adapter-conformance-harness/src/lib.rs");
const MODEL_CARGO: &str = include_str!("../../sol-adapter-conformance/Cargo.toml");
const MOCK_CARGO: &str = include_str!("../Cargo.toml");

fn profile_command(profile: MockAdapterProfile) -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio"))
        .arg(format!("--profile={}", profile.wire_name()))
}
fn probe_command(mode: &str) -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-transport-error-probe")).arg(mode)
}

fn validate_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap()
}

fn execute_request() -> ExecutePlanRequest {
    ExecutePlanRequest::from_json(EXECUTE_REQUEST).unwrap()
}

fn ready_session(
    command: ExternalAdapterCommand,
) -> sol_adapter_conformance_harness::ExternalAdapterSession {
    match ExternalAdapterHarness::default().launch(command).unwrap() {
        ExternalAdapterLaunch::Ready(session) => session,
        ExternalAdapterLaunch::ProtocolFailure(failure) => {
            panic!("expected ready session, received ProtocolFailure: {failure:?}")
        }
    }
}

#[test]
fn external_mock_command_bootstraps_checks_both_versions_and_runs_operations() {
    let mut session = ready_session(profile_command(MockAdapterProfile::Exact));

    assert_eq!(
        session.compatibility().adapter_protocol.outcome,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(
        session.compatibility().public_contract.outcome,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(
        session.compatibility().overall,
        CompatibilityOutcome::Compatible
    );
    assert_eq!(
        session.description().bootstrap.adapter_id,
        "org.simulationontology.mock.thermal"
    );

    assert!(matches!(
        session.validate_plan(&validate_request()).unwrap(),
        AdapterProtocolObservation::Success(_)
    ));
    assert!(matches!(
        session.execute_plan(&execute_request()).unwrap(),
        AdapterProtocolObservation::Success(_)
    ));

    let exit = session.shutdown().unwrap();
    assert!(exit.success());
    assert_eq!(exit.code(), Some(0));
    assert!(exit.stderr().is_empty());
}

#[test]
fn logical_protocol_failures_do_not_become_harness_errors() {
    let mut session = ready_session(profile_command(
        MockAdapterProfile::FailureValidateOperational,
    ));
    let observation = session.validate_plan(&validate_request()).unwrap();
    let AdapterProtocolObservation::ProtocolFailure(failure) = observation else {
        panic!("expected a logical ProtocolFailure observation");
    };
    assert_eq!(failure.category, FailureCategory::Operational);
    assert!(session.shutdown().unwrap().success());

    let launch = ExternalAdapterHarness::default()
        .launch(profile_command(MockAdapterProfile::FailureCompatibility))
        .unwrap();
    let ExternalAdapterLaunch::ProtocolFailure(failure) = launch else {
        panic!("bootstrap ProtocolFailure unexpectedly produced a ready session");
    };
    assert_eq!(failure.category, FailureCategory::Compatibility);
}

#[test]
fn spawn_timeout_transport_and_exit_evidence_are_deterministic() {
    let missing = ExternalAdapterHarness::default()
        .launch(ExternalAdapterCommand::new(format!(
            "sol-conformance-adapter-that-does-not-exist-{}",
            std::process::id()
        )))
        .unwrap_err();
    assert_eq!(missing.stage(), InvocationStage::ProcessStart);
    assert_eq!(
        missing.session_error_kind(),
        Some(AdapterSessionErrorKind::Spawn)
    );
    assert_eq!(
        missing.report_failure().kind(),
        HarnessFailureKind::AdapterInvocation
    );

    let timeout = ExternalAdapterHarness::new(Duration::from_millis(25))
        .launch(probe_command("bootstrap-timeout"))
        .unwrap_err();
    assert_eq!(timeout.stage(), InvocationStage::Bootstrap);
    assert_eq!(
        timeout.session_error_kind(),
        Some(AdapterSessionErrorKind::ResponseTimeout)
    );
    assert_eq!(
        timeout.report_failure().kind(),
        HarnessFailureKind::TransportExchange
    );

    let mut abnormal = ready_session(probe_command("abnormal-exit"));
    let transport = abnormal.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(transport.stage(), InvocationStage::ValidatePlan);
    assert_eq!(
        transport.session_error_kind(),
        Some(AdapterSessionErrorKind::StdoutEof)
    );
    assert_eq!(
        transport.report_failure().kind(),
        HarnessFailureKind::TransportExchange
    );
    let exit = abnormal.shutdown().unwrap();
    assert!(exit.is_abnormal());
    assert_eq!(exit.code(), Some(23));
    assert_eq!(exit.stderr(), b"opaque abnormal-exit diagnostic\n");
}

#[test]
fn harness_keeps_external_dependencies_and_identity_outside_core_semantics() {
    for dependency in [
        "sol-adapter-conformance",
        "sol-adapter-protocol",
        "sol-adapter-transport",
    ] {
        assert!(HARNESS_CARGO.contains(dependency));
    }
    for forbidden_dependency in [
        "sol-mock-adapter",
        "sol-core-evaluation",
        "tokio",
        "reqwest",
        "tonic",
        "moose",
        "comsol",
        "ansys",
    ] {
        assert!(!HARNESS_CARGO.contains(forbidden_dependency));
    }
    assert!(!MODEL_CARGO.contains("sol-adapter-transport"));
    assert!(MOCK_CARGO.contains("[dev-dependencies]"));
    assert!(MOCK_CARGO.contains("sol-adapter-conformance-harness"));
    assert!(!HARNESS_SOURCE.contains("std::process::Command"));
    assert!(!HARNESS_SOURCE.contains("process::id"));

    for counterexample in [
        "bootstrap ProtocolFailure -> harness failure",
        "process ID -> SOL semantic identity",
        "transport timeout -> adapter non-conformance",
        "harness launch success -> compatibility established",
        "adapter package -> Core runtime dependency",
        "provisional command spelling -> stable public interface",
    ] {
        assert!(HARNESS_BOUNDARY.contains(counterexample));
    }
}

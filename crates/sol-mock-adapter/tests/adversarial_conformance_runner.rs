use sol_adapter_conformance::{
    ConformanceCaseResult, ConformanceDetermination, HarnessFailureKind,
};
use sol_adapter_conformance_fixtures::{
    PublishedAdversarialFixtureSuite, PublishedAdversarialScenario,
};
use sol_adapter_conformance_harness::ExternalAdapterCommand;
use sol_adapter_protocol::SideEffectEvidence;
use sol_adapter_transport::{
    response_loss_recovery, AdapterTransportMethod, ReplayDisposition,
};
use sol_mock_adapter::MockAdapterProfile;
use std::collections::BTreeSet;
use std::path::PathBuf;

const PHASE3_RECORD: &str =
    include_str!("../../../docs/implementation/m0.6-adversarial-conformance-matrix.md");
const FIXTURE_RUNNER_CARGO: &str =
    include_str!("../../sol-adapter-conformance-fixtures/Cargo.toml");

fn published_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/adapter-protocol/0.1")
}

fn profile_command(profile: MockAdapterProfile) -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio")).arg(format!(
        "--profile={}",
        profile.wire_name()
    ))
}

fn transport_probe(mode: &str) -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-transport-error-probe")).arg(mode)
}

fn semantic_adversary(mode: &str) -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-conformance-adversary")).arg(mode)
}

fn valid_negative_reference_cases(
) -> Vec<(PublishedAdversarialScenario, ExternalAdapterCommand)> {
    use PublishedAdversarialScenario as Scenario;
    vec![
        (
            Scenario::CompatibilityMissing,
            profile_command(MockAdapterProfile::FailureCompatibility),
        ),
        (Scenario::TargetMismatch, profile_command(MockAdapterProfile::Exact)),
        (
            Scenario::MissingCapability,
            profile_command(MockAdapterProfile::Exact),
        ),
        (
            Scenario::UnsupportedAction,
            profile_command(MockAdapterProfile::Exact),
        ),
        (
            Scenario::PreflightPrerequisiteRejected,
            profile_command(MockAdapterProfile::PreflightPrerequisiteRejected),
        ),
        (
            Scenario::PreflightTransientUnavailable,
            profile_command(MockAdapterProfile::PreflightTransientUnavailable),
        ),
        (
            Scenario::ExecutionPartial,
            profile_command(MockAdapterProfile::ExecutionPartial),
        ),
        (
            Scenario::ExecutionUnsupported,
            profile_command(MockAdapterProfile::ExecutionUnsupported),
        ),
        (
            Scenario::ExecutionUnavailable,
            profile_command(MockAdapterProfile::ExecutionUnavailable),
        ),
        (
            Scenario::ExecutionAuthoritativeRejected,
            profile_command(MockAdapterProfile::ExecutionAuthoritativeRejected),
        ),
        (
            Scenario::ExecutionAlternateOrder,
            profile_command(MockAdapterProfile::ExecutionAlternateOrder),
        ),
        (
            Scenario::ExecutionParallelIndependent,
            profile_command(MockAdapterProfile::ExecutionParallelIndependent),
        ),
        (
            Scenario::PriorAlreadyRealized,
            profile_command(MockAdapterProfile::PriorAlreadyRealized),
        ),
        (
            Scenario::PriorPartialExecution,
            profile_command(MockAdapterProfile::PriorPartialExecution),
        ),
        (
            Scenario::PriorUnresolvedPrerequisite,
            profile_command(MockAdapterProfile::PriorUnresolvedPrerequisite),
        ),
        (
            Scenario::ValidateInvalidRequestFailure,
            profile_command(MockAdapterProfile::FailureInvalidRequest),
        ),
        (
            Scenario::ValidateOperationalFailure,
            profile_command(MockAdapterProfile::FailureValidateOperational),
        ),
        (
            Scenario::ExecuteFailureBeforeSideEffect,
            profile_command(MockAdapterProfile::FailureExecuteBeforeSideEffect),
        ),
        (
            Scenario::ExecuteFailureAmbiguous,
            profile_command(MockAdapterProfile::FailureExecuteAmbiguous),
        ),
    ]
}

#[test]
fn published_adversarial_valid_negative_matrix_is_conformant() {
    let suite = PublishedAdversarialFixtureSuite::new(published_fixture_dir());
    let report = suite.run_matrix(valid_negative_reference_cases());

    assert_eq!(report.determination(), ConformanceDetermination::Conformant);
    assert!(report
        .cases()
        .iter()
        .all(|case| matches!(case.result(), ConformanceCaseResult::Conformant(_))));

    let ids = report
        .cases()
        .iter()
        .map(|case| case.id().as_str())
        .collect::<Vec<_>>();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted, "Phase 0 report ordering must remain deterministic");
}

#[test]
fn transport_response_loss_is_not_adapter_nonconformance_and_never_authorizes_replay() {
    let suite = PublishedAdversarialFixtureSuite::new(published_fixture_dir());
    let record = suite.run_case(
        PublishedAdversarialScenario::ExecuteResponseLoss,
        transport_probe("execute-response-loss"),
    );

    let ConformanceCaseResult::HarnessFailure(failure) = record.result() else {
        panic!("execute response loss must remain harness/transport evidence");
    };
    assert_eq!(failure.kind(), HarnessFailureKind::TransportExchange);

    let recovery = response_loss_recovery(AdapterTransportMethod::ExecutePlan);
    assert_eq!(
        recovery.replay(),
        ReplayDisposition::TransportMustNotReplayExecution
    );
    assert_eq!(
        recovery.conservative_side_effects(),
        SideEffectEvidence::MayHaveOccurred
    );
}

#[test]
fn parsable_dependency_effect_and_provenance_violations_are_adapter_nonconformance() {
    use PublishedAdversarialScenario as Scenario;
    let suite = PublishedAdversarialFixtureSuite::new(published_fixture_dir());
    let report = suite.run_matrix([
        (
            Scenario::DependencyScheduleViolation,
            semantic_adversary("dependency-schedule"),
        ),
        (
            Scenario::AggregateEffectMismatch,
            semantic_adversary("aggregate-effect"),
        ),
        (
            Scenario::ProvenanceIdentityLeakage,
            semantic_adversary("provenance-identity"),
        ),
    ]);

    assert_eq!(
        report.determination(),
        ConformanceDetermination::NonConformant
    );
    assert!(report
        .cases()
        .iter()
        .all(|case| matches!(case.result(), ConformanceCaseResult::NonConformant(_))));
}

#[test]
fn adversarial_scenario_ids_are_complete_unique_and_not_a_stable_cli_contract() {
    let ids = PublishedAdversarialScenario::ALL
        .iter()
        .map(|scenario| scenario.case_id())
        .collect::<Vec<_>>();
    let unique = ids.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(ids.len(), unique.len());

    for forbidden_dependency in ["sol-mock-adapter", "moose", "comsol", "ansys"] {
        assert!(!FIXTURE_RUNNER_CARGO.contains(forbidden_dependency));
    }

    for counterexample in [
        "expected ProtocolFailure -> adapter non-conformance",
        "transport failure -> ProtocolFailure",
        "execute response loss -> automatic replay",
        "opaque provenance reference -> semantic identity",
        "adversarial case id -> stable CLI spelling",
        "conformance matrix -> solver-native physical correctness",
    ] {
        assert!(PHASE3_RECORD.contains(counterexample));
    }
}

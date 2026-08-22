use sol_adapter_conformance::{
    BackendValidationScope, ConformanceCaseResult, ConformanceDetermination, HarnessFailureKind,
};
use sol_adapter_conformance_fixtures::PublishedPositiveFixtureSuite;
use sol_adapter_conformance_harness::ExternalAdapterCommand;
use sol_mock_adapter::MockAdapterProfile;
use std::path::PathBuf;

const PHASE2_RECORD: &str =
    include_str!("../../../docs/implementation/m0.6-protocol-public-contract-fixture-execution.md");
const FIXTURE_RUNNER_CARGO: &str =
    include_str!("../../sol-adapter-conformance-fixtures/Cargo.toml");

fn published_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/adapter-protocol/0.1")
}

fn exact_external_command() -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio"))
        .arg(format!("--profile={}", MockAdapterProfile::Exact.wire_name()))
}

#[test]
fn published_positive_fixture_suite_is_conformant_through_external_command() {
    let report = PublishedPositiveFixtureSuite::new(published_fixture_dir())
        .run(exact_external_command());

    assert_eq!(report.determination(), ConformanceDetermination::Conformant);
    assert_eq!(
        report.backend_validation_scope(),
        BackendValidationScope::NotAssessedByConformance
    );
    assert!(report
        .cases()
        .iter()
        .all(|case| matches!(case.result(), ConformanceCaseResult::Conformant(_))));

    let ids = report
        .cases()
        .iter()
        .map(|case| case.id().as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "adapter-protocol.fixture-decode",
            "compatibility.adapter-protocol",
            "compatibility.public-contract",
            "describe-adapter.positive",
            "execute-plan.positive",
            "public-contract.reused-payloads",
            "validate-plan.positive",
        ]
    );
}

#[test]
fn missing_fixture_material_is_harness_failure_not_adapter_nonconformance() {
    let missing = published_fixture_dir().join("directory-that-does-not-exist");
    let report = PublishedPositiveFixtureSuite::new(missing).run(exact_external_command());

    assert_eq!(
        report.determination(),
        ConformanceDetermination::NotEstablished
    );
    assert_eq!(report.cases().len(), 1);
    let ConformanceCaseResult::HarnessFailure(failure) = report.cases()[0].result() else {
        panic!("missing fixture material must remain a harness failure");
    };
    assert_eq!(failure.kind(), HarnessFailureKind::FixtureLoad);
}

#[test]
fn phase2_keeps_fixture_execution_solver_independent_and_non_normative() {
    for required_dependency in [
        "sol-adapter-conformance",
        "sol-adapter-conformance-harness",
        "sol-adapter-protocol",
    ] {
        assert!(FIXTURE_RUNNER_CARGO.contains(required_dependency));
    }
    for forbidden_dependency in ["sol-mock-adapter", "moose", "comsol", "ansys"] {
        assert!(!FIXTURE_RUNNER_CARGO.contains(forbidden_dependency));
    }

    for counterexample in [
        "published fixture copy -> new semantic representation",
        "fixture load failure -> adapter non-conformance",
        "positive suite conformance -> solver-native physical correctness",
        "adapter implementation version -> Protocol/Public Contract compatibility",
        "runner case id -> stable CLI/output contract",
    ] {
        assert!(PHASE2_RECORD.contains(counterexample));
    }
}

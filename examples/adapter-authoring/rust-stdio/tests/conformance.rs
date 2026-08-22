use sol_adapter_conformance::{
    BackendValidationScope, ConformanceCaseResult, ConformanceDetermination,
};
use sol_adapter_conformance_fixtures::PublishedPositiveFixtureSuite;
use sol_adapter_conformance_harness::ExternalAdapterCommand;
use std::path::PathBuf;

const MANIFEST: &str = include_str!("../Cargo.toml");
const GUIDE: &str = include_str!("../../../../docs/guides/adapter-authoring-0.1.md");

fn published_fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../fixtures/adapter-protocol/0.1")
}

fn command() -> ExternalAdapterCommand {
    ExternalAdapterCommand::new(env!("CARGO_BIN_EXE_sol-adapter-authoring-skeleton"))
}

#[test]
fn skeleton_matches_the_published_positive_fixture_baseline_through_stdio() {
    let report = PublishedPositiveFixtureSuite::new(published_fixture_dir()).run(command());

    assert_eq!(report.determination(), ConformanceDetermination::Conformant);
    assert_eq!(
        report.backend_validation_scope(),
        BackendValidationScope::NotAssessedByConformance
    );
    assert!(report
        .cases()
        .iter()
        .all(|case| matches!(case.result(), ConformanceCaseResult::Conformant(_))));
}

#[test]
fn skeleton_has_no_mock_adapter_or_solver_runtime_dependency() {
    let manifest = MANIFEST.to_ascii_lowercase();
    for forbidden in ["sol-mock-adapter", "moose", "comsol", "ansys"] {
        assert!(
            !manifest.contains(forbidden),
            "authoring skeleton must not depend on {forbidden}"
        );
    }
}

#[test]
fn guide_preserves_the_non_normative_and_physical_correctness_boundaries() {
    let guide = GUIDE.to_ascii_lowercase();
    for required in [
        "reference scaffolding, not a new normative protocol surface",
        "does not prove backend physical or numerical correctness",
        "no automatic retry or replay authority",
        "backend-native identifiers belong only in opaque provenance",
        "exact conformance cli and serialized report format remain provisional",
    ] {
        assert!(guide.contains(required), "guide is missing boundary: {required}");
    }
}

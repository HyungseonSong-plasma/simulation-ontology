use sol_adapter_conformance::{
    BackendValidationScope, ConformanceCaseId, ConformanceCaseRecord, ConformanceCaseResult,
    ConformanceDetermination, ConformanceEvidence, ConformanceModelError, ConformanceReport,
    ConformanceScope, ConformanceViolation, HarnessFailure, HarnessFailureKind, PublishedContract,
};
use sol_adapter_protocol::ProtocolOperation;

fn conformant(id: &str, scope: ConformanceScope) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        ConformanceCaseId::new(id).unwrap(),
        scope,
        ConformanceCaseResult::Conformant(
            ConformanceEvidence::new("published expectation observed").unwrap(),
        ),
    )
}

fn non_conformant(id: &str, scope: ConformanceScope) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        ConformanceCaseId::new(id).unwrap(),
        scope,
        ConformanceCaseResult::NonConformant(
            ConformanceViolation::new("published expectation contradicted").unwrap(),
        ),
    )
}

fn harness_failure(id: &str, scope: ConformanceScope) -> ConformanceCaseRecord {
    ConformanceCaseRecord::new(
        ConformanceCaseId::new(id).unwrap(),
        scope,
        ConformanceCaseResult::HarnessFailure(
            HarnessFailure::new(
                HarnessFailureKind::AdapterInvocation,
                "adapter process could not be invoked",
            )
            .unwrap(),
        ),
    )
}

#[test]
fn accepted_scope_is_representable_without_transport_or_cli_policy() {
    let scopes = [
        ConformanceScope::Compatibility(PublishedContract::AdapterProtocol),
        ConformanceScope::Compatibility(PublishedContract::PublicContract),
        ConformanceScope::Schema(PublishedContract::AdapterProtocol),
        ConformanceScope::Schema(PublishedContract::PublicContract),
        ConformanceScope::Fixture(PublishedContract::AdapterProtocol),
        ConformanceScope::Fixture(PublishedContract::PublicContract),
        ConformanceScope::OperationSemantics(ProtocolOperation::DescribeAdapter),
        ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan),
        ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
        ConformanceScope::Scheduling,
        ConformanceScope::FailureAndReplay,
        ConformanceScope::Provenance,
    ];

    assert_eq!(scopes.len(), 12);
}

#[test]
fn all_observed_expectations_establish_conformance_only() {
    let report = ConformanceReport::new(vec![
        conformant(
            "compat.protocol.current",
            ConformanceScope::Compatibility(PublishedContract::AdapterProtocol),
        ),
        conformant(
            "operation.describe.canonical",
            ConformanceScope::OperationSemantics(ProtocolOperation::DescribeAdapter),
        ),
    ])
    .unwrap();

    assert_eq!(report.determination(), ConformanceDetermination::Conformant);
    assert_eq!(
        report.backend_validation_scope(),
        BackendValidationScope::NotAssessedByConformance
    );
}

#[test]
fn observed_violation_is_not_hidden_by_a_harness_failure() {
    let report = ConformanceReport::new(vec![
        harness_failure("fixture.unavailable", ConformanceScope::Provenance),
        non_conformant(
            "schema.protocol.invalid-shape",
            ConformanceScope::Schema(PublishedContract::AdapterProtocol),
        ),
    ])
    .unwrap();

    assert_eq!(
        report.determination(),
        ConformanceDetermination::NonConformant
    );
}

#[test]
fn harness_failure_does_not_accuse_the_adapter_of_non_conformance() {
    let report = ConformanceReport::new(vec![
        conformant("fixture.loaded", ConformanceScope::Provenance),
        harness_failure(
            "operation.execute.invocation",
            ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
        ),
    ])
    .unwrap();

    assert_eq!(
        report.determination(),
        ConformanceDetermination::NotEstablished
    );
}

#[test]
fn report_order_is_deterministic_and_case_ids_are_unique() {
    let report = ConformanceReport::new(vec![
        conformant("case.z", ConformanceScope::Scheduling),
        conformant("case.a", ConformanceScope::FailureAndReplay),
    ])
    .unwrap();

    assert_eq!(report.cases()[0].id().as_str(), "case.a");
    assert_eq!(report.cases()[1].id().as_str(), "case.z");

    let duplicate = ConformanceReport::new(vec![
        conformant("case.same", ConformanceScope::Scheduling),
        conformant("case.same", ConformanceScope::Provenance),
    ]);
    assert_eq!(
        duplicate,
        Err(ConformanceModelError::DuplicateCaseId(
            "case.same".to_owned()
        ))
    );
}

#[test]
fn blank_model_text_and_empty_reports_are_rejected() {
    assert_eq!(
        ConformanceCaseId::new("  "),
        Err(ConformanceModelError::EmptyField("conformance case id"))
    );
    assert_eq!(
        ConformanceEvidence::new("\n"),
        Err(ConformanceModelError::EmptyField("conformance evidence"))
    );
    assert_eq!(
        ConformanceViolation::new(""),
        Err(ConformanceModelError::EmptyField("conformance violation"))
    );
    assert_eq!(
        HarnessFailure::new(HarnessFailureKind::ResultDecoding, "\t"),
        Err(ConformanceModelError::EmptyField("harness failure detail"))
    );
    assert_eq!(
        ConformanceReport::new(Vec::new()),
        Err(ConformanceModelError::EmptyReport)
    );
}

#[test]
fn phase_zero_preserves_dependency_and_public_interface_boundaries() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(manifest_dir.join("Cargo.toml")).unwrap();
    let source = std::fs::read_to_string(manifest_dir.join("src/lib.rs")).unwrap();
    let boundary = std::fs::read_to_string(
        manifest_dir
            .join("../../docs/implementation/m0.6-conformance-runner-boundary-and-result-model.md"),
    )
    .unwrap();

    assert!(manifest.contains("sol-adapter-protocol"));
    for forbidden_dependency in [
        "sol-adapter-transport",
        "sol-core-evaluation",
        "sol-mock-adapter",
        "serde",
    ] {
        assert!(!manifest.contains(forbidden_dependency));
    }

    for forbidden_source_term in [
        "EvaluationStatus",
        "Serialize",
        "Deserialize",
        "std::process::Command",
        "AdapterProcessSession",
    ] {
        assert!(!source.contains(forbidden_source_term));
    }

    for counterexample in [
        "conformance success -> solver-native physical correctness",
        "ProtocolFailure -> conformance violation",
        "harness failure -> adapter non-conformance",
        "runner result -> SOL lifecycle state",
        "exact CLI spelling in Phase 0 -> stable public interface",
        "transport failure -> ProtocolFailure",
    ] {
        assert!(boundary.contains(counterexample));
    }
}

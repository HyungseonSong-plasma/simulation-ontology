use sol_adapter_protocol::{
    ActionExecutionState, ExecutePlanRequest, ExecutePlanResponse, ExecutionError, ExecutionOutcome,
    ValidatePlanRequest,
};
use sol_public_contract::{RealizationQualityDto, MappingSubjectDto};

const THERMAL_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const EXACT: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-exact-response.json");
const DEGRADED: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-degraded-response.json");
const UNSUPPORTED: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-unsupported-response.json");
const INDEPENDENT_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-independent-request.json");
const ALTERNATE: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-alternate-order-response.json"
);
const PARALLEL: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-parallel-response.json");
const PARTIAL: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-partial-response.json");
const UNAVAILABLE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-unavailable-response.json");
const CHANGED_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-changed-request.json");
const REJECTED: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-authoritative-rejection-response.json"
);
const PREFLIGHT_ACCEPTED: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");

const DEPENDENCY_VIOLATION: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-dependency-violation.json"
);
const MISSING_REPORT: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-missing-action-report.json"
);
const DUPLICATE_REPORT: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-duplicate-action-report.json"
);
const UNKNOWN_REPORT: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-unknown-action-report.json"
);
const STATE_BATCH_MISMATCH: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-state-batch-mismatch.json"
);
const AGGREGATE_MISMATCH: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-aggregate-effect-mismatch.json"
);
const BACKEND_NATIVE: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-backend-native-leakage.json"
);
const OPAQUE_IDENTITY: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-execution-opaque-reference-semantic-identity.json"
);

fn validated_response(
    response: &str,
    request: &ExecutePlanRequest,
) -> Result<ExecutePlanResponse, ExecutionError> {
    let mut response = ExecutePlanResponse::from_json(response)?;
    response.validate_against(request)?;
    Ok(response)
}

#[test]
fn execute_plan_resends_full_canonical_target_and_plan() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();

    assert_eq!(request.adapter_protocol_version, "0.1");
    assert_eq!(request.target.public_contract_version, "0.1");
    assert_eq!(request.plan.public_contract_version, "0.1");
    assert_eq!(request.plan.actions.len(), 3);

    let canonical = request.to_canonical_json().unwrap();
    for forbidden in [
        "validation_token",
        "acceptance_id",
        "lease_id",
        "plan_hash",
        "plan_digest",
        "jsonrpc",
        "retry_policy",
    ] {
        assert!(!canonical.contains(forbidden));
    }
}

#[test]
fn exact_execution_reports_every_action_and_aggregate_effect_union() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    let response = validated_response(EXACT, &request).unwrap();

    assert_eq!(response.execution, ExecutionOutcome::Completed);
    assert_eq!(response.action_reports.len(), request.plan.actions.len());
    assert!(response
        .action_reports
        .iter()
        .all(|report| report.state == ActionExecutionState::Completed));
    assert_eq!(response.effects.len(), 3);
    assert!(response.provenance.is_some());
}

#[test]
fn degraded_and_unsupported_effects_are_realization_data_not_lifecycle_results() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    let degraded = validated_response(DEGRADED, &request).unwrap();
    let unsupported = validated_response(UNSUPPORTED, &request).unwrap();

    assert_eq!(degraded.execution, ExecutionOutcome::Completed);
    assert_eq!(unsupported.execution, ExecutionOutcome::Completed);
    assert_eq!(degraded.effects[0].quality, RealizationQualityDto::Degraded);
    assert_eq!(unsupported.effects[0].quality, RealizationQualityDto::Unsupported);

    for fixture in [DEGRADED, UNSUPPORTED] {
        assert!(!fixture.contains("PASS"));
        assert!(!fixture.contains("FAIL"));
        assert!(!fixture.contains("BLOCKED"));
        assert!(!fixture.contains("INDETERMINATE"));
        assert!(!fixture.contains("comparison"));
        assert!(!fixture.contains("status"));
    }
}

#[test]
fn independent_actions_may_use_alternate_topological_or_parallel_schedule() {
    let request = ExecutePlanRequest::from_json(INDEPENDENT_REQUEST).unwrap();
    let alternate = validated_response(ALTERNATE, &request).unwrap();
    let parallel = validated_response(PARALLEL, &request).unwrap();

    assert_eq!(alternate.execution_batches[0], vec!["thermal.b"]);
    assert_eq!(alternate.execution_batches[1], vec!["thermal.a"]);
    assert_eq!(parallel.execution_batches[0], vec!["thermal.a", "thermal.b"]);
    assert_eq!(request.plan.topological_order().unwrap(), vec!["thermal.a", "thermal.b", "thermal.c"]);
}

#[test]
fn partial_execution_preserves_failure_and_dependency_skip_observability() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    let response = validated_response(PARTIAL, &request).unwrap();

    assert_eq!(response.execution, ExecutionOutcome::Partial);
    let material = response
        .action_reports
        .iter()
        .find(|report| report.action_id == "thermal.material")
        .unwrap();
    let solve = response
        .action_reports
        .iter()
        .find(|report| report.action_id == "thermal.solve")
        .unwrap();
    assert_eq!(material.state, ActionExecutionState::Failed);
    assert_eq!(solve.state, ActionExecutionState::SkippedDependency);
    assert_eq!(response.effects.len(), 2);
}

#[test]
fn authoritative_execution_can_reject_changed_plan_after_accepted_preflight() {
    let preflight = ValidatePlanRequest::from_json(PREFLIGHT_ACCEPTED).unwrap();
    let execute = ExecutePlanRequest::from_json(CHANGED_REQUEST).unwrap();
    assert_ne!(
        preflight.canonical_plan_identity().unwrap(),
        execute.canonical_plan_identity().unwrap()
    );

    let response = validated_response(REJECTED, &execute).unwrap();
    assert_eq!(response.execution, ExecutionOutcome::Rejected);
    assert!(response.execution_batches.is_empty());
    assert!(response
        .action_reports
        .iter()
        .all(|report| report.state == ActionExecutionState::NotStarted));
}

#[test]
fn authoritative_current_state_unavailability_is_distinct_from_rejection() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    let response = validated_response(UNAVAILABLE, &request).unwrap();

    assert_eq!(response.execution, ExecutionOutcome::Unavailable);
    assert!(response.execution_batches.is_empty());
    assert!(response
        .action_reports
        .iter()
        .any(|report| report.state == ActionExecutionState::Unavailable));
}

#[test]
fn dependency_violation_is_rejected() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    assert!(matches!(
        validated_response(DEPENDENCY_VIOLATION, &request),
        Err(ExecutionError::DependencyViolation { .. })
    ));
}

#[test]
fn action_report_coverage_is_exactly_once() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    assert!(matches!(
        validated_response(MISSING_REPORT, &request),
        Err(ExecutionError::ActionReportCoverage { .. })
    ));
    assert!(matches!(
        ExecutePlanResponse::from_json(DUPLICATE_REPORT),
        Err(ExecutionError::DuplicateActionReport(_))
    ));
    assert!(matches!(
        validated_response(UNKNOWN_REPORT, &request),
        Err(ExecutionError::ActionReportCoverage { .. })
    ));
}

#[test]
fn action_state_must_match_execution_batch_membership() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    assert!(matches!(
        validated_response(STATE_BATCH_MISMATCH, &request),
        Err(ExecutionError::InconsistentSchedule { .. })
    ));
}

#[test]
fn top_level_effects_must_equal_normalized_action_effect_union() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    assert_eq!(
        validated_response(AGGREGATE_MISMATCH, &request).unwrap_err(),
        ExecutionError::AggregateEffectsMismatch
    );
}

#[test]
fn backend_native_semantic_payload_is_rejected_but_opaque_provenance_is_allowed() {
    assert!(matches!(
        ExecutePlanResponse::from_json(BACKEND_NATIVE),
        Err(ExecutionError::ForbiddenField(field)) if field == "backend_object"
    ));

    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    let exact = validated_response(EXACT, &request).unwrap();
    let provenance = exact.provenance.unwrap();
    assert_eq!(provenance.opaque_references[0].namespace, "mock.job");
    assert_eq!(provenance.opaque_references[0].reference, "job-42");
}

#[test]
fn opaque_reference_cannot_be_reused_as_semantic_identity() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    assert!(matches!(
        validated_response(OPAQUE_IDENTITY, &request),
        Err(ExecutionError::OpaqueReferenceUsedAsSemanticIdentity(reference))
            if reference == "thermal.domain"
    ));
}

#[test]
fn realization_subjects_remain_public_contract_semantic_identity() {
    let request = ExecutePlanRequest::from_json(THERMAL_REQUEST).unwrap();
    let response = validated_response(EXACT, &request).unwrap();
    assert!(response.effects.iter().any(|effect| {
        matches!(
            &effect.subject,
            MappingSubjectDto::Entity { id, .. } if id == "thermal.temperature_field"
        )
    }));
}

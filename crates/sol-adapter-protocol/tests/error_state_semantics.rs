use serde_json::Value;
use sol_adapter_protocol::{
    conservative_execute_side_effect_evidence, diagnostic_context, plan_operation_idempotency,
    ActionExecutionState, ExecutePlanRequest, ExecutePlanResponse, ExecutionOutcome, FailureCategory,
    PlanOperation, PlanOperationIdempotency, PreflightOutcome, ProtocolFailure, ProtocolFailureError,
    ProtocolOperation, SideEffectEvidence, ValidatePlanResponse, DIAGNOSTIC_PRECONDITION_REJECTED,
    PRECONDITION_ALREADY_REALIZED, PRECONDITION_PARTIAL_PRIOR_EXECUTION,
    PRECONDITION_UNRESOLVED_PREREQUISITE,
};

const FAILURE_PROTOCOL: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-adapter-protocol-incompatible.json"
);
const FAILURE_PUBLIC: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-public-contract-incompatible.json"
);
const FAILURE_MISSING: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-compatibility-missing.json"
);
const FAILURE_BOOTSTRAP: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-malformed-bootstrap.json"
);
const FAILURE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/protocol-failure-invalid-request.json");
const FAILURE_VALIDATE: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-validate-operational.json"
);
const FAILURE_EXECUTE_NONE: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-execute-before-side-effect.json"
);
const FAILURE_EXECUTE_AMBIGUOUS: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-execute-ambiguous.json"
);

const VALIDATE_REJECTED: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-rejected-response.json");
const VALIDATE_UNAVAILABLE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-unavailable-response.json");
const REPEATED_VALIDATION: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/repeated-validation-same-state.json");

const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const EXECUTE_EXACT: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-exact-response.json");
const EXECUTE_UNAVAILABLE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-unavailable-response.json");
const ALREADY_REALIZED: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-already-realized-response.json"
);
const PARTIAL_PRIOR: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-partial-prior-rejection-response.json"
);
const UNRESOLVED_PREREQUISITE: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-unresolved-prerequisite-response.json"
);

const LIFECYCLE_CONFLATION: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-failure-lifecycle-conflation.json"
);
const RETRYABLE_FIELD: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-failure-retryable-field.json"
);
const RESPONSE_LOSS_SAFE_RETRY: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-response-loss-safe-retry.json"
);

fn validated_execute_response(input: &str) -> ExecutePlanResponse {
    let request = ExecutePlanRequest::from_json(EXECUTE_REQUEST).unwrap();
    let mut response = ExecutePlanResponse::from_json(input).unwrap();
    response.validate_against(&request).unwrap();
    response
}

fn response_precondition(response: &ExecutePlanResponse) -> String {
    let diagnostic = response
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == DIAGNOSTIC_PRECONDITION_REJECTED)
        .unwrap();
    diagnostic_context(diagnostic)
        .unwrap()
        .unwrap()
        .precondition
        .unwrap()
}

#[test]
fn failure_algebra_separates_compatibility_invalid_request_and_operational_failure() {
    for fixture in [FAILURE_PROTOCOL, FAILURE_PUBLIC, FAILURE_MISSING] {
        let failure = ProtocolFailure::from_json(fixture).unwrap();
        assert_eq!(failure.category, FailureCategory::Compatibility);
        assert_eq!(failure.side_effects, SideEffectEvidence::None);
        assert!(!failure.to_canonical_json().unwrap().contains("adapter_protocol_version"));
    }

    for fixture in [FAILURE_BOOTSTRAP, FAILURE_REQUEST] {
        let failure = ProtocolFailure::from_json(fixture).unwrap();
        assert_eq!(failure.category, FailureCategory::InvalidRequest);
        assert_eq!(failure.side_effects, SideEffectEvidence::None);
    }

    let validation_failure = ProtocolFailure::from_json(FAILURE_VALIDATE).unwrap();
    assert_eq!(validation_failure.category, FailureCategory::Operational);
    validation_failure
        .validate_for(ProtocolOperation::ValidatePlan)
        .unwrap();

    let execute_failure = ProtocolFailure::from_json(FAILURE_EXECUTE_AMBIGUOUS).unwrap();
    assert_eq!(execute_failure.category, FailureCategory::Operational);
    assert_eq!(
        execute_failure.side_effects,
        SideEffectEvidence::MayHaveOccurred
    );
    execute_failure
        .validate_for(ProtocolOperation::ExecutePlan)
        .unwrap();
}

#[test]
fn side_effect_evidence_is_conservative_and_not_a_retry_directive() {
    assert_eq!(
        conservative_execute_side_effect_evidence(None),
        SideEffectEvidence::MayHaveOccurred
    );

    let proven_none = ProtocolFailure::from_json(FAILURE_EXECUTE_NONE).unwrap();
    assert_eq!(
        conservative_execute_side_effect_evidence(Some(&proven_none)),
        SideEffectEvidence::None
    );

    let ambiguous = ProtocolFailure::from_json(FAILURE_EXECUTE_AMBIGUOUS).unwrap();
    assert_eq!(
        conservative_execute_side_effect_evidence(Some(&ambiguous)),
        SideEffectEvidence::MayHaveOccurred
    );

    let counterexample: Value = serde_json::from_str(RESPONSE_LOSS_SAFE_RETRY).unwrap();
    assert_eq!(counterexample["observation"], "no_execute_response");
    assert_eq!(counterexample["assumed_side_effects"], "none");
    assert_eq!(counterexample["assumed_safe_to_retry"], true);
    assert_eq!(
        conservative_execute_side_effect_evidence(None),
        SideEffectEvidence::MayHaveOccurred
    );
}

#[test]
fn validate_plan_is_idempotent_for_equivalent_input_and_state_but_execute_is_not() {
    assert_eq!(
        plan_operation_idempotency(PlanOperation::ValidatePlan),
        PlanOperationIdempotency::IdempotentForEquivalentInputAndState
    );
    assert_eq!(
        plan_operation_idempotency(PlanOperation::ExecutePlan),
        PlanOperationIdempotency::NonIdempotentByDefault
    );

    let fixture: Value = serde_json::from_str(REPEATED_VALIDATION).unwrap();
    assert_eq!(fixture["relevant_state_evidence"]["availability"], "available");
    let first = ValidatePlanResponse::from_json(
        &serde_json::to_string(&fixture["first_response"]).unwrap(),
    )
    .unwrap();
    let second = ValidatePlanResponse::from_json(
        &serde_json::to_string(&fixture["second_response"]).unwrap(),
    )
    .unwrap();
    assert_eq!(
        first.to_canonical_json().unwrap(),
        second.to_canonical_json().unwrap()
    );
}

#[test]
fn valid_negative_operation_responses_are_not_protocol_failures() {
    let rejected = ValidatePlanResponse::from_json(VALIDATE_REJECTED).unwrap();
    let unavailable = ValidatePlanResponse::from_json(VALIDATE_UNAVAILABLE).unwrap();
    assert_eq!(rejected.preflight, PreflightOutcome::Rejected);
    assert_eq!(unavailable.preflight, PreflightOutcome::Unavailable);

    let failed_or_unavailable_execution = validated_execute_response(EXECUTE_UNAVAILABLE);
    assert_eq!(
        failed_or_unavailable_execution.execution,
        ExecutionOutcome::Unavailable
    );

    for fixture in [VALIDATE_REJECTED, VALIDATE_UNAVAILABLE, EXECUTE_UNAVAILABLE] {
        assert!(!fixture.contains("PASS"));
        assert!(!fixture.contains("FAIL"));
        assert!(!fixture.contains("BLOCKED"));
        assert!(!fixture.contains("INDETERMINATE"));
    }
}

#[test]
fn repeated_execute_attempts_recheck_current_state_before_new_side_effects() {
    for (fixture, expected_precondition) in [
        (ALREADY_REALIZED, PRECONDITION_ALREADY_REALIZED),
        (PARTIAL_PRIOR, PRECONDITION_PARTIAL_PRIOR_EXECUTION),
        (
            UNRESOLVED_PREREQUISITE,
            PRECONDITION_UNRESOLVED_PREREQUISITE,
        ),
    ] {
        let response = validated_execute_response(fixture);
        assert_eq!(response.execution, ExecutionOutcome::Rejected);
        assert!(response.execution_batches.is_empty());
        assert!(response
            .action_reports
            .iter()
            .all(|report| report.state == ActionExecutionState::NotStarted));
        assert_eq!(response_precondition(&response), expected_precondition);
    }
}

#[test]
fn successful_execute_response_carries_semantic_evidence_without_lifecycle_classification() {
    let response = validated_execute_response(EXECUTE_EXACT);
    assert_eq!(response.execution, ExecutionOutcome::Completed);
    assert!(!response.effects.is_empty());

    let canonical = response.to_canonical_json().unwrap();
    assert!(!canonical.contains("evaluation_status"));
    assert!(!canonical.contains("comparison"));
}

#[test]
fn raw_failure_cannot_smuggle_lifecycle_or_retry_policy_fields() {
    assert!(matches!(
        ProtocolFailure::from_json(LIFECYCLE_CONFLATION),
        Err(ProtocolFailureError::ForbiddenField(field)) if field == "status"
    ));
    assert!(matches!(
        ProtocolFailure::from_json(RETRYABLE_FIELD),
        Err(ProtocolFailureError::ForbiddenField(field)) if field == "retryable"
    ));

    for fixture in [
        FAILURE_PROTOCOL,
        FAILURE_PUBLIC,
        FAILURE_MISSING,
        FAILURE_BOOTSTRAP,
        FAILURE_REQUEST,
        FAILURE_VALIDATE,
        FAILURE_EXECUTE_NONE,
        FAILURE_EXECUTE_AMBIGUOUS,
    ] {
        let canonical = ProtocolFailure::from_json(fixture)
            .unwrap()
            .to_canonical_json()
            .unwrap();
        assert!(!canonical.contains("PASS"));
        assert!(!canonical.contains("FAIL"));
        assert!(!canonical.contains("BLOCKED"));
        assert!(!canonical.contains("INDETERMINATE"));
        assert!(!canonical.contains("retryable"));
    }
}

#[test]
fn compatibility_and_invalid_request_failures_cannot_claim_possible_side_effects() {
    assert!(ProtocolFailure::new(
        FailureCategory::Compatibility,
        "protocol.example",
        "compatibility failed",
        SideEffectEvidence::MayHaveOccurred,
    )
    .is_err());
    assert!(ProtocolFailure::new(
        FailureCategory::InvalidRequest,
        "protocol.example",
        "request failed",
        SideEffectEvidence::MayHaveOccurred,
    )
    .is_err());

    let ambiguous = ProtocolFailure::operational(
        "execution may have started",
        SideEffectEvidence::MayHaveOccurred,
    )
    .unwrap();
    assert!(ambiguous
        .validate_for(ProtocolOperation::ValidatePlan)
        .is_err());
    ambiguous
        .validate_for(ProtocolOperation::ExecutePlan)
        .unwrap();
}

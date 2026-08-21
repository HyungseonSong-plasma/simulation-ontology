use std::fs;
use std::path::PathBuf;

use sol_adapter_protocol::{
    conservative_execute_side_effect_evidence, ExecutePlanRequest, ExecutePlanResponse,
    ExecutionOutcome, PlanOperation, PlanOperationIdempotency, ProtocolFailure, SideEffectEvidence,
    ValidatePlanRequest,
};
use sol_mock_adapter::{
    mock_plan_operation_idempotency, MockAdapter, MockExecutionState, MockPriorExecutionState,
    MockProtocolFailureState,
};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn fixture(path: &str) -> String {
    fs::read_to_string(fixture_root().join(path)).unwrap()
}

fn validate_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(&fixture(
        "adapter-protocol/0.1/validate-plan-accepted-request.json",
    ))
    .unwrap()
}

fn execute_request() -> ExecutePlanRequest {
    ExecutePlanRequest::from_json(&fixture(
        "adapter-protocol/0.1/execute-plan-thermal-request.json",
    ))
    .unwrap()
}

fn expected_failure(path: &str) -> ProtocolFailure {
    ProtocolFailure::from_json(&fixture(path)).unwrap()
}

fn expected_execute_response(path: &str) -> ExecutePlanResponse {
    let request = execute_request();
    let mut response = ExecutePlanResponse::from_json(&fixture(path)).unwrap();
    response.validate_against(&request).unwrap();
    response
}

#[test]
fn mock_protocol_failures_match_published_failure_algebra() {
    let request = validate_request();

    let compatibility = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::CompatibilityNotEstablished)
        .validate_plan_operation(&request)
        .unwrap_err();
    assert_eq!(
        compatibility,
        expected_failure("adapter-protocol/0.1/protocol-failure-compatibility-missing.json")
    );

    let invalid = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::InvalidRequest)
        .validate_plan_operation(&request)
        .unwrap_err();
    assert_eq!(
        invalid,
        expected_failure("adapter-protocol/0.1/protocol-failure-invalid-request.json")
    );

    let operational = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::ValidateOperational)
        .validate_plan_operation(&request)
        .unwrap_err();
    assert_eq!(
        operational,
        expected_failure("adapter-protocol/0.1/protocol-failure-validate-operational.json")
    );
}

#[test]
fn execute_operational_failures_preserve_side_effect_evidence() {
    let request = execute_request();

    let before = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::ExecuteOperationalBeforeSideEffect)
        .execute_plan_operation(&request)
        .unwrap_err();
    assert_eq!(
        before,
        expected_failure("adapter-protocol/0.1/protocol-failure-execute-before-side-effect.json")
    );
    assert_eq!(before.side_effects, SideEffectEvidence::None);

    let ambiguous = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::ExecuteOperationalAmbiguous)
        .execute_plan_operation(&request)
        .unwrap_err();
    assert_eq!(
        ambiguous,
        expected_failure("adapter-protocol/0.1/protocol-failure-execute-ambiguous.json")
    );
    assert_eq!(ambiguous.side_effects, SideEffectEvidence::MayHaveOccurred);
}

#[test]
fn repeated_execution_states_match_published_precondition_rejections() {
    let request = execute_request();

    for (state, fixture_path) in [
        (
            MockPriorExecutionState::AlreadyRealized,
            "adapter-protocol/0.1/execute-plan-already-realized-response.json",
        ),
        (
            MockPriorExecutionState::PartialPriorExecution,
            "adapter-protocol/0.1/execute-plan-partial-prior-rejection-response.json",
        ),
        (
            MockPriorExecutionState::UnresolvedPrerequisite,
            "adapter-protocol/0.1/execute-plan-unresolved-prerequisite-response.json",
        ),
    ] {
        let actual = MockAdapter::thermal()
            .with_prior_execution_state(state)
            .execute_plan_operation(&request)
            .unwrap();
        assert_eq!(actual, expected_execute_response(fixture_path));
    }
}

#[test]
fn validate_is_stable_but_execute_is_not_deduplicated_by_request_identity() {
    let validate_request = validate_request();
    let adapter = MockAdapter::thermal();
    let first = adapter.validate_plan_operation(&validate_request).unwrap();
    let second = adapter.validate_plan_operation(&validate_request).unwrap();
    assert_eq!(first, second);
    assert_eq!(
        first.to_canonical_json().unwrap(),
        second.to_canonical_json().unwrap()
    );

    assert_eq!(
        mock_plan_operation_idempotency(PlanOperation::ValidatePlan),
        PlanOperationIdempotency::IdempotentForEquivalentInputAndState
    );
    assert_eq!(
        mock_plan_operation_idempotency(PlanOperation::ExecutePlan),
        PlanOperationIdempotency::NonIdempotentByDefault
    );

    let execute_request = execute_request();
    let first_attempt = MockAdapter::thermal()
        .execute_plan_operation(&execute_request)
        .unwrap();
    assert_eq!(first_attempt.execution, ExecutionOutcome::Completed);

    let repeated_attempt = MockAdapter::thermal()
        .with_prior_execution_state(MockPriorExecutionState::AlreadyRealized)
        .execute_plan_operation(&execute_request)
        .unwrap();
    assert_eq!(repeated_attempt.execution, ExecutionOutcome::Rejected);
    assert_ne!(first_attempt, repeated_attempt);
}

#[test]
fn valid_negative_execution_results_remain_normal_responses() {
    let request = execute_request();

    let unavailable = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Unavailable)
        .execute_plan_operation(&request)
        .unwrap();
    assert_eq!(unavailable.execution, ExecutionOutcome::Unavailable);

    let partial = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Partial)
        .execute_plan_operation(&request)
        .unwrap();
    assert_eq!(partial.execution, ExecutionOutcome::Partial);
}

#[test]
fn response_absence_never_becomes_execute_replay_authority() {
    assert_eq!(
        conservative_execute_side_effect_evidence(None),
        SideEffectEvidence::MayHaveOccurred
    );

    let counterexample = fixture("counterexamples/adapter-protocol-response-loss-safe-retry.json");
    assert!(counterexample.contains("no_execute_response"));
    assert!(counterexample.contains("assumed_safe_to_retry"));
}

#[test]
fn protocol_failure_output_contains_no_lifecycle_or_replay_policy_fields() {
    for failure in [
        MockAdapter::thermal()
            .with_protocol_failure_state(MockProtocolFailureState::CompatibilityNotEstablished)
            .validate_plan_operation(&validate_request())
            .unwrap_err(),
        MockAdapter::thermal()
            .with_protocol_failure_state(MockProtocolFailureState::InvalidRequest)
            .validate_plan_operation(&validate_request())
            .unwrap_err(),
        MockAdapter::thermal()
            .with_protocol_failure_state(MockProtocolFailureState::ExecuteOperationalAmbiguous)
            .execute_plan_operation(&execute_request())
            .unwrap_err(),
    ] {
        let canonical = failure.to_canonical_json().unwrap();
        for forbidden in [
            "adapter_protocol_version",
            "retryable",
            "safe_to_retry",
            "idempotency_key",
            "resume_token",
            "evaluation_status",
            "lifecycle_status",
            "PASS",
            "FAIL",
            "BLOCKED",
            "INDETERMINATE",
        ] {
            assert!(
                !canonical.contains(forbidden),
                "forbidden field/state leaked: {forbidden}"
            );
        }
    }
}

#[test]
fn opaque_provenance_is_allowed_but_backend_native_semantic_identity_is_not() {
    let request = execute_request();
    let exact = MockAdapter::thermal()
        .execute_plan_operation(&request)
        .unwrap();
    let provenance = exact.provenance.as_ref().unwrap();
    assert!(provenance
        .opaque_references
        .iter()
        .any(|reference| reference.namespace == "mock.job"));

    let backend_native =
        fixture("counterexamples/adapter-protocol-execution-backend-native-leakage.json");
    assert!(ExecutePlanResponse::from_json(&backend_native).is_err());

    let opaque_identity = fixture(
        "counterexamples/adapter-protocol-execution-opaque-reference-semantic-identity.json",
    );
    let mut response = ExecutePlanResponse::from_json(&opaque_identity).unwrap();
    assert!(response.validate_against(&request).is_err());
}

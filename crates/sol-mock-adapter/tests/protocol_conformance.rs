use sol_adapter_protocol::{
    AdapterDescription, ExecutePlanRequest, ExecutePlanResponse, ExecutionOutcome, PreflightOutcome,
    ProtocolFailure, SideEffectEvidence, ValidatePlanRequest, ValidatePlanResponse,
};
use sol_mock_adapter::{
    MockAdapter, MockExecutionState, MockPreflightState, MockPriorExecutionState,
    MockProtocolFailureState,
};

const PUBLISHED_DESCRIPTION: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/dual-compatible-description.json");
const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const VALIDATE_ACCEPTED: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-response.json");
const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const EXECUTE_EXACT: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-exact-response.json");
const FAILURE_AMBIGUOUS: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/protocol-failure-execute-ambiguous.json");
const ALREADY_REALIZED: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-already-realized-response.json"
);

fn validate_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap()
}

fn execute_request() -> ExecutePlanRequest {
    ExecutePlanRequest::from_json(EXECUTE_REQUEST).unwrap()
}

#[test]
fn published_positive_operation_matrix_runs_through_mock_adapter() {
    let adapter = MockAdapter::thermal();

    let description = adapter.describe_adapter_operation().unwrap();
    let published_description = AdapterDescription::from_json(PUBLISHED_DESCRIPTION).unwrap();
    assert_eq!(
        description.bootstrap.supported_adapter_protocol_versions,
        published_description
            .bootstrap
            .supported_adapter_protocol_versions
    );
    assert_eq!(
        description.bootstrap.supported_public_contract_versions,
        published_description.bootstrap.supported_public_contract_versions
    );
    assert_eq!(description.targets, published_description.targets);

    let validation = adapter.validate_plan_operation(&validate_request()).unwrap();
    assert_eq!(
        validation,
        ValidatePlanResponse::from_json(VALIDATE_ACCEPTED).unwrap()
    );

    let execution = adapter.execute_plan_operation(&execute_request()).unwrap();
    assert_eq!(
        execution,
        ExecutePlanResponse::from_json(EXECUTE_EXACT).unwrap()
    );
}

#[test]
fn adversarial_matrix_keeps_operation_outcomes_failure_and_state_distinct() {
    let validation_request = validate_request();
    let execution_request = execute_request();

    let rejected_preflight = MockAdapter::thermal()
        .with_preflight_state(MockPreflightState::PrerequisiteRejected)
        .validate_plan_operation(&validation_request)
        .unwrap();
    assert_eq!(rejected_preflight.preflight, PreflightOutcome::Rejected);

    let unavailable_preflight = MockAdapter::thermal()
        .with_preflight_state(MockPreflightState::TransientUnavailable)
        .validate_plan_operation(&validation_request)
        .unwrap();
    assert_eq!(unavailable_preflight.preflight, PreflightOutcome::Unavailable);

    let partial_execution = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Partial)
        .execute_plan_operation(&execution_request)
        .unwrap();
    assert_eq!(partial_execution.execution, ExecutionOutcome::Partial);

    let unavailable_execution = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Unavailable)
        .execute_plan_operation(&execution_request)
        .unwrap();
    assert_eq!(unavailable_execution.execution, ExecutionOutcome::Unavailable);

    let ambiguous_failure = MockAdapter::thermal()
        .with_protocol_failure_state(MockProtocolFailureState::ExecuteOperationalAmbiguous)
        .execute_plan_operation(&execution_request)
        .unwrap_err();
    assert_eq!(
        ambiguous_failure,
        ProtocolFailure::from_json(FAILURE_AMBIGUOUS).unwrap()
    );
    assert_eq!(
        ambiguous_failure.side_effects,
        SideEffectEvidence::MayHaveOccurred
    );

    let already_realized = MockAdapter::thermal()
        .with_prior_execution_state(MockPriorExecutionState::AlreadyRealized)
        .execute_plan_operation(&execution_request)
        .unwrap();
    let mut published = ExecutePlanResponse::from_json(ALREADY_REALIZED).unwrap();
    published.validate_against(&execution_request).unwrap();
    assert_eq!(already_realized, published);
}

#[test]
fn alternate_and_parallel_schedules_remain_dependency_safe_reference_behavior() {
    let request = ExecutePlanRequest::from_json(include_str!(
        "../../../fixtures/adapter-protocol/0.1/execute-plan-independent-request.json"
    ))
    .unwrap();

    let alternate = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::AlternateOrder)
        .execute_plan_operation(&request)
        .unwrap();
    let mut expected_alternate = ExecutePlanResponse::from_json(include_str!(
        "../../../fixtures/adapter-protocol/0.1/execute-plan-alternate-order-response.json"
    ))
    .unwrap();
    expected_alternate.validate_against(&request).unwrap();
    assert_eq!(alternate, expected_alternate);

    let parallel = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::ParallelIndependent)
        .execute_plan_operation(&request)
        .unwrap();
    let mut expected_parallel = ExecutePlanResponse::from_json(include_str!(
        "../../../fixtures/adapter-protocol/0.1/execute-plan-parallel-response.json"
    ))
    .unwrap();
    expected_parallel.validate_against(&request).unwrap();
    assert_eq!(parallel, expected_parallel);
}

#[test]
fn conformance_surface_uses_published_dtos_without_transport_runtime() {
    let _: fn(&MockAdapter) -> Result<AdapterDescription, ProtocolFailure> =
        MockAdapter::describe_adapter_operation;
    let _: fn(&MockAdapter, &ValidatePlanRequest) -> Result<ValidatePlanResponse, ProtocolFailure> =
        MockAdapter::validate_plan_operation;
    let _: fn(&MockAdapter, &ExecutePlanRequest) -> Result<ExecutePlanResponse, ProtocolFailure> =
        MockAdapter::execute_plan_operation;

    let cargo = include_str!("../Cargo.toml").to_ascii_lowercase();
    for forbidden_dependency in [
        "jsonrpc", "tokio", "reqwest", "tonic", "zmq", "moose", "comsol", "ansys",
    ] {
        assert!(
            !cargo.contains(forbidden_dependency),
            "MockAdapter conformance introduced forbidden dependency: {forbidden_dependency}"
        );
    }
}

#[test]
fn conformance_output_never_claims_solver_physical_correctness_or_lifecycle_state() {
    let response = MockAdapter::thermal()
        .execute_plan_operation(&execute_request())
        .unwrap()
        .to_canonical_json()
        .unwrap();

    for forbidden in [
        "evaluation_status",
        "lifecycle_status",
        "PASS",
        "FAIL",
        "BLOCKED",
        "INDETERMINATE",
        "physical_correctness",
        "numerical_correctness",
        "jsonrpc",
        "request_id",
    ] {
        assert!(
            !response.contains(forbidden),
            "conformance output leaked non-Protocol authority: {forbidden}"
        );
    }
}

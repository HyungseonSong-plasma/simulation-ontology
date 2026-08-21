use std::fs;
use std::path::PathBuf;

use sol_adapter_protocol::{ExecutePlanRequest, ExecutePlanResponse, ValidatePlanRequest};
use sol_mock_adapter::{MockAdapter, MockExecutionState};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn fixture(path: &str) -> String {
    fs::read_to_string(fixture_root().join(path)).unwrap()
}

fn request(path: &str) -> ExecutePlanRequest {
    ExecutePlanRequest::from_json(&fixture(path)).unwrap()
}

fn expected(path: &str) -> ExecutePlanResponse {
    ExecutePlanResponse::from_json(&fixture(path)).unwrap()
}

#[test]
fn exact_thermal_execution_matches_published_reference_fixture() {
    let request = request("adapter-protocol/0.1/execute-plan-thermal-request.json");
    let response = MockAdapter::thermal().execute_plan(&request).unwrap();

    assert_eq!(
        response,
        expected("adapter-protocol/0.1/execute-plan-exact-response.json")
    );
}

#[test]
fn degraded_and_unsupported_are_realization_evidence_not_execution_rejection() {
    let request = request("adapter-protocol/0.1/execute-plan-thermal-request.json");

    let degraded = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Degraded)
        .execute_plan(&request)
        .unwrap();
    assert_eq!(
        degraded,
        expected("adapter-protocol/0.1/execute-plan-degraded-response.json")
    );

    let unsupported = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Unsupported)
        .execute_plan(&request)
        .unwrap();
    assert_eq!(
        unsupported,
        expected("adapter-protocol/0.1/execute-plan-unsupported-response.json")
    );
}

#[test]
fn partial_execution_preserves_terminal_action_coverage_and_effect_union() {
    let request = request("adapter-protocol/0.1/execute-plan-thermal-request.json");
    let response = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Partial)
        .execute_plan(&request)
        .unwrap();

    assert_eq!(
        response,
        expected("adapter-protocol/0.1/execute-plan-partial-response.json")
    );
}

#[test]
fn execute_time_unavailability_is_authoritative() {
    let request = request("adapter-protocol/0.1/execute-plan-thermal-request.json");
    let response = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::Unavailable)
        .execute_plan(&request)
        .unwrap();

    assert_eq!(
        response,
        expected("adapter-protocol/0.1/execute-plan-unavailable-response.json")
    );
}

#[test]
fn prior_accepted_preflight_does_not_authorize_changed_execute_request() {
    let adapter =
        MockAdapter::thermal().with_execution_state(MockExecutionState::AuthoritativeRejected);
    let preflight = ValidatePlanRequest::from_json(&fixture(
        "adapter-protocol/0.1/validate-plan-accepted-request.json",
    ))
    .unwrap();
    assert_eq!(
        adapter.validate_plan(&preflight).unwrap().preflight,
        sol_adapter_protocol::PreflightOutcome::Accepted
    );

    let changed = request("adapter-protocol/0.1/execute-plan-changed-request.json");
    let response = adapter.execute_plan(&changed).unwrap();
    assert_eq!(
        response,
        expected("adapter-protocol/0.1/execute-plan-authoritative-rejection-response.json")
    );
}

#[test]
fn independent_actions_may_use_an_alternate_valid_topological_order() {
    let request = request("adapter-protocol/0.1/execute-plan-independent-request.json");
    let response = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::AlternateOrder)
        .execute_plan(&request)
        .unwrap();

    assert_eq!(
        response,
        expected("adapter-protocol/0.1/execute-plan-alternate-order-response.json")
    );
}

#[test]
fn independent_actions_may_execute_in_the_same_dependency_safe_batch() {
    let request = request("adapter-protocol/0.1/execute-plan-independent-request.json");
    let response = MockAdapter::thermal()
        .with_execution_state(MockExecutionState::ParallelIndependent)
        .execute_plan(&request)
        .unwrap();

    assert_eq!(
        response,
        expected("adapter-protocol/0.1/execute-plan-parallel-response.json")
    );
}

#[test]
fn canonical_response_validation_keeps_opaque_provenance_non_semantic() {
    let request = request("adapter-protocol/0.1/execute-plan-thermal-request.json");
    let response = MockAdapter::thermal().execute_plan(&request).unwrap();
    let provenance = response.provenance.unwrap();

    assert_eq!(provenance.producer, "adapter.mock_thermal");
    assert_eq!(provenance.opaque_references[0].namespace, "mock.job");
    assert_eq!(provenance.opaque_references[0].reference, "job-42");
}

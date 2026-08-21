use std::fs;
use std::path::PathBuf;

use sol_adapter_protocol::{ValidatePlanRequest, ValidatePlanResponse};
use sol_mock_adapter::{MockAdapter, MockPreflightState};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

fn fixture(path: &str) -> String {
    fs::read_to_string(fixture_root().join(path)).unwrap()
}

fn accepted_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(&fixture(
        "adapter-protocol/0.1/validate-plan-accepted-request.json",
    ))
    .unwrap()
}

fn expected_response(path: &str) -> ValidatePlanResponse {
    ValidatePlanResponse::from_json(&fixture(path)).unwrap()
}

#[test]
fn accepted_thermal_preflight_matches_published_fixture() {
    let adapter = MockAdapter::thermal();
    let response = adapter.validate_plan(&accepted_request()).unwrap();
    let expected = expected_response("adapter-protocol/0.1/validate-plan-accepted-response.json");

    assert_eq!(response, expected);
}

#[test]
fn target_mismatch_is_deterministic_and_uses_published_diagnostics() {
    let adapter = MockAdapter::thermal();
    let mut request = accepted_request();
    request.target.target = "comsol".to_owned();
    request.target.required_capabilities = vec!["thermal.solve".to_owned()];

    let response = adapter.validate_plan(&request).unwrap();
    let expected =
        expected_response("counterexamples/adapter-protocol-target-mismatch-response.json");

    assert_eq!(response, expected);
}

#[test]
fn missing_capability_is_rejected_with_canonical_context() {
    let adapter = MockAdapter::thermal();
    let mut request = accepted_request();
    request.target.required_capabilities = vec!["thermal.radiation".to_owned()];

    let response = adapter.validate_plan(&request).unwrap();
    let expected =
        expected_response("counterexamples/adapter-protocol-missing-capability-response.json");

    assert_eq!(response, expected);
}

#[test]
fn unsupported_plan_action_is_rejected_without_changing_plan_semantics() {
    let adapter = MockAdapter::thermal();
    let mut request = accepted_request();
    request.plan.actions[2].id = "thermal.radiation".to_owned();

    let response = adapter.validate_plan(&request).unwrap();
    let expected =
        expected_response("counterexamples/adapter-protocol-unsupported-action-response.json");

    assert_eq!(response, expected);
}

#[test]
fn rejected_prerequisite_matches_published_preflight_shape() {
    let adapter =
        MockAdapter::thermal().with_preflight_state(MockPreflightState::PrerequisiteRejected);

    let response = adapter.validate_plan(&accepted_request()).unwrap();
    let expected =
        expected_response("counterexamples/adapter-protocol-precondition-rejected-response.json");

    assert_eq!(response, expected);
}

#[test]
fn transient_unavailability_matches_published_preflight_shape() {
    let adapter =
        MockAdapter::thermal().with_preflight_state(MockPreflightState::TransientUnavailable);

    let response = adapter.validate_plan(&accepted_request()).unwrap();
    let expected =
        expected_response("counterexamples/adapter-protocol-transient-unavailable-response.json");

    assert_eq!(response, expected);
}

#[test]
fn equivalent_request_and_equivalent_state_are_idempotent() {
    let adapter = MockAdapter::thermal();
    let request = accepted_request();

    let first = adapter.validate_plan(&request).unwrap();
    let second = adapter.validate_plan(&request).unwrap();

    assert_eq!(first, second);
    assert_eq!(
        first.to_canonical_json().unwrap(),
        second.to_canonical_json().unwrap()
    );
}

#[test]
fn relevant_mock_state_may_change_advisory_result_without_durable_authority() {
    let request = accepted_request();
    let ready = MockAdapter::thermal().validate_plan(&request).unwrap();
    let unavailable = MockAdapter::thermal()
        .with_preflight_state(MockPreflightState::TransientUnavailable)
        .validate_plan(&request)
        .unwrap();

    assert_ne!(ready, unavailable);
}

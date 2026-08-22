use sol_adapter_protocol::{
    ExecutePlanRequestV02, ExecutePlanResponse, ExecutePlanResponseV02, RealizationResponseError,
    ValidatePlanResponse, ValidatePlanResponseV02,
};
use std::fs;
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/adapter-protocol/0.2")
        .join(name);
    fs::read_to_string(path).unwrap()
}

#[test]
fn v02_preflight_response_is_explicit_and_not_v01() {
    let response =
        ValidatePlanResponseV02::from_json(&fixture("validate-plan-accepted-response.json"))
            .unwrap();
    assert_eq!(response.adapter_protocol_version, "0.2");
    assert!(ValidatePlanResponse::from_json(&fixture("validate-plan-accepted-response.json")).is_err());
}

#[test]
fn v02_exact_thermal_execution_validates_against_full_realization_request() {
    let request =
        ExecutePlanRequestV02::from_json(&fixture("thermal-realization-request.json")).unwrap();
    let mut response =
        ExecutePlanResponseV02::from_json(&fixture("execute-plan-exact-response.json")).unwrap();
    response.validate_against(&request).unwrap();

    let canonical = response.to_canonical_json().unwrap();
    assert!(canonical.contains("\"adapter_protocol_version\":\"0.2\""));
    assert!(canonical.contains("property.thermal_conductivity"));
    assert!(ExecutePlanResponse::from_json(&canonical).is_err());
}

#[test]
fn inherited_dependency_and_action_coverage_rules_still_apply() {
    let request =
        ExecutePlanRequestV02::from_json(&fixture("thermal-realization-request.json")).unwrap();
    let mut value: serde_json::Value =
        serde_json::from_str(&fixture("execute-plan-exact-response.json")).unwrap();
    value["execution_batches"] = serde_json::json!([
        ["thermal.material"],
        ["thermal.domain"],
        ["thermal.solve"]
    ]);
    let mut response = ExecutePlanResponseV02::from_json(&serde_json::to_string(&value).unwrap())
        .unwrap();
    assert!(matches!(
        response.validate_against(&request),
        Err(RealizationResponseError::InheritedExecutionInvariant(_))
    ));
}

#[test]
fn opaque_provenance_cannot_reuse_realization_spec_semantic_identity() {
    let request =
        ExecutePlanRequestV02::from_json(&fixture("thermal-realization-request.json")).unwrap();
    let mut value: serde_json::Value =
        serde_json::from_str(&fixture("execute-plan-exact-response.json")).unwrap();
    value["provenance"]["opaque_references"][0]["reference"] =
        serde_json::json!("unit.kelvin");
    let mut response = ExecutePlanResponseV02::from_json(&serde_json::to_string(&value).unwrap())
        .unwrap();
    assert!(matches!(
        response.validate_against(&request),
        Err(RealizationResponseError::OpaqueReferenceUsedAsSemanticIdentity(reference))
            if reference == "unit.kelvin"
    ));
}

#[test]
fn v02_responses_cannot_create_replay_or_execution_authority() {
    let mut preflight: serde_json::Value =
        serde_json::from_str(&fixture("validate-plan-accepted-response.json")).unwrap();
    preflight["validation_token"] = serde_json::json!("durable-token");
    assert!(matches!(
        ValidatePlanResponseV02::from_json(&serde_json::to_string(&preflight).unwrap()),
        Err(RealizationResponseError::ForbiddenField(field)) if field == "validation_token"
    ));

    let mut execution: serde_json::Value =
        serde_json::from_str(&fixture("execute-plan-exact-response.json")).unwrap();
    execution["replay_authority"] = serde_json::json!(true);
    assert!(matches!(
        ExecutePlanResponseV02::from_json(&serde_json::to_string(&execution).unwrap()),
        Err(RealizationResponseError::ForbiddenField(field)) if field == "replay_authority"
    ));
}

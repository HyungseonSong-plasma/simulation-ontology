use serde_json::Value;
use sol_adapter_protocol::{
    AdapterDescription, ExecutePlanRequest, ExecutePlanResponse, ExecutionOutcome, PreflightOutcome,
    ProtocolFailure, ValidatePlanRequest, ValidatePlanResponse,
};

const ADDITIVE_DESCRIPTION: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/schema-additive-extension-description.json"
);
const SEMANTIC_REDEFINITION: &str = include_str!(
    "../../../fixtures/counterexamples/adapter-protocol-semantic-redefinition-same-syntax.json"
);
const PREFLIGHT_ACCEPTED: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const CHANGED_EXECUTE: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-changed-request.json");
const AUTHORITATIVE_REJECTION: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/execute-plan-authoritative-rejection-response.json"
);
const COMPATIBILITY_FAILURE: &str = include_str!(
    "../../../fixtures/adapter-protocol/0.1/protocol-failure-adapter-protocol-incompatible.json"
);

#[test]
fn additive_unknown_fields_remain_extensions_without_redefining_known_semantics() {
    let description = AdapterDescription::from_json(ADDITIVE_DESCRIPTION).unwrap();
    assert!(description.extensions.contains_key("future_descriptor_metadata"));
    assert!(description.targets[0]
        .extensions
        .contains_key("future_target_metadata"));

    let canonical = description.to_canonical_json().unwrap();
    assert!(canonical.contains("future_descriptor_metadata"));
    assert_eq!(description.bootstrap.supported_adapter_protocol_versions, Some(vec!["0.1".into()]));
}

#[test]
fn unchanged_syntax_cannot_redefine_advisory_preflight_as_execution_authority() {
    let counterexample: Value = serde_json::from_str(SEMANTIC_REDEFINITION).unwrap();
    let syntax = serde_json::to_string(&counterexample["syntax_fixture"]).unwrap();
    let response = ValidatePlanResponse::from_json(&syntax).unwrap();
    assert_eq!(response.preflight, PreflightOutcome::Accepted);
    assert_ne!(
        counterexample["published_meaning"],
        counterexample["incompatible_redefinition"]
    );

    let validated = ValidatePlanRequest::from_json(PREFLIGHT_ACCEPTED).unwrap();
    let execute = ExecutePlanRequest::from_json(CHANGED_EXECUTE).unwrap();
    assert_ne!(
        validated.canonical_plan_identity().unwrap(),
        execute.canonical_plan_identity().unwrap()
    );

    let mut execution = ExecutePlanResponse::from_json(AUTHORITATIVE_REJECTION).unwrap();
    execution.validate_against(&execute).unwrap();
    assert_eq!(execution.execution, ExecutionOutcome::Rejected);
}

#[test]
fn compatibility_failure_is_protocol_data_without_negotiated_version_or_lifecycle_state() {
    let failure = ProtocolFailure::from_json(COMPATIBILITY_FAILURE).unwrap();
    let canonical = failure.to_canonical_json().unwrap();
    assert!(!canonical.contains("adapter_protocol_version"));
    assert!(!canonical.contains("evaluation_status"));
    assert!(!canonical.contains("BLOCKED"));
    assert!(!canonical.contains("INDETERMINATE"));
}

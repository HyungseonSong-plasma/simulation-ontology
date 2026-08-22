use sol_adapter_protocol::{
    assess_compatibility, AdapterBootstrap, CompatibilityOutcome, CompatibilitySupport,
    ExecutePlanRequestV02, RealizationRequestError, ValidatePlanRequest, ValidatePlanRequestV02,
    ADAPTER_PROTOCOL_VERSION, ADAPTER_PROTOCOL_VERSION_0_2,
};
use std::fs;
use std::path::PathBuf;

fn fixture() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/adapter-protocol/0.2/thermal-realization-request.json");
    fs::read_to_string(path).unwrap()
}

fn fixture_value() -> serde_json::Value {
    serde_json::from_str(&fixture()).unwrap()
}

#[test]
fn protocol_v02_is_explicit_and_v01_default_is_unchanged() {
    assert_eq!(ADAPTER_PROTOCOL_VERSION, "0.1");
    assert_eq!(ADAPTER_PROTOCOL_VERSION_0_2, "0.2");

    let request = ValidatePlanRequestV02::from_json(&fixture()).unwrap();
    assert_eq!(request.adapter_protocol_version, "0.2");

    // The pre-M0.8 request parser remains a Protocol/Public Contract 0.1 surface.
    assert!(ValidatePlanRequest::from_json(&fixture()).is_err());
}

#[test]
fn validate_and_execute_require_the_same_explicit_realization_payload() {
    let validate = ValidatePlanRequestV02::from_json(&fixture()).unwrap();
    let execute = ExecutePlanRequestV02::from_json(&fixture()).unwrap();

    assert_eq!(
        validate.canonical_plan_identity().unwrap(),
        execute.canonical_plan_identity().unwrap()
    );
    assert_eq!(
        validate.canonical_realization_identity().unwrap(),
        execute.canonical_realization_identity().unwrap()
    );
    assert!(validate
        .to_canonical_json()
        .unwrap()
        .contains("\"realization_spec\""));
    assert!(execute
        .to_canonical_json()
        .unwrap()
        .contains("\"realization_spec\""));
}

#[test]
fn realization_spec_is_required_for_both_operations() {
    let mut missing = fixture_value();
    missing.as_object_mut().unwrap().remove("realization_spec");
    let json = serde_json::to_string(&missing).unwrap();

    assert!(matches!(
        ValidatePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::InvalidRequest(_))
    ));
    assert!(matches!(
        ExecutePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::InvalidRequest(_))
    ));
}

#[test]
fn mixed_protocol_or_public_contract_versions_are_rejected() {
    let mut protocol = fixture_value();
    protocol["adapter_protocol_version"] = serde_json::json!("0.1");
    assert!(matches!(
        ValidatePlanRequestV02::from_json(&serde_json::to_string(&protocol).unwrap()),
        Err(RealizationRequestError::UnsupportedProtocolVersion(version)) if version == "0.1"
    ));

    let mut mixed_public = fixture_value();
    mixed_public["target"]["public_contract_version"] = serde_json::json!("0.1");
    assert!(matches!(
        ValidatePlanRequestV02::from_json(&serde_json::to_string(&mixed_public).unwrap()),
        Err(RealizationRequestError::IncoherentPublicContractVersions { .. })
    ));

    let mut all_old_public = fixture_value();
    all_old_public["target"]["public_contract_version"] = serde_json::json!("0.1");
    all_old_public["plan"]["public_contract_version"] = serde_json::json!("0.1");
    all_old_public["realization_spec"]["public_contract_version"] = serde_json::json!("0.1");
    assert!(matches!(
        ValidatePlanRequestV02::from_json(&serde_json::to_string(&all_old_public).unwrap()),
        Err(RealizationRequestError::UnsupportedPublicContractVersion(version)) if version == "0.1"
    ));
}

#[test]
fn plan_realization_referential_integrity_is_checked_before_adapter_use() {
    let mut mismatch = fixture_value();
    mismatch["realization_spec"]["action_bindings"]
        .as_array_mut()
        .unwrap()
        .retain(|binding| binding["action_id"] != "thermal.material");

    assert!(matches!(
        ExecutePlanRequestV02::from_json(&serde_json::to_string(&mismatch).unwrap()),
        Err(RealizationRequestError::PlanRealizationMismatch(detail))
            if detail.contains("thermal.material")
    ));
}

#[test]
fn protocol_v02_adds_no_durable_authority_or_replay_token() {
    let mut forbidden = fixture_value();
    forbidden["replay_authority"] = serde_json::json!("again");
    assert!(matches!(
        ExecutePlanRequestV02::from_json(&serde_json::to_string(&forbidden).unwrap()),
        Err(RealizationRequestError::ForbiddenField(field)) if field == "replay_authority"
    ));
}

#[test]
fn v02_compatibility_requires_both_explicit_axes() {
    let exact_v02_adapter = AdapterBootstrap {
        adapter_id: "sol.adapter.moose".to_owned(),
        adapter_version: "0.1.0".to_owned(),
        supported_adapter_protocol_versions: Some(vec!["0.2".to_owned()]),
        supported_public_contract_versions: Some(vec!["0.2".to_owned()]),
        extensions: Default::default(),
    };
    let assessment =
        assess_compatibility(&CompatibilitySupport::realization_v02(), &exact_v02_adapter).unwrap();
    assert_eq!(assessment.overall, CompatibilityOutcome::Compatible);
    assert_eq!(assessment.adapter_protocol.selected_version.as_deref(), Some("0.2"));
    assert_eq!(assessment.public_contract.selected_version.as_deref(), Some("0.2"));

    let wrong_public_axis = AdapterBootstrap {
        supported_public_contract_versions: Some(vec!["0.1".to_owned()]),
        ..exact_v02_adapter
    };
    let assessment =
        assess_compatibility(&CompatibilitySupport::realization_v02(), &wrong_public_axis).unwrap();
    assert_eq!(assessment.adapter_protocol.outcome, CompatibilityOutcome::Compatible);
    assert_eq!(assessment.public_contract.outcome, CompatibilityOutcome::Incompatible);
    assert_eq!(assessment.overall, CompatibilityOutcome::Incompatible);
}

use sol_adapter_conformance::{
    BackendValidationScope, ConformanceCaseId, ConformanceCaseRecord, ConformanceCaseResult,
    ConformanceEvidence, ConformanceReport, ConformanceScope, PublishedContract,
};
use sol_adapter_protocol::{
    assess_compatibility, parse_bootstrap, CompatibilityOutcome, CompatibilitySupport,
    ExecutePlanRequestV02, ExecutePlanResponseV02, ProtocolOperation, ValidatePlanRequestV02,
    ValidatePlanResponseV02,
};
use sol_public_contract::{MappingPlanDtoV02, RealizationSpecDtoV02};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

fn repo_fixture(path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures")
        .join(path);
    fs::read_to_string(path).unwrap()
}

fn protocol_request_value() -> serde_json::Value {
    serde_json::from_str(&repo_fixture(
        "adapter-protocol/0.2/thermal-realization-request.json",
    ))
    .unwrap()
}

#[test]
fn published_profile_has_the_complete_required_case_set() {
    let profile: serde_json::Value = serde_json::from_str(&repo_fixture(
        "adapter-conformance/0.2/realization-profile.json",
    ))
    .unwrap();
    assert_eq!(profile["profile_id"], "sol.realization-conformance.0.2");
    assert_eq!(profile["adapter_protocol_version"], "0.2");
    assert_eq!(profile["public_contract_version"], "0.2");
    assert_eq!(
        profile["backend_validation_scope"],
        "not_assessed_by_conformance"
    );

    let ids = profile["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| case["id"].as_str().unwrap())
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
        "realization-v02.distinguishability",
        "realization-v02.dual-axis-compatibility",
        "realization-v02.execute-roundtrip",
        "realization-v02.hidden-semantics-rejected",
        "realization-v02.mixed-version-rejected",
        "realization-v02.no-physical-correctness-claim",
        "realization-v02.referential-integrity",
        "realization-v02.required-realization-spec",
        "realization-v02.validate-roundtrip",
    ]);
    assert_eq!(ids, expected);
}

#[test]
fn required_realization_spec_and_referential_integrity_are_executable_rules() {
    let mut missing = protocol_request_value();
    missing.as_object_mut().unwrap().remove("realization_spec");
    assert!(ValidatePlanRequestV02::from_json(&serde_json::to_string(&missing).unwrap()).is_err());
    assert!(ExecutePlanRequestV02::from_json(&serde_json::to_string(&missing).unwrap()).is_err());

    let mut broken_binding = protocol_request_value();
    broken_binding["realization_spec"]["action_bindings"]
        .as_array_mut()
        .unwrap()
        .retain(|binding| binding["action_id"] != "thermal.material");
    assert!(
        ValidatePlanRequestV02::from_json(&serde_json::to_string(&broken_binding).unwrap())
            .is_err()
    );
}

#[test]
fn same_plan_different_realization_specs_are_distinguishable() {
    let plan = MappingPlanDtoV02::from_json(&repo_fixture(
        "public-contract/0.2/thermal-mapping-plan.json",
    ))
    .unwrap();
    let spec_a = RealizationSpecDtoV02::from_json(&repo_fixture(
        "public-contract/0.2/thermal-realization-spec.json",
    ))
    .unwrap();
    let spec_b = RealizationSpecDtoV02::from_json(&repo_fixture(
        "public-contract/0.2/thermal-realization-spec-alternate-values.json",
    ))
    .unwrap();

    spec_a.validate_against_plan(&plan).unwrap();
    spec_b.validate_against_plan(&plan).unwrap();
    assert_ne!(
        spec_a.to_canonical_json().unwrap(),
        spec_b.to_canonical_json().unwrap()
    );
    assert_eq!(
        plan.to_canonical_json().unwrap(),
        MappingPlanDtoV02::from_json(&repo_fixture(
            "public-contract/0.2/thermal-mapping-plan.json"
        ))
        .unwrap()
        .to_canonical_json()
        .unwrap()
    );
}

#[test]
fn hidden_plan_semantics_and_mixed_versions_are_rejected() {
    let mut hidden = protocol_request_value();
    hidden["plan"]["actions"][2]["physics"] = serde_json::json!({"equation": "heat"});
    assert!(ValidatePlanRequestV02::from_json(&serde_json::to_string(&hidden).unwrap()).is_err());

    let mut wrong_protocol = protocol_request_value();
    wrong_protocol["adapter_protocol_version"] = serde_json::json!("0.1");
    assert!(
        ValidatePlanRequestV02::from_json(&serde_json::to_string(&wrong_protocol).unwrap())
            .is_err()
    );

    let mut mixed_public = protocol_request_value();
    mixed_public["target"]["public_contract_version"] = serde_json::json!("0.1");
    assert!(
        ExecutePlanRequestV02::from_json(&serde_json::to_string(&mixed_public).unwrap()).is_err()
    );
}

#[test]
fn dual_axis_v02_compatibility_is_required() {
    let bootstrap = parse_bootstrap(&repo_fixture(
        "adapter-protocol/0.2/realization-compatible-bootstrap.json",
    ))
    .unwrap();
    let compatible =
        assess_compatibility(&CompatibilitySupport::realization_v02(), &bootstrap).unwrap();
    assert_eq!(compatible.overall, CompatibilityOutcome::Compatible);
    assert_eq!(
        compatible.adapter_protocol.selected_version.as_deref(),
        Some("0.2")
    );
    assert_eq!(
        compatible.public_contract.selected_version.as_deref(),
        Some("0.2")
    );

    let mut wrong_public = bootstrap;
    wrong_public.supported_public_contract_versions = Some(vec!["0.1".to_owned()]);
    let incompatible =
        assess_compatibility(&CompatibilitySupport::realization_v02(), &wrong_public).unwrap();
    assert_eq!(incompatible.overall, CompatibilityOutcome::Incompatible);
}

#[test]
fn published_v02_validate_and_execute_roundtrips_are_conformant() {
    let validate_request = ValidatePlanRequestV02::from_json(&repo_fixture(
        "adapter-protocol/0.2/thermal-realization-request.json",
    ))
    .unwrap();
    let validate_response = ValidatePlanResponseV02::from_json(&repo_fixture(
        "adapter-protocol/0.2/validate-plan-accepted-response.json",
    ))
    .unwrap();
    assert_eq!(validate_request.adapter_protocol_version, "0.2");
    assert_eq!(validate_response.adapter_protocol_version, "0.2");

    let execute_request = ExecutePlanRequestV02::from_json(&repo_fixture(
        "adapter-protocol/0.2/thermal-realization-request.json",
    ))
    .unwrap();
    let mut execute_response = ExecutePlanResponseV02::from_json(&repo_fixture(
        "adapter-protocol/0.2/execute-plan-exact-response.json",
    ))
    .unwrap();
    execute_response.validate_against(&execute_request).unwrap();

    let report = ConformanceReport::new(vec![
        ConformanceCaseRecord::new(
            ConformanceCaseId::new("realization-v02.validate-roundtrip").unwrap(),
            ConformanceScope::OperationSemantics(ProtocolOperation::ValidatePlan),
            ConformanceCaseResult::Conformant(
                ConformanceEvidence::new("Protocol 0.2 thermal validate roundtrip accepted")
                    .unwrap(),
            ),
        ),
        ConformanceCaseRecord::new(
            ConformanceCaseId::new("realization-v02.execute-roundtrip").unwrap(),
            ConformanceScope::OperationSemantics(ProtocolOperation::ExecutePlan),
            ConformanceCaseResult::Conformant(
                ConformanceEvidence::new("Protocol 0.2 thermal execute roundtrip accepted")
                    .unwrap(),
            ),
        ),
        ConformanceCaseRecord::new(
            ConformanceCaseId::new("realization-v02.public-contract").unwrap(),
            ConformanceScope::Fixture(PublishedContract::PublicContract),
            ConformanceCaseResult::Conformant(
                ConformanceEvidence::new("Public Contract 0.2 realization fixtures accepted")
                    .unwrap(),
            ),
        ),
    ])
    .unwrap();
    assert_eq!(
        report.backend_validation_scope(),
        BackendValidationScope::NotAssessedByConformance
    );
}

use sol_public_contract::{MappingPlanDtoV02, RealizationSpecDtoV02, RealizationSpecError};
use std::fs;
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/public-contract/0.2")
        .join(name);
    fs::read_to_string(path).unwrap()
}

fn positive_plan() -> MappingPlanDtoV02 {
    MappingPlanDtoV02::from_json(&fixture("thermal-mapping-plan.json")).unwrap()
}

fn positive_spec_value() -> serde_json::Value {
    serde_json::from_str(&fixture("thermal-realization-spec.json")).unwrap()
}

fn parse_spec(value: &serde_json::Value) -> Result<RealizationSpecDtoV02, RealizationSpecError> {
    RealizationSpecDtoV02::from_json(&serde_json::to_string(value).unwrap())
}

#[test]
fn thermal_realization_spec_is_sufficient_and_plan_bound() {
    let plan = positive_plan();
    let spec = RealizationSpecDtoV02::from_json(&fixture("thermal-realization-spec.json")).unwrap();
    spec.validate_against_plan(&plan).unwrap();

    let canonical = spec.to_canonical_json().unwrap();
    for required in [
        "ThermalTransport",
        "Field",
        "SteadyHeatEquation",
        "FourierLaw",
        "ThermalConductivity",
        "DirichletTemperatureBoundaryCondition",
        "StationaryAnalysis",
        "MaximumTemperature",
        "thermal.conductivity",
        "thermal.temperature",
        "unit.watt_per_meter_kelvin",
        "unit.kelvin",
        "scope.main_domain",
        "scope.hot_wall",
    ] {
        assert!(canonical.contains(required), "missing {required}");
    }
}

#[test]
fn same_plan_different_realization_values_remain_distinguishable() {
    let plan = positive_plan();
    let plan_a = plan.to_canonical_json().unwrap();
    let plan_b = positive_plan().to_canonical_json().unwrap();
    assert_eq!(plan_a, plan_b);

    let spec_a =
        RealizationSpecDtoV02::from_json(&fixture("thermal-realization-spec.json")).unwrap();
    let spec_b = RealizationSpecDtoV02::from_json(&fixture(
        "thermal-realization-spec-alternate-values.json",
    ))
    .unwrap();
    spec_a.validate_against_plan(&plan).unwrap();
    spec_b.validate_against_plan(&plan).unwrap();
    assert_ne!(
        spec_a.to_canonical_json().unwrap(),
        spec_b.to_canonical_json().unwrap()
    );
}

#[test]
fn missing_unknown_and_duplicate_action_bindings_are_rejected() {
    let plan = positive_plan();

    let mut missing = positive_spec_value();
    missing["action_bindings"]
        .as_array_mut()
        .unwrap()
        .retain(|binding| binding["action_id"] != "thermal.material");
    let missing = parse_spec(&missing).unwrap();
    assert!(matches!(
        missing.validate_against_plan(&plan),
        Err(RealizationSpecError::MissingActionBinding(action)) if action == "thermal.material"
    ));

    let mut unknown = positive_spec_value();
    let mut unknown_binding = unknown["action_bindings"][0].clone();
    unknown_binding["action_id"] = serde_json::json!("thermal.unknown");
    unknown["action_bindings"]
        .as_array_mut()
        .unwrap()
        .push(unknown_binding);
    let unknown = parse_spec(&unknown).unwrap();
    assert!(matches!(
        unknown.validate_against_plan(&plan),
        Err(RealizationSpecError::UnknownActionBinding(action)) if action == "thermal.unknown"
    ));

    let mut duplicate = positive_spec_value();
    let duplicate_binding = duplicate["action_bindings"][0].clone();
    duplicate["action_bindings"]
        .as_array_mut()
        .unwrap()
        .push(duplicate_binding);
    assert!(matches!(
        parse_spec(&duplicate),
        Err(RealizationSpecError::DuplicateActionBinding(_))
    ));
}

#[test]
fn unresolved_subject_and_scope_are_rejected() {
    let mut subject = positive_spec_value();
    subject["action_bindings"][0]["subjects"][0]["id"] = serde_json::json!("thermal.missing");
    assert!(matches!(
        parse_spec(&subject),
        Err(RealizationSpecError::UnresolvedReference(reference)) if reference == "thermal.missing"
    ));

    let mut scope = positive_spec_value();
    scope["action_bindings"][0]["scopes"][0] = serde_json::json!("scope.missing");
    assert!(matches!(
        parse_spec(&scope),
        Err(RealizationSpecError::UnresolvedScope(reference)) if reference == "scope.missing"
    ));
}

#[test]
fn malformed_units_and_backend_native_identity_are_rejected() {
    let mut unit = positive_spec_value();
    let entities = unit["entities"].as_array_mut().unwrap();
    let conductivity = entities
        .iter_mut()
        .find(|entity| entity["id"] == "property.thermal_conductivity")
        .unwrap();
    conductivity["parameters"][0]["quantity"]["unit"] = serde_json::json!("W/m/K");
    assert!(matches!(
        parse_spec(&unit),
        Err(RealizationSpecError::InvalidIdentifier {
            field: "quantity unit",
            ..
        })
    ));

    let mut native = positive_spec_value();
    native["evidence"] = serde_json::json!({
        "nested": {"backend_native_id": "moose:Kernel/1"}
    });
    assert!(matches!(
        parse_spec(&native),
        Err(RealizationSpecError::BackendNativeLeakage(field)) if field == "backend_native_id"
    ));
}

#[test]
fn plan_action_cannot_hide_realization_semantics_in_extensions() {
    let mut plan: serde_json::Value =
        serde_json::from_str(&fixture("thermal-mapping-plan.json")).unwrap();
    plan["actions"][2]["physics"] = serde_json::json!({"equation": "heat"});
    assert!(matches!(
        MappingPlanDtoV02::from_json(&serde_json::to_string(&plan).unwrap()),
        Err(RealizationSpecError::HiddenRealizationExtension(field)) if field == "physics"
    ));
}

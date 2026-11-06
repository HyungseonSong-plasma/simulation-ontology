//! SOL #172: characterization only. This does not authorize plasma capability,
//! dimensional correctness, a new HIT operation, or scientific acceptance.
use serde_json::{json, Value};
use sol_adapter_protocol::{
    ExecutePlanRequestV02, RealizationRequestError, ValidatePlanRequestV02,
};

fn request_value() -> Value {
    serde_json::from_str(include_str!(
        "../../../fixtures/adapter-protocol/0.2/thermal-realization-request.json"
    ))
    .unwrap()
}

fn parse_both(value: &Value) -> (ValidatePlanRequestV02, ExecutePlanRequestV02) {
    let json = serde_json::to_string(value).unwrap();
    (
        ValidatePlanRequestV02::from_json(&json).unwrap(),
        ExecutePlanRequestV02::from_json(&json).unwrap(),
    )
}

fn set_conductivity_unit(request: &mut Value, unit: &str) {
    let material = request["realization_spec"]["entities"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|entity| entity["id"] == "property.thermal_conductivity")
        .unwrap();
    material["parameters"][0]["quantity"]["unit"] = json!(unit);
}

#[test]
fn published_v02_thermal_request_remains_valid_for_both_operations() {
    let (validate, execute) = parse_both(&request_value());
    assert_eq!(validate.adapter_protocol_version, "0.2");
    assert_eq!(
        validate.canonical_realization_identity().unwrap(),
        execute.canonical_realization_identity().unwrap()
    );
}

#[test]
fn valid_reference_with_incorrect_dimension_is_not_rejected_by_v02_shape_checks() {
    // The published thermal conductivity needs W/(m K), not temperature.
    // This intentionally valid canonical reference is dimensionally incorrect.
    // Successful parsing here MUST NOT be reported as scientific validation.
    let mut request = request_value();
    set_conductivity_unit(&mut request, "unit.kelvin");

    let (validate, execute) = parse_both(&request);
    assert_eq!(
        validate
            .realization_spec
            .entities
            .iter()
            .find(|entity| entity.id == "property.thermal_conductivity")
            .unwrap()
            .parameters[0]
            .quantity
            .unit,
        "unit.kelvin"
    );
    assert_eq!(
        validate.canonical_realization_identity().unwrap(),
        execute.canonical_realization_identity().unwrap()
    );
}

#[test]
fn malformed_unit_reference_is_rejected_by_both_v02_request_parsers() {
    let mut request = request_value();
    set_conductivity_unit(&mut request, "W/m/K");
    let json = serde_json::to_string(&request).unwrap();
    assert!(matches!(
        ValidatePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::InvalidPublicPayload {
            field: "realization_spec",
            ..
        })
    ));
    assert!(matches!(
        ExecutePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::InvalidPublicPayload {
            field: "realization_spec",
            ..
        })
    ));
}

#[test]
fn non_spatial_scope_member_fails_before_adapter_execution() {
    let mut request = request_value();
    request["realization_spec"]["scopes"][1]["members"] = json!(["thermal.energy_conservation"]);
    let json = serde_json::to_string(&request).unwrap();
    assert!(matches!(
        ExecutePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::InvalidPublicPayload {
            field: "realization_spec",
            ..
        })
    ));
}

#[test]
fn reference_valid_but_scientifically_wrong_scope_needs_independent_oracle() {
    // Both members are declared spatial_model entities; the replacement is
    // structurally resolvable yet physically wrong for the thermal domain.
    let mut request = request_value();
    request["realization_spec"]["scopes"][1]["members"] = json!(["boundary.hot_wall"]);
    parse_both(&request);
}

#[test]
fn missing_plan_action_binding_is_rejected_as_plan_realization_mismatch() {
    let mut request = request_value();
    request["realization_spec"]["action_bindings"]
        .as_array_mut()
        .unwrap()
        .retain(|binding| binding["action_id"] != "thermal.material");
    let json = serde_json::to_string(&request).unwrap();
    assert!(matches!(
        ValidatePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::PlanRealizationMismatch(detail))
            if detail.contains("thermal.material")
    ));
    assert!(matches!(
        ExecutePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::PlanRealizationMismatch(detail))
            if detail.contains("thermal.material")
    ));
}

#[test]
fn syntactically_valid_capability_request_is_not_adapter_support_proof() {
    // A syntactically valid plasma capability can be carried by the envelope,
    // but an actual adapter must separately advertise it to be selected.
    let mut request = request_value();
    request["target"]["required_capabilities"] = json!(["plasma.electron_energy_diffusion"]);
    let (validate, _) = parse_both(&request);
    assert_eq!(
        validate.target.required_capabilities,
        vec!["plasma.electron_energy_diffusion"]
    );
}

#[test]
fn backend_native_hit_identity_cannot_become_canonical_realization_identity() {
    let mut request = request_value();
    request["realization_spec"]["backend_native_id"] = json!("FVKernels/mean_en_diffusion");
    let json = serde_json::to_string(&request).unwrap();
    assert!(matches!(
        ValidatePlanRequestV02::from_json(&json),
        Err(RealizationRequestError::InvalidPublicPayload {
            field: "realization_spec",
            ..
        })
    ));
}

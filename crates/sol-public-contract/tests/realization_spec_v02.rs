use sol_public_contract::{
    BackendTargetDtoV02, MappingPlanDtoV02, RealizationSpecDtoV02, RealizationSpecError,
    PUBLIC_CONTRACT_VERSION, PUBLIC_CONTRACT_VERSION_0_2,
};

const PLAN: &str = r#"
{
  "public_contract_version": "0.2",
  "actions": [
    {"id": "thermal.solve", "dependencies": ["thermal.domain"]},
    {"id": "thermal.domain", "dependencies": []}
  ]
}
"#;

const SPEC: &str = r#"
{
  "public_contract_version": "0.2",
  "ontology_version": "0.1",
  "source_model": "model.thermal_reference",
  "entities": [
    {
      "id": "thermal.energy_conservation",
      "kind": "mathematical_model",
      "semantic_type": "SteadyHeatEquation",
      "parameters": []
    },
    {
      "id": "domain.main",
      "kind": "spatial_model",
      "semantic_type": "LineDomain1D",
      "parameters": [
        {
          "semantic_parameter": "geometry.length",
          "quantity": {"value": 1.0, "unit": "unit.meter"}
        }
      ]
    }
  ],
  "scopes": [
    {"id": "scope.main_domain", "members": ["domain.main"]}
  ],
  "relations": [
    {
      "kind": "defined_on",
      "source": "thermal.energy_conservation",
      "target": "scope.main_domain"
    },
    {
      "kind": "analyzed_by",
      "source": "model.thermal_reference",
      "target": "thermal.energy_conservation"
    }
  ],
  "action_bindings": [
    {
      "action_id": "thermal.solve",
      "subjects": [
        {"subject_kind": "entity", "id": "thermal.energy_conservation"}
      ],
      "scopes": ["scope.main_domain"]
    },
    {
      "action_id": "thermal.domain",
      "subjects": [
        {"subject_kind": "entity", "id": "domain.main"}
      ],
      "scopes": ["scope.main_domain"]
    }
  ]
}
"#;

#[test]
fn v02_is_additive_and_v01_default_is_unchanged() {
    assert_eq!(PUBLIC_CONTRACT_VERSION, "0.1");
    assert_eq!(PUBLIC_CONTRACT_VERSION_0_2, "0.2");

    let target =
        BackendTargetDtoV02::new("moose", vec!["thermal.steady_conduction".to_owned()]).unwrap();
    assert_eq!(target.public_contract_version, "0.2");

    let v01_target = r#"{
      "public_contract_version":"0.1",
      "target":"mock",
      "required_capabilities":[]
    }"#;
    assert!(sol_public_contract::BackendTargetDto::from_json(v01_target).is_ok());
    assert!(BackendTargetDtoV02::from_json(v01_target).is_err());
}

#[test]
fn plan_keeps_dag_semantics_without_hidden_realization_meaning() {
    let plan = MappingPlanDtoV02::from_json(PLAN).unwrap();
    assert_eq!(
        plan.topological_order().unwrap(),
        vec!["thermal.domain", "thermal.solve"]
    );

    let hidden = r#"
    {
      "public_contract_version":"0.2",
      "actions":[
        {
          "id":"thermal.solve",
          "dependencies":[],
          "physics":{"equation":"heat"}
        }
      ]
    }
    "#;
    assert!(matches!(
        MappingPlanDtoV02::from_json(hidden),
        Err(RealizationSpecError::HiddenRealizationExtension(field)) if field == "physics"
    ));
}

#[test]
fn realization_spec_resolves_entities_scopes_relations_and_actions() {
    let plan = MappingPlanDtoV02::from_json(PLAN).unwrap();
    let spec = RealizationSpecDtoV02::from_json(SPEC).unwrap();
    spec.validate_against_plan(&plan).unwrap();

    let canonical = spec.to_canonical_json().unwrap();
    let reparsed = RealizationSpecDtoV02::from_json(&canonical).unwrap();
    assert_eq!(reparsed, spec);
}

#[test]
fn quantity_unit_must_be_a_canonical_reference() {
    let invalid = SPEC.replace("unit.meter", "m");
    assert!(matches!(
        RealizationSpecDtoV02::from_json(&invalid),
        Err(RealizationSpecError::InvalidIdentifier {
            field: "quantity unit",
            ..
        })
    ));
}

#[test]
fn scope_members_must_resolve_to_spatial_entities() {
    let mut invalid: serde_json::Value = serde_json::from_str(SPEC).unwrap();
    invalid["scopes"][0]["members"] = serde_json::json!(["thermal.energy_conservation"]);
    assert!(matches!(
        RealizationSpecDtoV02::from_json(&serde_json::to_string(&invalid).unwrap()),
        Err(RealizationSpecError::NonSpatialScopeMember { .. })
    ));
}

#[test]
fn action_binding_must_cover_plan_exactly() {
    let plan = MappingPlanDtoV02::from_json(PLAN).unwrap();
    let mut missing: serde_json::Value = serde_json::from_str(SPEC).unwrap();
    missing["action_bindings"]
        .as_array_mut()
        .unwrap()
        .retain(|binding| binding["action_id"] != "thermal.domain");

    let spec = RealizationSpecDtoV02::from_json(&serde_json::to_string(&missing).unwrap()).unwrap();
    assert!(matches!(
        spec.validate_against_plan(&plan),
        Err(RealizationSpecError::MissingActionBinding(action)) if action == "thermal.domain"
    ));
}

#[test]
fn backend_native_identity_is_rejected_recursively() {
    let mut invalid: serde_json::Value = serde_json::from_str(SPEC).unwrap();
    invalid["evidence"] = serde_json::json!({"backend_native_id": "mesh/1"});
    assert!(matches!(
        RealizationSpecDtoV02::from_json(&serde_json::to_string(&invalid).unwrap()),
        Err(RealizationSpecError::BackendNativeLeakage(field)) if field == "backend_native_id"
    ));
}

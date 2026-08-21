use sol_public_contract::{
    validate_simulation, Diagnostic, SimulationDto, ValidationReport, ValidationReportError,
    DIAGNOSTIC_DUPLICATE_ID, DIAGNOSTIC_NON_SPATIAL_SCOPE_MEMBER,
    DIAGNOSTIC_UNRESOLVED_RELATION_ENDPOINT,
};

const THERMAL: &str = include_str!("../../../fixtures/public-contract/0.1/thermal-simulation.json");
const THERMAL_REPORT: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-validation-report.json");
const UNRESOLVED_REFERENCE: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-unresolved-reference.json");
const INVALID_SCOPE: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-invalid-scope-membership.json");
const DUPLICATE_ID: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-duplicate-id.json");
const MALFORMED_DIAGNOSTIC: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-malformed-diagnostic.json");

#[test]
fn thermal_public_model_round_trips_and_validates() {
    let simulation = SimulationDto::from_json(THERMAL).unwrap();
    assert!(simulation.has_supported_contract_version());
    assert_eq!(simulation.model.scopes.len(), 2);
    assert_eq!(simulation.model.scopes[0].members, vec!["domain.main"]);

    let report = validate_simulation(&simulation);
    assert!(report.valid);
    assert!(report.diagnostics.is_empty());

    let expected_report = ValidationReport::from_json(THERMAL_REPORT).unwrap();
    assert_eq!(report, expected_report);

    let canonical = simulation.to_canonical_json().unwrap();
    let reparsed = SimulationDto::from_json(&canonical).unwrap();
    assert_eq!(simulation, reparsed);
}

#[test]
fn unresolved_relation_reference_produces_stable_diagnostic() {
    let simulation = SimulationDto::from_json(UNRESOLVED_REFERENCE).unwrap();
    let report = validate_simulation(&simulation);

    assert!(!report.valid);
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_UNRESOLVED_RELATION_ENDPOINT));
}

#[test]
fn non_spatial_scope_membership_is_rejected() {
    let simulation = SimulationDto::from_json(INVALID_SCOPE).unwrap();
    let report = validate_simulation(&simulation);

    assert!(!report.valid);
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_NON_SPATIAL_SCOPE_MEMBER));
}

#[test]
fn duplicate_canonical_id_is_rejected() {
    let simulation = SimulationDto::from_json(DUPLICATE_ID).unwrap();
    let report = validate_simulation(&simulation);

    assert!(!report.valid);
    assert!(report
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == DIAGNOSTIC_DUPLICATE_ID));
}

#[test]
fn malformed_diagnostic_is_rejected() {
    assert!(matches!(
        ValidationReport::from_json(MALFORMED_DIAGNOSTIC),
        Err(ValidationReportError::InvalidDto(_))
    ));

    let malformed_shape = r#"{
        "public_contract_version":"0.1",
        "valid":false,
        "diagnostics":[{
            "code":"",
            "severity":"error",
            "subject":"model.example",
            "detail":"empty diagnostic code"
        }]
    }"#;
    assert!(matches!(
        ValidationReport::from_json(malformed_shape),
        Err(ValidationReportError::MalformedDiagnostic { .. })
    ));
}

#[test]
fn diagnostic_order_and_exact_duplicate_behavior_are_deterministic() {
    let duplicate = Diagnostic::error(
        "model.zeta",
        Some("model.example".to_owned()),
        "zeta diagnostic",
    );
    let earlier = Diagnostic::error(
        "model.alpha",
        Some("model.example".to_owned()),
        "alpha diagnostic",
    );

    let report = ValidationReport::new(vec![duplicate.clone(), earlier, duplicate]);
    assert_eq!(report.diagnostics.len(), 2);
    assert_eq!(report.diagnostics[0].code, "model.alpha");
    assert_eq!(report.diagnostics[1].code, "model.zeta");
}

#[test]
fn typed_dtos_preserve_unknown_optional_fields() {
    let input = r#"{
        "public_contract_version":"0.1",
        "ontology_version":"0.1",
        "future_root":{"opaque":true},
        "model":{
            "id":"model.extension_test",
            "physics":[],
            "mathematical":[],
            "constitutive":[],
            "spatial":[],
            "scopes":[],
            "material":[],
            "conditions":[],
            "numerical":[],
            "observations":[],
            "future_model_option":42
        },
        "tasks":[],
        "relations":[]
    }"#;

    let simulation = SimulationDto::from_json(input).unwrap();
    let canonical = simulation.to_canonical_json().unwrap();

    assert!(canonical.contains("\"future_root\""));
    assert!(canonical.contains("\"future_model_option\""));
}

#[test]
fn validation_report_round_trip_is_canonical() {
    let report = ValidationReport::from_json(THERMAL_REPORT).unwrap();
    let canonical = report.to_canonical_json().unwrap();
    let reparsed = ValidationReport::from_json(&canonical).unwrap();

    assert_eq!(report, reparsed);
}

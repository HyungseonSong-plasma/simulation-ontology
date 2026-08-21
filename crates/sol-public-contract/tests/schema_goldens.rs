use sol_public_contract::{
    BackendTargetDto, CanonicalDocument, EvaluationResultDto, MappingClaimsDto, MappingPlanDto,
    RealizationEffectDocumentDto, SimulationDto, ValidationReport,
};

const THERMAL_SIMULATION: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-simulation.json");
const THERMAL_VALIDATION_REPORT: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-validation-report.json");
const THERMAL_MAPPING_CLAIMS: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-mapping-claims.json");
const THERMAL_MAPPING_PLAN: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-mapping-plan.json");
const THERMAL_BACKEND_TARGET: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-backend-target.json");
const THERMAL_REALIZATION_EFFECT: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-realization-effect.json");
const THERMAL_EVALUATION: &str =
    include_str!("../../../fixtures/public-contract/0.1/thermal-evaluation.json");

fn fixture_canonical_json(input: &str) -> String {
    CanonicalDocument::parse(input)
        .unwrap()
        .to_canonical_json()
        .unwrap()
}

#[test]
fn thermal_simulation_canonical_output_matches_golden() {
    let dto = SimulationDto::from_json(THERMAL_SIMULATION).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_SIMULATION)
    );
}

#[test]
fn thermal_validation_report_canonical_output_matches_golden() {
    let dto = ValidationReport::from_json(THERMAL_VALIDATION_REPORT).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_VALIDATION_REPORT)
    );
}

#[test]
fn thermal_mapping_claims_canonical_output_matches_golden() {
    let dto = MappingClaimsDto::from_json(THERMAL_MAPPING_CLAIMS).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_MAPPING_CLAIMS)
    );
}

#[test]
fn thermal_mapping_plan_canonical_output_matches_golden() {
    let dto = MappingPlanDto::from_json(THERMAL_MAPPING_PLAN).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_MAPPING_PLAN)
    );
}

#[test]
fn thermal_backend_target_canonical_output_matches_golden() {
    let dto = BackendTargetDto::from_json(THERMAL_BACKEND_TARGET).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_BACKEND_TARGET)
    );
}

#[test]
fn thermal_realization_effect_canonical_output_matches_golden() {
    let dto = RealizationEffectDocumentDto::from_json(THERMAL_REALIZATION_EFFECT).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_REALIZATION_EFFECT)
    );
}

#[test]
fn thermal_evaluation_canonical_output_matches_golden() {
    let dto = EvaluationResultDto::from_json(THERMAL_EVALUATION).unwrap();
    assert_eq!(
        dto.to_canonical_json().unwrap(),
        fixture_canonical_json(THERMAL_EVALUATION)
    );
}

use serde_json::Number;
use sol_public_contract::{
    CanonicalDocument, ContractVersion, ObservationOutcomeDtoV03, ObservationResultError,
    ScalarObservationResultDtoV03, SimulationRunRecordDtoV03,
};

const MAXIMUM_TEMPERATURE_RESULT: &str =
    include_str!("../../../fixtures/public-contract/0.3/thermal-maximum-temperature-result.json");
const PRODUCED_RUN: &str =
    include_str!("../../../fixtures/public-contract/0.3/thermal-run-record-produced.json");
const NOT_PRODUCED_RUN: &str =
    include_str!("../../../fixtures/public-contract/0.3/thermal-run-record-not-produced.json");
const MISSING_OUTCOME: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-v03-missing-observation-outcome.json"
);
const BACKEND_SUBSTITUTION: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-v03-backend-native-observation-substitution.json"
);
const RUN_MISMATCH: &str =
    include_str!("../../../fixtures/counterexamples/public-contract-v03-run-result-mismatch.json");
const BACKEND_EXTENSION: &str = include_str!(
    "../../../fixtures/counterexamples/public-contract-v03-backend-native-result-extension.json"
);

fn fixture_canonical_json(input: &str) -> String {
    CanonicalDocument::parse_for(input, ContractVersion::result_v03())
        .unwrap()
        .to_canonical_json()
        .unwrap()
}

#[test]
fn maximum_temperature_scalar_result_roundtrips_canonically() {
    let result = ScalarObservationResultDtoV03::from_json(MAXIMUM_TEMPERATURE_RESULT).unwrap();
    assert_eq!(
        result.identity_key(),
        (
            "run.thermal_reference_001",
            "observation.maximum_temperature"
        )
    );
    assert_eq!(result.source, "thermal.temperature_field");
    assert_eq!(result.scope, "scope.main_domain");
    assert_eq!(result.unit, "unit.kelvin");
    assert_eq!(
        result.to_canonical_json().unwrap(),
        fixture_canonical_json(MAXIMUM_TEMPERATURE_RESULT)
    );
}

#[test]
fn produced_run_has_exact_requested_observation_coverage() {
    let record = SimulationRunRecordDtoV03::from_json(PRODUCED_RUN).unwrap();
    assert_eq!(
        record.requested_observations,
        vec!["observation.maximum_temperature"]
    );
    assert_eq!(record.observation_outcomes.len(), 1);
    assert!(matches!(
        &record.observation_outcomes[0],
        ObservationOutcomeDtoV03::Produced { result }
            if result.observation == "observation.maximum_temperature"
                && result.run_id == record.run_id
    ));
    assert_eq!(
        record.to_canonical_json().unwrap(),
        fixture_canonical_json(PRODUCED_RUN)
    );
}

#[test]
fn non_production_is_a_terminal_outcome_not_an_inferred_result() {
    let record = SimulationRunRecordDtoV03::from_json(NOT_PRODUCED_RUN).unwrap();
    assert!(matches!(
        &record.observation_outcomes[0],
        ObservationOutcomeDtoV03::NotProduced { observation, diagnostics }
            if observation == "observation.maximum_temperature"
                && diagnostics.len() == 1
    ));
    assert_eq!(
        record.to_canonical_json().unwrap(),
        fixture_canonical_json(NOT_PRODUCED_RUN)
    );
}

#[test]
fn missing_terminal_outcome_is_rejected() {
    assert!(matches!(
        SimulationRunRecordDtoV03::from_json(MISSING_OUTCOME),
        Err(ObservationResultError::MissingObservationOutcome(observation))
            if observation == "observation.maximum_temperature"
    ));
}

#[test]
fn backend_native_observation_name_cannot_substitute_for_canonical_request_identity() {
    assert!(matches!(
        SimulationRunRecordDtoV03::from_json(BACKEND_SUBSTITUTION),
        Err(ObservationResultError::MissingObservationOutcome(observation))
            if observation == "observation.maximum_temperature"
    ));
}

#[test]
fn produced_result_must_reference_the_containing_run() {
    assert!(matches!(
        SimulationRunRecordDtoV03::from_json(RUN_MISMATCH),
        Err(ObservationResultError::ResultRunMismatch { observation, .. })
            if observation == "observation.maximum_temperature"
    ));
}

#[test]
fn backend_native_result_extension_is_rejected_even_when_schema_shape_is_valid() {
    assert!(matches!(
        ScalarObservationResultDtoV03::from_json(BACKEND_EXTENSION),
        Err(ObservationResultError::BackendNativeLeakage(field))
            if field == "backend_native_id"
    ));
}

#[test]
fn value_and_unit_changes_remain_distinguishable() {
    let base = ScalarObservationResultDtoV03::from_json(MAXIMUM_TEMPERATURE_RESULT).unwrap();

    let mut changed_value = base.clone();
    changed_value.value = Number::from_f64(401.0).unwrap();
    assert_ne!(
        base.to_canonical_json().unwrap(),
        changed_value.to_canonical_json().unwrap()
    );

    let mut changed_unit = base.clone();
    changed_unit.unit = "unit.celsius".to_owned();
    assert_ne!(
        base.to_canonical_json().unwrap(),
        changed_unit.to_canonical_json().unwrap()
    );
}

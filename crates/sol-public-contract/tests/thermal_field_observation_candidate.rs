//! Candidate-only Thermal field ObservationModel tests for M0.10 Phase 2.
//! The fixture authority is NOT an independent Physics or ontology approval.
use serde_json::{json, Value};
use sol_public_contract::{
    validate_simulation, DatasetCapableSimulationRunRecordDtoV03, DatasetObservationResultDtoV03,
    DatasetPublicationErrorV03, FieldContractError, FieldPayloadDtoV1, FieldSemanticAuthorityV03,
    InMemoryFieldPayloadResolverV03, ObservationResultError, SimulationDto,
    SimulationRunRecordDtoV03,
};

const MODEL: &str =
    include_str!("../../../fixtures/thermal/thermal-reference-field-observations-candidate.json");
const ORIGINAL_MODEL: &str = include_str!("../../../fixtures/thermal/thermal-reference.json");
const MIXED: &str = include_str!(
    "../../../fixtures/public-contract/0.3/thermal-mixed-run-synthetic-candidate.json"
);
const PAYLOAD: &str = include_str!(
    "../../../fixtures/public-contract/0.3/thermal-field-payload-synthetic-candidate.json"
);
const ORIGINAL_SCALAR: &str =
    include_str!("../../../fixtures/public-contract/0.3/thermal-run-record-produced.json");
const FIELD_OBSERVATION: &str = "observation.temperature_field";
const SCALAR_OBSERVATION: &str = "observation.maximum_temperature";
const SOURCE_FIELD: &str = "thermal.temperature_field";

fn raw() -> Value {
    serde_json::from_str(MIXED).unwrap()
}

fn parse(value: &Value) -> Result<DatasetCapableSimulationRunRecordDtoV03, ObservationResultError> {
    DatasetCapableSimulationRunRecordDtoV03::from_json(&value.to_string())
}

fn candidate_model() -> Value {
    serde_json::from_str(MODEL).unwrap()
}

fn model_has_distinct_source_bindings(model: &Value) -> bool {
    let Some(observations) = model["model"]["observations"].as_array() else {
        return false;
    };
    let count = |id: &str| {
        observations
            .iter()
            .filter(|obs| obs["id"] == id && obs["kind"] == "observation_model")
            .count()
    };
    if count(FIELD_OBSERVATION) != 1 || count(SCALAR_OBSERVATION) != 1 {
        return false;
    }
    if model["model"]["mathematical"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|entity| entity["id"] == SOURCE_FIELD && entity["kind"] == "mathematical_model")
        .count()
        != 1
    {
        return false;
    }
    let Some(relations) = model["relations"].as_array() else {
        return false;
    };
    [FIELD_OBSERVATION, SCALAR_OBSERVATION].iter().all(|obs| {
        relations
            .iter()
            .filter(|r| {
                r["kind"] == "observed_by" && r["source"] == SOURCE_FIELD && r["target"] == *obs
            })
            .count()
            == 1
    })
}

// Fixture-only binding: these values are synthetic, not approved real MOOSE results.
struct CandidateFixtureAuthority;
impl FieldSemanticAuthorityV03 for CandidateFixtureAuthority {
    fn validate_canonical_meaning(
        &self,
        result: &DatasetObservationResultDtoV03,
    ) -> Result<(), FieldContractError> {
        let manifest = &result.manifest;
        if result.observation != FIELD_OBSERVATION
            || result.source != SOURCE_FIELD
            || result.scope != "scope.main_domain"
            || manifest.quantity != "quantity.temperature"
            || manifest.unit != "unit.kelvin"
            || manifest.support.scope != "scope.main_domain"
            || manifest.geometry.coordinate_system != "coordinate.cartesian_1d"
            || manifest.geometry.coordinate_units != ["unit.meter"]
            || manifest.geometry.embedding_dimension != 1
            || manifest.geometry.topological_dimension != 1
            || manifest.sampling.kind != sol_public_contract::FieldSamplingKindDtoV03::Stationary
        {
            return Err(FieldContractError(
                "unapproved candidate thermal field binding".to_owned(),
            ));
        }
        Ok(())
    }
}

fn allowed_resolver() -> InMemoryFieldPayloadResolverV03 {
    let field = DatasetCapableSimulationRunRecordDtoV03::from_json(MIXED)
        .unwrap()
        .dataset_results()
        .next()
        .unwrap()
        .clone();
    let payload = FieldPayloadDtoV1::from_json(PAYLOAD)
        .unwrap()
        .to_profile_json()
        .unwrap()
        .into_bytes();
    assert_eq!(payload.len(), field.data_reference.byte_length);
    let mut resolver = InMemoryFieldPayloadResolverV03::new();
    resolver
        .insert_immutable(&field.data_reference.object_id, payload)
        .unwrap();
    resolver.grant(&field.data_reference.object_id).unwrap();
    resolver
}

#[test]
fn candidate_intent_is_distinct_from_legacy_field_source_and_scalar_observation() {
    let original: Value = serde_json::from_str(ORIGINAL_MODEL).unwrap();
    let model = candidate_model();
    assert_eq!(
        original["model"]["observations"].as_array().unwrap().len(),
        1
    );
    assert_eq!(model["model"]["observations"].as_array().unwrap().len(), 2);
    assert_eq!(
        original["model"]["mathematical"],
        model["model"]["mathematical"]
    );
    assert!(model_has_distinct_source_bindings(&model));

    // Run the existing public model reference validation without changing it.
    let mut model_dto = model.clone();
    model_dto["public_contract_version"] = json!("0.1");
    let parsed = SimulationDto::from_json(&model_dto.to_string()).unwrap();
    assert!(validate_simulation(&parsed).valid);

    // This intent constraint is fixture-level; generic graph validation does not
    // independently prove the physical meaning of an observed_by edge.
    let mut disconnected = model;
    disconnected["relations"]
        .as_array_mut()
        .unwrap()
        .retain(|rel| rel["target"] != FIELD_OBSERVATION);
    assert!(!model_has_distinct_source_bindings(&disconnected));
}

#[test]
fn two_distinct_requested_observations_have_one_same_run_terminal_outcome_each() {
    let record = DatasetCapableSimulationRunRecordDtoV03::from_json(MIXED).unwrap();
    assert_eq!(
        record.requested_observations,
        [SCALAR_OBSERVATION.to_owned(), FIELD_OBSERVATION.to_owned()]
    );
    assert_eq!(record.observation_outcomes.len(), 2);
    assert_eq!(record.dataset_results().count(), 1);

    let wire = record.to_canonical_json_unverified().unwrap();
    let normalized: Value = serde_json::from_str(&wire).unwrap();
    let scalar = normalized["observation_outcomes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|outcome| outcome["result"]["observation"] == SCALAR_OBSERVATION)
        .unwrap();
    assert!(scalar.get("result_type").is_none());
    let dataset = normalized["observation_outcomes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|outcome| outcome["result"]["observation"] == FIELD_OBSERVATION)
        .unwrap();
    assert_eq!(dataset["result_type"], "dataset");
    assert_eq!(scalar["result"]["run_id"], dataset["result"]["run_id"]);
    assert_eq!(scalar["result"]["source"], dataset["result"]["source"]);
    assert_eq!(scalar["result"]["scope"], dataset["result"]["scope"]);

    // Publication proof here uses *only* synthetic FixtureAuthority and
    // in-memory resolver. It is not a verified real MOOSE canonical result.
    let published = record
        .publish_verified(&allowed_resolver(), &CandidateFixtureAuthority)
        .unwrap();
    assert_eq!(published.to_canonical_json(), wire);
    assert!(SimulationRunRecordDtoV03::from_json(MIXED).is_err());
}

#[test]
fn missing_duplicate_unrequested_and_wrong_run_outcomes_fail_closed() {
    let mut missing = raw();
    missing["observation_outcomes"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(matches!(
        parse(&missing),
        Err(ObservationResultError::MissingObservationOutcome(id)) if id == FIELD_OBSERVATION
    ));

    let mut duplicate = raw();
    let already = duplicate["observation_outcomes"][0].clone();
    duplicate["observation_outcomes"]
        .as_array_mut()
        .unwrap()
        .push(already);
    assert!(matches!(
        parse(&duplicate),
        Err(ObservationResultError::DuplicateObservationOutcome(id)) if id == SCALAR_OBSERVATION
    ));

    let mut unrequested = raw();
    unrequested["requested_observations"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(matches!(
        parse(&unrequested),
        Err(ObservationResultError::UnexpectedObservationOutcome(id)) if id == FIELD_OBSERVATION
    ));

    let mut mismatched = raw();
    mismatched["observation_outcomes"][1]["result"]["run_id"] = json!("run.other");
    assert!(matches!(
        parse(&mismatched),
        Err(ObservationResultError::ResultRunMismatch { .. })
    ));
}

#[test]
fn field_absence_has_its_own_terminal_not_produced_without_relabeling_scalar() {
    let mut missing_field = raw();
    missing_field["observation_outcomes"][1] = json!({
        "status":"not_produced",
        "observation":FIELD_OBSERVATION,
        "diagnostics":[{
            "code":"result.no_data",
            "severity":"error",
            "subject":FIELD_OBSERVATION,
            "detail":"No temperature field was produced for this run."
        }]
    });
    let record = parse(&missing_field).unwrap();
    assert_eq!(record.dataset_results().count(), 0);
    assert_eq!(record.observation_outcomes.len(), 2);
    // The scalar result remains produced without requiring a dataset resolver.
    record
        .publish_verified(
            &InMemoryFieldPayloadResolverV03::new(),
            &CandidateFixtureAuthority,
        )
        .unwrap();
}

#[test]
fn wrong_field_meaning_or_shape_does_not_pass_fixture_publication() {
    let resolver = allowed_resolver();
    for (path, wrong) in [
        ("quantity", json!("quantity.electric_potential")),
        ("unit", json!("unit.volt")),
    ] {
        let mut value = raw();
        value["observation_outcomes"][1]["result"]["manifest"][path] = wrong;
        let record = parse(&value).unwrap();
        assert!(matches!(
            record.publish_verified(&resolver, &CandidateFixtureAuthority),
            Err(DatasetPublicationErrorV03::Semantic(_))
        ));
    }
    let mut wrong_scope = raw();
    wrong_scope["observation_outcomes"][1]["result"]["scope"] = json!("scope.hot_wall");
    wrong_scope["observation_outcomes"][1]["result"]["manifest"]["support"]["scope"] =
        json!("scope.hot_wall");
    let record = parse(&wrong_scope).unwrap();
    assert!(matches!(
        record.publish_verified(&resolver, &CandidateFixtureAuthority),
        Err(DatasetPublicationErrorV03::Semantic(_))
    ));

    let mut wrong_sampling = raw();
    wrong_sampling["observation_outcomes"][1]["result"]["manifest"]["sampling"] = json!({
        "kind":"time_point","time_value":1,"time_unit":"unit.second"
    });
    let record = parse(&wrong_sampling).unwrap();
    assert!(matches!(
        record.publish_verified(&resolver, &CandidateFixtureAuthority),
        Err(DatasetPublicationErrorV03::Semantic(_))
    ));

    let mut wrong_observation = raw();
    wrong_observation["observation_outcomes"][1]["result"]["observation"] =
        json!(SCALAR_OBSERVATION);
    assert!(matches!(
        parse(&wrong_observation),
        Err(ObservationResultError::DuplicateObservationOutcome(_))
    ));

    let mut wrong_shape = raw();
    wrong_shape["observation_outcomes"][1]["result"]["manifest"]["support"]["entity_count"] =
        json!(2);
    assert!(parse(&wrong_shape).is_err());

    let mut unlabeled_dataset = raw();
    unlabeled_dataset["observation_outcomes"][1]
        .as_object_mut()
        .unwrap()
        .remove("result_type");
    assert!(parse(&unlabeled_dataset).is_err());
}

#[test]
fn dataset_production_does_not_implicitly_produce_missing_scalar_maximum() {
    // The candidate's approved meaning permits independent terminal outcomes.
    // This is synthetic fixture validation, NOT a real MOOSE scalar producer.
    let mut no_scalar = raw();
    no_scalar["observation_outcomes"][0] = json!({
        "status":"not_produced",
        "observation":SCALAR_OBSERVATION,
        "diagnostics":[{
            "code":"result.no_data",
            "severity":"error",
            "subject":SCALAR_OBSERVATION,
            "detail":"No scalar maximum temperature was produced for this run."
        }]
    });
    let record = parse(&no_scalar).unwrap();
    assert_eq!(record.dataset_results().count(), 1);
    assert_eq!(record.observation_outcomes.len(), 2);

    // A present field still requires actual authorized payload resolution.
    assert!(record
        .publish_verified(
            &InMemoryFieldPayloadResolverV03::new(),
            &CandidateFixtureAuthority,
        )
        .is_err());

    let published = record
        .publish_verified(&allowed_resolver(), &CandidateFixtureAuthority)
        .unwrap();
    let output: Value = serde_json::from_str(&published.to_canonical_json()).unwrap();
    assert_eq!(output["observation_outcomes"][0]["status"], "not_produced");
    assert_eq!(
        output["observation_outcomes"][0]["observation"],
        json!(SCALAR_OBSERVATION)
    );
    assert_eq!(output["observation_outcomes"][1]["status"], "produced");
    assert_eq!(output["observation_outcomes"][1]["result_type"], "dataset");
    // Publishing a Dataset does not synthesize a ScalarObservationResult.
    assert!(output["observation_outcomes"][0].get("result").is_none());
}

#[test]
fn legacy_scalar_contract_is_not_redefined_by_new_observation_candidate() {
    let legacy = SimulationRunRecordDtoV03::from_json(ORIGINAL_SCALAR).unwrap();
    let opted = DatasetCapableSimulationRunRecordDtoV03::from_json(ORIGINAL_SCALAR).unwrap();
    assert_eq!(
        legacy.to_canonical_json().unwrap(),
        opted.to_canonical_json_unverified().unwrap()
    );
    // No change to the existing legacy model and run fixture was required.
    let model: Value = serde_json::from_str(ORIGINAL_MODEL).unwrap();
    assert_eq!(model["model"]["observations"].as_array().unwrap().len(), 1);
}

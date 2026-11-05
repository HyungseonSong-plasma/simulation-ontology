use serde_json::{json, Value};
use sol_public_contract::{
    resolve_verified_field, DataResolutionErrorV03, DatasetCapableSimulationRunRecordDtoV03,
    DatasetObservationResultDtoV03, DatasetPublicationErrorV03, FieldContractError,
    FieldPayloadDtoV1, FieldSemanticAuthorityV03, InMemoryFieldPayloadResolverV03,
    ObservationResultError, SimulationRunRecordDtoV03,
};

const MIXED: &str =
    include_str!("../../../fixtures/public-contract/0.3/mixed-run-record-dataset-opt-in.json");
const LEGACY: &str =
    include_str!("../../../fixtures/public-contract/0.3/thermal-run-record-produced.json");
const LEGACY_MISSING: &str =
    include_str!("../../../fixtures/public-contract/0.3/thermal-run-record-not-produced.json");
const FIELD: &str =
    include_str!("../../../fixtures/public-contract/0.3/field-payload-2d-synthetic.json");

fn edit(input: &str, f: impl FnOnce(&mut Value)) -> String {
    let mut value: Value = serde_json::from_str(input).unwrap();
    f(&mut value);
    serde_json::to_string(&value).unwrap()
}

fn parsed() -> DatasetCapableSimulationRunRecordDtoV03 {
    DatasetCapableSimulationRunRecordDtoV03::from_json(MIXED).unwrap()
}

fn sample_field() -> (DatasetObservationResultDtoV03, Vec<u8>) {
    let record = parsed();
    let field = record.dataset_results().next().unwrap().clone();
    let data = FieldPayloadDtoV1::from_json(FIELD)
        .unwrap()
        .to_profile_json()
        .unwrap()
        .into_bytes();
    assert_eq!(data.len(), field.data_reference.byte_length);
    (field, data)
}

fn resolver_for(
    result: &DatasetObservationResultDtoV03,
    bytes: Vec<u8>,
) -> InMemoryFieldPayloadResolverV03 {
    let mut resolver = InMemoryFieldPayloadResolverV03::new();
    resolver
        .insert_immutable(&result.data_reference.object_id, bytes)
        .unwrap();
    resolver.grant(&result.data_reference.object_id).unwrap();
    resolver
}

/// Test-only, explicit approved canonical binding; raw MOOSE field evidence
/// has not been mapped to this registry and must not pass as a scientific fact.
struct FixtureOntology;
impl FieldSemanticAuthorityV03 for FixtureOntology {
    fn validate_canonical_meaning(
        &self,
        result: &DatasetObservationResultDtoV03,
    ) -> Result<(), FieldContractError> {
        if result.observation != "observation.potential_2d"
            || result.source != "field.electric_potential"
            || result.scope != "scope.domain_2d"
            || result.manifest.quantity != "quantity.electric_potential"
            || result.manifest.unit != "unit.volt"
            || result.manifest.geometry.coordinate_system != "coordinate.cartesian_2d"
            || result.manifest.geometry.coordinate_units
                != vec!["unit.meter".to_owned(), "unit.meter".to_owned()]
            || result.manifest.sampling.time_unit.as_deref() != Some("unit.second")
        {
            return Err(FieldContractError(
                "unapproved fixture ontology mapping".into(),
            ));
        }
        Ok(())
    }
}

#[test]
fn original_scalar_wire_and_rust_api_remain_unchanged() {
    for input in [LEGACY, LEGACY_MISSING] {
        let old = SimulationRunRecordDtoV03::from_json(input).unwrap();
        let new = DatasetCapableSimulationRunRecordDtoV03::from_json(input).unwrap();
        assert_eq!(
            old.to_canonical_json().unwrap(),
            new.to_canonical_json_unverified().unwrap()
        );
    }
    assert!(SimulationRunRecordDtoV03::from_json(MIXED).is_err());
}

#[test]
fn explicit_dataset_type_roundtrips_with_scalar_and_not_produced() {
    let record = parsed();
    assert_eq!(record.requested_observations.len(), 3);
    assert_eq!(record.observation_outcomes.len(), 3);
    let canonical = record.to_canonical_json_unverified().unwrap();
    assert_eq!(
        DatasetCapableSimulationRunRecordDtoV03::from_json(&canonical).unwrap(),
        record
    );
    let raw: Value = serde_json::from_str(&canonical).unwrap();
    let outcomes = raw["observation_outcomes"].as_array().unwrap();
    let scalar = outcomes
        .iter()
        .find(|x| x["result"]["value"] == 400.0)
        .unwrap();
    assert!(scalar.get("result_type").is_none());
    let dataset = outcomes
        .iter()
        .find(|x| x["result_type"] == "dataset")
        .unwrap();
    assert_eq!(dataset["status"], "produced");
    assert_eq!(
        dataset["result"]["data_reference"]["payload_profile"],
        "sol.field-json.v1"
    );
    let absent = outcomes
        .iter()
        .find(|x| x["status"] == "not_produced")
        .unwrap();
    assert!(absent["diagnostics"].as_array().unwrap().len() > 0);
}

#[test]
fn unknown_and_ambiguous_type_markers_are_not_silently_downgraded() {
    for changed in [
        edit(MIXED, |v| {
            v["observation_outcomes"][1]["result_type"] = "future".into()
        }),
        edit(MIXED, |v| {
            v["observation_outcomes"][1]
                .as_object_mut()
                .unwrap()
                .remove("result_type");
        }),
        edit(MIXED, |v| {
            v["observation_outcomes"][0]["result_type"] = "dataset".into()
        }),
        edit(MIXED, |v| {
            v["observation_outcomes"][1]["status"] = "produced_dataset".into()
        }),
        edit(MIXED, |v| {
            v["observation_outcomes"][1]["result"]["value"] = 1.into()
        }),
        edit(MIXED, |v| {
            v["observation_outcomes"][1]["native_file"] = "plasma.e".into()
        }),
    ] {
        assert!(DatasetCapableSimulationRunRecordDtoV03::from_json(&changed).is_err());
    }
}

#[test]
fn exact_terminal_coverage_holds_across_all_result_categories() {
    let duplicated = edit(MIXED, |v| {
        let duplicate = v["observation_outcomes"][0].clone();
        v["observation_outcomes"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
    });
    assert!(matches!(
        DatasetCapableSimulationRunRecordDtoV03::from_json(&duplicated),
        Err(ObservationResultError::DuplicateObservationOutcome(_))
    ));

    let missing = edit(MIXED, |v| {
        v["observation_outcomes"].as_array_mut().unwrap().pop();
    });
    assert!(matches!(
        DatasetCapableSimulationRunRecordDtoV03::from_json(&missing),
        Err(ObservationResultError::MissingObservationOutcome(_))
    ));
    let unknown = edit(MIXED, |v| {
        v["requested_observations"].as_array_mut().unwrap().pop();
    });
    assert!(matches!(
        DatasetCapableSimulationRunRecordDtoV03::from_json(&unknown),
        Err(ObservationResultError::UnexpectedObservationOutcome(_))
    ));
    let wrong_run = edit(MIXED, |v| {
        v["observation_outcomes"][1]["result"]["run_id"] = "run.foreign".into();
    });
    assert!(matches!(
        DatasetCapableSimulationRunRecordDtoV03::from_json(&wrong_run),
        Err(ObservationResultError::ResultRunMismatch { .. })
    ));
}

#[test]
fn in_memory_resolver_is_authorized_and_immutable() {
    let (result, bytes) = sample_field();
    let id = &result.data_reference.object_id;
    let mut resolver = InMemoryFieldPayloadResolverV03::new();
    resolver.insert_immutable(id, bytes.clone()).unwrap();
    assert!(matches!(
        resolve_verified_field(&result, &resolver),
        Err(DataResolutionErrorV03::AccessDenied(_))
    ));
    resolver.grant(id).unwrap();
    assert_eq!(
        resolve_verified_field(&result, &resolver)
            .unwrap()
            .values
            .len(),
        4
    );
    assert!(matches!(
        resolver.insert_immutable(id, b"changed".to_vec()),
        Err(DataResolutionErrorV03::ImmutableObjectConflict(_))
    ));
    resolver.revoke(id);
    assert!(matches!(
        resolve_verified_field(&result, &resolver),
        Err(DataResolutionErrorV03::AccessDenied(_))
    ));
    resolver.grant(id).unwrap();
    resolver.remove(id);
    assert!(matches!(
        resolve_verified_field(&result, &resolver),
        Err(DataResolutionErrorV03::ObjectNotFound(_))
    ));
    assert!(resolver.insert_immutable("plasma.e", bytes).is_err());
}

#[test]
fn only_verified_bytes_and_semantic_binding_can_publish_dataset() {
    let record = parsed();
    let (result, bytes) = sample_field();
    let mut resolver = resolver_for(&result, bytes);
    let published = record
        .publish_verified(&resolver, &FixtureOntology)
        .unwrap();
    let frozen = published.to_canonical_json();
    assert_eq!(
        DatasetCapableSimulationRunRecordDtoV03::from_json(&frozen).unwrap(),
        parsed()
    );
    resolver.revoke(&result.data_reference.object_id);
    assert!(matches!(
        resolve_verified_field(&result, &resolver),
        Err(DataResolutionErrorV03::AccessDenied(_))
    ));
    assert_eq!(published.to_canonical_json(), frozen);
    // Historical produced must not be rewritten as not_produced by read failure.
    assert!(frozen.contains("\"result_type\":\"dataset\""));
}

#[test]
fn byte_corruption_missing_data_and_wrong_length_block_publication() {
    let record = parsed();
    let (field, bytes) = sample_field();
    let mut resolver = InMemoryFieldPayloadResolverV03::new();
    resolver.grant(&field.data_reference.object_id).unwrap();
    assert!(matches!(
        record.publish_verified(&resolver, &FixtureOntology),
        Err(DatasetPublicationErrorV03::Resolution(
            DataResolutionErrorV03::ObjectNotFound(_)
        ))
    ));

    let mut changed = bytes.clone();
    let last = changed.len() - 1;
    changed[last] = if changed[last] == b'0' { b'1' } else { b'0' };
    let corrupt = resolver_for(&field, changed);
    assert!(matches!(
        record.publish_verified(&corrupt, &FixtureOntology),
        Err(DatasetPublicationErrorV03::Resolution(
            DataResolutionErrorV03::IntegrityMismatch { .. }
        ))
    ));

    let short = resolver_for(&field, bytes[..bytes.len() - 1].to_vec());
    assert!(matches!(
        record.publish_verified(&short, &FixtureOntology),
        Err(DatasetPublicationErrorV03::Resolution(
            DataResolutionErrorV03::ByteLengthMismatch { .. }
        ))
    ));
}

#[test]
fn meaning_mismatch_prevents_publication_even_when_sha256_is_valid() {
    let (field, bytes) = sample_field();
    let resolver = resolver_for(&field, bytes);
    let mut record = parsed();
    let mut raw: Value =
        serde_json::from_str(&record.to_canonical_json_unverified().unwrap()).unwrap();
    raw["observation_outcomes"][1]["result"]["manifest"]["quantity"] =
        "MOOSE.native_variable".into();
    record = DatasetCapableSimulationRunRecordDtoV03::from_json(&raw.to_string()).unwrap();
    assert!(matches!(
        record.publish_verified(&resolver, &FixtureOntology),
        Err(DatasetPublicationErrorV03::Semantic(_))
    ));
}

#[test]
fn old_scalar_run_publishes_without_needing_dataset_resolver() {
    let record = DatasetCapableSimulationRunRecordDtoV03::from_json(LEGACY).unwrap();
    let resolver = InMemoryFieldPayloadResolverV03::new();
    let published = record
        .publish_verified(&resolver, &FixtureOntology)
        .unwrap();
    assert_eq!(
        published.to_canonical_json(),
        SimulationRunRecordDtoV03::from_json(LEGACY)
            .unwrap()
            .to_canonical_json()
            .unwrap()
    );
}

#[test]
fn publication_syntax_validation_does_not_override_real_ontology_mapping() {
    let (_, bytes) = sample_field();
    let r: Value = serde_json::from_str(MIXED).unwrap();
    let id = r["observation_outcomes"][1]["result"]["data_reference"]["object_id"]
        .as_str()
        .unwrap();
    let mut resolver = InMemoryFieldPayloadResolverV03::new();
    resolver.insert_immutable(id, bytes).unwrap();
    resolver.grant(id).unwrap();
    let mut wrong = r.clone();
    wrong["observation_outcomes"][1]["result"]["manifest"]["unit"] = "unit.kelvin".into();
    let record = DatasetCapableSimulationRunRecordDtoV03::from_json(&wrong.to_string()).unwrap();
    assert!(matches!(
        record.publish_verified(&resolver, &FixtureOntology),
        Err(DatasetPublicationErrorV03::Semantic(_))
    ));
    // This probe is synthetic and is NOT a validated MOOSE quantity mapping.
    assert_eq!(
        json!("unit.kelvin"),
        wrong["observation_outcomes"][1]["result"]["manifest"]["unit"]
    );
}

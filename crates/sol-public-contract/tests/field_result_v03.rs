use serde_json::Value;
use sol_public_contract::{
    DatasetObservationResultDtoV03, FieldAssociationDtoV03, FieldPayloadDtoV1,
    FieldSamplingKindDtoV03,
};

const RESULT_1D: &str =
    include_str!("../../../fixtures/public-contract/0.3/dataset-1d-synthetic.json");
const PAYLOAD_1D: &str =
    include_str!("../../../fixtures/public-contract/0.3/field-payload-1d-synthetic.json");
const RESULT_2D: &str =
    include_str!("../../../fixtures/public-contract/0.3/dataset-2d-synthetic.json");
const PAYLOAD_2D: &str =
    include_str!("../../../fixtures/public-contract/0.3/field-payload-2d-synthetic.json");
const RESULT_INTERFACE: &str =
    include_str!("../../../fixtures/public-contract/0.3/dataset-interface-synthetic.json");
const PAYLOAD_INTERFACE: &str =
    include_str!("../../../fixtures/public-contract/0.3/field-payload-interface-synthetic.json");

const SOURCE_2D: &str =
    include_str!("../../../fixtures/source-evidence/moose/m010/field_2d_potential_exodus.json");
const SOURCE_INTERFACE: &str = include_str!(
    "../../../fixtures/source-evidence/moose/m010/field_1d_interface_sigma_s_exodus.json"
);
const SOURCE_PROVENANCE: &str =
    include_str!("../../../fixtures/source-evidence/moose/m010/source_provenance.json");

fn mutate(base: &str, edit: impl FnOnce(&mut Value)) -> String {
    let mut value: Value = serde_json::from_str(base).unwrap();
    edit(&mut value);
    serde_json::to_string(&value).unwrap()
}

#[test]
fn bounded_scalar_field_payloads_are_dimension_agnostic() {
    for (manifest_json, payload_json) in [
        (RESULT_1D, PAYLOAD_1D),
        (RESULT_2D, PAYLOAD_2D),
        (RESULT_INTERFACE, PAYLOAD_INTERFACE),
    ] {
        let result = DatasetObservationResultDtoV03::from_json(manifest_json).unwrap();
        let payload = FieldPayloadDtoV1::from_json(payload_json).unwrap();
        result.validate_payload(&payload).unwrap();
        assert_eq!(
            DatasetObservationResultDtoV03::from_json(&result.to_canonical_json().unwrap())
                .unwrap(),
            result
        );
        assert_eq!(
            FieldPayloadDtoV1::from_json(&payload.to_profile_json().unwrap()).unwrap(),
            payload
        );
    }
}

#[test]
fn vertex_and_lower_dimensional_element_association_remain_distinct() {
    let node = DatasetObservationResultDtoV03::from_json(RESULT_2D).unwrap();
    let face = DatasetObservationResultDtoV03::from_json(RESULT_INTERFACE).unwrap();
    assert_eq!(node.manifest.geometry.embedding_dimension, 2);
    assert_eq!(face.manifest.geometry.embedding_dimension, 2);
    assert_eq!(node.manifest.support.carrier_dimension, 2);
    assert_eq!(face.manifest.support.carrier_dimension, 1);
    assert_eq!(node.manifest.support.entity_dimension, 0);
    assert_eq!(face.manifest.support.entity_dimension, 1);
    assert_eq!(
        node.manifest.support.location,
        FieldAssociationDtoV03::Vertex
    );
    assert_eq!(face.manifest.support.location, FieldAssociationDtoV03::Cell);
    assert_eq!(
        node.manifest.sampling.kind,
        FieldSamplingKindDtoV03::TimePoint
    );
}

#[test]
fn manifest_rejects_scope_association_and_quantity_errors() {
    let wrong_scope = mutate(RESULT_INTERFACE, |value| {
        value["manifest"]["support"]["scope"] = "scope.somewhere_else".into();
    });
    assert!(DatasetObservationResultDtoV03::from_json(&wrong_scope).is_err());

    let wrong_dimension = mutate(RESULT_INTERFACE, |value| {
        value["manifest"]["support"]["entity_dimension"] = 0.into();
    });
    assert!(DatasetObservationResultDtoV03::from_json(&wrong_dimension).is_err());

    let wrong_quantity = mutate(RESULT_INTERFACE, |value| {
        value["manifest"]["quantity"] = "MOOSE.native_variable".into();
    });
    // Canonical reference syntax alone cannot establish physics/ontology type resolution.
    let dto = DatasetObservationResultDtoV03::from_json(&wrong_quantity).unwrap();
    assert_ne!(dto.manifest.quantity, "quantity.surface_charge_density");
    // That stronger semantic reference resolution remains a tracked Validator gate.
}

#[test]
fn native_provenance_cannot_be_inserted_into_canonical_dataset_dto() {
    let injected = mutate(RESULT_2D, |value| {
        value["backend_native_id"] = "MOOSE.plasma.e".into();
    });
    assert!(DatasetObservationResultDtoV03::from_json(&injected).is_err());

    let nested = mutate(RESULT_2D, |value| {
        value["manifest"]["geometry"]["exodus_block"] = 20.into();
    });
    assert!(DatasetObservationResultDtoV03::from_json(&nested).is_err());

    let corrupt_reference = mutate(RESULT_2D, |value| {
        value["data_reference"]["integrity"]["digest"] = "not-a-sha256".into();
    });
    assert!(DatasetObservationResultDtoV03::from_json(&corrupt_reference).is_err());
}

#[test]
fn missing_or_mismatched_payload_cannot_satisfy_field_structure() {
    let manifest = DatasetObservationResultDtoV03::from_json(RESULT_2D).unwrap();
    let wrong_quantity = DatasetObservationResultDtoV03::from_json(RESULT_INTERFACE).unwrap();
    let mut payload = FieldPayloadDtoV1::from_json(PAYLOAD_2D).unwrap();

    assert!(wrong_quantity.validate_payload(&payload).is_err());
    payload.cells[0].vertices[0] = 1000;
    assert!(manifest.validate_payload(&payload).is_err());

    let mut payload = FieldPayloadDtoV1::from_json(PAYLOAD_2D).unwrap();
    payload.values.pop();
    assert!(manifest.validate_payload(&payload).is_err());

    let mut payload = FieldPayloadDtoV1::from_json(PAYLOAD_2D).unwrap();
    payload.coordinates[0].pop();
    assert!(manifest.validate_payload(&payload).is_err());

    let mut payload = FieldPayloadDtoV1::from_json(PAYLOAD_2D).unwrap();
    payload.support_indices[1] = payload.support_indices[0];
    assert!(manifest.validate_payload(&payload).is_err());
}

#[test]
fn standalone_1d_and_embedded_1d_are_not_conflated() {
    let standalone = DatasetObservationResultDtoV03::from_json(RESULT_1D).unwrap();
    let embedded = DatasetObservationResultDtoV03::from_json(RESULT_INTERFACE).unwrap();
    assert_eq!(standalone.manifest.geometry.embedding_dimension, 1);
    assert_eq!(embedded.manifest.geometry.embedding_dimension, 2);
    assert_eq!(standalone.manifest.support.carrier_dimension, 1);
    assert_eq!(embedded.manifest.support.carrier_dimension, 1);
}

#[test]
fn actual_moose_evidence_has_valid_array_structure_but_no_claimed_canonical_units() {
    let provenance: Value = serde_json::from_str(SOURCE_PROVENANCE).unwrap();
    let volume: Value = serde_json::from_str(SOURCE_2D).unwrap();
    let interface: Value = serde_json::from_str(SOURCE_INTERFACE).unwrap();

    let expected_sha = provenance["source_sha256"].as_str().unwrap();
    assert_eq!(volume["source_sha256"].as_str(), Some(expected_sha));
    assert_eq!(interface["source_sha256"].as_str(), Some(expected_sha));
    assert_eq!(
        volume["time_value_as_stored"],
        interface["time_value_as_stored"]
    );

    for (record, coordinates, cells, count, dimension) in [
        (&volume, 1721, "cells_tri3", 3155, 0),
        (&interface, 54, "cells_edge2", 53, 1),
    ] {
        assert_eq!(
            record["coordinates_xy"].as_array().unwrap().len(),
            coordinates
        );
        assert_eq!(record[cells].as_array().unwrap().len(), count);
        assert_eq!(record["entity_dimension"].as_i64(), Some(dimension));
        assert!(record["unit_from_file"].is_null());
        assert!(record["time_unit_from_file"].is_null());
    }
    assert_eq!(volume["values"].as_array().unwrap().len(), 1721);
    assert_eq!(interface["values"].as_array().unwrap().len(), 53);
    // Neither evidence blob constitutes an independent 1D MOOSE execution.
    assert_eq!(interface["embedding_dimension"].as_i64(), Some(2));
}

#[test]
fn actual_moose_field_arrays_fit_profile_v1_without_claiming_canonical_units() {
    for (source_text, exemplar_json, cells_key, kind, association) in [
        (SOURCE_2D, RESULT_2D, "cells_tri3", "tri3", "vertex"),
        (
            SOURCE_INTERFACE,
            RESULT_INTERFACE,
            "cells_edge2",
            "line2",
            "cell",
        ),
    ] {
        let source: Value = serde_json::from_str(source_text).unwrap();
        let cells: Vec<Value> = source[cells_key]
            .as_array()
            .unwrap()
            .iter()
            .map(|cell| serde_json::json!({"type": kind, "vertices": cell["vertices"]}))
            .collect();
        let values = source["values"].as_array().unwrap();
        let upper = if association == "vertex" {
            source["coordinates_xy"].as_array().unwrap().len()
        } else {
            cells.len()
        };
        let payload: FieldPayloadDtoV1 = serde_json::from_value(serde_json::json!({
            "profile": "sol.field-json.v1",
            "coordinates": source["coordinates_xy"],
            "cells": cells,
            "support_indices": (0..upper).collect::<Vec<_>>(),
            "values": values,
        }))
        .unwrap();

        // This exemplar is only a geometry/cardinality envelope for the raw
        // actual values; it is NOT an assertion that actual physical unit or
        // canonical quantity/scope has been verified by Physics.
        let mut result = DatasetObservationResultDtoV03::from_json(exemplar_json).unwrap();
        result.manifest.support.entity_count = upper;
        result.manifest.value.value_count = values.len();
        result.manifest.payload_layout.coordinate_count = payload.coordinates.len();
        result.manifest.payload_layout.cell_count = payload.cells.len();
        result.manifest.payload_layout.element_types = vec![kind.to_owned()];
        result.validate_payload(&payload).unwrap();
        assert!(!payload.to_profile_json().unwrap().contains("block_id"));
    }
}

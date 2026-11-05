//! Bounded Public Contract 0.3 field dataset support (M0.10 Phase 2).
//! No backend artifact is a canonical identity; byte equality is not scientific equality.
use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{
    canonicalize_value, is_canonical_reference, CanonicalDocument, ContractVersion,
    PUBLIC_CONTRACT_VERSION_0_3,
};

pub const FIELD_JSON_PROFILE_V1: &str = "sol.field-json.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetObservationResultDtoV03 {
    pub public_contract_version: String,
    pub observation: String,
    pub run_id: String,
    pub source: String,
    pub scope: String,
    pub manifest: DatasetManifestDtoV03,
    pub data_reference: DataReferenceDtoV03,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetManifestDtoV03 {
    pub quantity: String,
    pub unit: String,
    pub geometry: FieldGeometryDtoV03,
    pub support: FieldSupportDtoV03,
    pub value: FieldValueShapeDtoV03,
    pub sampling: FieldSamplingDtoV03,
    pub payload_layout: FieldPayloadLayoutDtoV03,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldGeometryDtoV03 {
    pub coordinate_system: String,
    pub coordinate_units: Vec<String>,
    pub embedding_dimension: u8,
    pub topological_dimension: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldAssociationDtoV03 {
    Vertex,
    Cell,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSupportDtoV03 {
    pub scope: String,
    pub carrier_dimension: u8,
    pub entity_dimension: u8,
    pub location: FieldAssociationDtoV03,
    pub entity_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldValueShapeDtoV03 {
    pub rank: u8,
    pub components: Vec<String>,
    pub value_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldSamplingKindDtoV03 {
    Stationary,
    TimePoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSamplingDtoV03 {
    pub kind: FieldSamplingKindDtoV03,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_value: Option<Number>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldPayloadLayoutDtoV03 {
    pub coordinate_count: usize,
    pub cell_count: usize,
    pub element_types: Vec<String>,
    pub index_base: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataReferenceDtoV03 {
    pub object_id: String,
    pub payload_profile: String,
    pub byte_length: usize,
    pub integrity: FieldIntegrityDtoV03,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldIntegrityDtoV03 {
    pub algorithm: String,
    pub digest: String,
}

/// Explicit data bytes, not a solver-native file or a semantic equality key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldPayloadDtoV1 {
    pub profile: String,
    pub coordinates: Vec<Vec<Number>>,
    pub cells: Vec<FieldCellDtoV1>,
    pub support_indices: Vec<usize>,
    pub values: Vec<Number>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldCellDtoV1 {
    #[serde(rename = "type")]
    pub cell_type: String,
    pub vertices: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldContractError(pub String);

impl Display for FieldContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Public Contract 0.3 field dataset: {}", self.0)
    }
}
impl Error for FieldContractError {}

fn bad(message: impl Into<String>) -> FieldContractError {
    FieldContractError(message.into())
}
fn reference(value: &str, name: &str) -> Result<(), FieldContractError> {
    if is_canonical_reference(value) {
        Ok(())
    } else {
        Err(bad(format!("invalid canonical reference {name}: {value}")))
    }
}
fn topology(name: &str) -> Option<(u8, usize)> {
    match name {
        "line2" => Some((1, 2)),
        "tri3" => Some((2, 3)),
        "quad4" => Some((2, 4)),
        _ => None,
    }
}

impl DatasetObservationResultDtoV03 {
    pub fn from_json(input: &str) -> Result<Self, FieldContractError> {
        let document = CanonicalDocument::parse_for(input, ContractVersion::result_v03())
            .map_err(|error| bad(error.to_string()))?;
        let mut result: Self = serde_json::from_value(document.value().clone())
            .map_err(|error| bad(error.to_string()))?;
        result.validate_and_normalize()?;
        Ok(result)
    }

    pub fn to_canonical_json(&self) -> Result<String, FieldContractError> {
        let mut normalized = self.clone();
        normalized.validate_and_normalize()?;
        let value = serde_json::to_value(normalized).map_err(|error| bad(error.to_string()))?;
        serde_json::to_string(&canonicalize_value(value)).map_err(|error| bad(error.to_string()))
    }

    pub fn identity_key(&self) -> (&str, &str) {
        (&self.run_id, &self.observation)
    }

    pub fn validate_and_normalize(&mut self) -> Result<(), FieldContractError> {
        if self.public_contract_version != PUBLIC_CONTRACT_VERSION_0_3 {
            return Err(bad("unsupported public contract version"));
        }
        for (name, value) in [
            ("observation", self.observation.as_str()),
            ("run_id", self.run_id.as_str()),
            ("source", self.source.as_str()),
            ("scope", self.scope.as_str()),
            ("quantity", self.manifest.quantity.as_str()),
            ("unit", self.manifest.unit.as_str()),
            (
                "coordinate_system",
                self.manifest.geometry.coordinate_system.as_str(),
            ),
            ("support.scope", self.manifest.support.scope.as_str()),
            (
                "data_reference.object_id",
                self.data_reference.object_id.as_str(),
            ),
        ] {
            reference(value, name)?;
        }
        if self.scope != self.manifest.support.scope {
            return Err(bad("manifest support scope differs from result scope"));
        }
        let geom = &self.manifest.geometry;
        if !(1..=2).contains(&geom.embedding_dimension)
            || !(1..=geom.embedding_dimension).contains(&geom.topological_dimension)
            || geom.coordinate_units.len() != usize::from(geom.embedding_dimension)
        {
            return Err(bad("invalid 1D/2D geometry dimensions or coordinate units"));
        }
        for unit in &geom.coordinate_units {
            reference(unit, "coordinate unit")?;
        }
        let support = &self.manifest.support;
        if support.carrier_dimension != geom.topological_dimension {
            return Err(bad("carrier dimension must match field mesh topology"));
        }
        if support.entity_count == 0 {
            return Err(bad("empty field support"));
        }
        let associated_dimension = match support.location {
            FieldAssociationDtoV03::Vertex => 0,
            FieldAssociationDtoV03::Cell => support.carrier_dimension,
        };
        if support.entity_dimension != associated_dimension {
            return Err(bad("field association and entity dimension disagree"));
        }
        let value = &self.manifest.value;
        if value.rank != 0 || !value.components.is_empty() {
            return Err(bad("v1 profile supports scalar fields only"));
        }
        if value.value_count != support.entity_count {
            return Err(bad("value_count must match entity_count for scalar field"));
        }
        let sampling = &self.manifest.sampling;
        match sampling.kind {
            FieldSamplingKindDtoV03::Stationary => {
                if sampling.time_value.is_some() || sampling.time_unit.is_some() {
                    return Err(bad("stationary field cannot declare time"));
                }
            }
            FieldSamplingKindDtoV03::TimePoint => {
                if sampling.time_value.is_none() {
                    return Err(bad("time_point field requires time_value"));
                }
                reference(
                    sampling
                        .time_unit
                        .as_deref()
                        .ok_or_else(|| bad("missing time_unit"))?,
                    "time_unit",
                )?;
            }
        }
        let layout = &mut self.manifest.payload_layout;
        if layout.index_base != 0 || layout.coordinate_count == 0 || layout.cell_count == 0 {
            return Err(bad("v1 requires nonempty mesh and zero-based indices"));
        }
        let types: BTreeSet<_> = layout.element_types.iter().map(String::as_str).collect();
        if types.is_empty() || types.len() != layout.element_types.len() {
            return Err(bad("duplicate or missing element types"));
        }
        for item in &types {
            if topology(item).map(|(dim, _)| dim) != Some(geom.topological_dimension) {
                return Err(bad("unsupported or dimension-mismatched element type"));
            }
        }
        layout.element_types.sort();
        let data = &self.data_reference;
        if data.payload_profile != FIELD_JSON_PROFILE_V1
            || data.byte_length == 0
            || data.integrity.algorithm != "sha256"
            || data.integrity.digest.len() != 64
            || !data
                .integrity
                .digest
                .bytes()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        {
            return Err(bad(
                "unsupported data reference, integrity or payload profile",
            ));
        }
        Ok(())
    }

    /// Structural correspondence only. Byte-digest verification belongs to the resolver.
    /// This does not establish canonical content equality or scientific correctness.
    pub fn validate_payload(&self, payload: &FieldPayloadDtoV1) -> Result<(), FieldContractError> {
        let mut result = self.clone();
        result.validate_and_normalize()?;
        if payload.profile != self.data_reference.payload_profile {
            return Err(bad("payload profile mismatch"));
        }
        let layout = &self.manifest.payload_layout;
        let geom = &self.manifest.geometry;
        if payload.coordinates.len() != layout.coordinate_count
            || payload.cells.len() != layout.cell_count
            || payload.values.len() != self.manifest.value.value_count
            || payload.support_indices.len() != self.manifest.support.entity_count
        {
            return Err(bad("payload geometry/value/support cardinality mismatch"));
        }
        for coord in &payload.coordinates {
            if coord.len() != usize::from(geom.embedding_dimension) {
                return Err(bad("coordinate dimension mismatch"));
            }
        }
        let mut seen_types = BTreeSet::new();
        for cell in &payload.cells {
            let (dimension, arity) =
                topology(&cell.cell_type).ok_or_else(|| bad("unsupported payload cell type"))?;
            if dimension != geom.topological_dimension || cell.vertices.len() != arity {
                return Err(bad("payload cell type or vertex arity mismatch"));
            }
            let mut unique = BTreeSet::new();
            for index in &cell.vertices {
                if *index >= payload.coordinates.len() || !unique.insert(*index) {
                    return Err(bad("invalid, repeated or out-of-range connectivity index"));
                }
            }
            seen_types.insert(cell.cell_type.as_str());
        }
        let declared: BTreeSet<_> = layout.element_types.iter().map(String::as_str).collect();
        if seen_types != declared {
            return Err(bad("manifest element types do not match payload cells"));
        }
        let upper = match self.manifest.support.location {
            FieldAssociationDtoV03::Vertex => payload.coordinates.len(),
            FieldAssociationDtoV03::Cell => payload.cells.len(),
        };
        let mut unique = BTreeSet::new();
        for index in &payload.support_indices {
            if *index >= upper || !unique.insert(*index) {
                return Err(bad("invalid or duplicate support index"));
            }
        }
        Ok(())
    }
}

impl FieldPayloadDtoV1 {
    pub fn from_json(input: &str) -> Result<Self, FieldContractError> {
        serde_json::from_str(input).map_err(|error| bad(error.to_string()))
    }

    /// Deterministic JSON bytes for this exact profile; object-key order is non-semantic.
    /// Array reindexing equivalence and floating-point/units normalization are NOT inferred.
    pub fn to_profile_json(&self) -> Result<String, FieldContractError> {
        let value = serde_json::to_value(self).map_err(|error| bad(error.to_string()))?;
        serde_json::to_string(&canonicalize_value(value)).map_err(|error| bad(error.to_string()))
    }
}

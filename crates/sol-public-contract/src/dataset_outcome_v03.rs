//! Explicit Public Contract 0.3 Dataset-capable run record opt-in.
//! This module does not change the existing scalar-only Rust DTO or its wire format.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

use crate::observation_result_v03::{
    reject_backend_native_extensions, require_canonical_reference, require_v03,
};
use crate::{
    canonicalize_value, CanonicalDocument, ContractVersion, DatasetObservationResultDtoV03,
    Diagnostic, Extensions, ObservationOutcomeDtoV03, ObservationResultError,
    ScalarObservationResultDtoV03,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProducedStatusOptInDtoV03 {
    #[serde(rename = "produced")]
    Produced,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotProducedStatusOptInDtoV03 {
    #[serde(rename = "not_produced")]
    NotProduced,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatasetResultTypeOptInDtoV03 {
    #[serde(rename = "dataset")]
    Dataset,
}

/// The original scalar JSON has no result_type and is left exactly unchanged.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScalarProducedOutcomeOptInDtoV03 {
    pub status: ProducedStatusOptInDtoV03,
    pub result: ScalarObservationResultDtoV03,
}

/// A new dataset outcome must explicitly opt in with result_type:"dataset".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DatasetProducedOutcomeOptInDtoV03 {
    pub status: ProducedStatusOptInDtoV03,
    pub result_type: DatasetResultTypeOptInDtoV03,
    pub result: DatasetObservationResultDtoV03,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotProducedOutcomeOptInDtoV03 {
    pub status: NotProducedStatusOptInDtoV03,
    pub observation: String,
    pub diagnostics: Vec<Diagnostic>,
}

/// Strict disjoint union. Unsupported and hybrid result types fail closed.
/// Existing scalar-only consumers still use ObservationOutcomeDtoV03.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatasetCapableObservationOutcomeDtoV03 {
    Scalar(ScalarProducedOutcomeOptInDtoV03),
    Dataset(Box<DatasetProducedOutcomeOptInDtoV03>),
    NotProduced(NotProducedOutcomeOptInDtoV03),
}

impl DatasetCapableObservationOutcomeDtoV03 {
    pub fn observation(&self) -> &str {
        match self {
            Self::Scalar(outcome) => &outcome.result.observation,
            Self::Dataset(outcome) => &outcome.result.observation,
            Self::NotProduced(outcome) => &outcome.observation,
        }
    }

    fn normalize(&mut self) -> Result<(), ObservationResultError> {
        match self {
            Self::Scalar(outcome) => {
                let mut original = ObservationOutcomeDtoV03::Produced {
                    result: outcome.result.clone(),
                };
                original.normalize()?;
                if let ObservationOutcomeDtoV03::Produced { result } = original {
                    outcome.result = result;
                }
            }
            Self::Dataset(outcome) => {
                outcome
                    .result
                    .validate_and_normalize()
                    .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
            }
            Self::NotProduced(outcome) => {
                let mut original = ObservationOutcomeDtoV03::NotProduced {
                    observation: outcome.observation.clone(),
                    diagnostics: outcome.diagnostics.clone(),
                };
                original.normalize()?;
                if let ObservationOutcomeDtoV03::NotProduced { diagnostics, .. } = original {
                    outcome.diagnostics = diagnostics;
                }
            }
        }
        Ok(())
    }

    fn produced_run_id(&self) -> Option<&str> {
        match self {
            Self::Scalar(outcome) => Some(&outcome.result.run_id),
            Self::Dataset(outcome) => Some(&outcome.result.run_id),
            Self::NotProduced(_) => None,
        }
    }
}

/// Opt-in mixed run DTO. Parsing / struct normalization alone DOES NOT authorize
/// publication of a produced dataset: only publish_verified can do so.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetCapableSimulationRunRecordDtoV03 {
    pub public_contract_version: String,
    pub run_id: String,
    pub source_model: String,
    pub task: String,
    pub analysis: String,
    #[serde(default)]
    pub requested_observations: Vec<String>,
    #[serde(default)]
    pub observation_outcomes: Vec<DatasetCapableObservationOutcomeDtoV03>,
    #[serde(flatten, default)]
    pub extensions: Extensions,
}

impl DatasetCapableSimulationRunRecordDtoV03 {
    pub fn from_json(input: &str) -> Result<Self, ObservationResultError> {
        let document = CanonicalDocument::parse_for(input, ContractVersion::result_v03())
            .map_err(ObservationResultError::Contract)?;
        let mut record: Self = serde_json::from_value(document.value().clone())
            .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
        record.normalize()?;
        Ok(record)
    }

    /// A canonical parser-only representation (e.g. historical record read).
    /// This is not authoritative publication evidence; use publish_verified.
    pub fn to_canonical_json_unverified(&self) -> Result<String, ObservationResultError> {
        let mut normalized = self.clone();
        normalized.normalize()?;
        let value = serde_json::to_value(normalized)
            .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
        serde_json::to_string(&canonicalize_value(value))
            .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))
    }

    pub fn normalize(&mut self) -> Result<(), ObservationResultError> {
        require_v03(&self.public_contract_version)?;
        for (name, reference) in [
            ("simulation run id", &self.run_id),
            ("simulation run source model", &self.source_model),
            ("simulation run task", &self.task),
            ("simulation run analysis", &self.analysis),
        ] {
            require_canonical_reference(name, reference)?;
        }
        reject_backend_native_extensions(&self.extensions)?;

        let mut requested = BTreeSet::new();
        for observation in &self.requested_observations {
            require_canonical_reference("requested observation", observation)?;
            if !requested.insert(observation.clone()) {
                return Err(ObservationResultError::DuplicateRequestedObservation(
                    observation.clone(),
                ));
            }
        }
        self.requested_observations.sort();

        for outcome in &mut self.observation_outcomes {
            outcome.normalize()?;
        }
        self.observation_outcomes
            .sort_by(|a, b| a.observation().cmp(b.observation()));

        let mut actual = BTreeSet::new();
        for outcome in &self.observation_outcomes {
            let observation = outcome.observation().to_owned();
            if !actual.insert(observation.clone()) {
                return Err(ObservationResultError::DuplicateObservationOutcome(
                    observation,
                ));
            }
            if let Some(run) = outcome.produced_run_id() {
                if run != self.run_id {
                    return Err(ObservationResultError::ResultRunMismatch {
                        observation,
                        expected_run: self.run_id.clone(),
                        actual_run: run.to_owned(),
                    });
                }
            }
        }
        for observation in &requested {
            if !actual.contains(observation) {
                return Err(ObservationResultError::MissingObservationOutcome(
                    observation.clone(),
                ));
            }
        }
        for observation in &actual {
            if !requested.contains(observation) {
                return Err(ObservationResultError::UnexpectedObservationOutcome(
                    observation.clone(),
                ));
            }
        }
        Ok(())
    }

    pub fn dataset_results(&self) -> impl Iterator<Item = &DatasetObservationResultDtoV03> {
        self.observation_outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                DatasetCapableObservationOutcomeDtoV03::Dataset(envelope) => Some(&envelope.result),
                _ => None,
            })
    }

    /// Only a resolved-and-semantically-verified record may be issued as
    /// PublishedDatasetCapableRunRecordV03. Later resolution errors never mutate it.
    pub fn publish_verified(
        &self,
        resolver: &dyn crate::FieldPayloadResolverV03,
        semantics: &dyn crate::FieldSemanticAuthorityV03,
    ) -> Result<PublishedDatasetCapableRunRecordV03, crate::DatasetPublicationErrorV03> {
        let mut record = self.clone();
        record
            .normalize()
            .map_err(crate::DatasetPublicationErrorV03::InvalidRecord)?;
        for result in record.dataset_results() {
            semantics
                .validate_canonical_meaning(result)
                .map_err(crate::DatasetPublicationErrorV03::Semantic)?;
            crate::resolve_verified_field(result, resolver)
                .map_err(crate::DatasetPublicationErrorV03::Resolution)?;
        }
        Ok(PublishedDatasetCapableRunRecordV03 { record })
    }
}

/// Publication proof is constructed only by publish_verified, not by parsing.
/// Its record is immutable through this interface.
#[derive(Debug, Clone)]
pub struct PublishedDatasetCapableRunRecordV03 {
    record: DatasetCapableSimulationRunRecordDtoV03,
}

impl PublishedDatasetCapableRunRecordV03 {
    pub fn record(&self) -> &DatasetCapableSimulationRunRecordDtoV03 {
        &self.record
    }
    pub fn to_canonical_json(&self) -> String {
        // Publication already validated and normalized the record.
        let value: Value = serde_json::to_value(&self.record)
            .expect("validated Public Contract 0.3 record must serialize");
        serde_json::to_string(&canonicalize_value(value))
            .expect("validated Public Contract 0.3 JSON must serialize")
    }
}

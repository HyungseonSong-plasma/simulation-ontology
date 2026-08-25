use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{
    canonicalize_value, is_canonical_reference, ContractDocumentError, ContractVersion, Diagnostic,
    DiagnosticSeverity, Extensions, PUBLIC_CONTRACT_VERSION_0_3,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScalarObservationResultDtoV03 {
    pub public_contract_version: String,
    pub observation: String,
    pub run_id: String,
    pub source: String,
    pub scope: String,
    pub value: Number,
    pub unit: String,
    #[serde(flatten, default)]
    pub extensions: Extensions,
}

impl ScalarObservationResultDtoV03 {
    pub fn new(
        observation: impl Into<String>,
        run_id: impl Into<String>,
        source: impl Into<String>,
        scope: impl Into<String>,
        value: Number,
        unit: impl Into<String>,
    ) -> Result<Self, ObservationResultError> {
        let mut result = Self {
            public_contract_version: PUBLIC_CONTRACT_VERSION_0_3.to_owned(),
            observation: observation.into(),
            run_id: run_id.into(),
            source: source.into(),
            scope: scope.into(),
            value,
            unit: unit.into(),
            extensions: BTreeMap::new(),
        };
        result.normalize()?;
        Ok(result)
    }

    pub fn from_json(input: &str) -> Result<Self, ObservationResultError> {
        let value = parse_v03_document(input)?;
        let mut result: Self = serde_json::from_value(value)
            .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
        result.normalize()?;
        Ok(result)
    }

    pub fn to_canonical_json(&self) -> Result<String, ObservationResultError> {
        let mut result = self.clone();
        result.normalize()?;
        canonical_json_v03(&result)
    }

    pub fn identity_key(&self) -> (&str, &str) {
        (&self.run_id, &self.observation)
    }

    fn normalize(&mut self) -> Result<(), ObservationResultError> {
        require_v03(&self.public_contract_version)?;
        require_canonical_reference("observation result observation", &self.observation)?;
        require_canonical_reference("observation result run", &self.run_id)?;
        require_canonical_reference("observation result source", &self.source)?;
        require_canonical_reference("observation result scope", &self.scope)?;
        require_canonical_reference("observation result unit", &self.unit)?;
        reject_backend_native_extensions(&self.extensions)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ObservationOutcomeDtoV03 {
    Produced {
        result: ScalarObservationResultDtoV03,
    },
    NotProduced {
        observation: String,
        diagnostics: Vec<Diagnostic>,
    },
}

impl ObservationOutcomeDtoV03 {
    pub fn observation(&self) -> &str {
        match self {
            Self::Produced { result } => &result.observation,
            Self::NotProduced { observation, .. } => observation,
        }
    }

    fn normalize(&mut self) -> Result<(), ObservationResultError> {
        match self {
            Self::Produced { result } => result.normalize(),
            Self::NotProduced {
                observation,
                diagnostics,
            } => {
                require_canonical_reference("non-production observation", observation)?;
                if diagnostics.is_empty() {
                    return Err(ObservationResultError::MissingNonProductionEvidence(
                        observation.clone(),
                    ));
                }
                for (index, diagnostic) in diagnostics.iter().enumerate() {
                    diagnostic.validate_shape().map_err(|error| {
                        ObservationResultError::MalformedDiagnostic {
                            observation: observation.clone(),
                            index,
                            reason: error.to_string(),
                        }
                    })?;
                    if let Some(subject) = &diagnostic.subject {
                        if subject != observation {
                            return Err(ObservationResultError::DiagnosticSubjectMismatch {
                                observation: observation.clone(),
                                subject: subject.clone(),
                            });
                        }
                    }
                    reject_backend_native_extensions(&diagnostic.extensions)?;
                }
                diagnostics.sort_by_key(diagnostic_key);
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationRunRecordDtoV03 {
    pub public_contract_version: String,
    pub run_id: String,
    pub source_model: String,
    pub task: String,
    pub analysis: String,
    #[serde(default)]
    pub requested_observations: Vec<String>,
    #[serde(default)]
    pub observation_outcomes: Vec<ObservationOutcomeDtoV03>,
    #[serde(flatten, default)]
    pub extensions: Extensions,
}

impl SimulationRunRecordDtoV03 {
    pub fn new(
        run_id: impl Into<String>,
        source_model: impl Into<String>,
        task: impl Into<String>,
        analysis: impl Into<String>,
        requested_observations: Vec<String>,
        observation_outcomes: Vec<ObservationOutcomeDtoV03>,
    ) -> Result<Self, ObservationResultError> {
        let mut record = Self {
            public_contract_version: PUBLIC_CONTRACT_VERSION_0_3.to_owned(),
            run_id: run_id.into(),
            source_model: source_model.into(),
            task: task.into(),
            analysis: analysis.into(),
            requested_observations,
            observation_outcomes,
            extensions: BTreeMap::new(),
        };
        record.normalize()?;
        Ok(record)
    }

    pub fn from_json(input: &str) -> Result<Self, ObservationResultError> {
        let value = parse_v03_document(input)?;
        let mut record: Self = serde_json::from_value(value)
            .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
        record.normalize()?;
        Ok(record)
    }

    pub fn to_canonical_json(&self) -> Result<String, ObservationResultError> {
        let mut record = self.clone();
        record.normalize()?;
        canonical_json_v03(&record)
    }

    fn normalize(&mut self) -> Result<(), ObservationResultError> {
        require_v03(&self.public_contract_version)?;
        require_canonical_reference("simulation run id", &self.run_id)?;
        require_canonical_reference("simulation run source model", &self.source_model)?;
        require_canonical_reference("simulation run task", &self.task)?;
        require_canonical_reference("simulation run analysis", &self.analysis)?;
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
            .sort_by(|left, right| left.observation().cmp(right.observation()));

        let mut outcome_ids = BTreeSet::new();
        for outcome in &self.observation_outcomes {
            let observation = outcome.observation().to_owned();
            if !outcome_ids.insert(observation.clone()) {
                return Err(ObservationResultError::DuplicateObservationOutcome(
                    observation,
                ));
            }
            if let ObservationOutcomeDtoV03::Produced { result } = outcome {
                if result.run_id != self.run_id {
                    return Err(ObservationResultError::ResultRunMismatch {
                        observation: result.observation.clone(),
                        expected_run: self.run_id.clone(),
                        actual_run: result.run_id.clone(),
                    });
                }
            }
        }

        for observation in &requested {
            if !outcome_ids.contains(observation) {
                return Err(ObservationResultError::MissingObservationOutcome(
                    observation.clone(),
                ));
            }
        }
        for observation in &outcome_ids {
            if !requested.contains(observation) {
                return Err(ObservationResultError::UnexpectedObservationOutcome(
                    observation.clone(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationResultError {
    Contract(ContractDocumentError),
    InvalidDto(String),
    InvalidIdentifier {
        field: &'static str,
        value: String,
    },
    DuplicateRequestedObservation(String),
    DuplicateObservationOutcome(String),
    MissingObservationOutcome(String),
    UnexpectedObservationOutcome(String),
    ResultRunMismatch {
        observation: String,
        expected_run: String,
        actual_run: String,
    },
    MissingNonProductionEvidence(String),
    MalformedDiagnostic {
        observation: String,
        index: usize,
        reason: String,
    },
    DiagnosticSubjectMismatch {
        observation: String,
        subject: String,
    },
    BackendNativeLeakage(String),
}

impl Display for ObservationResultError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contract(error) => write!(formatter, "{error}"),
            Self::InvalidDto(detail) => write!(formatter, "invalid Public Contract 0.3 result DTO: {detail}"),
            Self::InvalidIdentifier { field, value } => {
                write!(formatter, "invalid {field} reference: {value}")
            }
            Self::DuplicateRequestedObservation(observation) => {
                write!(formatter, "duplicate requested observation: {observation}")
            }
            Self::DuplicateObservationOutcome(observation) => {
                write!(formatter, "duplicate terminal observation outcome: {observation}")
            }
            Self::MissingObservationOutcome(observation) => {
                write!(formatter, "missing terminal observation outcome: {observation}")
            }
            Self::UnexpectedObservationOutcome(observation) => {
                write!(formatter, "terminal outcome references unrequested observation: {observation}")
            }
            Self::ResultRunMismatch {
                observation,
                expected_run,
                actual_run,
            } => write!(
                formatter,
                "produced result run mismatch for {observation}: expected {expected_run}, got {actual_run}"
            ),
            Self::MissingNonProductionEvidence(observation) => write!(
                formatter,
                "non-produced observation requires diagnostic evidence: {observation}"
            ),
            Self::MalformedDiagnostic {
                observation,
                index,
                reason,
            } => write!(
                formatter,
                "malformed non-production diagnostic for {observation} at index {index}: {reason}"
            ),
            Self::DiagnosticSubjectMismatch {
                observation,
                subject,
            } => write!(
                formatter,
                "non-production diagnostic subject {subject} does not match observation {observation}"
            ),
            Self::BackendNativeLeakage(field) => write!(
                formatter,
                "backend-native result/provenance field is not canonical observation-result data: {field}"
            ),
        }
    }
}

impl Error for ObservationResultError {}

fn parse_v03_document(input: &str) -> Result<Value, ObservationResultError> {
    let document = crate::CanonicalDocument::parse_for(input, ContractVersion::result_v03())
        .map_err(ObservationResultError::Contract)?;
    Ok(document.value().clone())
}

fn canonical_json_v03<T: Serialize>(value: &T) -> Result<String, ObservationResultError> {
    let value = serde_json::to_value(value)
        .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
    let raw = serde_json::to_string(&canonicalize_value(value))
        .map_err(|error| ObservationResultError::InvalidDto(error.to_string()))?;
    crate::CanonicalDocument::parse_for(&raw, ContractVersion::result_v03())
        .map_err(ObservationResultError::Contract)?;
    Ok(raw)
}

fn require_v03(version: &str) -> Result<(), ObservationResultError> {
    if version == PUBLIC_CONTRACT_VERSION_0_3 {
        Ok(())
    } else {
        Err(ObservationResultError::Contract(
            ContractDocumentError::UnsupportedVersion(version.to_owned()),
        ))
    }
}

fn require_canonical_reference(
    field: &'static str,
    value: &str,
) -> Result<(), ObservationResultError> {
    if is_canonical_reference(value) {
        Ok(())
    } else {
        Err(ObservationResultError::InvalidIdentifier {
            field,
            value: value.to_owned(),
        })
    }
}

fn reject_backend_native_extensions(extensions: &Extensions) -> Result<(), ObservationResultError> {
    for (key, value) in extensions {
        if is_backend_native_key(key) {
            return Err(ObservationResultError::BackendNativeLeakage(key.clone()));
        }
        reject_backend_native_value(value)?;
    }
    Ok(())
}

fn reject_backend_native_value(value: &Value) -> Result<(), ObservationResultError> {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if is_backend_native_key(key) {
                    return Err(ObservationResultError::BackendNativeLeakage(key.clone()));
                }
                reject_backend_native_value(child)?;
            }
        }
        Value::Array(values) => {
            for child in values {
                reject_backend_native_value(child)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn is_backend_native_key(key: &str) -> bool {
    matches!(
        key,
        "adapter_native"
            | "backend_native"
            | "backend_native_id"
            | "backend_object"
            | "backend_result"
            | "backend_result_id"
            | "native_object"
            | "native_result"
            | "solver_object"
            | "solver_result"
            | "artifact_path"
            | "provenance"
    )
}

fn diagnostic_key(diagnostic: &Diagnostic) -> String {
    format!(
        "{}:{}:{}:{}",
        severity_key(diagnostic.severity),
        diagnostic.code,
        diagnostic.subject.as_deref().unwrap_or_default(),
        diagnostic.detail
    )
}

fn severity_key(severity: DiagnosticSeverity) -> &'static str {
    match severity {
        DiagnosticSeverity::Error => "0-error",
        DiagnosticSeverity::Warning => "1-warning",
        DiagnosticSeverity::Info => "2-info",
    }
}

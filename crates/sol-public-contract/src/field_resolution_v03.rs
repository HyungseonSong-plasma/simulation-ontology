//! SOL-controlled Field payload resolution and verified publication gate.
//! In-memory resolver is a bounded conformance model, NOT production storage.
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::field_sha256::sha256_hex;
use crate::{
    is_canonical_reference, DatasetObservationResultDtoV03, FieldContractError, FieldPayloadDtoV1,
    ObservationResultError,
};

pub trait FieldPayloadResolverV03 {
    /// Access checks happen in the resolver; never accept a backend-native file path.
    fn resolve_bytes(&self, object_id: &str) -> Result<Vec<u8>, DataResolutionErrorV03>;
}

pub trait FieldSemanticAuthorityV03 {
    /// Caller-supplied trusted ontology/Physics mapping, not syntax-only validation.
    fn validate_canonical_meaning(
        &self,
        result: &DatasetObservationResultDtoV03,
    ) -> Result<(), FieldContractError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataResolutionErrorV03 {
    InvalidReference(String),
    AccessDenied(String),
    ObjectNotFound(String),
    ImmutableObjectConflict(String),
    ByteLengthMismatch { expected: usize, actual: usize },
    IntegrityMismatch { expected: String, actual: String },
    InvalidProfile(String),
    NoncanonicalEncoding,
    ShapeMismatch(String),
}

impl Display for DataResolutionErrorV03 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidReference(id) => write!(f, "invalid SOL data reference: {id}"),
            Self::AccessDenied(id) => write!(f, "data access denied: {id}"),
            Self::ObjectNotFound(id) => write!(f, "data object unavailable: {id}"),
            Self::ImmutableObjectConflict(id) => {
                write!(f, "immutable data object cannot change bytes: {id}")
            }
            Self::ByteLengthMismatch { expected, actual } => {
                write!(
                    f,
                    "dataset byte length mismatch: expected {expected}, got {actual}"
                )
            }
            Self::IntegrityMismatch { expected, actual } => {
                write!(
                    f,
                    "dataset SHA-256 mismatch: expected {expected}, got {actual}"
                )
            }
            Self::InvalidProfile(error) => write!(f, "invalid dataset payload: {error}"),
            Self::NoncanonicalEncoding => write!(f, "noncanonical dataset-profile byte encoding"),
            Self::ShapeMismatch(error) => write!(f, "dataset metadata/payload mismatch: {error}"),
        }
    }
}
impl Error for DataResolutionErrorV03 {}

#[derive(Debug)]
pub enum DatasetPublicationErrorV03 {
    InvalidRecord(ObservationResultError),
    Semantic(FieldContractError),
    Resolution(DataResolutionErrorV03),
}

impl Display for DatasetPublicationErrorV03 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRecord(error) => write!(f, "invalid publication record: {error}"),
            Self::Semantic(error) => write!(f, "unverified dataset meaning: {error}"),
            Self::Resolution(error) => write!(f, "unverified dataset bytes: {error}"),
        }
    }
}
impl Error for DatasetPublicationErrorV03 {}

/// Resolve exact bytes, verify SHA-256/length/profile and check field topology.
/// This is necessary but NOT sufficient for scientific semantic acceptance.
pub fn resolve_verified_field(
    result: &DatasetObservationResultDtoV03,
    resolver: &dyn FieldPayloadResolverV03,
) -> Result<FieldPayloadDtoV1, DataResolutionErrorV03> {
    let mut validated = result.clone();
    validated
        .validate_and_normalize()
        .map_err(|error| DataResolutionErrorV03::InvalidProfile(error.to_string()))?;
    let reference = &validated.data_reference;
    let bytes = resolver.resolve_bytes(&reference.object_id)?;
    if bytes.len() != reference.byte_length {
        return Err(DataResolutionErrorV03::ByteLengthMismatch {
            expected: reference.byte_length,
            actual: bytes.len(),
        });
    }
    let actual = sha256_hex(&bytes);
    if actual != reference.integrity.digest {
        return Err(DataResolutionErrorV03::IntegrityMismatch {
            expected: reference.integrity.digest.clone(),
            actual,
        });
    }
    let raw = std::str::from_utf8(&bytes)
        .map_err(|error| DataResolutionErrorV03::InvalidProfile(error.to_string()))?;
    let payload = FieldPayloadDtoV1::from_json(raw)
        .map_err(|error| DataResolutionErrorV03::InvalidProfile(error.to_string()))?;
    let canonical = payload
        .to_profile_json()
        .map_err(|error| DataResolutionErrorV03::InvalidProfile(error.to_string()))?;
    if canonical.as_bytes() != bytes {
        return Err(DataResolutionErrorV03::NoncanonicalEncoding);
    }
    validated
        .validate_payload(&payload)
        .map_err(|error| DataResolutionErrorV03::ShapeMismatch(error.to_string()))?;
    Ok(payload)
}

/// A per-caller, explicitly authorized in-memory resolver for conformance only.
/// Immutable on successful insert; access is fail-closed by default.
#[derive(Debug, Clone, Default)]
pub struct InMemoryFieldPayloadResolverV03 {
    objects: BTreeMap<String, Vec<u8>>,
    granted: BTreeSet<String>,
}

impl InMemoryFieldPayloadResolverV03 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_immutable(
        &mut self,
        object_id: &str,
        bytes: Vec<u8>,
    ) -> Result<(), DataResolutionErrorV03> {
        if !is_canonical_reference(object_id) || !object_id.starts_with("data.") {
            return Err(DataResolutionErrorV03::InvalidReference(
                object_id.to_owned(),
            ));
        }
        if let Some(existing) = self.objects.get(object_id) {
            if *existing != bytes {
                return Err(DataResolutionErrorV03::ImmutableObjectConflict(
                    object_id.to_owned(),
                ));
            }
            return Ok(());
        }
        self.objects.insert(object_id.to_owned(), bytes);
        Ok(())
    }

    /// Explicit authorization grant, independent of whether the object still exists.
    pub fn grant(&mut self, object_id: &str) -> Result<(), DataResolutionErrorV03> {
        if !is_canonical_reference(object_id) || !object_id.starts_with("data.") {
            return Err(DataResolutionErrorV03::InvalidReference(
                object_id.to_owned(),
            ));
        }
        self.granted.insert(object_id.to_owned());
        Ok(())
    }

    pub fn revoke(&mut self, object_id: &str) {
        self.granted.remove(object_id);
    }

    /// Models retention/availability loss after a result has been published.
    pub fn remove(&mut self, object_id: &str) {
        self.objects.remove(object_id);
    }
}

impl FieldPayloadResolverV03 for InMemoryFieldPayloadResolverV03 {
    fn resolve_bytes(&self, object_id: &str) -> Result<Vec<u8>, DataResolutionErrorV03> {
        if !is_canonical_reference(object_id) || !object_id.starts_with("data.") {
            return Err(DataResolutionErrorV03::InvalidReference(
                object_id.to_owned(),
            ));
        }
        if !self.granted.contains(object_id) {
            return Err(DataResolutionErrorV03::AccessDenied(object_id.to_owned()));
        }
        self.objects
            .get(object_id)
            .cloned()
            .ok_or_else(|| DataResolutionErrorV03::ObjectNotFound(object_id.to_owned()))
    }
}

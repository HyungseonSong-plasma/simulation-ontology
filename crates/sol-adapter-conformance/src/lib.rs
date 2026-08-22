#![forbid(unsafe_code)]

//! Boundary types for reusable Adapter Protocol and Public Contract conformance tooling.
//!
//! This crate models what a conformance runner may determine. It intentionally does not
//! define process invocation, a command-line interface, serialized report output, or
//! backend-native physical and numerical validation.

use sol_adapter_protocol::ProtocolOperation;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublishedContract {
    AdapterProtocol,
    PublicContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceScope {
    Compatibility(PublishedContract),
    Schema(PublishedContract),
    Fixture(PublishedContract),
    OperationSemantics(ProtocolOperation),
    Scheduling,
    FailureAndReplay,
    Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConformanceCaseId(String);

impl ConformanceCaseId {
    pub fn new(value: impl Into<String>) -> Result<Self, ConformanceModelError> {
        let value = value.into();
        require_non_blank("conformance case id", &value)?;
        if value.trim() != value {
            return Err(ConformanceModelError::SurroundingWhitespace(
                "conformance case id",
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ConformanceCaseId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceEvidence(String);

impl ConformanceEvidence {
    pub fn new(value: impl Into<String>) -> Result<Self, ConformanceModelError> {
        let value = value.into();
        require_non_blank("conformance evidence", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceViolation(String);

impl ConformanceViolation {
    pub fn new(value: impl Into<String>) -> Result<Self, ConformanceModelError> {
        let value = value.into();
        require_non_blank("conformance violation", &value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HarnessFailureKind {
    FixtureLoad,
    AdapterInvocation,
    TransportExchange,
    ResultDecoding,
    RunnerInvariant,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessFailure {
    kind: HarnessFailureKind,
    detail: String,
}

impl HarnessFailure {
    pub fn new(
        kind: HarnessFailureKind,
        detail: impl Into<String>,
    ) -> Result<Self, ConformanceModelError> {
        let detail = detail.into();
        require_non_blank("harness failure detail", &detail)?;
        Ok(Self { kind, detail })
    }

    pub const fn kind(&self) -> HarnessFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConformanceCaseResult {
    Conformant(ConformanceEvidence),
    NonConformant(ConformanceViolation),
    HarnessFailure(HarnessFailure),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceCaseRecord {
    id: ConformanceCaseId,
    scope: ConformanceScope,
    result: ConformanceCaseResult,
}

impl ConformanceCaseRecord {
    pub const fn new(
        id: ConformanceCaseId,
        scope: ConformanceScope,
        result: ConformanceCaseResult,
    ) -> Self {
        Self { id, scope, result }
    }

    pub const fn id(&self) -> &ConformanceCaseId {
        &self.id
    }

    pub const fn scope(&self) -> ConformanceScope {
        self.scope
    }

    pub const fn result(&self) -> &ConformanceCaseResult {
        &self.result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceDetermination {
    Conformant,
    NonConformant,
    NotEstablished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendValidationScope {
    NotAssessedByConformance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConformanceReport {
    cases: Vec<ConformanceCaseRecord>,
}

impl ConformanceReport {
    pub fn new(mut cases: Vec<ConformanceCaseRecord>) -> Result<Self, ConformanceModelError> {
        if cases.is_empty() {
            return Err(ConformanceModelError::EmptyReport);
        }

        cases.sort_by(|left, right| left.id.cmp(&right.id));
        if let Some(duplicate) = cases
            .windows(2)
            .find(|pair| pair[0].id == pair[1].id)
            .map(|pair| pair[0].id.as_str().to_owned())
        {
            return Err(ConformanceModelError::DuplicateCaseId(duplicate));
        }

        Ok(Self { cases })
    }

    pub fn cases(&self) -> &[ConformanceCaseRecord] {
        &self.cases
    }

    pub fn determination(&self) -> ConformanceDetermination {
        if self
            .cases
            .iter()
            .any(|case| matches!(&case.result, ConformanceCaseResult::NonConformant(_)))
        {
            ConformanceDetermination::NonConformant
        } else if self
            .cases
            .iter()
            .any(|case| matches!(&case.result, ConformanceCaseResult::HarnessFailure(_)))
        {
            ConformanceDetermination::NotEstablished
        } else {
            ConformanceDetermination::Conformant
        }
    }

    pub const fn backend_validation_scope(&self) -> BackendValidationScope {
        BackendValidationScope::NotAssessedByConformance
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConformanceModelError {
    EmptyField(&'static str),
    SurroundingWhitespace(&'static str),
    EmptyReport,
    DuplicateCaseId(String),
}

impl Display for ConformanceModelError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField(field) => write!(formatter, "{field} must not be blank"),
            Self::SurroundingWhitespace(field) => {
                write!(formatter, "{field} must not contain surrounding whitespace")
            }
            Self::EmptyReport => write!(formatter, "conformance report must contain a case"),
            Self::DuplicateCaseId(id) => {
                write!(formatter, "duplicate conformance case id: {id}")
            }
        }
    }
}

impl Error for ConformanceModelError {}

fn require_non_blank(field: &'static str, value: &str) -> Result<(), ConformanceModelError> {
    if value.trim().is_empty() {
        Err(ConformanceModelError::EmptyField(field))
    } else {
        Ok(())
    }
}

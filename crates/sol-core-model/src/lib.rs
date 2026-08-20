#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Version of the semantic-core bootstrap contract.
pub const CORE_BOOTSTRAP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Names that are reserved for backend-specific realization layers and must not
/// become dependencies of the solver-independent semantic Core.
pub const BACKEND_BOUNDARY_MARKERS: [&str; 4] = ["moose", "comsol", "ansys", "adapter-"];

/// Returns true when a manifest line appears to introduce a backend-specific
/// dependency into the semantic Core.
pub fn violates_backend_boundary(manifest_line: &str) -> bool {
    let normalized = manifest_line.trim().to_ascii_lowercase();

    if normalized.is_empty() || normalized.starts_with('#') {
        return false;
    }

    BACKEND_BOUNDARY_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

/// Top-level semantic categories defined by Core Simulation Ontology v0.1.
///
/// Phase 1 keeps these categories explicit while deferring canonical identity,
/// reference resolution, and cross-category constraints to later phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    PhysicsModel,
    MathematicalModel,
    ConstitutiveModel,
    SpatialModel,
    MaterialModel,
    ConditionModel,
    NumericalModel,
    ObservationModel,
    Analysis,
    SolverConfiguration,
}

/// A solver-independent semantic entity.
///
/// `id` is a document-level reference key in Phase 1. Canonical identity and
/// namespace semantics are introduced by the Phase 2 resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologyEntity {
    pub id: String,
    pub kind: EntityKind,
    pub semantic_type: String,
    pub label: String,
}

/// The model-definition side of a simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationModel {
    pub id: String,
    pub physics: Vec<OntologyEntity>,
    pub mathematical: Vec<OntologyEntity>,
    pub constitutive: Vec<OntologyEntity>,
    pub spatial: Vec<OntologyEntity>,
    pub material: Vec<OntologyEntity>,
    pub conditions: Vec<OntologyEntity>,
    pub numerical: Vec<OntologyEntity>,
    pub observations: Vec<OntologyEntity>,
}

/// The computational-experiment side of a simulation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationTask {
    pub id: String,
    pub analyses: Vec<OntologyEntity>,
    pub solver_configurations: Vec<OntologyEntity>,
}

/// Core semantic relation vocabulary from Ontology v0.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    RepresentedBy,
    ClosedBy,
    ParameterizedBy,
    DefinedOn,
    DiscretizedBy,
    AppliedTo,
    AnalyzedBy,
    SolvedBy,
    Produces,
    ObservedBy,
}

/// A first-class semantic graph edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticRelation {
    pub kind: RelationKind,
    pub source: String,
    pub target: String,
}

/// Solver-independent representation of one simulation definition and its tasks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Simulation {
    pub ontology_version: String,
    pub model: SimulationModel,
    pub tasks: Vec<SimulationTask>,
    pub relations: Vec<SemanticRelation>,
}

impl Simulation {
    /// Deserialize a simulation document from JSON.
    pub fn from_json(input: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(input)
    }

    /// Serialize a simulation document to stable human-readable JSON.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{violates_backend_boundary, RelationKind};

    #[test]
    fn backend_dependency_marker_is_detected() {
        assert!(violates_backend_boundary(
            "moose-adapter = { path = \"../moose\" }"
        ));
    }

    #[test]
    fn ordinary_core_dependency_is_allowed() {
        assert!(!violates_backend_boundary("serde = \"1\""));
    }

    #[test]
    fn relation_vocabulary_uses_stable_snake_case_names() {
        let json = serde_json::to_string(&RelationKind::RepresentedBy).unwrap();
        assert_eq!(json, "\"represented_by\"");
    }
}

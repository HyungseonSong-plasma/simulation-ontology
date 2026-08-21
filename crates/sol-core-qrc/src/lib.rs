#![forbid(unsafe_code)]

use sol_core_identity::{CanonicalId, ResolvedGraph, ResolvedNodeKind};
use sol_core_model::{EntityKind, RelationKind};

/// A solver-independent constraint evaluated against the resolved semantic graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// Requires the referenced canonical node to have the expected Core entity kind.
    EntityType { entity: CanonicalId, expected: EntityKind },
    /// Requires a specific semantic relation between two canonical nodes.
    Relation { source: CanonicalId, kind: RelationKind, target: CanonicalId },
    /// Requires a source node to have at least one outgoing relation of the specified kind to a target entity of the expected Core kind.
    RequiredRelation { source: CanonicalId, kind: RelationKind, target_kind: EntityKind },
    /// Restricts how many outgoing relations of a given kind may target a specific Core entity kind.
    RelationCardinality { source: CanonicalId, kind: RelationKind, target_kind: EntityKind, min: usize, max: Option<usize> },
    /// Requires every relation endpoint in the resolved graph to resolve to a canonical node.
    ReferenceIntegrity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintViolation {
    MissingEntity { entity: CanonicalId },
    TypeMismatch { entity: CanonicalId, expected: EntityKind, actual: ResolvedNodeKind },
    MissingRelation { source: CanonicalId, kind: RelationKind, target: CanonicalId },
    MissingRequiredRelation { source: CanonicalId, kind: RelationKind, target_kind: EntityKind },
    InvalidCardinalityBounds { min: usize, max: usize },
    CardinalityMismatch { source: CanonicalId, kind: RelationKind, target_kind: EntityKind, min: usize, max: Option<usize>, actual: usize },
    DanglingRelationEndpoint { relation_kind: RelationKind, endpoint: CanonicalId },
}

/// Evaluate one QRC constraint without introducing backend realization semantics.
pub fn evaluate(graph: &ResolvedGraph, constraint: &Constraint) -> Result<(), ConstraintViolation> {
    match constraint {
        Constraint::EntityType { entity, expected } => {
            let node = graph.resolve(entity).ok_or_else(|| ConstraintViolation::MissingEntity { entity: entity.clone() })?;
            if node.kind == ResolvedNodeKind::Entity(*expected) { Ok(()) } else { Err(ConstraintViolation::TypeMismatch { entity: entity.clone(), expected: *expected, actual: node.kind }) }
        }
        Constraint::Relation { source, kind, target } => {
            if graph.resolve(source).is_none() { return Err(ConstraintViolation::MissingEntity { entity: source.clone() }); }
            if graph.resolve(target).is_none() { return Err(ConstraintViolation::MissingEntity { entity: target.clone() }); }
            if graph.relations().iter().any(|relation| relation.source == *source && relation.kind == *kind && relation.target == *target) { Ok(()) } else { Err(ConstraintViolation::MissingRelation { source: source.clone(), kind: *kind, target: target.clone() }) }
        }
        Constraint::RequiredRelation { source, kind, target_kind } => {
            if graph.resolve(source).is_none() { return Err(ConstraintViolation::MissingEntity { entity: source.clone() }); }
            let exists = graph.relations().iter().any(|relation| relation.source == *source && relation.kind == *kind && graph.resolve(&relation.target).is_some_and(|target| target.kind == ResolvedNodeKind::Entity(*target_kind)));
            if exists { Ok(()) } else { Err(ConstraintViolation::MissingRequiredRelation { source: source.clone(), kind: *kind, target_kind: *target_kind }) }
        }
        Constraint::RelationCardinality { source, kind, target_kind, min, max } => {
            if graph.resolve(source).is_none() { return Err(ConstraintViolation::MissingEntity { entity: source.clone() }); }
            if let Some(maximum) = max { if maximum < min { return Err(ConstraintViolation::InvalidCardinalityBounds { min: *min, max: *maximum }); } }
            let actual = graph.relations().iter().filter(|relation| relation.source == *source && relation.kind == *kind && graph.resolve(&relation.target).is_some_and(|target| target.kind == ResolvedNodeKind::Entity(*target_kind))).count();
            let within_maximum = max.as_ref().map_or(true, |maximum| actual <= *maximum);
            if actual >= *min && within_maximum { Ok(()) } else { Err(ConstraintViolation::CardinalityMismatch { source: source.clone(), kind: *kind, target_kind: *target_kind, min: *min, max: *max, actual }) }
        }
        Constraint::ReferenceIntegrity => {
            for relation in graph.relations() {
                if graph.resolve(&relation.source).is_none() { return Err(ConstraintViolation::DanglingRelationEndpoint { relation_kind: relation.kind, endpoint: relation.source.clone() }); }
                if graph.resolve(&relation.target).is_none() { return Err(ConstraintViolation::DanglingRelationEndpoint { relation_kind: relation.kind, endpoint: relation.target.clone() }); }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate, Constraint, ConstraintViolation};
    use sol_core_identity::{CanonicalId, IdentityResolver, ResolveError};
    use sol_core_model::{EntityKind, OntologyEntity, RelationKind, SemanticRelation, Simulation, SimulationModel};

    fn thermal_simulation() -> Simulation {
        Simulation {
            ontology_version: "0.1".to_owned(),
            model: SimulationModel {
                id: "thermal.model".to_owned(),
                physics: vec![OntologyEntity { id: "thermal.heat_transfer".to_owned(), kind: EntityKind::PhysicsModel, semantic_type: "heat_transfer".to_owned(), label: "Heat Transfer".to_owned() }],
                mathematical: vec![OntologyEntity { id: "thermal.energy_conservation".to_owned(), kind: EntityKind::MathematicalModel, semantic_type: "energy_conservation".to_owned(), label: "Energy Conservation".to_owned() }],
                constitutive: vec![], spatial: vec![], material: vec![], conditions: vec![], numerical: vec![], observations: vec![],
            },
            tasks: vec![],
            relations: vec![SemanticRelation { kind: RelationKind::RepresentedBy, source: "thermal.heat_transfer".to_owned(), target: "thermal.energy_conservation".to_owned() }],
        }
    }

    fn thermal_graph() -> sol_core_identity::ResolvedGraph { IdentityResolver::resolve(&thermal_simulation()).unwrap() }

    #[test]
    fn thermal_mathematical_model_satisfies_type_constraint() {
        let graph = thermal_graph();
        let constraint = Constraint::EntityType { entity: "thermal.energy_conservation".parse().unwrap(), expected: EntityKind::MathematicalModel };
        assert_eq!(evaluate(&graph, &constraint), Ok(()));
    }

    #[test]
    fn physics_model_cannot_satisfy_mathematical_model_constraint() {
        let graph = thermal_graph();
        let entity: CanonicalId = "thermal.heat_transfer".parse().unwrap();
        let constraint = Constraint::EntityType { entity: entity.clone(), expected: EntityKind::MathematicalModel };
        assert_eq!(evaluate(&graph, &constraint), Err(ConstraintViolation::TypeMismatch { entity, expected: EntityKind::MathematicalModel, actual: sol_core_identity::ResolvedNodeKind::Entity(EntityKind::PhysicsModel) }));
    }

    #[test]
    fn unresolved_entity_cannot_satisfy_type_constraint() {
        let graph = thermal_graph();
        let entity: CanonicalId = "thermal.missing".parse().unwrap();
        let constraint = Constraint::EntityType { entity: entity.clone(), expected: EntityKind::MathematicalModel };
        assert_eq!(evaluate(&graph, &constraint), Err(ConstraintViolation::MissingEntity { entity }));
    }

    #[test]
    fn thermal_represented_by_relation_satisfies_relation_constraint() {
        let graph = thermal_graph();
        let constraint = Constraint::Relation { source: "thermal.heat_transfer".parse().unwrap(), kind: RelationKind::RepresentedBy, target: "thermal.energy_conservation".parse().unwrap() };
        assert_eq!(evaluate(&graph, &constraint), Ok(()));
    }

    #[test]
    fn wrong_relation_kind_is_rejected() {
        let graph = thermal_graph();
        let source: CanonicalId = "thermal.heat_transfer".parse().unwrap();
        let target: CanonicalId = "thermal.energy_conservation".parse().unwrap();
        let constraint = Constraint::Relation { source: source.clone(), kind: RelationKind::ClosedBy, target: target.clone() };
        assert_eq!(evaluate(&graph, &constraint), Err(ConstraintViolation::MissingRelation { source, kind: RelationKind::ClosedBy, target }));
    }

    #[test]
    fn thermal_physics_requires_mathematical_representation() {
        let graph = thermal_graph();
        let constraint = Constraint::RequiredRelation { source: "thermal.heat_transfer".parse().unwrap(), kind: RelationKind::RepresentedBy, target_kind: EntityKind::MathematicalModel };
        assert_eq!(evaluate(&graph, &constraint), Ok(()));
    }

    #[test]
    fn missing_required_relation_is_rejected() {
        let graph = thermal_graph();
        let source: CanonicalId = "thermal.heat_transfer".parse().unwrap();
        let constraint = Constraint::RequiredRelation { source: source.clone(), kind: RelationKind::ClosedBy, target_kind: EntityKind::ConstitutiveModel };
        assert_eq!(evaluate(&graph, &constraint), Err(ConstraintViolation::MissingRequiredRelation { source, kind: RelationKind::ClosedBy, target_kind: EntityKind::ConstitutiveModel }));
    }

    #[test]
    fn thermal_represented_by_cardinality_is_exactly_one() {
        let graph = thermal_graph();
        let constraint = Constraint::RelationCardinality { source: "thermal.heat_transfer".parse().unwrap(), kind: RelationKind::RepresentedBy, target_kind: EntityKind::MathematicalModel, min: 1, max: Some(1) };
        assert_eq!(evaluate(&graph, &constraint), Ok(()));
    }

    #[test]
    fn relation_cardinality_violation_is_rejected() {
        let graph = thermal_graph();
        let source: CanonicalId = "thermal.heat_transfer".parse().unwrap();
        let constraint = Constraint::RelationCardinality { source: source.clone(), kind: RelationKind::RepresentedBy, target_kind: EntityKind::MathematicalModel, min: 2, max: Some(2) };
        assert_eq!(evaluate(&graph, &constraint), Err(ConstraintViolation::CardinalityMismatch { source, kind: RelationKind::RepresentedBy, target_kind: EntityKind::MathematicalModel, min: 2, max: Some(2), actual: 1 }));
    }

    #[test]
    fn invalid_cardinality_bounds_are_rejected() {
        let graph = thermal_graph();
        let constraint = Constraint::RelationCardinality { source: "thermal.heat_transfer".parse().unwrap(), kind: RelationKind::RepresentedBy, target_kind: EntityKind::MathematicalModel, min: 2, max: Some(1) };
        assert_eq!(evaluate(&graph, &constraint), Err(ConstraintViolation::InvalidCardinalityBounds { min: 2, max: 1 }));
    }

    #[test]
    fn thermal_graph_has_reference_integrity() {
        let graph = thermal_graph();
        assert_eq!(evaluate(&graph, &Constraint::ReferenceIntegrity), Ok(()));
    }

    #[test]
    fn unresolved_relation_endpoint_is_rejected_before_qrc_evaluation() {
        let mut simulation = thermal_simulation();
        simulation.relations[0].target = "thermal.missing".to_owned();
        let error = IdentityResolver::resolve(&simulation).unwrap_err();
        assert!(matches!(error, ResolveError::UnresolvedEndpoint { .. }));
    }
}

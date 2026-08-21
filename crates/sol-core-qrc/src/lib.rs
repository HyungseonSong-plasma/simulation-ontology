#![forbid(unsafe_code)]

use sol_core_identity::{CanonicalId, ResolvedGraph, ResolvedNodeKind};
use sol_core_model::{EntityKind, RelationKind};

/// A solver-independent constraint evaluated against the resolved semantic graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Constraint {
    /// Requires the referenced canonical node to have the expected Core entity kind.
    EntityType {
        entity: CanonicalId,
        expected: EntityKind,
    },
    /// Requires a specific semantic relation between two canonical nodes.
    Relation {
        source: CanonicalId,
        kind: RelationKind,
        target: CanonicalId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintViolation {
    MissingEntity {
        entity: CanonicalId,
    },
    TypeMismatch {
        entity: CanonicalId,
        expected: EntityKind,
        actual: ResolvedNodeKind,
    },
    MissingRelation {
        source: CanonicalId,
        kind: RelationKind,
        target: CanonicalId,
    },
}

/// Evaluate one QRC constraint without introducing backend realization semantics.
pub fn evaluate(graph: &ResolvedGraph, constraint: &Constraint) -> Result<(), ConstraintViolation> {
    match constraint {
        Constraint::EntityType { entity, expected } => {
            let node = graph
                .resolve(entity)
                .ok_or_else(|| ConstraintViolation::MissingEntity {
                    entity: entity.clone(),
                })?;

            if node.kind == ResolvedNodeKind::Entity(*expected) {
                Ok(())
            } else {
                Err(ConstraintViolation::TypeMismatch {
                    entity: entity.clone(),
                    expected: *expected,
                    actual: node.kind,
                })
            }
        }
        Constraint::Relation {
            source,
            kind,
            target,
        } => {
            if graph.resolve(source).is_none() {
                return Err(ConstraintViolation::MissingEntity {
                    entity: source.clone(),
                });
            }
            if graph.resolve(target).is_none() {
                return Err(ConstraintViolation::MissingEntity {
                    entity: target.clone(),
                });
            }

            let relation_exists = graph.relations().iter().any(|relation| {
                relation.source == *source && relation.kind == *kind && relation.target == *target
            });

            if relation_exists {
                Ok(())
            } else {
                Err(ConstraintViolation::MissingRelation {
                    source: source.clone(),
                    kind: *kind,
                    target: target.clone(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate, Constraint, ConstraintViolation};
    use sol_core_identity::{CanonicalId, IdentityResolver};
    use sol_core_model::{
        EntityKind, OntologyEntity, RelationKind, SemanticRelation, Simulation, SimulationModel,
    };

    fn thermal_graph() -> sol_core_identity::ResolvedGraph {
        let simulation = Simulation {
            ontology_version: "0.1".to_owned(),
            model: SimulationModel {
                id: "thermal.model".to_owned(),
                physics: vec![OntologyEntity {
                    id: "thermal.heat_transfer".to_owned(),
                    kind: EntityKind::PhysicsModel,
                    semantic_type: "heat_transfer".to_owned(),
                    label: "Heat Transfer".to_owned(),
                }],
                mathematical: vec![OntologyEntity {
                    id: "thermal.energy_conservation".to_owned(),
                    kind: EntityKind::MathematicalModel,
                    semantic_type: "energy_conservation".to_owned(),
                    label: "Energy Conservation".to_owned(),
                }],
                constitutive: vec![],
                spatial: vec![],
                material: vec![],
                conditions: vec![],
                numerical: vec![],
                observations: vec![],
            },
            tasks: vec![],
            relations: vec![SemanticRelation {
                kind: RelationKind::RepresentedBy,
                source: "thermal.heat_transfer".to_owned(),
                target: "thermal.energy_conservation".to_owned(),
            }],
        };
        IdentityResolver::resolve(&simulation).unwrap()
    }

    #[test]
    fn thermal_mathematical_model_satisfies_type_constraint() {
        let graph = thermal_graph();
        let constraint = Constraint::EntityType {
            entity: "thermal.energy_conservation".parse().unwrap(),
            expected: EntityKind::MathematicalModel,
        };
        assert_eq!(evaluate(&graph, &constraint), Ok(()));
    }

    #[test]
    fn physics_model_cannot_satisfy_mathematical_model_constraint() {
        let graph = thermal_graph();
        let entity: CanonicalId = "thermal.heat_transfer".parse().unwrap();
        let constraint = Constraint::EntityType {
            entity: entity.clone(),
            expected: EntityKind::MathematicalModel,
        };
        assert_eq!(
            evaluate(&graph, &constraint),
            Err(ConstraintViolation::TypeMismatch {
                entity,
                expected: EntityKind::MathematicalModel,
                actual: sol_core_identity::ResolvedNodeKind::Entity(EntityKind::PhysicsModel),
            })
        );
    }

    #[test]
    fn unresolved_entity_cannot_satisfy_type_constraint() {
        let graph = thermal_graph();
        let entity: CanonicalId = "thermal.missing".parse().unwrap();
        let constraint = Constraint::EntityType {
            entity: entity.clone(),
            expected: EntityKind::MathematicalModel,
        };
        assert_eq!(
            evaluate(&graph, &constraint),
            Err(ConstraintViolation::MissingEntity { entity })
        );
    }

    #[test]
    fn thermal_represented_by_relation_satisfies_relation_constraint() {
        let graph = thermal_graph();
        let constraint = Constraint::Relation {
            source: "thermal.heat_transfer".parse().unwrap(),
            kind: RelationKind::RepresentedBy,
            target: "thermal.energy_conservation".parse().unwrap(),
        };
        assert_eq!(evaluate(&graph, &constraint), Ok(()));
    }

    #[test]
    fn wrong_relation_kind_is_rejected() {
        let graph = thermal_graph();
        let source: CanonicalId = "thermal.heat_transfer".parse().unwrap();
        let target: CanonicalId = "thermal.energy_conservation".parse().unwrap();
        let constraint = Constraint::Relation {
            source: source.clone(),
            kind: RelationKind::ClosedBy,
            target: target.clone(),
        };
        assert_eq!(
            evaluate(&graph, &constraint),
            Err(ConstraintViolation::MissingRelation {
                source,
                kind: RelationKind::ClosedBy,
                target,
            })
        );
    }
}

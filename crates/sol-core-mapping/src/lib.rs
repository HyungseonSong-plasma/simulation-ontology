#![forbid(unsafe_code)]

use sol_core_identity::{CanonicalId, ResolvedGraph, ResolvedNodeKind};
use sol_core_model::{EntityKind, RelationKind};

/// Solver-independent semantic subject pattern that a mapping rule can match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MappingSubjectPattern {
    Entity(EntityKind),
    Relation(RelationKind),
}

/// Declarative rule describing when a semantic subject may be realized by a
/// backend capability. The rule carries no backend-native object or solver API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingRule {
    pub id: String,
    pub subject: MappingSubjectPattern,
    pub required_capability: String,
}

impl MappingRule {
    pub fn new(
        id: impl Into<String>,
        subject: MappingSubjectPattern,
        required_capability: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            subject,
            required_capability: required_capability.into(),
        }
    }
}

/// Concrete semantic subject referenced by a mapping claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MappingSubjectRef {
    Entity(CanonicalId),
    Relation {
        source: CanonicalId,
        kind: RelationKind,
        target: CanonicalId,
    },
}

/// Concrete assertion that a mapping rule applies to a semantic subject.
/// Backend-native identity and realization effects are intentionally absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingClaim {
    pub rule_id: String,
    pub subject: MappingSubjectRef,
}

impl MappingClaim {
    pub fn new(rule_id: impl Into<String>, subject: MappingSubjectRef) -> Self {
        Self {
            rule_id: rule_id.into(),
            subject,
        }
    }
}

/// Evaluate whether a rule's semantic pattern applies to a resolved Core
/// subject. Backend capability availability is evaluated later by adapters.
pub fn is_applicable(
    rule: &MappingRule,
    graph: &ResolvedGraph,
    subject: &MappingSubjectRef,
) -> bool {
    match (&rule.subject, subject) {
        (MappingSubjectPattern::Entity(expected), MappingSubjectRef::Entity(id)) => graph
            .resolve(id)
            .is_some_and(|node| node.kind == ResolvedNodeKind::Entity(*expected)),
        (
            MappingSubjectPattern::Relation(expected),
            MappingSubjectRef::Relation {
                source,
                kind,
                target,
            },
        ) => {
            *kind == *expected
                && graph.relations().iter().any(|relation| {
                    relation.source == *source
                        && relation.kind == *kind
                        && relation.target == *target
                })
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        is_applicable, MappingClaim, MappingRule, MappingSubjectPattern, MappingSubjectRef,
    };
    use sol_core_identity::IdentityResolver;
    use sol_core_model::{
        EntityKind, OntologyEntity, RelationKind, SemanticRelation, Simulation, SimulationModel,
    };

    fn thermal_graph() -> sol_core_identity::ResolvedGraph {
        let simulation = Simulation {
            ontology_version: "0.1".to_owned(),
            model: SimulationModel {
                id: "model.thermal_reference".to_owned(),
                physics: vec![OntologyEntity {
                    id: "thermal.transport".to_owned(),
                    kind: EntityKind::PhysicsModel,
                    semantic_type: "ThermalTransport".to_owned(),
                    label: "Thermal transport".to_owned(),
                }],
                mathematical: vec![OntologyEntity {
                    id: "thermal.energy_conservation".to_owned(),
                    kind: EntityKind::MathematicalModel,
                    semantic_type: "Equation".to_owned(),
                    label: "Energy conservation".to_owned(),
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
                source: "thermal.transport".to_owned(),
                target: "thermal.energy_conservation".to_owned(),
            }],
        };

        IdentityResolver::resolve(&simulation).unwrap()
    }

    #[test]
    fn thermal_equation_rule_is_solver_independent() {
        let rule = MappingRule::new(
            "thermal.energy-equation",
            MappingSubjectPattern::Entity(EntityKind::MathematicalModel),
            "equation.realization",
        );

        assert_eq!(rule.id, "thermal.energy-equation");
        assert_eq!(
            rule.subject,
            MappingSubjectPattern::Entity(EntityKind::MathematicalModel)
        );
        assert_eq!(rule.required_capability, "equation.realization");
    }

    #[test]
    fn relation_rule_preserves_semantic_relation_kind() {
        let rule = MappingRule::new(
            "thermal.represented-by",
            MappingSubjectPattern::Relation(RelationKind::RepresentedBy),
            "semantic-relation.realization",
        );

        assert_eq!(
            rule.subject,
            MappingSubjectPattern::Relation(RelationKind::RepresentedBy)
        );
    }

    #[test]
    fn thermal_entity_claim_references_canonical_semantic_subject() {
        let claim = MappingClaim::new(
            "thermal.energy-equation",
            MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap()),
        );

        assert_eq!(claim.rule_id, "thermal.energy-equation");
        assert_eq!(
            claim.subject,
            MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap())
        );
    }

    #[test]
    fn thermal_relation_claim_preserves_canonical_endpoints() {
        let claim = MappingClaim::new(
            "thermal.represented-by",
            MappingSubjectRef::Relation {
                source: "thermal.transport".parse().unwrap(),
                kind: RelationKind::RepresentedBy,
                target: "thermal.energy_conservation".parse().unwrap(),
            },
        );

        assert_eq!(
            claim.subject,
            MappingSubjectRef::Relation {
                source: "thermal.transport".parse().unwrap(),
                kind: RelationKind::RepresentedBy,
                target: "thermal.energy_conservation".parse().unwrap(),
            }
        );
    }

    #[test]
    fn thermal_mathematical_model_matches_entity_rule() {
        let graph = thermal_graph();
        let rule = MappingRule::new(
            "thermal.energy-equation",
            MappingSubjectPattern::Entity(EntityKind::MathematicalModel),
            "equation.realization",
        );
        let subject = MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap());

        assert!(is_applicable(&rule, &graph, &subject));
    }

    #[test]
    fn physics_model_does_not_match_mathematical_rule() {
        let graph = thermal_graph();
        let rule = MappingRule::new(
            "thermal.energy-equation",
            MappingSubjectPattern::Entity(EntityKind::MathematicalModel),
            "equation.realization",
        );
        let subject = MappingSubjectRef::Entity("thermal.transport".parse().unwrap());

        assert!(!is_applicable(&rule, &graph, &subject));
    }

    #[test]
    fn thermal_represented_by_edge_matches_relation_rule() {
        let graph = thermal_graph();
        let rule = MappingRule::new(
            "thermal.represented-by",
            MappingSubjectPattern::Relation(RelationKind::RepresentedBy),
            "semantic-relation.realization",
        );
        let subject = MappingSubjectRef::Relation {
            source: "thermal.transport".parse().unwrap(),
            kind: RelationKind::RepresentedBy,
            target: "thermal.energy_conservation".parse().unwrap(),
        };

        assert!(is_applicable(&rule, &graph, &subject));
    }

    #[test]
    fn missing_relation_does_not_match_relation_rule() {
        let graph = thermal_graph();
        let rule = MappingRule::new(
            "thermal.closed-by",
            MappingSubjectPattern::Relation(RelationKind::ClosedBy),
            "semantic-relation.realization",
        );
        let subject = MappingSubjectRef::Relation {
            source: "thermal.transport".parse().unwrap(),
            kind: RelationKind::ClosedBy,
            target: "thermal.energy_conservation".parse().unwrap(),
        };

        assert!(!is_applicable(&rule, &graph, &subject));
    }
}

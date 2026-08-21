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

/// Evidence supporting why a concrete mapping claim is asserted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingEvidence {
    pub source: String,
    pub detail: String,
}

impl MappingEvidence {
    pub fn new(source: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            detail: detail.into(),
        }
    }
}

/// Provenance describing who or what produced a mapping claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingProvenance {
    pub producer: String,
    pub revision: Option<String>,
}

impl MappingProvenance {
    pub fn new(producer: impl Into<String>, revision: Option<String>) -> Self {
        Self {
            producer: producer.into(),
            revision,
        }
    }
}

/// Concrete assertion that a mapping rule applies to a semantic subject.
/// Backend-native identity and realization effects are intentionally absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingClaim {
    pub rule_id: String,
    pub subject: MappingSubjectRef,
    pub evidence: Vec<MappingEvidence>,
    pub provenance: Option<MappingProvenance>,
}

impl MappingClaim {
    pub fn new(rule_id: impl Into<String>, subject: MappingSubjectRef) -> Self {
        Self {
            rule_id: rule_id.into(),
            subject,
            evidence: Vec::new(),
            provenance: None,
        }
    }

    pub fn with_evidence(mut self, evidence: MappingEvidence) -> Self {
        self.evidence.push(evidence);
        self
    }

    pub fn with_provenance(mut self, provenance: MappingProvenance) -> Self {
        self.provenance = Some(provenance);
        self
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

/// Generate mapping claims in deterministic rule/subject order from the
/// resolved semantic graph. Capability availability is not decided here.
pub fn generate_mapping_claims(
    graph: &ResolvedGraph,
    rules: &[MappingRule],
) -> Vec<MappingClaim> {
    let mut ordered_rules: Vec<&MappingRule> = rules.iter().collect();
    ordered_rules.sort_by(|left, right| left.id.cmp(&right.id));

    let mut claims = Vec::new();

    for rule in ordered_rules {
        let mut candidates: Vec<(String, MappingSubjectRef)> = match rule.subject {
            MappingSubjectPattern::Entity(expected_kind) => graph
                .nodes()
                .iter()
                .filter_map(|(id, node)| {
                    if node.kind == ResolvedNodeKind::Entity(expected_kind) {
                        Some((format!("entity:{id}"), MappingSubjectRef::Entity(id.clone())))
                    } else {
                        None
                    }
                })
                .collect(),
            MappingSubjectPattern::Relation(expected_kind) => graph
                .relations()
                .iter()
                .filter_map(|relation| {
                    if relation.kind == expected_kind {
                        Some((
                            format!(
                                "relation:{}:{:?}:{}",
                                relation.source, relation.kind, relation.target
                            ),
                            MappingSubjectRef::Relation {
                                source: relation.source.clone(),
                                kind: relation.kind,
                                target: relation.target.clone(),
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect(),
        };

        candidates.sort_by(|left, right| left.0.cmp(&right.0));

        for (_, subject) in candidates {
            if is_applicable(rule, graph, &subject) {
                claims.push(
                    MappingClaim::new(&rule.id, subject)
                        .with_evidence(MappingEvidence::new(
                            "semantic-applicability",
                            format!("matched rule {}", rule.id),
                        ))
                        .with_provenance(MappingProvenance::new(
                            "sol-core-mapping",
                            Some(env!("CARGO_PKG_VERSION").to_owned()),
                        )),
                );
            }
        }
    }

    claims
}

#[cfg(test)]
mod tests {
    use super::{
        generate_mapping_claims, is_applicable, MappingClaim, MappingEvidence, MappingProvenance,
        MappingRule, MappingSubjectPattern, MappingSubjectRef,
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
    fn thermal_claim_carries_evidence_and_provenance() {
        let claim = MappingClaim::new(
            "thermal.energy-equation",
            MappingSubjectRef::Entity("thermal.energy_conservation".parse().unwrap()),
        )
        .with_evidence(MappingEvidence::new(
            "thermal-reference",
            "mathematical model matches equation realization rule",
        ))
        .with_provenance(MappingProvenance::new(
            "sol-core-mapping",
            Some("0.1".to_owned()),
        ));

        assert_eq!(claim.evidence.len(), 1);
        assert_eq!(claim.evidence[0].source, "thermal-reference");
        assert_eq!(
            claim.provenance,
            Some(MappingProvenance::new(
                "sol-core-mapping",
                Some("0.1".to_owned())
            ))
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

    #[test]
    fn thermal_mapping_claims_are_deterministic() {
        let graph = thermal_graph();
        let rules = vec![
            MappingRule::new(
                "z.represented-by",
                MappingSubjectPattern::Relation(RelationKind::RepresentedBy),
                "semantic-relation.realization",
            ),
            MappingRule::new(
                "a.energy-equation",
                MappingSubjectPattern::Entity(EntityKind::MathematicalModel),
                "equation.realization",
            ),
        ];

        let first = generate_mapping_claims(&graph, &rules);
        let second = generate_mapping_claims(&graph, &rules);

        assert_eq!(first, second);
        assert_eq!(first.len(), 2);
        assert_eq!(first[0].rule_id, "a.energy-equation");
        assert_eq!(first[1].rule_id, "z.represented-by");
        assert_eq!(first[0].evidence[0].source, "semantic-applicability");
        assert_eq!(
            first[0].provenance.as_ref().map(|value| value.producer.as_str()),
            Some("sol-core-mapping")
        );
    }
}

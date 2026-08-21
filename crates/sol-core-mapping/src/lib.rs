#![forbid(unsafe_code)]

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

#[cfg(test)]
mod tests {
    use super::{MappingRule, MappingSubjectPattern};
    use sol_core_model::{EntityKind, RelationKind};

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
}

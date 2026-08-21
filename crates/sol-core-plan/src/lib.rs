#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

/// Stable, solver-independent identifier for an action in a mapping plan.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlanActionId(String);

impl PlanActionId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A semantic planning action. Dependencies name actions that must precede it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanAction {
    pub id: PlanActionId,
    pub dependencies: BTreeSet<PlanActionId>,
}

impl PlanAction {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: PlanActionId::new(id),
            dependencies: BTreeSet::new(),
        }
    }

    pub fn depends_on(mut self, dependency: impl Into<String>) -> Self {
        self.dependencies.insert(PlanActionId::new(dependency));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    DuplicateAction(PlanActionId),
    MissingDependency {
        action: PlanActionId,
        dependency: PlanActionId,
    },
    CycleDetected(Vec<PlanActionId>),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MappingPlan {
    actions: BTreeMap<PlanActionId, PlanAction>,
}

impl MappingPlan {
    pub fn from_actions(actions: impl IntoIterator<Item = PlanAction>) -> Result<Self, PlanError> {
        let mut plan = Self::default();
        for action in actions {
            if plan
                .actions
                .insert(action.id.clone(), action.clone())
                .is_some()
            {
                return Err(PlanError::DuplicateAction(action.id));
            }
        }
        plan.validate_dependencies()?;
        plan.topological_order()?;
        Ok(plan)
    }

    pub fn topological_order(&self) -> Result<Vec<PlanActionId>, PlanError> {
        self.validate_dependencies()?;
        let mut remaining: BTreeMap<_, _> = self
            .actions
            .iter()
            .map(|(id, action)| (id.clone(), action.dependencies.clone()))
            .collect();
        let mut ordered = Vec::with_capacity(remaining.len());

        while !remaining.is_empty() {
            let ready: Vec<_> = remaining
                .iter()
                .filter(|(_, dependencies)| dependencies.is_empty())
                .map(|(id, _)| id.clone())
                .collect();

            if ready.is_empty() {
                return Err(PlanError::CycleDetected(
                    remaining.keys().cloned().collect(),
                ));
            }

            for id in ready {
                remaining.remove(&id);
                for dependencies in remaining.values_mut() {
                    dependencies.remove(&id);
                }
                ordered.push(id);
            }
        }

        Ok(ordered)
    }

    fn validate_dependencies(&self) -> Result<(), PlanError> {
        for action in self.actions.values() {
            for dependency in &action.dependencies {
                if !self.actions.contains_key(dependency) {
                    return Err(PlanError::MissingDependency {
                        action: action.id.clone(),
                        dependency: dependency.clone(),
                    });
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{MappingPlan, PlanAction, PlanActionId, PlanError};

    #[test]
    fn thermal_plan_orders_dependencies_deterministically() {
        let plan = MappingPlan::from_actions([
            PlanAction::new("thermal.solve").depends_on("thermal.material"),
            PlanAction::new("thermal.material").depends_on("thermal.domain"),
            PlanAction::new("thermal.domain"),
        ])
        .unwrap();

        assert_eq!(
            plan.topological_order().unwrap(),
            vec![
                PlanActionId::new("thermal.domain"),
                PlanActionId::new("thermal.material"),
                PlanActionId::new("thermal.solve"),
            ]
        );
    }

    #[test]
    fn cyclic_plan_is_rejected() {
        let error = MappingPlan::from_actions([
            PlanAction::new("thermal.material").depends_on("thermal.solve"),
            PlanAction::new("thermal.solve").depends_on("thermal.material"),
        ])
        .unwrap_err();

        assert!(matches!(error, PlanError::CycleDetected(_)));
    }
}

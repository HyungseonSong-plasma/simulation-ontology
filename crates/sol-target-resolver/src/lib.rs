#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use sol_core_plan::MappingPlan;
use sol_mock_adapter::{Adapter, AdapterCapability};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackendTarget(String);

impl BackendTarget {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn mock() -> Self {
        Self::new("mock")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BackendCapability(String);

impl BackendCapability {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&AdapterCapability> for BackendCapability {
    fn from(value: &AdapterCapability) -> Self {
        Self::new(value.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterDescriptor {
    pub id: String,
    pub target: BackendTarget,
    pub capabilities: BTreeSet<BackendCapability>,
}

impl AdapterDescriptor {
    pub fn from_adapter(
        id: impl Into<String>,
        target: BackendTarget,
        adapter: &impl Adapter,
    ) -> Self {
        Self {
            id: id.into(),
            target,
            capabilities: adapter
                .capabilities()
                .iter()
                .map(BackendCapability::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveTargetError {
    InvalidPlan,
    NoCompatibleAdapter,
}

pub fn resolve_target<'a>(
    target: &BackendTarget,
    plan: &MappingPlan,
    required: &BTreeSet<BackendCapability>,
    descriptors: &'a [AdapterDescriptor],
) -> Result<&'a AdapterDescriptor, ResolveTargetError> {
    plan.topological_order()
        .map_err(|_| ResolveTargetError::InvalidPlan)?;

    descriptors
        .iter()
        .filter(|descriptor| &descriptor.target == target)
        .filter(|descriptor| required.is_subset(&descriptor.capabilities))
        .min_by(|left, right| left.id.cmp(&right.id))
        .ok_or(ResolveTargetError::NoCompatibleAdapter)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use sol_core_plan::{MappingPlan, PlanAction};
    use sol_mock_adapter::MockAdapter;

    use super::{
        resolve_target, AdapterDescriptor, BackendCapability, BackendTarget, ResolveTargetError,
    };

    fn thermal_plan() -> MappingPlan {
        MappingPlan::from_actions([
            PlanAction::new("thermal.domain"),
            PlanAction::new("thermal.material").depends_on("thermal.domain"),
            PlanAction::new("thermal.solve").depends_on("thermal.material"),
        ])
        .unwrap()
    }

    fn thermal_requirements() -> BTreeSet<BackendCapability> {
        [
            BackendCapability::new("thermal.domain"),
            BackendCapability::new("thermal.material"),
            BackendCapability::new("thermal.solve"),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn adapter_descriptor_is_derived_from_mock_adapter_capabilities() {
        let adapter = MockAdapter::thermal();
        let descriptor =
            AdapterDescriptor::from_adapter("mock.thermal", BackendTarget::mock(), &adapter);
        assert!(descriptor
            .capabilities
            .contains(&BackendCapability::new("thermal.solve")));
    }

    #[test]
    fn thermal_plan_selects_mock_adapter_deterministically() {
        let adapter = MockAdapter::thermal();
        let descriptors = vec![
            AdapterDescriptor::from_adapter("mock.z-secondary", BackendTarget::mock(), &adapter),
            AdapterDescriptor::from_adapter("mock.a-primary", BackendTarget::mock(), &adapter),
        ];

        let selected = resolve_target(
            &BackendTarget::mock(),
            &thermal_plan(),
            &thermal_requirements(),
            &descriptors,
        )
        .unwrap();

        assert_eq!(selected.id, "mock.a-primary");
    }

    #[test]
    fn incompatible_capabilities_are_rejected() {
        let adapter = MockAdapter::thermal();
        let descriptors = vec![AdapterDescriptor::from_adapter(
            "mock.thermal",
            BackendTarget::mock(),
            &adapter,
        )];
        let required = [BackendCapability::new("thermal.radiation")]
            .into_iter()
            .collect();

        assert_eq!(
            resolve_target(
                &BackendTarget::mock(),
                &thermal_plan(),
                &required,
                &descriptors,
            ),
            Err(ResolveTargetError::NoCompatibleAdapter)
        );
    }
}

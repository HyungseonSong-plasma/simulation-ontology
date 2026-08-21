#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use sol_adapter_protocol::{AdapterDescription, TargetDeclaration};
use sol_core_plan::MappingPlan;
use sol_public_contract::BackendTargetDto;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterDescriptor {
    pub id: String,
    pub target: BackendTarget,
    pub capabilities: BTreeSet<BackendCapability>,
}

impl AdapterDescriptor {
    pub fn from_description(description: &AdapterDescription, target: &TargetDeclaration) -> Self {
        Self {
            id: description.bootstrap.adapter_id.clone(),
            target: BackendTarget::new(target.target.clone()),
            capabilities: target
                .capabilities
                .iter()
                .map(|capability| BackendCapability::new(capability.capability.clone()))
                .collect(),
        }
    }
}

pub fn descriptors_from_description(description: &AdapterDescription) -> Vec<AdapterDescriptor> {
    description
        .targets
        .iter()
        .map(|target| AdapterDescriptor::from_description(description, target))
        .collect()
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

pub fn resolve_public_target<'a>(
    target: &BackendTargetDto,
    descriptions: &'a [AdapterDescription],
) -> Result<(&'a AdapterDescription, &'a TargetDeclaration), ResolveTargetError> {
    let required: BTreeSet<_> = target.required_capabilities.iter().cloned().collect();

    descriptions
        .iter()
        .flat_map(|description| {
            description
                .targets
                .iter()
                .map(move |declaration| (description, declaration))
        })
        .filter(|(_, declaration)| declaration.target == target.target)
        .filter(|(_, declaration)| {
            let available: BTreeSet<_> = declaration
                .capabilities
                .iter()
                .map(|capability| capability.capability.clone())
                .collect();
            required.is_subset(&available)
        })
        .min_by(|(left, _), (right, _)| left.bootstrap.adapter_id.cmp(&right.bootstrap.adapter_id))
        .ok_or(ResolveTargetError::NoCompatibleAdapter)
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use sol_adapter_protocol::{
        AdapterBootstrap, AdapterDescription, CapabilityDeclaration, TargetDeclaration,
    };
    use sol_core_plan::{MappingPlan, PlanAction};
    use sol_public_contract::BackendTargetDto;

    use super::{
        descriptors_from_description, resolve_public_target, resolve_target, BackendCapability,
        BackendTarget, ResolveTargetError,
    };

    fn thermal_plan() -> MappingPlan {
        MappingPlan::from_actions([
            PlanAction::new("thermal.domain"),
            PlanAction::new("thermal.material").depends_on("thermal.domain"),
            PlanAction::new("thermal.solve").depends_on("thermal.material"),
        ])
        .unwrap()
    }

    fn description(id: &str, capabilities: &[&str]) -> AdapterDescription {
        AdapterDescription {
            bootstrap: AdapterBootstrap {
                adapter_id: id.to_owned(),
                adapter_version: "0.1.0".to_owned(),
                supported_adapter_protocol_versions: Some(vec!["0.1".to_owned()]),
                supported_public_contract_versions: Some(vec!["0.1".to_owned()]),
                extensions: BTreeMap::new(),
            },
            targets: vec![TargetDeclaration {
                target: "mock".to_owned(),
                capabilities: capabilities
                    .iter()
                    .map(|capability| CapabilityDeclaration {
                        capability: (*capability).to_owned(),
                        revision: None,
                        extensions: BTreeMap::new(),
                    })
                    .collect(),
                extensions: BTreeMap::new(),
            }],
            extensions: BTreeMap::new(),
        }
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
    fn descriptor_is_derived_from_protocol_description_not_mock_trait() {
        let description = description(
            "adapter.mock_thermal",
            &["thermal.domain", "thermal.material", "thermal.solve"],
        );
        let descriptors = descriptors_from_description(&description);

        assert_eq!(descriptors.len(), 1);
        assert!(descriptors[0]
            .capabilities
            .contains(&BackendCapability::new("thermal.solve")));
    }

    #[test]
    fn core_plan_selects_adapter_description_deterministically() {
        let first = description(
            "adapter.z_secondary",
            &["thermal.domain", "thermal.material", "thermal.solve"],
        );
        let second = description(
            "adapter.a_primary",
            &["thermal.domain", "thermal.material", "thermal.solve"],
        );
        let descriptors = [
            descriptors_from_description(&first).remove(0),
            descriptors_from_description(&second).remove(0),
        ];

        let selected = resolve_target(
            &BackendTarget::mock(),
            &thermal_plan(),
            &thermal_requirements(),
            &descriptors,
        )
        .unwrap();

        assert_eq!(selected.id, "adapter.a_primary");
    }

    #[test]
    fn public_target_resolution_uses_declared_target_and_capability_evidence() {
        let description = description(
            "adapter.mock_thermal",
            &["thermal.domain", "thermal.material", "thermal.solve"],
        );
        let target = BackendTargetDto {
            public_contract_version: "0.1".to_owned(),
            target: "mock".to_owned(),
            required_capabilities: vec!["thermal.domain".to_owned(), "thermal.solve".to_owned()],
            extensions: BTreeMap::new(),
        };

        let (selected, declaration) =
            resolve_public_target(&target, std::slice::from_ref(&description)).unwrap();
        assert_eq!(selected.bootstrap.adapter_id, "adapter.mock_thermal");
        assert_eq!(declaration.target, "mock");
    }

    #[test]
    fn incompatible_capabilities_are_rejected() {
        let description = description("adapter.mock_thermal", &["thermal.domain"]);
        let descriptors = descriptors_from_description(&description);
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

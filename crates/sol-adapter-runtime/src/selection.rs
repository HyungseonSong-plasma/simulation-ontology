use std::collections::BTreeSet;

use sol_adapter_protocol::{CompatibilityAssessment, CompatibilityOutcome};
use sol_target_resolver::{BackendCapability, BackendTarget};

use crate::{AdapterDiscoveryError, AdapterInstanceId, AdapterRegistrationId, RunningAdapter};

/// Solver-neutral projection of one target declaration observed from a live adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTargetProjection {
    target: BackendTarget,
    capabilities: BTreeSet<BackendCapability>,
}

impl RuntimeTargetProjection {
    pub fn target(&self) -> &BackendTarget {
        &self.target
    }

    pub fn capabilities(&self) -> &BTreeSet<BackendCapability> {
        &self.capabilities
    }
}

/// Solver-neutral runtime view suitable for later CLI/SDK/GUI projection.
///
/// Registration and instance IDs remain local operational identifiers. Adapter
/// implementation metadata, compatibility, target, and capability evidence all
/// originate from the live Adapter Protocol description rather than registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRuntimeProjection {
    registration_id: AdapterRegistrationId,
    instance_id: AdapterInstanceId,
    adapter_id: String,
    adapter_version: String,
    compatibility: CompatibilityAssessment,
    targets: Vec<RuntimeTargetProjection>,
}

impl AdapterRuntimeProjection {
    pub fn from_running(adapter: &RunningAdapter) -> Result<Self, AdapterDiscoveryError> {
        let evidence = adapter.discover_live_evidence()?;
        let targets = evidence
            .targets()
            .iter()
            .map(|declaration| RuntimeTargetProjection {
                target: BackendTarget::new(declaration.target.clone()),
                capabilities: declaration
                    .capabilities
                    .iter()
                    .map(|capability| BackendCapability::new(capability.capability.clone()))
                    .collect(),
            })
            .collect();

        Ok(Self {
            registration_id: adapter.instance().registration_id().clone(),
            instance_id: adapter.instance().id().clone(),
            adapter_id: evidence.adapter_id().to_owned(),
            adapter_version: evidence.adapter_version().to_owned(),
            compatibility: evidence.compatibility().clone(),
            targets,
        })
    }

    pub fn registration_id(&self) -> &AdapterRegistrationId {
        &self.registration_id
    }

    pub fn instance_id(&self) -> &AdapterInstanceId {
        &self.instance_id
    }

    pub fn adapter_id(&self) -> &str {
        &self.adapter_id
    }

    pub fn adapter_version(&self) -> &str {
        &self.adapter_version
    }

    pub fn compatibility(&self) -> &CompatibilityAssessment {
        &self.compatibility
    }

    pub fn targets(&self) -> &[RuntimeTargetProjection] {
        &self.targets
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterSelectionRequest {
    target: BackendTarget,
    required_capabilities: BTreeSet<BackendCapability>,
}

impl AdapterSelectionRequest {
    pub fn new(target: BackendTarget) -> Self {
        Self {
            target,
            required_capabilities: BTreeSet::new(),
        }
    }

    pub fn require(mut self, capability: BackendCapability) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    pub fn target(&self) -> &BackendTarget {
        &self.target
    }

    pub fn required_capabilities(&self) -> &BTreeSet<BackendCapability> {
        &self.required_capabilities
    }
}

/// One eligible runtime candidate. This contains only solver-neutral runtime and
/// Protocol evidence; no backend-native object/API identity participates in selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterSelectionCandidate {
    registration_id: AdapterRegistrationId,
    instance_id: AdapterInstanceId,
    adapter_id: String,
    adapter_version: String,
    target: BackendTarget,
    capabilities: BTreeSet<BackendCapability>,
}

impl AdapterSelectionCandidate {
    pub fn registration_id(&self) -> &AdapterRegistrationId {
        &self.registration_id
    }

    pub fn instance_id(&self) -> &AdapterInstanceId {
        &self.instance_id
    }

    pub fn adapter_id(&self) -> &str {
        &self.adapter_id
    }

    pub fn adapter_version(&self) -> &str {
        &self.adapter_version
    }

    pub fn target(&self) -> &BackendTarget {
        &self.target
    }

    pub fn capabilities(&self) -> &BTreeSet<BackendCapability> {
        &self.capabilities
    }
}

/// Selection never silently chooses between multiple equally eligible live adapters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterSelectionOutcome {
    NoCompatibleCandidate,
    Selected(AdapterSelectionCandidate),
    Ambiguous(Vec<AdapterSelectionCandidate>),
}

pub fn select_adapter(
    request: &AdapterSelectionRequest,
    projections: &[AdapterRuntimeProjection],
) -> AdapterSelectionOutcome {
    let mut candidates: Vec<_> = projections
        .iter()
        .filter(|projection| projection.compatibility.overall == CompatibilityOutcome::Compatible)
        .flat_map(|projection| {
            projection.targets.iter().filter_map(move |target| {
                if &target.target != request.target()
                    || !request
                        .required_capabilities()
                        .is_subset(&target.capabilities)
                {
                    return None;
                }

                Some(AdapterSelectionCandidate {
                    registration_id: projection.registration_id.clone(),
                    instance_id: projection.instance_id.clone(),
                    adapter_id: projection.adapter_id.clone(),
                    adapter_version: projection.adapter_version.clone(),
                    target: target.target.clone(),
                    capabilities: target.capabilities.clone(),
                })
            })
        })
        .collect();

    candidates.sort_by(|left, right| {
        left.adapter_id
            .cmp(&right.adapter_id)
            .then_with(|| left.registration_id.cmp(&right.registration_id))
            .then_with(|| left.instance_id.cmp(&right.instance_id))
    });

    match candidates.len() {
        0 => AdapterSelectionOutcome::NoCompatibleCandidate,
        1 => AdapterSelectionOutcome::Selected(candidates.remove(0)),
        _ => AdapterSelectionOutcome::Ambiguous(candidates),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use sol_adapter_protocol::{AxisCompatibility, CompatibilityAssessment, CompatibilityOutcome};
    use sol_target_resolver::{BackendCapability, BackendTarget};

    use crate::{AdapterInstanceId, AdapterRegistrationId};

    use super::{
        select_adapter, AdapterRuntimeProjection, AdapterSelectionOutcome, AdapterSelectionRequest,
        RuntimeTargetProjection,
    };

    fn compatibility(outcome: CompatibilityOutcome) -> CompatibilityAssessment {
        let axis = AxisCompatibility {
            outcome,
            selected_version: if outcome == CompatibilityOutcome::Compatible {
                Some("0.1".to_owned())
            } else {
                None
            },
        };
        CompatibilityAssessment {
            adapter_protocol: axis.clone(),
            public_contract: axis,
            overall: outcome,
        }
    }

    fn projection(
        local: &str,
        adapter_id: &str,
        compatibility_outcome: CompatibilityOutcome,
        target: &str,
        capabilities: &[&str],
    ) -> AdapterRuntimeProjection {
        AdapterRuntimeProjection {
            registration_id: AdapterRegistrationId::new(format!("registration.{local}")),
            instance_id: AdapterInstanceId::new(format!("instance.{local}")),
            adapter_id: adapter_id.to_owned(),
            adapter_version: "7.5.0".to_owned(),
            compatibility: compatibility(compatibility_outcome),
            targets: vec![RuntimeTargetProjection {
                target: BackendTarget::new(target),
                capabilities: capabilities
                    .iter()
                    .map(|capability| BackendCapability::new(*capability))
                    .collect::<BTreeSet<_>>(),
            }],
        }
    }

    fn request() -> AdapterSelectionRequest {
        AdapterSelectionRequest::new(BackendTarget::new("thermal_target"))
            .require(BackendCapability::new("thermal.domain"))
            .require(BackendCapability::new("thermal.solve"))
    }

    #[test]
    fn zero_eligible_candidates_is_explicit() {
        let wrong_target = projection(
            "wrong-target",
            "adapter.alpha",
            CompatibilityOutcome::Compatible,
            "other_target",
            &["thermal.domain", "thermal.solve"],
        );
        let missing_capability = projection(
            "missing-capability",
            "adapter.beta",
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain"],
        );
        let incompatible = projection(
            "incompatible",
            "adapter.gamma",
            CompatibilityOutcome::Incompatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );

        assert_eq!(
            select_adapter(
                &request(),
                &[wrong_target, missing_capability, incompatible]
            ),
            AdapterSelectionOutcome::NoCompatibleCandidate
        );
    }

    #[test]
    fn exactly_one_eligible_candidate_is_selected() {
        let candidate = projection(
            "selected",
            "adapter.alpha",
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.material", "thermal.solve"],
        );

        let AdapterSelectionOutcome::Selected(selected) =
            select_adapter(&request(), std::slice::from_ref(&candidate))
        else {
            panic!("expected exactly one selected candidate");
        };

        assert_eq!(selected.adapter_id(), "adapter.alpha");
        assert_eq!(selected.target().as_str(), "thermal_target");
        assert!(selected
            .capabilities()
            .contains(&BackendCapability::new("thermal.solve")));
    }

    #[test]
    fn multiple_candidates_are_reported_ambiguously_in_deterministic_order() {
        let z = projection(
            "z",
            "adapter.zeta",
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        let a2 = projection(
            "a2",
            "adapter.alpha",
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        let a1 = projection(
            "a1",
            "adapter.alpha",
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );

        let AdapterSelectionOutcome::Ambiguous(candidates) =
            select_adapter(&request(), &[z, a2, a1])
        else {
            panic!("multiple eligible candidates must not be silently selected");
        };

        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.instance_id().as_str())
                .collect::<Vec<_>>(),
            vec!["instance.a1", "instance.a2", "instance.z"]
        );
    }

    #[test]
    fn unknown_compatibility_is_not_eligible() {
        let unknown = projection(
            "unknown",
            "adapter.alpha",
            CompatibilityOutcome::Unknown,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        assert_eq!(
            select_adapter(&request(), &[unknown]),
            AdapterSelectionOutcome::NoCompatibleCandidate
        );
    }
}

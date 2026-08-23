use std::collections::BTreeSet;

use sol_adapter_protocol::{CompatibilityAssessment, CompatibilityOutcome};
use sol_target_resolver::{BackendCapability, BackendTarget};

use crate::{
    AdapterDiscoveryError, AdapterInstanceId, AdapterRegistrationId, RunningAdapter,
    RuntimeContractProfile,
};

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
/// `profile` records which explicit Adapter Protocol/Public Contract pair the
/// compatibility assessment belongs to; evidence from different profiles is not
/// interchangeable during selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRuntimeProjection {
    registration_id: AdapterRegistrationId,
    instance_id: AdapterInstanceId,
    adapter_id: String,
    adapter_version: String,
    profile: RuntimeContractProfile,
    compatibility: CompatibilityAssessment,
    targets: Vec<RuntimeTargetProjection>,
}

impl AdapterRuntimeProjection {
    /// Preserve the accepted M0.7 projection behavior as an explicit 0.1 path.
    pub fn from_running(adapter: &RunningAdapter) -> Result<Self, AdapterDiscoveryError> {
        Self::from_running_for(adapter, RuntimeContractProfile::v01())
    }

    /// Project live evidence for one explicitly selected runtime contract profile.
    pub fn from_running_for(
        adapter: &RunningAdapter,
        profile: RuntimeContractProfile,
    ) -> Result<Self, AdapterDiscoveryError> {
        let evidence = adapter.discover_live_evidence_for(profile)?;
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
            profile,
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

    pub fn profile(&self) -> RuntimeContractProfile {
        self.profile
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
    profile: RuntimeContractProfile,
    target: BackendTarget,
    required_capabilities: BTreeSet<BackendCapability>,
}

impl AdapterSelectionRequest {
    /// Preserve the accepted M0.7 selection API as an explicit 0.1 request.
    pub fn new(target: BackendTarget) -> Self {
        Self::for_profile(target, RuntimeContractProfile::v01())
    }

    pub fn for_profile(target: BackendTarget, profile: RuntimeContractProfile) -> Self {
        Self {
            profile,
            target,
            required_capabilities: BTreeSet::new(),
        }
    }

    pub fn require(mut self, capability: BackendCapability) -> Self {
        self.required_capabilities.insert(capability);
        self
    }

    pub fn profile(&self) -> RuntimeContractProfile {
        self.profile
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
    profile: RuntimeContractProfile,
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

    pub fn profile(&self) -> RuntimeContractProfile {
        self.profile
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
        .filter(|projection| {
            projection.profile == request.profile()
                && projection.compatibility.overall == CompatibilityOutcome::Compatible
        })
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
                    profile: projection.profile,
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

    use crate::{AdapterInstanceId, AdapterRegistrationId, RuntimeContractProfile};

    use super::{
        select_adapter, AdapterRuntimeProjection, AdapterSelectionOutcome, AdapterSelectionRequest,
        RuntimeTargetProjection,
    };

    fn compatibility(
        outcome: CompatibilityOutcome,
        profile: RuntimeContractProfile,
    ) -> CompatibilityAssessment {
        let selected_version = if outcome == CompatibilityOutcome::Compatible {
            Some(profile.adapter_protocol_version().to_string())
        } else {
            None
        };
        CompatibilityAssessment {
            adapter_protocol: AxisCompatibility {
                outcome,
                selected_version: selected_version.clone(),
            },
            public_contract: AxisCompatibility {
                outcome,
                selected_version: if outcome == CompatibilityOutcome::Compatible {
                    Some(profile.public_contract_version().to_string())
                } else {
                    None
                },
            },
            overall: outcome,
        }
    }

    fn projection_for(
        local: &str,
        adapter_id: &str,
        profile: RuntimeContractProfile,
        compatibility_outcome: CompatibilityOutcome,
        target: &str,
        capabilities: &[&str],
    ) -> AdapterRuntimeProjection {
        AdapterRuntimeProjection {
            registration_id: AdapterRegistrationId::new(format!("registration.{local}")),
            instance_id: AdapterInstanceId::new(format!("instance.{local}")),
            adapter_id: adapter_id.to_owned(),
            adapter_version: "7.5.0".to_owned(),
            profile,
            compatibility: compatibility(compatibility_outcome, profile),
            targets: vec![RuntimeTargetProjection {
                target: BackendTarget::new(target),
                capabilities: capabilities
                    .iter()
                    .map(|capability| BackendCapability::new(*capability))
                    .collect::<BTreeSet<_>>(),
            }],
        }
    }

    fn projection(
        local: &str,
        adapter_id: &str,
        compatibility_outcome: CompatibilityOutcome,
        target: &str,
        capabilities: &[&str],
    ) -> AdapterRuntimeProjection {
        projection_for(
            local,
            adapter_id,
            RuntimeContractProfile::v01(),
            compatibility_outcome,
            target,
            capabilities,
        )
    }

    fn request() -> AdapterSelectionRequest {
        AdapterSelectionRequest::new(BackendTarget::new("thermal_target"))
            .require(BackendCapability::new("thermal.domain"))
            .require(BackendCapability::new("thermal.solve"))
    }

    fn request_v02() -> AdapterSelectionRequest {
        AdapterSelectionRequest::for_profile(
            BackendTarget::new("thermal_target"),
            RuntimeContractProfile::realization_v02(),
        )
        .require(BackendCapability::new("thermal.domain"))
        .require(BackendCapability::new("thermal.solve"))
    }

    #[test]
    fn default_selection_request_preserves_v01_behavior() {
        assert_eq!(request().profile(), RuntimeContractProfile::v01());
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
        assert_eq!(selected.profile(), RuntimeContractProfile::v01());
        assert_eq!(selected.target().as_str(), "thermal_target");
        assert!(selected
            .capabilities()
            .contains(&BackendCapability::new("thermal.solve")));
    }

    #[test]
    fn explicit_v02_request_selects_only_v02_projection() {
        let v01 = projection_for(
            "v01",
            "adapter.same",
            RuntimeContractProfile::v01(),
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        let v02 = projection_for(
            "v02",
            "adapter.same",
            RuntimeContractProfile::realization_v02(),
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );

        let AdapterSelectionOutcome::Selected(selected) =
            select_adapter(&request_v02(), &[v01, v02])
        else {
            panic!("expected the explicit v0.2 projection to be selected");
        };
        assert_eq!(selected.instance_id().as_str(), "instance.v02");
        assert_eq!(
            selected.profile(),
            RuntimeContractProfile::realization_v02()
        );
    }

    #[test]
    fn no_hidden_v02_to_v01_fallback_occurs() {
        let v01 = projection_for(
            "v01",
            "adapter.v01",
            RuntimeContractProfile::v01(),
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        assert_eq!(
            select_adapter(&request_v02(), &[v01]),
            AdapterSelectionOutcome::NoCompatibleCandidate
        );
    }

    #[test]
    fn v02_incompatible_or_unknown_evidence_is_not_eligible() {
        for outcome in [
            CompatibilityOutcome::Incompatible,
            CompatibilityOutcome::Unknown,
        ] {
            let projection = projection_for(
                "v02-negative",
                "adapter.v02",
                RuntimeContractProfile::realization_v02(),
                outcome,
                "thermal_target",
                &["thermal.domain", "thermal.solve"],
            );
            assert_eq!(
                select_adapter(&request_v02(), &[projection]),
                AdapterSelectionOutcome::NoCompatibleCandidate
            );
        }
    }

    #[test]
    fn multiple_v02_candidates_remain_explicit_and_deterministic() {
        let z = projection_for(
            "z",
            "adapter.zeta",
            RuntimeContractProfile::realization_v02(),
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        let a2 = projection_for(
            "a2",
            "adapter.alpha",
            RuntimeContractProfile::realization_v02(),
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );
        let a1 = projection_for(
            "a1",
            "adapter.alpha",
            RuntimeContractProfile::realization_v02(),
            CompatibilityOutcome::Compatible,
            "thermal_target",
            &["thermal.domain", "thermal.solve"],
        );

        let AdapterSelectionOutcome::Ambiguous(candidates) =
            select_adapter(&request_v02(), &[z, a2, a1])
        else {
            panic!("multiple eligible v0.2 candidates must remain ambiguous");
        };

        assert_eq!(
            candidates
                .iter()
                .map(|candidate| candidate.instance_id().as_str())
                .collect::<Vec<_>>(),
            vec!["instance.a1", "instance.a2", "instance.z"]
        );
        assert!(candidates
            .iter()
            .all(|candidate| candidate.profile() == RuntimeContractProfile::realization_v02()));
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

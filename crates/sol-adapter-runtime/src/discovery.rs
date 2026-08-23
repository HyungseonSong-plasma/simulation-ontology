use sol_adapter_protocol::{
    assess_compatibility, AdapterDescription, CompatibilityAssessment, CompatibilityOutcome,
    ProtocolError, TargetDeclaration,
};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{AdapterInstanceId, RunningAdapter, RuntimeContractProfile};

/// Runtime-local evidence observed from one bootstrapped external adapter process.
///
/// This is not canonical SOL model data and is not a lifecycle result. The
/// description is the normalized Adapter Protocol `describe_adapter` result;
/// compatibility is computed from that live bootstrap evidence against one
/// explicit runtime contract profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveAdapterEvidence {
    instance_id: AdapterInstanceId,
    profile: RuntimeContractProfile,
    description: AdapterDescription,
    compatibility: CompatibilityAssessment,
}

impl LiveAdapterEvidence {
    fn from_description(
        instance_id: AdapterInstanceId,
        description: &AdapterDescription,
        profile: RuntimeContractProfile,
    ) -> Result<Self, ProtocolError> {
        let compatibility =
            assess_compatibility(&profile.compatibility_support(), &description.bootstrap)?;
        Ok(Self {
            instance_id,
            profile,
            description: description.clone(),
            compatibility,
        })
    }

    /// Local operational identity of the process/session from which this
    /// evidence was observed. It is not canonical SOL semantic identity.
    pub fn instance_id(&self) -> &AdapterInstanceId {
        &self.instance_id
    }

    /// Explicit runtime contract pair against which the live description was assessed.
    pub fn profile(&self) -> RuntimeContractProfile {
        self.profile
    }

    pub fn adapter_id(&self) -> &str {
        &self.description.bootstrap.adapter_id
    }

    /// Adapter implementation/package version is observed metadata only and
    /// never establishes Protocol/Public Contract compatibility by itself.
    pub fn adapter_version(&self) -> &str {
        &self.description.bootstrap.adapter_version
    }

    pub fn compatibility(&self) -> &CompatibilityAssessment {
        &self.compatibility
    }

    pub fn compatibility_outcome(&self) -> CompatibilityOutcome {
        self.compatibility.overall
    }

    /// Solver-neutral target/capability evidence as declared by the live
    /// Adapter Protocol description. Registration metadata is not consulted.
    pub fn targets(&self) -> &[TargetDeclaration] {
        &self.description.targets
    }

    pub fn target(&self, target: &str) -> Option<&TargetDeclaration> {
        self.description
            .targets
            .iter()
            .find(|declaration| declaration.target == target)
    }

    pub fn description(&self) -> &AdapterDescription {
        &self.description
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterDiscoveryError {
    MissingLiveDescription,
    InvalidCompatibilityEvidence(ProtocolError),
}

impl Display for AdapterDiscoveryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingLiveDescription => write!(
                formatter,
                "bootstrapped adapter instance has no live AdapterDescription evidence"
            ),
            Self::InvalidCompatibilityEvidence(error) => Display::fmt(error, formatter),
        }
    }
}

impl Error for AdapterDiscoveryError {}

impl RunningAdapter {
    /// Inspect live evidence for the accepted M0.7 0.1 runtime profile.
    ///
    /// This preserves the existing no-argument API and behavior. Callers that
    /// require another integrated runtime profile must select it explicitly.
    pub fn discover_live_evidence(&self) -> Result<LiveAdapterEvidence, AdapterDiscoveryError> {
        self.discover_live_evidence_for(RuntimeContractProfile::v01())
    }

    /// Inspect live compatibility, target, and capability evidence for one
    /// explicitly selected runtime contract profile.
    pub fn discover_live_evidence_for(
        &self,
        profile: RuntimeContractProfile,
    ) -> Result<LiveAdapterEvidence, AdapterDiscoveryError> {
        let description = self
            .description()
            .ok_or(AdapterDiscoveryError::MissingLiveDescription)?;
        LiveAdapterEvidence::from_description(self.instance().id().clone(), description, profile)
            .map_err(AdapterDiscoveryError::InvalidCompatibilityEvidence)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use sol_adapter_protocol::{AdapterDescription, CompatibilityOutcome};

    use crate::{
        AdapterCommand, AdapterInstanceId, AdapterRegistration, AdapterRegistrationId,
        AdapterRegistry, RunningAdapter, RuntimeContractProfile,
    };

    use super::LiveAdapterEvidence;

    const DUAL_COMPATIBLE: &str =
        include_str!("../../../fixtures/adapter-protocol/0.1/dual-compatible-description.json");
    const DUAL_V02_DESCRIPTION: &str = include_str!(
        "../../../fixtures/adapter-protocol/0.2/realization-compatible-description.json"
    );
    const PROTOCOL_INCOMPATIBLE: &str = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-protocol-incompatible.json"
    );
    const MISSING_PROTOCOL: &str = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-missing-protocol-support.json"
    );

    fn evidence_from_fixture_for(
        fixture: &str,
        profile: RuntimeContractProfile,
    ) -> LiveAdapterEvidence {
        let description = AdapterDescription::from_json(fixture).unwrap();
        LiveAdapterEvidence::from_description(
            AdapterInstanceId::new("instance.fixture"),
            &description,
            profile,
        )
        .unwrap()
    }

    fn evidence_from_fixture(fixture: &str) -> LiveAdapterEvidence {
        evidence_from_fixture_for(fixture, RuntimeContractProfile::v01())
    }

    fn mock_adapter_binary() -> PathBuf {
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("runtime crate must live under workspace/crates");
        let executable = if cfg!(windows) {
            "sol-mock-adapter-stdio.exe"
        } else {
            "sol-mock-adapter-stdio"
        };
        workspace.join("target").join("debug").join(executable)
    }

    #[test]
    fn compatible_live_description_exposes_protocol_targets_and_capabilities() {
        let evidence = evidence_from_fixture(DUAL_COMPATIBLE);

        assert_eq!(evidence.profile(), RuntimeContractProfile::v01());
        assert_eq!(evidence.adapter_id(), "adapter.mock");
        assert_eq!(evidence.adapter_version(), "1.4.0");
        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert_eq!(
            evidence
                .compatibility()
                .adapter_protocol
                .selected_version
                .as_deref(),
            Some("0.1")
        );
        assert_eq!(
            evidence
                .compatibility()
                .public_contract
                .selected_version
                .as_deref(),
            Some("0.1")
        );

        let target = evidence.target("mock").unwrap();
        assert_eq!(
            target
                .capabilities
                .iter()
                .map(|capability| capability.capability.as_str())
                .collect::<Vec<_>>(),
            vec!["thermal.domain", "thermal.material", "thermal.solve"]
        );
    }

    #[test]
    fn dual_version_description_can_be_projected_to_explicit_v02_profile() {
        let evidence = evidence_from_fixture_for(
            DUAL_V02_DESCRIPTION,
            RuntimeContractProfile::realization_v02(),
        );

        assert_eq!(
            evidence.profile(),
            RuntimeContractProfile::realization_v02()
        );
        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert_eq!(
            evidence
                .compatibility()
                .adapter_protocol
                .selected_version
                .as_deref(),
            Some("0.2")
        );
        assert_eq!(
            evidence
                .compatibility()
                .public_contract
                .selected_version
                .as_deref(),
            Some("0.2")
        );
        assert_eq!(
            evidence
                .target("mock")
                .unwrap()
                .capabilities
                .iter()
                .map(|capability| capability.capability.as_str())
                .collect::<Vec<_>>(),
            vec!["thermal.domain", "thermal.material", "thermal.solve"]
        );
    }

    #[test]
    fn same_dual_version_adapter_requires_explicit_profile_selection() {
        let v01 = evidence_from_fixture_for(DUAL_V02_DESCRIPTION, RuntimeContractProfile::v01());
        let v02 = evidence_from_fixture_for(
            DUAL_V02_DESCRIPTION,
            RuntimeContractProfile::realization_v02(),
        );

        assert_eq!(
            v01.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert_eq!(
            v02.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert_eq!(
            v01.compatibility()
                .adapter_protocol
                .selected_version
                .as_deref(),
            Some("0.1")
        );
        assert_eq!(
            v02.compatibility()
                .adapter_protocol
                .selected_version
                .as_deref(),
            Some("0.2")
        );
    }

    #[test]
    fn package_version_does_not_promote_incompatible_live_evidence() {
        let compatible = evidence_from_fixture(DUAL_COMPATIBLE);
        let incompatible = evidence_from_fixture(PROTOCOL_INCOMPATIBLE);

        assert_eq!(compatible.adapter_version(), incompatible.adapter_version());
        assert_eq!(compatible.adapter_version(), "1.4.0");
        assert_eq!(
            compatible.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert_eq!(
            incompatible.compatibility_outcome(),
            CompatibilityOutcome::Incompatible
        );
    }

    #[test]
    fn missing_live_support_is_unknown_not_compatible() {
        let evidence = evidence_from_fixture(MISSING_PROTOCOL);
        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Unknown
        );
    }

    #[test]
    fn v01_only_live_description_does_not_silently_gain_v02_runtime_support() {
        let evidence =
            evidence_from_fixture_for(DUAL_COMPATIBLE, RuntimeContractProfile::realization_v02());

        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Incompatible
        );
    }

    #[test]
    fn running_adapter_discovers_evidence_only_after_real_bootstrap() {
        let mut registry = AdapterRegistry::new();
        let registration_id = AdapterRegistrationId::new("mock.discovery");
        registry
            .register(AdapterRegistration::new(
                registration_id.clone(),
                AdapterCommand::new(mock_adapter_binary()),
            ))
            .unwrap();

        let running = RunningAdapter::launch(
            registry.get(&registration_id).unwrap(),
            AdapterInstanceId::new("instance.discovery"),
        )
        .unwrap();
        let evidence = running.discover_live_evidence().unwrap();

        assert_eq!(evidence.instance_id().as_str(), "instance.discovery");
        assert_eq!(evidence.profile(), RuntimeContractProfile::v01());
        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert!(evidence.target("mock").is_some());

        let v02 = running
            .discover_live_evidence_for(RuntimeContractProfile::realization_v02())
            .unwrap();
        assert_eq!(
            v02.compatibility_outcome(),
            CompatibilityOutcome::Incompatible
        );

        assert!(running.shutdown().unwrap().success);
    }
}

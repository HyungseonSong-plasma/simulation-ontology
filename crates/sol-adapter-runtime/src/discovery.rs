use sol_adapter_protocol::{
    assess_compatibility, AdapterDescription, CompatibilityAssessment, CompatibilityOutcome,
    CompatibilitySupport, ProtocolError, TargetDeclaration,
};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{AdapterInstanceId, RunningAdapter};

/// Runtime-local evidence observed from one bootstrapped external adapter process.
///
/// This is not canonical SOL model data and is not a lifecycle result. The
/// description is the normalized Adapter Protocol `describe_adapter` result;
/// compatibility is computed from that live bootstrap evidence against the
/// runtime's explicit support profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveAdapterEvidence {
    instance_id: AdapterInstanceId,
    description: AdapterDescription,
    compatibility: CompatibilityAssessment,
}

impl LiveAdapterEvidence {
    fn from_description(
        instance_id: AdapterInstanceId,
        description: &AdapterDescription,
        runtime_support: &CompatibilitySupport,
    ) -> Result<Self, ProtocolError> {
        let compatibility = assess_compatibility(runtime_support, &description.bootstrap)?;
        Ok(Self {
            instance_id,
            description: description.clone(),
            compatibility,
        })
    }

    /// Local operational identity of the process/session from which this
    /// evidence was observed. It is not canonical SOL semantic identity.
    pub fn instance_id(&self) -> &AdapterInstanceId {
        &self.instance_id
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
    /// Inspect live compatibility, target, and capability evidence for the
    /// currently bootstrapped adapter instance.
    ///
    /// M0.7 intentionally evaluates the published 0.1 runtime pair here. M0.8
    /// published an additive 0.2 contract, but runtime support for 0.2 must be
    /// integrated explicitly rather than inferred from the mere existence of
    /// 0.2 DTOs/schemas.
    pub fn discover_live_evidence(&self) -> Result<LiveAdapterEvidence, AdapterDiscoveryError> {
        let description = self
            .description()
            .ok_or(AdapterDiscoveryError::MissingLiveDescription)?;
        LiveAdapterEvidence::from_description(
            self.instance().id().clone(),
            description,
            &CompatibilitySupport::current(),
        )
        .map_err(AdapterDiscoveryError::InvalidCompatibilityEvidence)
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use sol_adapter_protocol::{
        AdapterDescription, CompatibilityOutcome, CompatibilitySupport,
    };

    use crate::{
        AdapterCommand, AdapterInstanceId, AdapterRegistration, AdapterRegistrationId,
        AdapterRegistry, RunningAdapter,
    };

    use super::LiveAdapterEvidence;

    const DUAL_COMPATIBLE: &str = include_str!(
        "../../../fixtures/adapter-protocol/0.1/dual-compatible-description.json"
    );
    const PROTOCOL_INCOMPATIBLE: &str = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-protocol-incompatible.json"
    );
    const MISSING_PROTOCOL: &str = include_str!(
        "../../../fixtures/counterexamples/adapter-protocol-missing-protocol-support.json"
    );

    fn evidence_from_fixture(fixture: &str) -> LiveAdapterEvidence {
        let description = AdapterDescription::from_json(fixture).unwrap();
        LiveAdapterEvidence::from_description(
            AdapterInstanceId::new("instance.fixture"),
            &description,
            &CompatibilitySupport::current(),
        )
        .unwrap()
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

        assert_eq!(evidence.adapter_id(), "adapter.mock");
        assert_eq!(evidence.adapter_version(), "1.4.0");
        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert_eq!(
            evidence.compatibility().adapter_protocol.selected_version.as_deref(),
            Some("0.1")
        );
        assert_eq!(
            evidence.compatibility().public_contract.selected_version.as_deref(),
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
        assert_eq!(evidence.compatibility_outcome(), CompatibilityOutcome::Unknown);
    }

    #[test]
    fn published_v02_contract_does_not_silently_expand_m07_runtime_support() {
        let description = AdapterDescription::from_json(DUAL_COMPATIBLE).unwrap();
        let v02_only = LiveAdapterEvidence::from_description(
            AdapterInstanceId::new("instance.v02-check"),
            &description,
            &CompatibilitySupport::realization_v02(),
        )
        .unwrap();

        assert_eq!(
            v02_only.compatibility_outcome(),
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
        assert_eq!(
            evidence.compatibility_outcome(),
            CompatibilityOutcome::Compatible
        );
        assert!(evidence.target("mock").is_some());

        assert!(running.shutdown().unwrap().success);
    }
}

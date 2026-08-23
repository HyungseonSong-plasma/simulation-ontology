use sol_adapter_protocol::{AdapterProtocolVersion, CompatibilitySupport};
use sol_public_contract::ContractVersion;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Explicit solver-neutral contract pair selected for one runtime interoperability path.
///
/// This is runtime configuration, not canonical SOL model identity. A profile states
/// which already-published Adapter Protocol/Public Contract pair the runtime intends
/// to use with a live adapter. Merely having DTOs or schemas for a version does not
/// make that version an active runtime profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RuntimeContractProfile {
    adapter_protocol: AdapterProtocolVersion,
    public_contract: ContractVersion,
}

impl RuntimeContractProfile {
    /// Existing M0.7 runtime profile. Its behavior remains the default 0.1 path.
    pub const fn v01() -> Self {
        Self {
            adapter_protocol: AdapterProtocolVersion::current(),
            public_contract: ContractVersion::supported(),
        }
    }

    /// Explicit realization profile published by M0.8 and integrated by M0.9.
    pub const fn realization_v02() -> Self {
        Self {
            adapter_protocol: AdapterProtocolVersion::realization_v02(),
            public_contract: ContractVersion::realization_v02(),
        }
    }

    /// Construct only a runtime pair that SOL has explicitly integrated.
    ///
    /// In particular, mixed pairs such as Protocol 0.1 + Public Contract 0.2 are
    /// rejected rather than being treated as partially compatible.
    pub fn try_new(
        adapter_protocol: AdapterProtocolVersion,
        public_contract: ContractVersion,
    ) -> Result<Self, RuntimeProfileError> {
        let candidate = Self {
            adapter_protocol,
            public_contract,
        };
        if candidate == Self::v01() || candidate == Self::realization_v02() {
            Ok(candidate)
        } else {
            Err(RuntimeProfileError::UnsupportedPair {
                adapter_protocol,
                public_contract,
            })
        }
    }

    pub const fn adapter_protocol_version(self) -> AdapterProtocolVersion {
        self.adapter_protocol
    }

    pub const fn public_contract_version(self) -> ContractVersion {
        self.public_contract
    }

    /// Exact compatibility support used to assess live `describe_adapter` evidence.
    pub fn compatibility_support(self) -> CompatibilitySupport {
        CompatibilitySupport {
            adapter_protocol_versions: Some(vec![self.adapter_protocol.to_string()]),
            public_contract_versions: Some(vec![self.public_contract.to_string()]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeProfileError {
    UnsupportedPair {
        adapter_protocol: AdapterProtocolVersion,
        public_contract: ContractVersion,
    },
}

impl Display for RuntimeProfileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedPair {
                adapter_protocol,
                public_contract,
            } => write!(
                formatter,
                "unsupported runtime contract profile: Adapter Protocol {adapter_protocol} + Public Contract {public_contract}"
            ),
        }
    }
}

impl Error for RuntimeProfileError {}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use sol_adapter_protocol::AdapterProtocolVersion;
    use sol_public_contract::ContractVersion;
    use sol_target_resolver::BackendTarget;

    use crate::{AdapterInstance, AdapterRegistration};

    use super::{RuntimeContractProfile, RuntimeProfileError};

    #[test]
    fn published_runtime_profiles_are_explicit_exact_pairs() {
        let v01 = RuntimeContractProfile::v01();
        assert_eq!(
            v01.adapter_protocol_version(),
            AdapterProtocolVersion::current()
        );
        assert_eq!(v01.public_contract_version(), ContractVersion::supported());

        let v02 = RuntimeContractProfile::realization_v02();
        assert_eq!(
            v02.adapter_protocol_version(),
            AdapterProtocolVersion::realization_v02()
        );
        assert_eq!(
            v02.public_contract_version(),
            ContractVersion::realization_v02()
        );
    }

    #[test]
    fn mixed_version_pairs_are_rejected_before_runtime_use() {
        assert_eq!(
            RuntimeContractProfile::try_new(
                AdapterProtocolVersion::current(),
                ContractVersion::realization_v02(),
            ),
            Err(RuntimeProfileError::UnsupportedPair {
                adapter_protocol: AdapterProtocolVersion::current(),
                public_contract: ContractVersion::realization_v02(),
            })
        );
        assert_eq!(
            RuntimeContractProfile::try_new(
                AdapterProtocolVersion::realization_v02(),
                ContractVersion::supported(),
            ),
            Err(RuntimeProfileError::UnsupportedPair {
                adapter_protocol: AdapterProtocolVersion::realization_v02(),
                public_contract: ContractVersion::supported(),
            })
        );
    }

    #[test]
    fn unpublished_future_pair_is_not_implicitly_runtime_supported() {
        assert!(RuntimeContractProfile::try_new(
            AdapterProtocolVersion::new(0, 3),
            ContractVersion::new(0, 3),
        )
        .is_err());
    }

    #[test]
    fn profile_projects_to_exact_dual_axis_compatibility_support() {
        let support = RuntimeContractProfile::realization_v02().compatibility_support();
        assert_eq!(
            support.adapter_protocol_versions,
            Some(vec!["0.2".to_owned()])
        );
        assert_eq!(
            support.public_contract_versions,
            Some(vec!["0.2".to_owned()])
        );
    }

    #[test]
    fn runtime_profile_is_not_target_registration_or_instance_identity() {
        assert_ne!(
            TypeId::of::<RuntimeContractProfile>(),
            TypeId::of::<BackendTarget>()
        );
        assert_ne!(
            TypeId::of::<RuntimeContractProfile>(),
            TypeId::of::<AdapterRegistration>()
        );
        assert_ne!(
            TypeId::of::<RuntimeContractProfile>(),
            TypeId::of::<AdapterInstance>()
        );
    }
}

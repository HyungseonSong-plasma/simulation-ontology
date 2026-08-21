//! Protocol-facing MockAdapter reference behavior for the published SOL adapter contract.
//!
//! M0.4 phases add deterministic in-process reference operations here while preserving
//! the pre-existing Rust-only helper API in `lib.rs` as a separate compatibility layer.
//!
//! This module is intentionally transport-independent and solver-independent.

use sol_adapter_protocol::{
    AdapterBootstrap, AdapterDescription, CapabilityDeclaration, CompatibilitySupport,
    ProtocolError, TargetDeclaration, ADAPTER_PROTOCOL_VERSION,
};

use crate::{Adapter, MockAdapter};

pub const MOCK_ADAPTER_ID: &str = "adapter.mock";
pub const MOCK_TARGET: &str = "mock";
pub const MOCK_CAPABILITY_REVISION: &str = "mock-capabilities-1";

impl MockAdapter {
    pub fn describe_adapter(&self) -> Result<AdapterDescription, ProtocolError> {
        let support = CompatibilitySupport::current();
        let capabilities = self
            .capabilities()
            .iter()
            .map(|capability| CapabilityDeclaration {
                capability: capability.as_str().to_owned(),
                revision: (capability.as_str() == "thermal.solve")
                    .then(|| MOCK_CAPABILITY_REVISION.to_owned()),
                extensions: Default::default(),
            })
            .collect();

        let description = AdapterDescription {
            bootstrap: AdapterBootstrap {
                adapter_id: MOCK_ADAPTER_ID.to_owned(),
                adapter_version: env!("CARGO_PKG_VERSION").to_owned(),
                supported_adapter_protocol_versions: support.adapter_protocol_versions,
                supported_public_contract_versions: support.public_contract_versions,
                extensions: Default::default(),
            },
            targets: vec![TargetDeclaration {
                target: MOCK_TARGET.to_owned(),
                capabilities,
                extensions: Default::default(),
            }],
            extensions: Default::default(),
        };

        let canonical = description.to_canonical_json()?;
        let normalized = AdapterDescription::from_json(&canonical)?;
        debug_assert_eq!(
            normalized.bootstrap.supported_adapter_protocol_versions,
            Some(vec![ADAPTER_PROTOCOL_VERSION.to_owned()])
        );
        Ok(normalized)
    }
}

//! Protocol-facing MockAdapter reference behavior for the published SOL adapter contract.
//!
//! M0.4 phases add deterministic in-process reference operations here while preserving
//! the pre-existing Rust-only helper API in `lib.rs` as a separate compatibility layer.
//!
//! This module is intentionally transport-independent and solver-independent.

use sol_adapter_protocol::{
    protocol_diagnostic, AdapterBootstrap, AdapterDescription, AdapterProtocolDiagnosticContext,
    CapabilityDeclaration, CompatibilitySupport, PreflightError, PreflightOutcome, ProtocolError,
    TargetDeclaration, ValidatePlanRequest, ValidatePlanResponse, ADAPTER_PROTOCOL_VERSION,
    DIAGNOSTIC_MISSING_CAPABILITY, DIAGNOSTIC_PRECONDITION_REJECTED,
    DIAGNOSTIC_TARGET_MISMATCH, DIAGNOSTIC_TRANSIENT_UNAVAILABLE, DIAGNOSTIC_UNSUPPORTED_ACTION,
};

use crate::{Adapter, MockAdapter};

pub const MOCK_ADAPTER_ID: &str = "adapter.mock";
pub const MOCK_TARGET: &str = "mock";
pub const MOCK_CAPABILITY_REVISION: &str = "mock-capabilities-1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MockPreflightState {
    #[default]
    Ready,
    PrerequisiteRejected,
    TransientUnavailable,
}

impl MockAdapter {
    pub fn with_preflight_state(mut self, state: MockPreflightState) -> Self {
        self.protocol_preflight_state = state;
        self
    }

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

    pub fn validate_plan(
        &self,
        request: &ValidatePlanRequest,
    ) -> Result<ValidatePlanResponse, PreflightError> {
        let canonical_request = request.to_canonical_json()?;
        let request = ValidatePlanRequest::from_json(&canonical_request)?;
        let target_compatible = request.target.target == MOCK_TARGET;
        let declared_capabilities = self
            .capabilities()
            .iter()
            .map(|capability| capability.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        let missing_capabilities = request
            .target
            .required_capabilities
            .iter()
            .filter(|capability| {
                !target_compatible || !declared_capabilities.contains(capability.as_str())
            })
            .cloned()
            .collect::<Vec<_>>();
        let capabilities_satisfied = target_compatible && missing_capabilities.is_empty();

        let mut diagnostics = Vec::new();
        if !target_compatible {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_TARGET_MISMATCH,
                "requested target is not addressed by this adapter",
                AdapterProtocolDiagnosticContext {
                    target: Some(request.target.target.clone()),
                    ..Default::default()
                },
            )?);
        }

        for capability in &missing_capabilities {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_MISSING_CAPABILITY,
                if target_compatible {
                    "required capability is not declared"
                } else {
                    "capabilities cannot be satisfied for the unmatched target"
                },
                AdapterProtocolDiagnosticContext {
                    capability: Some(capability.clone()),
                    ..Default::default()
                },
            )?);
        }

        if !target_compatible && missing_capabilities.is_empty() {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_MISSING_CAPABILITY,
                "capabilities cannot be satisfied for the unmatched target",
                AdapterProtocolDiagnosticContext::default(),
            )?);
        }

        let unsupported_actions = request
            .plan
            .actions
            .iter()
            .filter(|action| !declared_capabilities.contains(action.id.as_str()))
            .map(|action| action.id.clone())
            .collect::<Vec<_>>();
        for action_id in unsupported_actions {
            diagnostics.push(protocol_diagnostic(
                DIAGNOSTIC_UNSUPPORTED_ACTION,
                "plan action is not supported by the adapter",
                AdapterProtocolDiagnosticContext {
                    plan_action_id: Some(action_id),
                    ..Default::default()
                },
            )?);
        }

        if !target_compatible || !capabilities_satisfied || !diagnostics.is_empty() {
            return ValidatePlanResponse::new(
                target_compatible,
                capabilities_satisfied,
                PreflightOutcome::Rejected,
                diagnostics,
            );
        }

        match self.protocol_preflight_state {
            MockPreflightState::Ready => ValidatePlanResponse::new(
                true,
                true,
                PreflightOutcome::Accepted,
                Vec::new(),
            ),
            MockPreflightState::PrerequisiteRejected => ValidatePlanResponse::new(
                true,
                true,
                PreflightOutcome::Rejected,
                vec![protocol_diagnostic(
                    DIAGNOSTIC_PRECONDITION_REJECTED,
                    "adapter-specific prerequisite is not satisfied",
                    AdapterProtocolDiagnosticContext {
                        precondition: Some("mesh.loaded".to_owned()),
                        ..Default::default()
                    },
                )?],
            ),
            MockPreflightState::TransientUnavailable => ValidatePlanResponse::new(
                true,
                true,
                PreflightOutcome::Unavailable,
                vec![protocol_diagnostic(
                    DIAGNOSTIC_TRANSIENT_UNAVAILABLE,
                    "backend is temporarily unavailable",
                    AdapterProtocolDiagnosticContext {
                        precondition: Some("backend.available".to_owned()),
                        ..Default::default()
                    },
                )?],
            ),
        }
    }
}

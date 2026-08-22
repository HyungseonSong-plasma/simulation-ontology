//! Deterministic subprocess profiles for transport-boundary conformance tests.
//!
//! Profiles select existing MockAdapter reference states before the stdio worker starts.
//! They are CLI test controls only: profile names are never Adapter Protocol fields,
//! semantic identities, lifecycle states, or replay authority.

use crate::{
    MockAdapter, MockExecutionState, MockPreflightState, MockPriorExecutionState,
    MockProtocolFailureState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MockAdapterProfile {
    Exact,
    PreflightPrerequisiteRejected,
    PreflightTransientUnavailable,
    ExecutionDegraded,
    ExecutionUnsupported,
    ExecutionPartial,
    ExecutionUnavailable,
    ExecutionAuthoritativeRejected,
    ExecutionAlternateOrder,
    ExecutionParallelIndependent,
    FailureCompatibility,
    FailureInvalidRequest,
    FailureValidateOperational,
    FailureExecuteBeforeSideEffect,
    FailureExecuteAmbiguous,
    PriorAlreadyRealized,
    PriorPartialExecution,
    PriorUnresolvedPrerequisite,
}

impl MockAdapterProfile {
    pub const ALL: [Self; 18] = [
        Self::Exact,
        Self::PreflightPrerequisiteRejected,
        Self::PreflightTransientUnavailable,
        Self::ExecutionDegraded,
        Self::ExecutionUnsupported,
        Self::ExecutionPartial,
        Self::ExecutionUnavailable,
        Self::ExecutionAuthoritativeRejected,
        Self::ExecutionAlternateOrder,
        Self::ExecutionParallelIndependent,
        Self::FailureCompatibility,
        Self::FailureInvalidRequest,
        Self::FailureValidateOperational,
        Self::FailureExecuteBeforeSideEffect,
        Self::FailureExecuteAmbiguous,
        Self::PriorAlreadyRealized,
        Self::PriorPartialExecution,
        Self::PriorUnresolvedPrerequisite,
    ];

    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::PreflightPrerequisiteRejected => "preflight-prerequisite-rejected",
            Self::PreflightTransientUnavailable => "preflight-transient-unavailable",
            Self::ExecutionDegraded => "execution-degraded",
            Self::ExecutionUnsupported => "execution-unsupported",
            Self::ExecutionPartial => "execution-partial",
            Self::ExecutionUnavailable => "execution-unavailable",
            Self::ExecutionAuthoritativeRejected => "execution-authoritative-rejected",
            Self::ExecutionAlternateOrder => "execution-alternate-order",
            Self::ExecutionParallelIndependent => "execution-parallel-independent",
            Self::FailureCompatibility => "failure-compatibility",
            Self::FailureInvalidRequest => "failure-invalid-request",
            Self::FailureValidateOperational => "failure-validate-operational",
            Self::FailureExecuteBeforeSideEffect => "failure-execute-before-side-effect",
            Self::FailureExecuteAmbiguous => "failure-execute-ambiguous",
            Self::PriorAlreadyRealized => "prior-already-realized",
            Self::PriorPartialExecution => "prior-partial-execution",
            Self::PriorUnresolvedPrerequisite => "prior-unresolved-prerequisite",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        Some(match value {
            "exact" => Self::Exact,
            "preflight-prerequisite-rejected" => Self::PreflightPrerequisiteRejected,
            "preflight-transient-unavailable" => Self::PreflightTransientUnavailable,
            "execution-degraded" => Self::ExecutionDegraded,
            "execution-unsupported" => Self::ExecutionUnsupported,
            "execution-partial" => Self::ExecutionPartial,
            "execution-unavailable" => Self::ExecutionUnavailable,
            "execution-authoritative-rejected" => Self::ExecutionAuthoritativeRejected,
            "execution-alternate-order" => Self::ExecutionAlternateOrder,
            "execution-parallel-independent" => Self::ExecutionParallelIndependent,
            "failure-compatibility" => Self::FailureCompatibility,
            "failure-invalid-request" => Self::FailureInvalidRequest,
            "failure-validate-operational" => Self::FailureValidateOperational,
            "failure-execute-before-side-effect" => Self::FailureExecuteBeforeSideEffect,
            "failure-execute-ambiguous" => Self::FailureExecuteAmbiguous,
            "prior-already-realized" => Self::PriorAlreadyRealized,
            "prior-partial-execution" => Self::PriorPartialExecution,
            "prior-unresolved-prerequisite" => Self::PriorUnresolvedPrerequisite,
            _ => return None,
        })
    }

    pub fn adapter(self) -> MockAdapter {
        match self {
            Self::Exact => MockAdapter::thermal(),
            Self::PreflightPrerequisiteRejected => MockAdapter::thermal()
                .with_preflight_state(MockPreflightState::PrerequisiteRejected),
            Self::PreflightTransientUnavailable => MockAdapter::thermal()
                .with_preflight_state(MockPreflightState::TransientUnavailable),
            Self::ExecutionDegraded => {
                MockAdapter::thermal().with_execution_state(MockExecutionState::Degraded)
            }
            Self::ExecutionUnsupported => {
                MockAdapter::thermal().with_execution_state(MockExecutionState::Unsupported)
            }
            Self::ExecutionPartial => {
                MockAdapter::thermal().with_execution_state(MockExecutionState::Partial)
            }
            Self::ExecutionUnavailable => {
                MockAdapter::thermal().with_execution_state(MockExecutionState::Unavailable)
            }
            Self::ExecutionAuthoritativeRejected => MockAdapter::thermal()
                .with_execution_state(MockExecutionState::AuthoritativeRejected),
            Self::ExecutionAlternateOrder => {
                MockAdapter::thermal().with_execution_state(MockExecutionState::AlternateOrder)
            }
            Self::ExecutionParallelIndependent => {
                MockAdapter::thermal().with_execution_state(MockExecutionState::ParallelIndependent)
            }
            Self::FailureCompatibility => MockAdapter::thermal()
                .with_protocol_failure_state(MockProtocolFailureState::CompatibilityNotEstablished),
            Self::FailureInvalidRequest => MockAdapter::thermal()
                .with_protocol_failure_state(MockProtocolFailureState::InvalidRequest),
            Self::FailureValidateOperational => MockAdapter::thermal()
                .with_protocol_failure_state(MockProtocolFailureState::ValidateOperational),
            Self::FailureExecuteBeforeSideEffect => MockAdapter::thermal()
                .with_protocol_failure_state(
                    MockProtocolFailureState::ExecuteOperationalBeforeSideEffect,
                ),
            Self::FailureExecuteAmbiguous => MockAdapter::thermal()
                .with_protocol_failure_state(MockProtocolFailureState::ExecuteOperationalAmbiguous),
            Self::PriorAlreadyRealized => MockAdapter::thermal()
                .with_prior_execution_state(MockPriorExecutionState::AlreadyRealized),
            Self::PriorPartialExecution => MockAdapter::thermal()
                .with_prior_execution_state(MockPriorExecutionState::PartialPriorExecution),
            Self::PriorUnresolvedPrerequisite => MockAdapter::thermal()
                .with_prior_execution_state(MockPriorExecutionState::UnresolvedPrerequisite),
        }
    }
}

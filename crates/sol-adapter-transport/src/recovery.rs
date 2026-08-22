use crate::AdapterTransportMethod;
use sol_adapter_protocol::{ProtocolFailure, SideEffectEvidence};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconnectDisposition {
    CallerMayOpenNewBootstrappedSession,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayDisposition {
    CallerMayReissueDescription,
    CallerMayReissueEquivalentValidation,
    TransportMustNotReplayExecution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResponseLossRecovery {
    operation: AdapterTransportMethod,
    reconnect: ReconnectDisposition,
    replay: ReplayDisposition,
    conservative_side_effects: SideEffectEvidence,
}

impl ResponseLossRecovery {
    pub const fn operation(self) -> AdapterTransportMethod {
        self.operation
    }

    pub const fn reconnect(self) -> ReconnectDisposition {
        self.reconnect
    }

    pub const fn replay(self) -> ReplayDisposition {
        self.replay
    }

    pub const fn conservative_side_effects(self) -> SideEffectEvidence {
        self.conservative_side_effects
    }
}

pub const fn response_loss_recovery(
    operation: AdapterTransportMethod,
) -> ResponseLossRecovery {
    let (replay, conservative_side_effects) = match operation {
        AdapterTransportMethod::DescribeAdapter => (
            ReplayDisposition::CallerMayReissueDescription,
            SideEffectEvidence::None,
        ),
        AdapterTransportMethod::ValidatePlan => (
            ReplayDisposition::CallerMayReissueEquivalentValidation,
            SideEffectEvidence::None,
        ),
        AdapterTransportMethod::ExecutePlan => (
            ReplayDisposition::TransportMustNotReplayExecution,
            SideEffectEvidence::MayHaveOccurred,
        ),
    };

    ResponseLossRecovery {
        operation,
        reconnect: ReconnectDisposition::CallerMayOpenNewBootstrappedSession,
        replay,
        conservative_side_effects,
    }
}

pub const fn protocol_failure_authorizes_automatic_replay(
    _failure: &ProtocolFailure,
) -> bool {
    false
}

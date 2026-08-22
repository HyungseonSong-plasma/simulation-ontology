#![forbid(unsafe_code)]

//! Transport-owned JSON-RPC 2.0/stdin-stdout mapping for the published
//! Adapter Protocol 0.1 operations.
//!
//! This crate owns wire method names and the protocol-result/transport-error
//! channel boundary. Adapter Protocol DTO meaning remains in
//! `sol-adapter-protocol`. It also owns deterministic JSON-RPC envelopes,
//! request correlation, line-delimited stdio framing, and local subprocess
//! session lifecycle. Transport/process error categories and opaque diagnostic
//! evidence are deterministic; replay and reconnect behavior remain in later
//! M0.5 phases.

mod framing;
mod json_rpc;
mod process;
mod recovery;

pub use framing::*;
pub use json_rpc::*;
pub use process::*;
pub use recovery::*;

use sol_adapter_protocol::ProtocolOperation;

pub const DESCRIBE_ADAPTER_METHOD: &str = "describe_adapter";
pub const VALIDATE_PLAN_METHOD: &str = "validate_plan";
pub const EXECUTE_PLAN_METHOD: &str = "execute_plan";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AdapterTransportMethod {
    DescribeAdapter,
    ValidatePlan,
    ExecutePlan,
}

impl AdapterTransportMethod {
    pub const ALL: [Self; 3] = [Self::DescribeAdapter, Self::ValidatePlan, Self::ExecutePlan];

    pub const fn wire_name(self) -> &'static str {
        match self {
            Self::DescribeAdapter => DESCRIBE_ADAPTER_METHOD,
            Self::ValidatePlan => VALIDATE_PLAN_METHOD,
            Self::ExecutePlan => EXECUTE_PLAN_METHOD,
        }
    }

    pub const fn protocol_operation(self) -> ProtocolOperation {
        match self {
            Self::DescribeAdapter => ProtocolOperation::DescribeAdapter,
            Self::ValidatePlan => ProtocolOperation::ValidatePlan,
            Self::ExecutePlan => ProtocolOperation::ExecutePlan,
        }
    }

    pub const fn request_payload(self) -> RequestPayloadKind {
        match self {
            Self::DescribeAdapter => RequestPayloadKind::None,
            Self::ValidatePlan => RequestPayloadKind::ValidatePlanRequest,
            Self::ExecutePlan => RequestPayloadKind::ExecutePlanRequest,
        }
    }

    pub const fn success_payload(self) -> SuccessPayloadKind {
        match self {
            Self::DescribeAdapter => SuccessPayloadKind::AdapterDescription,
            Self::ValidatePlan => SuccessPayloadKind::ValidatePlanResponse,
            Self::ExecutePlan => SuccessPayloadKind::ExecutePlanResponse,
        }
    }

    pub fn parse(wire_name: &str) -> Option<Self> {
        match wire_name {
            DESCRIBE_ADAPTER_METHOD => Some(Self::DescribeAdapter),
            VALIDATE_PLAN_METHOD => Some(Self::ValidatePlan),
            EXECUTE_PLAN_METHOD => Some(Self::ExecutePlan),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestPayloadKind {
    None,
    ValidatePlanRequest,
    ExecutePlanRequest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuccessPayloadKind {
    AdapterDescription,
    ValidatePlanResponse,
    ExecutePlanResponse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchOutcomeKind {
    ProtocolSuccess,
    ProtocolFailure,
    TransportRejection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonRpcResponseChannel {
    Result,
    Error,
}

pub const fn response_channel(outcome: DispatchOutcomeKind) -> JsonRpcResponseChannel {
    match outcome {
        DispatchOutcomeKind::ProtocolSuccess | DispatchOutcomeKind::ProtocolFailure => {
            JsonRpcResponseChannel::Result
        }
        DispatchOutcomeKind::TransportRejection => JsonRpcResponseChannel::Error,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        response_channel, AdapterTransportMethod, DispatchOutcomeKind, JsonRpcResponseChannel,
        RequestPayloadKind, SuccessPayloadKind,
    };
    use sol_adapter_protocol::ProtocolOperation;
    use std::collections::BTreeSet;

    #[test]
    fn published_operations_have_exact_one_to_one_wire_names() {
        let methods = AdapterTransportMethod::ALL;
        let names = methods
            .iter()
            .map(|method| method.wire_name())
            .collect::<BTreeSet<_>>();

        assert_eq!(names.len(), methods.len());
        for method in methods {
            assert_eq!(
                AdapterTransportMethod::parse(method.wire_name()),
                Some(method)
            );
        }
    }

    #[test]
    fn method_mapping_preserves_protocol_operation_identity() {
        assert_eq!(
            AdapterTransportMethod::DescribeAdapter.protocol_operation(),
            ProtocolOperation::DescribeAdapter
        );
        assert_eq!(
            AdapterTransportMethod::ValidatePlan.protocol_operation(),
            ProtocolOperation::ValidatePlan
        );
        assert_eq!(
            AdapterTransportMethod::ExecutePlan.protocol_operation(),
            ProtocolOperation::ExecutePlan
        );
    }

    #[test]
    fn request_and_success_payloads_are_fixed_by_method() {
        assert_eq!(
            AdapterTransportMethod::DescribeAdapter.request_payload(),
            RequestPayloadKind::None
        );
        assert_eq!(
            AdapterTransportMethod::DescribeAdapter.success_payload(),
            SuccessPayloadKind::AdapterDescription
        );
        assert_eq!(
            AdapterTransportMethod::ValidatePlan.request_payload(),
            RequestPayloadKind::ValidatePlanRequest
        );
        assert_eq!(
            AdapterTransportMethod::ValidatePlan.success_payload(),
            SuccessPayloadKind::ValidatePlanResponse
        );
        assert_eq!(
            AdapterTransportMethod::ExecutePlan.request_payload(),
            RequestPayloadKind::ExecutePlanRequest
        );
        assert_eq!(
            AdapterTransportMethod::ExecutePlan.success_payload(),
            SuccessPayloadKind::ExecutePlanResponse
        );
    }

    #[test]
    fn unapproved_aliases_and_namespaces_are_not_methods() {
        for invalid in [
            "adapter.describe",
            "adapter.validate_plan",
            "adapter.execute_plan",
            "describe",
            "validate",
            "execute",
            "DescribeAdapter",
            "execute_plan ",
            "",
        ] {
            assert_eq!(AdapterTransportMethod::parse(invalid), None);
        }
    }

    #[test]
    fn protocol_failures_stay_in_the_result_channel() {
        assert_eq!(
            response_channel(DispatchOutcomeKind::ProtocolSuccess),
            JsonRpcResponseChannel::Result
        );
        assert_eq!(
            response_channel(DispatchOutcomeKind::ProtocolFailure),
            JsonRpcResponseChannel::Result
        );
        assert_eq!(
            response_channel(DispatchOutcomeKind::TransportRejection),
            JsonRpcResponseChannel::Error
        );
    }
}

use sol_adapter_protocol::{PreflightError, ProtocolError, ValidatePlanRequest};
use sol_adapter_transport::{
    response_channel, AdapterTransportMethod, DispatchOutcomeKind, JsonRpcResponseChannel,
};

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const BOUNDARY: &str =
    include_str!("../../../docs/implementation/m0.5-transport-boundary-and-method-mapping.md");
const TRANSPORT_CARGO: &str = include_str!("../Cargo.toml");
const PROTOCOL_CARGO: &str = include_str!("../../sol-adapter-protocol/Cargo.toml");

fn inject_top_level_string(input: &str, key: &str) -> String {
    input.replacen('{', &format!("{{\"{key}\":\"transport-only\","), 1)
}

#[test]
fn transport_markers_do_not_enter_protocol_payloads() {
    for marker in [
        "jsonrpc",
        "request_id",
        "jsonrpc_id",
        "transport_id",
        "stdio_frame",
        "process_id",
        "retry_policy",
        "reconnect_policy",
        "transport",
    ] {
        let injected = inject_top_level_string(VALIDATE_REQUEST, marker);
        assert!(matches!(
            ValidatePlanRequest::from_json(&injected),
            Err(PreflightError::Protocol(ProtocolError::TransportLeakage(field)))
                if field == marker
        ));
    }
}

#[test]
fn unknown_or_aliased_methods_are_transport_rejections() {
    for method in [
        "adapter.describe",
        "adapter.validate_plan",
        "adapter.execute_plan",
        "DescribeAdapter",
        "execute_plan ",
        "",
    ] {
        assert_eq!(AdapterTransportMethod::parse(method), None);
    }
    assert_eq!(
        response_channel(DispatchOutcomeKind::TransportRejection),
        JsonRpcResponseChannel::Error
    );
}

#[test]
fn protocol_failure_is_not_a_json_rpc_error() {
    assert_eq!(
        response_channel(DispatchOutcomeKind::ProtocolFailure),
        JsonRpcResponseChannel::Result
    );
}

#[test]
fn dependency_direction_keeps_transport_out_of_protocol() {
    assert!(TRANSPORT_CARGO.contains("sol-adapter-protocol"));
    assert!(!PROTOCOL_CARGO.contains("sol-adapter-transport"));

    for forbidden_dependency in [
        "jsonrpsee",
        "tokio",
        "reqwest",
        "tonic",
        "zmq",
        "moose",
        "comsol",
        "ansys",
    ] {
        assert!(
            !TRANSPORT_CARGO.contains(forbidden_dependency),
            "Phase 0 introduced premature runtime dependency: {forbidden_dependency}"
        );
    }
}

#[test]
fn boundary_document_records_required_rejected_interpretations() {
    for required in [
        "ProtocolFailure -> JSON-RPC error",
        "JSON-RPC error -> ProtocolFailure",
        "request ID -> Protocol payload",
        "transport/process outcome -> SOL lifecycle state",
        "same request ID -> replay or dedup authority",
        "network transport -> M0.5 scope",
    ] {
        assert!(
            BOUNDARY.contains(required),
            "missing Phase 0 counterexample: {required}"
        );
    }
}

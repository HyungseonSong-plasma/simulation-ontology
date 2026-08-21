use serde_json::{json, Value};
use sol_adapter_protocol::{ProtocolFailure, ValidatePlanRequest};
use sol_adapter_transport::{
    decode_request, encode_stdio_frame, AdapterTransportMethod, JsonRpcErrorKind, JsonRpcResponse,
    ProtocolOutcome, RequestDisposition, RequestId, RequestIdError, RequestIdGenerator,
    ResponseDecodeError, StdioFrameDecoder, StdioFrameError, TransportRequest,
    JSON_RPC_INVALID_PARAMS, JSON_RPC_INVALID_REQUEST, JSON_RPC_METHOD_NOT_FOUND,
    JSON_RPC_PARSE_ERROR, MAX_SAFE_JSON_REQUEST_ID,
};

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const PHASE_1_BOUNDARY: &str = include_str!(
    "../../../docs/implementation/m0.5-json-rpc-envelope-framing-and-request-identity.md"
);
const TRANSPORT_CARGO: &str = include_str!("../Cargo.toml");

fn rejected(input: &str) -> sol_adapter_transport::JsonRpcErrorResponse {
    match decode_request(input) {
        RequestDisposition::Reject(response) => response,
        other => panic!("expected rejection, got {other:?}"),
    }
}

#[test]
fn request_ids_are_safe_monotonic_correlation_values() {
    assert_eq!(RequestId::new(0).unwrap().value(), 0);
    assert!(matches!(
        RequestId::new(MAX_SAFE_JSON_REQUEST_ID + 1),
        Err(RequestIdError::OutsideSafeIntegerRange(_))
    ));

    let mut generator = RequestIdGenerator::new();
    assert_eq!(generator.allocate().unwrap().value(), 1);
    assert_eq!(generator.allocate().unwrap().value(), 2);

    let mut final_id = RequestIdGenerator::starting_at(MAX_SAFE_JSON_REQUEST_ID).unwrap();
    assert_eq!(
        final_id.allocate().unwrap().value(),
        MAX_SAFE_JSON_REQUEST_ID
    );
    assert_eq!(final_id.allocate(), Err(RequestIdError::Exhausted));
}

#[test]
fn requests_use_exact_methods_and_canonical_envelopes() {
    let request = TransportRequest::new(
        RequestId::new(7).unwrap(),
        AdapterTransportMethod::ValidatePlan,
        Some(json!({"z": 1, "a": 2})),
    )
    .unwrap();

    assert_eq!(
        request.to_json(),
        r#"{"id":7,"jsonrpc":"2.0","method":"validate_plan","params":{"a":2,"z":1}}"#
    );
    assert_eq!(
        decode_request(&request.to_json()),
        RequestDisposition::Dispatch(request)
    );

    let describe = TransportRequest::new(
        RequestId::new(8).unwrap(),
        AdapterTransportMethod::DescribeAdapter,
        None,
    )
    .unwrap();
    assert_eq!(
        describe.to_json(),
        r#"{"id":8,"jsonrpc":"2.0","method":"describe_adapter"}"#
    );
}

#[test]
fn standard_transport_rejections_are_deterministic() {
    let parse = rejected("{");
    assert_eq!(parse.id(), None);
    assert_eq!(parse.error().code(), JSON_RPC_PARSE_ERROR);
    assert_eq!(
        parse.to_json(),
        r#"{"error":{"code":-32700,"message":"Parse error"},"id":null,"jsonrpc":"2.0"}"#
    );

    for invalid in [
        "[]",
        r#"{"id":"string","jsonrpc":"2.0","method":"describe_adapter"}"#,
        r#"{"id":-1,"jsonrpc":"2.0","method":"describe_adapter"}"#,
        r#"{"id":1.5,"jsonrpc":"2.0","method":"describe_adapter"}"#,
        r#"{"id":9007199254740992,"jsonrpc":"2.0","method":"describe_adapter"}"#,
    ] {
        let response = rejected(invalid);
        assert_eq!(response.id(), None);
        assert_eq!(response.error().code(), JSON_RPC_INVALID_REQUEST);
    }

    let invalid_version =
        rejected(r#"{"id":3,"jsonrpc":"1.0","method":"describe_adapter"}"#);
    assert_eq!(invalid_version.id(), Some(RequestId::new(3).unwrap()));
    assert_eq!(invalid_version.error().code(), JSON_RPC_INVALID_REQUEST);

    let unknown = rejected(r#"{"id":4,"jsonrpc":"2.0","method":"adapter.describe"}"#);
    assert_eq!(unknown.id(), Some(RequestId::new(4).unwrap()));
    assert_eq!(unknown.error().code(), JSON_RPC_METHOD_NOT_FOUND);

    let invalid_params = rejected(
        r#"{"id":5,"jsonrpc":"2.0","method":"validate_plan","params":[]}"#,
    );
    assert_eq!(invalid_params.id(), Some(RequestId::new(5).unwrap()));
    assert_eq!(invalid_params.error().code(), JSON_RPC_INVALID_PARAMS);
}

#[test]
fn notifications_are_neither_dispatched_nor_answered() {
    for notification in [
        r#"{"jsonrpc":"2.0","method":"describe_adapter"}"#,
        r#"{"jsonrpc":"2.0","method":"validate_plan","params":{}}"#,
        r#"{"jsonrpc":"2.0","method":"execute_plan","params":{}}"#,
        r#"{"jsonrpc":"2.0","method":"unknown"}"#,
    ] {
        assert_eq!(
            decode_request(notification),
            RequestDisposition::IgnoreNotification
        );
    }
}

#[test]
fn protocol_params_remain_canonical_pass_through_data() {
    let payload: Value = serde_json::from_str(VALIDATE_REQUEST).unwrap();
    let request = TransportRequest::new(
        RequestId::new(11).unwrap(),
        AdapterTransportMethod::ValidatePlan,
        Some(payload.clone()),
    )
    .unwrap();

    let encoded: Value = serde_json::from_str(&request.to_json()).unwrap();
    assert_eq!(encoded.get("params"), Some(&payload));
    assert!(encoded.get("params").unwrap().get("id").is_none());

    let decoded = match decode_request(&request.to_json()) {
        RequestDisposition::Dispatch(decoded) => decoded,
        other => panic!("expected dispatch, got {other:?}"),
    };
    assert_eq!(decoded.params(), Some(&payload));
    ValidatePlanRequest::from_json(&serde_json::to_string(decoded.params().unwrap()).unwrap())
        .unwrap();
}

#[test]
fn protocol_success_and_failure_both_use_the_result_channel() {
    let id = RequestId::new(21).unwrap();
    let success = JsonRpcResponse::protocol_success(id, json!({"accepted": true})).unwrap();
    let success_json = success.to_json();
    assert!(success_json.contains(r#""kind":"protocol_success""#));
    assert!(success_json.contains(r#""result""#));
    assert!(!success_json.contains(r#""error""#));
    assert_eq!(
        JsonRpcResponse::from_json(&success_json, AdapterTransportMethod::ValidatePlan).unwrap(),
        success
    );

    let failure = ProtocolFailure::invalid_request("request is not canonical").unwrap();
    let failure_response = JsonRpcResponse::protocol_failure(
        id,
        AdapterTransportMethod::ValidatePlan,
        &failure,
    )
    .unwrap();
    let failure_json = failure_response.to_json();
    assert!(failure_json.contains(r#""kind":"protocol_failure""#));
    assert!(failure_json.contains(r#""result""#));
    assert!(!failure_json.contains(r#""error""#));

    let decoded = JsonRpcResponse::from_json(
        &failure_json,
        AdapterTransportMethod::ValidatePlan,
    )
    .unwrap();
    assert!(matches!(
        decoded,
        JsonRpcResponse::ProtocolResult {
            outcome: ProtocolOutcome::Failure(decoded_failure),
            ..
        } if decoded_failure == failure
    ));
}

#[test]
fn transport_errors_stay_out_of_protocol_results() {
    let response = JsonRpcResponse::transport_error(
        Some(RequestId::new(31).unwrap()),
        JsonRpcErrorKind::MethodNotFound,
    );
    let json = response.to_json();
    assert!(json.contains(r#""error""#));
    assert!(!json.contains(r#""result""#));
    assert_eq!(
        JsonRpcResponse::from_json(&json, AdapterTransportMethod::DescribeAdapter).unwrap(),
        response
    );
}

#[test]
fn response_ids_must_correlate_exactly() {
    let expected = RequestId::new(40).unwrap();
    let matching = JsonRpcResponse::protocol_success(expected, json!({"ok": true})).unwrap();
    matching.correlate(expected).unwrap();

    let other = JsonRpcResponse::protocol_success(
        RequestId::new(41).unwrap(),
        json!({"ok": true}),
    )
    .unwrap();
    let mismatch = other.correlate(expected).unwrap_err();
    assert_eq!(mismatch.expected, expected);
    assert_eq!(mismatch.actual, Some(RequestId::new(41).unwrap()));

    let uncorrelated = JsonRpcResponse::transport_error(None, JsonRpcErrorKind::ParseError);
    assert_eq!(uncorrelated.correlate(expected).unwrap_err().actual, None);
}

#[test]
fn malformed_response_envelopes_are_rejected() {
    for invalid in [
        r#"{"id":1,"jsonrpc":"2.0","result":{},"error":{}}"#,
        r#"{"id":1,"jsonrpc":"2.0","result":{"kind":"unknown","payload":{}}}"#,
        r#"{"id":null,"jsonrpc":"2.0","result":{"kind":"protocol_success","payload":{}}}"#,
        r#"{"id":1,"jsonrpc":"2.0","result":{"kind":"protocol_success","payload":[]}}"#,
    ] {
        assert!(matches!(
            JsonRpcResponse::from_json(invalid, AdapterTransportMethod::DescribeAdapter),
            Err(ResponseDecodeError::InvalidEnvelope(_))
        ));
    }
}

#[test]
fn stdio_framing_handles_chunks_multiple_messages_and_crlf() {
    let first = r#"{"id":1,"jsonrpc":"2.0","method":"describe_adapter"}"#;
    let second = r#"{"id":2,"jsonrpc":"2.0","method":"describe_adapter"}"#;
    assert_eq!(
        encode_stdio_frame(first).unwrap(),
        format!("{first}\n").into_bytes()
    );

    let mut decoder = StdioFrameDecoder::new();
    assert!(decoder.push(&first.as_bytes()[..20]).is_empty());
    let remainder = format!("{}\r\n{second}\n", &first[20..]);
    let frames = decoder.push(remainder.as_bytes());
    assert_eq!(frames, vec![Ok(first.to_owned()), Ok(second.to_owned())]);
    assert_eq!(decoder.pending_bytes(), 0);
    decoder.finish().unwrap();
}

#[test]
fn stdio_framing_rejects_empty_invalid_and_unterminated_frames() {
    let mut decoder = StdioFrameDecoder::new();
    assert_eq!(decoder.push(b" \t\r\n"), vec![Err(StdioFrameError::EmptyFrame)]);
    assert_eq!(
        decoder.push(&[0xff, b'\n']),
        vec![Err(StdioFrameError::InvalidUtf8)]
    );
    assert!(decoder.push(br#"{"jsonrpc":"2.0"}"#).is_empty());
    assert_eq!(
        decoder.finish(),
        Err(StdioFrameError::UnterminatedFrame { buffered_bytes: 17 })
    );

    assert!(matches!(
        encode_stdio_frame("[]"),
        Err(StdioFrameError::NonObjectMessage)
    ));
}

#[test]
fn batches_are_not_part_of_the_local_single_call_profile() {
    let response = rejected(
        r#"[{"id":1,"jsonrpc":"2.0","method":"describe_adapter"}]"#,
    );
    assert_eq!(response.id(), None);
    assert_eq!(response.error().code(), JSON_RPC_INVALID_REQUEST);
}

#[test]
fn phase_1_boundary_records_safety_counterexamples() {
    for required in [
        "request ID -> Protocol payload",
        "same request ID -> retry or replay authority",
        "notification -> execute without an answer",
        "ProtocolFailure -> JSON-RPC error",
        "batch -> implicit operation ordering",
        "network transport -> Phase 1 scope",
    ] {
        assert!(
            PHASE_1_BOUNDARY.contains(required),
            "missing Phase 1 counterexample: {required}"
        );
    }

    for forbidden_dependency in ["jsonrpsee", "tokio", "reqwest", "tonic", "zmq"] {
        assert!(!TRANSPORT_CARGO.contains(forbidden_dependency));
    }
}

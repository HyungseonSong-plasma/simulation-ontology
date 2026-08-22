use serde_json::Value;
use sol_adapter_protocol::{ValidatePlanRequest, FAILURE_OPERATIONAL};
use sol_adapter_transport::{
    AdapterOperationResult, AdapterProcessCommand, AdapterProcessSession, AdapterSessionError,
    AdapterSessionErrorKind, AdapterSessionState, ResponseDecodeError, StdioFrameError,
    JSON_RPC_INVALID_REQUEST, JSON_RPC_METHOD_NOT_FOUND, JSON_RPC_PARSE_ERROR,
};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const ERROR_PROFILE: &str = include_str!(
    "../../../docs/implementation/m0.5-transport-and-process-error-propagation.md"
);

fn probe_command(mode: &str) -> AdapterProcessCommand {
    AdapterProcessCommand::new(env!("CARGO_BIN_EXE_sol-transport-error-probe")).arg(mode)
}

fn validate_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap()
}

#[test]
fn malformed_json_and_invalid_envelopes_are_response_errors() {
    let mut malformed = AdapterProcessSession::spawn(probe_command("malformed-json")).unwrap();
    let error = malformed.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::ResponseEnvelope);
    assert!(matches!(
        error,
        AdapterSessionError::Response(ResponseDecodeError::InvalidJson(_))
    ));
    assert_eq!(malformed.state(), AdapterSessionState::Faulted);
    assert!(malformed.shutdown().unwrap().success);

    let mut invalid = AdapterProcessSession::spawn(probe_command("invalid-envelope")).unwrap();
    let error = invalid.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::ResponseEnvelope);
    assert!(matches!(
        error,
        AdapterSessionError::Response(ResponseDecodeError::InvalidEnvelope(_))
    ));
    assert_eq!(invalid.state(), AdapterSessionState::Faulted);
    assert!(invalid.shutdown().unwrap().success);
}

#[test]
fn non_protocol_stdout_is_never_treated_as_a_protocol_result() {
    let mut human = AdapterProcessSession::spawn(probe_command("human-stdout")).unwrap();
    let error = human.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::ResponseEnvelope);
    assert!(matches!(
        error,
        AdapterSessionError::Response(ResponseDecodeError::InvalidJson(_))
    ));
    assert!(human.shutdown().unwrap().success);

    let mut invalid_utf8 =
        AdapterProcessSession::spawn(probe_command("invalid-utf8")).unwrap();
    let error = invalid_utf8
        .validate_plan(&validate_request())
        .unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::Framing);
    assert_eq!(
        error,
        AdapterSessionError::Framing(StdioFrameError::InvalidUtf8)
    );
    assert!(invalid_utf8.shutdown().unwrap().success);
}

#[test]
fn eof_crash_and_abnormal_exit_keep_operation_and_process_evidence_separate() {
    let mut eof = AdapterProcessSession::spawn(probe_command("eof")).unwrap();
    let error = eof.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = eof.shutdown().unwrap();
    assert!(exit.success);
    assert_eq!(exit.code, Some(0));
    assert!(exit.stderr.is_empty());

    let mut crash = AdapterProcessSession::spawn(probe_command("crash")).unwrap();
    let error = crash.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = crash.shutdown().unwrap();
    assert!(exit.is_abnormal());
    assert_ne!(exit.code, Some(0));
    assert!(String::from_utf8_lossy(&exit.stderr).contains("intentional transport probe crash"));

    let mut abnormal = AdapterProcessSession::spawn(probe_command("abnormal-exit")).unwrap();
    let error = abnormal.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = abnormal.shutdown().unwrap();
    assert!(exit.is_abnormal());
    assert_eq!(exit.code, Some(23));
    assert_eq!(exit.stderr, b"opaque abnormal-exit diagnostic\n".to_vec());
}

#[test]
fn a_closed_child_stdin_is_reported_as_broken_pipe_without_retry() {
    let mut session = AdapterProcessSession::spawn(probe_command("broken-pipe")).unwrap();
    thread::sleep(Duration::from_millis(100));

    let error = session.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdinBrokenPipe);
    assert!(matches!(error, AdapterSessionError::BrokenPipe(_)));
    assert_eq!(session.state(), AdapterSessionState::Faulted);

    let exit = session.shutdown().unwrap();
    assert!(exit.is_abnormal());
    assert_eq!(exit.code, Some(29));
    assert_eq!(
        exit.stderr,
        b"probe closed stdin by exiting after bootstrap\n".to_vec()
    );
}

#[test]
fn stderr_is_preserved_as_opaque_bytes_only_at_the_process_boundary() {
    let session = AdapterProcessSession::spawn(probe_command("stderr-bytes")).unwrap();
    let exit = session.shutdown().unwrap();
    assert!(exit.success);
    assert_eq!(exit.stderr, vec![b'o', b'p', b'a', b'q', b'u', b'e', b':', 0xff, b'\n']);
}

#[test]
fn a_valid_operation_protocol_failure_remains_a_protocol_result() {
    let mut session =
        AdapterProcessSession::spawn(probe_command("protocol-failure")).unwrap();
    let result = session.validate_plan(&validate_request()).unwrap();
    let AdapterOperationResult::ProtocolFailure(failure) = result else {
        panic!("injected logical failure did not remain ProtocolFailure");
    };
    assert_eq!(failure.code, FAILURE_OPERATIONAL);
    assert_eq!(session.state(), AdapterSessionState::Ready);
    assert!(session.shutdown().unwrap().success);
}

#[test]
fn worker_returns_standard_json_rpc_errors_for_bad_transport_requests() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = BufReader::new(child.stdout.take().unwrap());
    let mut stderr = child.stderr.take().unwrap();

    assert_json_rpc_error(
        raw_exchange(&mut input, &mut output, "{not-json}\n"),
        JSON_RPC_PARSE_ERROR,
        Value::Null,
    );
    assert_json_rpc_error(
        raw_exchange(
            &mut input,
            &mut output,
            "{\"jsonrpc\":\"1.0\",\"method\":\"describe_adapter\",\"id\":3}\n",
        ),
        JSON_RPC_INVALID_REQUEST,
        Value::from(3),
    );
    assert_json_rpc_error(
        raw_exchange(
            &mut input,
            &mut output,
            "{\"jsonrpc\":\"2.0\",\"method\":\"unknown_method\",\"id\":7}\n",
        ),
        JSON_RPC_METHOD_NOT_FOUND,
        Value::from(7),
    );

    drop(input);
    drop(output);
    let status = child.wait().unwrap();
    let mut diagnostics = Vec::new();
    stderr.read_to_end(&mut diagnostics).unwrap();
    assert!(status.success());
    assert!(diagnostics.is_empty());
}

#[test]
fn documented_error_matrix_rejects_semantic_conflation_and_silent_retry() {
    for required in [
        "transport/process error -> ProtocolFailure",
        "transport/process error -> PASS/FAIL/BLOCKED/INDETERMINATE",
        "response loss -> automatic retry",
        "stderr text -> Protocol or SOL semantics",
        "child exit code -> Adapter Protocol outcome",
    ] {
        assert!(
            ERROR_PROFILE.contains(required),
            "missing Phase 3 counterexample: {required}"
        );
    }
}

fn raw_exchange(
    input: &mut impl Write,
    output: &mut impl BufRead,
    request: &str,
) -> Value {
    input.write_all(request.as_bytes()).unwrap();
    input.flush().unwrap();
    let mut response = String::new();
    output.read_line(&mut response).unwrap();
    serde_json::from_str(&response).unwrap()
}

fn assert_json_rpc_error(response: Value, code: i32, id: Value) {
    assert_eq!(response.get("jsonrpc"), Some(&Value::from("2.0")));
    assert_eq!(response.get("id"), Some(&id));
    assert_eq!(response.pointer("/error/code"), Some(&Value::from(code)));
    assert!(response.get("result").is_none());
}

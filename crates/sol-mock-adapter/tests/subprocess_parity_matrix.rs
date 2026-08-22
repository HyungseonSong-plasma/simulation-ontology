use sol_adapter_protocol::{
    ExecutePlanRequest, ExecutePlanResponse, ProtocolFailure, SideEffectEvidence,
    ValidatePlanRequest, ValidatePlanResponse,
};
use sol_adapter_transport::{
    response_loss_recovery, AdapterOperationResult, AdapterProcessCommand, AdapterProcessSession,
    AdapterSessionError, AdapterSessionErrorKind, AdapterTransportMethod, ReplayDisposition,
};
use sol_mock_adapter::{MockAdapterProfile, MockProtocolFailureState};

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");
const EXECUTE_CHANGED_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-changed-request.json");
const EXECUTE_INDEPENDENT_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-independent-request.json");
const EXIT_AUDIT: &str =
    include_str!("../../../docs/implementation/m0.5-json-rpc-stdio-transport-exit-audit.md");
const TRANSPORT_CARGO: &str = include_str!("../../sol-adapter-transport/Cargo.toml");
const MOCK_CARGO: &str = include_str!("../Cargo.toml");

fn profile_command(profile: MockAdapterProfile) -> AdapterProcessCommand {
    AdapterProcessCommand::new(env!("CARGO_BIN_EXE_sol-mock-adapter-stdio"))
        .arg(format!("--profile={}", profile.wire_name()))
}

fn probe_command(mode: &str) -> AdapterProcessCommand {
    AdapterProcessCommand::new(env!("CARGO_BIN_EXE_sol-transport-error-probe")).arg(mode)
}

fn validate_request() -> ValidatePlanRequest {
    ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap()
}

fn execute_request(source: &str) -> ExecutePlanRequest {
    ExecutePlanRequest::from_json(source).unwrap()
}

fn subprocess_validate(
    profile: MockAdapterProfile,
    request: &ValidatePlanRequest,
) -> Result<ValidatePlanResponse, ProtocolFailure> {
    let mut session = AdapterProcessSession::spawn(profile_command(profile)).unwrap();
    let result = session.validate_plan(request).unwrap();
    let exit = session.shutdown().unwrap();
    assert!(
        exit.success,
        "profile {} exited abnormally",
        profile.wire_name()
    );
    assert!(
        exit.stderr.is_empty(),
        "profile {} wrote stderr",
        profile.wire_name()
    );
    match result {
        AdapterOperationResult::Success(response) => Ok(response),
        AdapterOperationResult::ProtocolFailure(failure) => Err(failure),
    }
}

fn subprocess_execute(
    profile: MockAdapterProfile,
    request: &ExecutePlanRequest,
) -> Result<ExecutePlanResponse, ProtocolFailure> {
    let mut session = AdapterProcessSession::spawn(profile_command(profile)).unwrap();
    let result = session.execute_plan(request).unwrap();
    let exit = session.shutdown().unwrap();
    assert!(
        exit.success,
        "profile {} exited abnormally",
        profile.wire_name()
    );
    assert!(
        exit.stderr.is_empty(),
        "profile {} wrote stderr",
        profile.wire_name()
    );
    match result {
        AdapterOperationResult::Success(response) => Ok(response),
        AdapterOperationResult::ProtocolFailure(failure) => Err(failure),
    }
}

fn canonical_validate(outcome: &Result<ValidatePlanResponse, ProtocolFailure>) -> String {
    match outcome {
        Ok(response) => format!("success:{}", response.to_canonical_json().unwrap()),
        Err(failure) => format!("failure:{}", failure.to_canonical_json().unwrap()),
    }
}

fn canonical_execute(outcome: &Result<ExecutePlanResponse, ProtocolFailure>) -> String {
    match outcome {
        Ok(response) => format!("success:{}", response.to_canonical_json().unwrap()),
        Err(failure) => format!("failure:{}", failure.to_canonical_json().unwrap()),
    }
}

#[test]
fn every_cli_profile_has_a_stable_round_trip_name() {
    for profile in MockAdapterProfile::ALL {
        assert_eq!(
            MockAdapterProfile::parse(profile.wire_name()),
            Some(profile),
            "profile did not parse: {}",
            profile.wire_name()
        );
    }
    assert_eq!(MockAdapterProfile::parse("unknown"), None);
}

#[test]
fn describe_success_and_compatibility_failure_match_in_process_semantics() {
    let expected = MockAdapterProfile::Exact
        .adapter()
        .describe_adapter_operation()
        .unwrap()
        .to_canonical_json()
        .unwrap();
    let session = AdapterProcessSession::spawn(profile_command(MockAdapterProfile::Exact)).unwrap();
    let actual = session.description().unwrap().to_canonical_json().unwrap();
    assert_eq!(actual, expected);
    let exit = session.shutdown().unwrap();
    assert!(exit.success);
    assert!(exit.stderr.is_empty());

    let expected_failure = MockAdapterProfile::FailureCompatibility
        .adapter()
        .describe_adapter_operation()
        .unwrap_err();
    let error = match AdapterProcessSession::spawn(profile_command(
        MockAdapterProfile::FailureCompatibility,
    )) {
        Ok(session) => {
            drop(session);
            panic!("compatibility failure unexpectedly produced a ready session");
        }
        Err(error) => error,
    };
    let AdapterSessionError::BootstrapProtocolFailure(actual_failure) = error else {
        panic!("compatibility failure crossed the process boundary as {error:?}");
    };
    assert_eq!(
        actual_failure.to_canonical_json().unwrap(),
        expected_failure.to_canonical_json().unwrap()
    );
}

#[test]
fn validate_matrix_is_canonically_equal_across_in_process_and_subprocess_paths() {
    let request = validate_request();
    for profile in [
        MockAdapterProfile::Exact,
        MockAdapterProfile::PreflightPrerequisiteRejected,
        MockAdapterProfile::PreflightTransientUnavailable,
        MockAdapterProfile::FailureInvalidRequest,
        MockAdapterProfile::FailureValidateOperational,
    ] {
        let expected = profile.adapter().validate_plan_operation(&request);
        let actual = subprocess_validate(profile, &request);
        assert_eq!(
            canonical_validate(&actual),
            canonical_validate(&expected),
            "validate parity failed for {}",
            profile.wire_name()
        );
    }

    let mut target_mismatch = validate_request();
    target_mismatch.target.target = "comsol".to_owned();
    target_mismatch.target.required_capabilities = vec!["thermal.solve".to_owned()];

    let mut missing_capability = validate_request();
    missing_capability.target.required_capabilities = vec!["thermal.radiation".to_owned()];

    let mut unsupported_action = validate_request();
    unsupported_action.plan.actions[2].id = "thermal.radiation".to_owned();

    for (name, request) in [
        ("target-mismatch", target_mismatch),
        ("missing-capability", missing_capability),
        ("unsupported-action", unsupported_action),
    ] {
        let expected = MockAdapterProfile::Exact
            .adapter()
            .validate_plan_operation(&request);
        let actual = subprocess_validate(MockAdapterProfile::Exact, &request);
        assert_eq!(
            canonical_validate(&actual),
            canonical_validate(&expected),
            "validate adversarial parity failed for {name}"
        );
    }
}

#[test]
fn execute_matrix_is_canonically_equal_across_in_process_and_subprocess_paths() {
    let thermal = execute_request(EXECUTE_REQUEST);
    for profile in [
        MockAdapterProfile::Exact,
        MockAdapterProfile::ExecutionDegraded,
        MockAdapterProfile::ExecutionUnsupported,
        MockAdapterProfile::ExecutionPartial,
        MockAdapterProfile::ExecutionUnavailable,
        MockAdapterProfile::FailureInvalidRequest,
        MockAdapterProfile::FailureExecuteBeforeSideEffect,
        MockAdapterProfile::FailureExecuteAmbiguous,
        MockAdapterProfile::PriorAlreadyRealized,
        MockAdapterProfile::PriorPartialExecution,
        MockAdapterProfile::PriorUnresolvedPrerequisite,
    ] {
        let expected = profile.adapter().execute_plan_operation(&thermal);
        let actual = subprocess_execute(profile, &thermal);
        assert_eq!(
            canonical_execute(&actual),
            canonical_execute(&expected),
            "execute parity failed for {}",
            profile.wire_name()
        );
    }

    let changed = execute_request(EXECUTE_CHANGED_REQUEST);
    let expected = MockAdapterProfile::ExecutionAuthoritativeRejected
        .adapter()
        .execute_plan_operation(&changed);
    let actual = subprocess_execute(MockAdapterProfile::ExecutionAuthoritativeRejected, &changed);
    assert_eq!(canonical_execute(&actual), canonical_execute(&expected));

    let independent = execute_request(EXECUTE_INDEPENDENT_REQUEST);
    for profile in [
        MockAdapterProfile::ExecutionAlternateOrder,
        MockAdapterProfile::ExecutionParallelIndependent,
    ] {
        let expected = profile.adapter().execute_plan_operation(&independent);
        let actual = subprocess_execute(profile, &independent);
        assert_eq!(
            canonical_execute(&actual),
            canonical_execute(&expected),
            "independent execute parity failed for {}",
            profile.wire_name()
        );
    }
}

#[test]
fn malformed_envelope_abnormal_exit_and_ambiguous_execute_loss_stay_transport_evidence() {
    let mut invalid = AdapterProcessSession::spawn(probe_command("invalid-envelope")).unwrap();
    let error = invalid.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::ResponseEnvelope);
    assert!(invalid.shutdown().unwrap().success);

    let mut abnormal = AdapterProcessSession::spawn(probe_command("abnormal-exit")).unwrap();
    let error = abnormal.validate_plan(&validate_request()).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = abnormal.shutdown().unwrap();
    assert!(exit.is_abnormal());
    assert_eq!(exit.code, Some(23));
    assert_eq!(exit.stderr, b"opaque abnormal-exit diagnostic\n".to_vec());

    let request = execute_request(EXECUTE_REQUEST);
    let mut lost = AdapterProcessSession::spawn(probe_command("execute-response-loss")).unwrap();
    let error = lost.execute_plan(&request).unwrap_err();
    assert_eq!(error.kind(), AdapterSessionErrorKind::StdoutEof);
    let exit = lost.shutdown().unwrap();
    assert_eq!(
        exit.stderr,
        b"execute_plan request received before response loss\n".to_vec()
    );

    let recovery = response_loss_recovery(AdapterTransportMethod::ExecutePlan);
    assert_eq!(
        recovery.replay(),
        ReplayDisposition::TransportMustNotReplayExecution
    );
    assert_eq!(
        recovery.conservative_side_effects(),
        SideEffectEvidence::MayHaveOccurred
    );

    let replacement = AdapterProcessSession::spawn(probe_command("no-replay-observer")).unwrap();
    let exit = replacement.shutdown().unwrap();
    assert!(exit.success);
    assert!(exit.stderr.is_empty());
}

#[test]
fn canonical_protocol_outputs_and_dependencies_reject_boundary_leakage() {
    let description = MockAdapterProfile::Exact
        .adapter()
        .describe_adapter_operation()
        .unwrap()
        .to_canonical_json()
        .unwrap();
    let validation = MockAdapterProfile::Exact
        .adapter()
        .validate_plan_operation(&validate_request())
        .unwrap()
        .to_canonical_json()
        .unwrap();
    let execution = MockAdapterProfile::Exact
        .adapter()
        .execute_plan_operation(&execute_request(EXECUTE_REQUEST))
        .unwrap()
        .to_canonical_json()
        .unwrap();
    let failure = MockAdapterProfile::Exact
        .adapter()
        .with_protocol_failure_state(MockProtocolFailureState::ExecuteOperationalAmbiguous)
        .execute_plan_operation(&execute_request(EXECUTE_REQUEST))
        .unwrap_err()
        .to_canonical_json()
        .unwrap();

    for canonical in [description, validation, execution, failure] {
        for forbidden in [
            "\"jsonrpc\"",
            "\"request_id\"",
            "\"process_id\"",
            "\"session_state\"",
            "\"retryable\"",
            "\"safe_to_retry\"",
            "comsol",
            "ansys",
        ] {
            assert!(
                !canonical.contains(forbidden),
                "transport/lifecycle/backend field leaked: {forbidden}"
            );
        }
    }

    for dependency in ["tokio", "reqwest", "tonic", "zmq", "comsol", "ansys"] {
        assert!(!TRANSPORT_CARGO.contains(dependency));
        assert!(!MOCK_CARGO.contains(dependency));
    }

    for required in [
        "in-process result -> subprocess result",
        "process exit -> Adapter Protocol outcome",
        "lost execute response -> automatic replay",
        "profile name -> semantic identity",
        "stderr -> SOL lifecycle state",
    ] {
        assert!(
            EXIT_AUDIT.contains(required),
            "exit audit is missing rejected counterexample: {required}"
        );
    }
}

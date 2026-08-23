use sol_external_runtime_v02_consumer::{
    adapter_from_env, fixture_from_env, prove_bootstrap_failure_is_runtime_error,
    prove_execute_response_loss_has_no_replay_authority,
    prove_multiple_v02_candidates_are_ambiguous, prove_v01_not_eligible_for_v02,
    run_external_v02_flow,
};

#[test]
fn standalone_external_command_completes_explicit_v02_runtime_flow() {
    let adapter = adapter_from_env("SOL_RUNTIME_V02_ADAPTER").unwrap();
    let fixture = fixture_from_env().unwrap();
    let evidence = run_external_v02_flow(&adapter, &fixture).unwrap();

    assert_eq!(evidence["adapter_protocol_version"], "0.2");
    assert_eq!(evidence["public_contract_version"], "0.2");
    assert_eq!(evidence["selection"], "selected");
    assert_eq!(evidence["validate"], "accepted");
    assert_eq!(evidence["execute"], "completed");
    assert_eq!(evidence["shutdown"], "success");
    assert_eq!(evidence["execute_response_loss_replay"], "forbidden");
}

#[test]
fn v01_external_adapter_is_not_eligible_for_explicit_v02_runtime() {
    let adapter = adapter_from_env("SOL_RUNTIME_V01_ADAPTER").unwrap();
    prove_v01_not_eligible_for_v02(&adapter).unwrap();
}

#[test]
fn two_external_v02_candidates_remain_ambiguous() {
    let adapter = adapter_from_env("SOL_RUNTIME_V02_ADAPTER").unwrap();
    prove_multiple_v02_candidates_are_ambiguous(&adapter).unwrap();
}

#[test]
fn external_bootstrap_failure_remains_runtime_failure() {
    let adapter = adapter_from_env("SOL_RUNTIME_V01_ADAPTER").unwrap();
    prove_bootstrap_failure_is_runtime_error(&adapter).unwrap();
}

#[test]
fn execute_response_loss_never_authorizes_transport_replay() {
    prove_execute_response_loss_has_no_replay_authority().unwrap();
}

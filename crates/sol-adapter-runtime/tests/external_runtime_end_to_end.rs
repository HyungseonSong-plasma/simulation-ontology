use std::path::{Path, PathBuf};

use sol_adapter_protocol::{ExecutePlanRequest, SideEffectEvidence, ValidatePlanRequest};
use sol_adapter_runtime::{
    select_adapter, AdapterCommand, AdapterInstanceId, AdapterRegistration,
    AdapterRegistrationId, AdapterRegistry, AdapterRuntimeError, AdapterRuntimeProjection,
    AdapterSelectionOutcome, AdapterSelectionRequest, RunningAdapter,
};
use sol_adapter_transport::{
    AdapterOperationResult, AdapterSessionError, AdapterTransportMethod, ReplayDisposition,
};
use sol_target_resolver::{BackendCapability, BackendTarget};

const VALIDATE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/validate-plan-accepted-request.json");
const EXECUTE_REQUEST: &str =
    include_str!("../../../fixtures/adapter-protocol/0.1/execute-plan-thermal-request.json");

fn mock_adapter_binary() -> PathBuf {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("runtime crate must live under workspace/crates");
    let executable = if cfg!(windows) {
        "sol-mock-adapter-stdio.exe"
    } else {
        "sol-mock-adapter-stdio"
    };
    let path = workspace.join("target").join("debug").join(executable);
    assert!(path.is_file(), "mock adapter binary missing at {}", path.display());
    path
}

fn thermal_selection_request() -> AdapterSelectionRequest {
    AdapterSelectionRequest::new(BackendTarget::new("mock"))
        .require(BackendCapability::new("thermal.domain"))
        .require(BackendCapability::new("thermal.material"))
        .require(BackendCapability::new("thermal.solve"))
}

#[test]
fn explicit_registration_to_selected_external_protocol_dispatch_is_end_to_end() {
    let mut registry = AdapterRegistry::new();
    let registration_id = AdapterRegistrationId::new("external.mock.thermal");
    registry
        .register(AdapterRegistration::new(
            registration_id.clone(),
            AdapterCommand::new(mock_adapter_binary()),
        ))
        .unwrap();

    let mut running = RunningAdapter::launch(
        registry.get(&registration_id).unwrap(),
        AdapterInstanceId::new("instance.external.mock.thermal"),
    )
    .unwrap();

    let projection = AdapterRuntimeProjection::from_running(&running).unwrap();
    let selected = select_adapter(&thermal_selection_request(), std::slice::from_ref(&projection));
    let AdapterSelectionOutcome::Selected(candidate) = selected else {
        panic!("one compatible external adapter must be selected");
    };
    assert_eq!(candidate.registration_id(), &registration_id);
    assert_eq!(candidate.instance_id(), running.instance().id());
    assert_eq!(candidate.target().as_str(), "mock");

    let validate = ValidatePlanRequest::from_json(VALIDATE_REQUEST).unwrap();
    assert!(matches!(
        running.validate_plan(&validate).unwrap(),
        AdapterOperationResult::Success(_)
    ));

    let execute = ExecutePlanRequest::from_json(EXECUTE_REQUEST).unwrap();
    assert!(matches!(
        running.execute_plan(&execute).unwrap(),
        AdapterOperationResult::Success(_)
    ));

    let loss = RunningAdapter::response_loss_policy(AdapterTransportMethod::ExecutePlan);
    assert_eq!(
        loss.replay(),
        ReplayDisposition::TransportMustNotReplayExecution
    );
    assert_eq!(
        loss.conservative_side_effects(),
        SideEffectEvidence::MayHaveOccurred
    );
    assert!(!RunningAdapter::execution_response_loss_allows_transport_replay());

    assert!(running.shutdown().unwrap().success);
}

#[test]
fn process_spawn_failure_remains_runtime_transport_evidence() {
    let mut registry = AdapterRegistry::new();
    let registration_id = AdapterRegistrationId::new("missing.external.adapter");
    registry
        .register(AdapterRegistration::new(
            registration_id.clone(),
            AdapterCommand::new(format!(
                "sol-adapter-command-that-does-not-exist-{}",
                std::process::id()
            )),
        ))
        .unwrap();

    assert!(matches!(
        RunningAdapter::launch(
            registry.get(&registration_id).unwrap(),
            AdapterInstanceId::new("instance.missing")
        ),
        Err(AdapterRuntimeError::Transport(AdapterSessionError::Spawn(_)))
    ));
}

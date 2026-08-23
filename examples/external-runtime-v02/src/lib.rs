use serde_json::{json, Value};
use sol_adapter_protocol::{
    ExecutePlanRequestV02, ExecutionOutcome, PreflightOutcome, ValidatePlanRequestV02,
};
use sol_adapter_runtime::{
    select_adapter, AdapterCommand, AdapterInstanceId, AdapterRegistration, AdapterRegistrationId,
    AdapterRegistry, AdapterRuntimeProjection, AdapterSelectionOutcome, AdapterSelectionRequest,
    RunningAdapter, RuntimeContractProfile,
};
use sol_adapter_transport::{AdapterOperationResult, AdapterTransportMethod, ReplayDisposition};
use sol_target_resolver::{BackendCapability, BackendTarget};
use std::fs;
use std::path::{Path, PathBuf};

const TARGET: &str = "mock";
const REQUIRED_CAPABILITIES: [&str; 3] = ["thermal.domain", "thermal.material", "thermal.solve"];

pub fn adapter_from_env(name: &str) -> Result<PathBuf, String> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .ok_or_else(|| format!("missing required environment variable {name}"))
}

pub fn fixture_from_env() -> Result<PathBuf, String> {
    adapter_from_env("SOL_RUNTIME_V02_FIXTURE")
}

pub fn run_external_v02_flow(adapter: &Path, fixture: &Path) -> Result<Value, String> {
    let mut registry = AdapterRegistry::new();
    let registration_id = AdapterRegistrationId::new("external.runtime.v02");
    registry
        .register(AdapterRegistration::new(
            registration_id.clone(),
            AdapterCommand::new(adapter),
        ))
        .map_err(debug_error)?;

    let mut running = RunningAdapter::launch(
        registry
            .get(&registration_id)
            .ok_or_else(|| "external v0.2 registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.v02.instance"),
    )
    .map_err(debug_error)?;

    let profile = RuntimeContractProfile::realization_v02();
    let projection =
        AdapterRuntimeProjection::from_running_for(&running, profile).map_err(debug_error)?;
    let request = selection_request(profile);
    let selected = match select_adapter(&request, std::slice::from_ref(&projection)) {
        AdapterSelectionOutcome::Selected(candidate) => candidate,
        other => {
            let _ = running.shutdown();
            return Err(format!(
                "expected exactly one external v0.2 candidate, got {other:?}"
            ));
        }
    };

    let fixture_json = fs::read_to_string(fixture).map_err(|error| error.to_string())?;
    let validate_request = ValidatePlanRequestV02::from_json(&fixture_json).map_err(debug_error)?;
    let execute_request = ExecutePlanRequestV02::from_json(&fixture_json).map_err(debug_error)?;

    let validate_response = match running
        .validate_plan_v02(&validate_request)
        .map_err(debug_error)?
    {
        AdapterOperationResult::Success(response) => response,
        AdapterOperationResult::ProtocolFailure(failure) => {
            let _ = running.shutdown();
            return Err(format!(
                "external v0.2 validation returned ProtocolFailure: {failure:?}"
            ));
        }
    };
    if validate_response.preflight != PreflightOutcome::Accepted {
        let _ = running.shutdown();
        return Err(format!(
            "expected accepted external v0.2 preflight, got {:?}",
            validate_response.preflight
        ));
    }

    let execute_response = match running
        .execute_plan_v02(&execute_request)
        .map_err(debug_error)?
    {
        AdapterOperationResult::Success(response) => response,
        AdapterOperationResult::ProtocolFailure(failure) => {
            let _ = running.shutdown();
            return Err(format!(
                "external v0.2 execution returned ProtocolFailure: {failure:?}"
            ));
        }
    };
    if execute_response.execution != ExecutionOutcome::Completed {
        let _ = running.shutdown();
        return Err(format!(
            "expected completed external v0.2 execution, got {:?}",
            execute_response.execution
        ));
    }

    let no_replay = RunningAdapter::response_loss_policy(AdapterTransportMethod::ExecutePlan)
        .replay()
        == ReplayDisposition::TransportMustNotReplayExecution
        && !RunningAdapter::execution_response_loss_allows_transport_replay();
    let exit = running.shutdown().map_err(debug_error)?;
    if !exit.success {
        return Err(format!(
            "external v0.2 adapter did not shut down successfully: {:?}",
            exit.code
        ));
    }

    Ok(json!({
        "adapter_id": selected.adapter_id(),
        "adapter_protocol_version": selected.profile().adapter_protocol_version().to_string(),
        "public_contract_version": selected.profile().public_contract_version().to_string(),
        "target": selected.target().as_str(),
        "required_capabilities": REQUIRED_CAPABILITIES,
        "selection": "selected",
        "validate": "accepted",
        "execute": "completed",
        "shutdown": "success",
        "execute_response_loss_replay": if no_replay { "forbidden" } else { "unexpectedly_allowed" }
    }))
}

pub fn prove_v01_not_eligible_for_v02(adapter: &Path) -> Result<(), String> {
    let mut registry = AdapterRegistry::new();
    let registration_id = AdapterRegistrationId::new("external.runtime.v01");
    registry
        .register(AdapterRegistration::new(
            registration_id.clone(),
            AdapterCommand::new(adapter),
        ))
        .map_err(debug_error)?;
    let running = RunningAdapter::launch(
        registry
            .get(&registration_id)
            .ok_or_else(|| "external v0.1 registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.v01.instance"),
    )
    .map_err(debug_error)?;

    let profile = RuntimeContractProfile::realization_v02();
    let projection =
        AdapterRuntimeProjection::from_running_for(&running, profile).map_err(debug_error)?;
    let outcome = select_adapter(&selection_request(profile), &[projection]);
    let exit = running.shutdown().map_err(debug_error)?;
    if !exit.success {
        return Err("v0.1 negative-control adapter failed to shut down".to_owned());
    }
    match outcome {
        AdapterSelectionOutcome::NoCompatibleCandidate => Ok(()),
        other => Err(format!(
            "v0.1 adapter was unexpectedly eligible for explicit v0.2 selection: {other:?}"
        )),
    }
}

pub fn prove_multiple_v02_candidates_are_ambiguous(adapter: &Path) -> Result<(), String> {
    let mut first_registry = AdapterRegistry::new();
    let first_id = AdapterRegistrationId::new("external.runtime.v02.first");
    first_registry
        .register(AdapterRegistration::new(
            first_id.clone(),
            AdapterCommand::new(adapter),
        ))
        .map_err(debug_error)?;
    let first = RunningAdapter::launch(
        first_registry
            .get(&first_id)
            .ok_or_else(|| "first v0.2 registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.v02.first.instance"),
    )
    .map_err(debug_error)?;

    let mut second_registry = AdapterRegistry::new();
    let second_id = AdapterRegistrationId::new("external.runtime.v02.second");
    second_registry
        .register(AdapterRegistration::new(
            second_id.clone(),
            AdapterCommand::new(adapter),
        ))
        .map_err(debug_error)?;
    let second = RunningAdapter::launch(
        second_registry
            .get(&second_id)
            .ok_or_else(|| "second v0.2 registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.v02.second.instance"),
    )
    .map_err(debug_error)?;

    let profile = RuntimeContractProfile::realization_v02();
    let first_projection =
        AdapterRuntimeProjection::from_running_for(&first, profile).map_err(debug_error)?;
    let second_projection =
        AdapterRuntimeProjection::from_running_for(&second, profile).map_err(debug_error)?;
    let outcome = select_adapter(
        &selection_request(profile),
        &[first_projection, second_projection],
    );

    let first_exit = first.shutdown().map_err(debug_error)?;
    let second_exit = second.shutdown().map_err(debug_error)?;
    if !first_exit.success || !second_exit.success {
        return Err("ambiguity-control adapters failed to shut down".to_owned());
    }

    match outcome {
        AdapterSelectionOutcome::Ambiguous(candidates) if candidates.len() == 2 => Ok(()),
        other => Err(format!(
            "two compatible v0.2 adapters were not reported as ambiguous: {other:?}"
        )),
    }
}

pub fn prove_bootstrap_failure_is_runtime_error(adapter: &Path) -> Result<(), String> {
    let mut registry = AdapterRegistry::new();
    let registration_id = AdapterRegistrationId::new("external.runtime.bootstrap.failure");
    registry
        .register(AdapterRegistration::new(
            registration_id.clone(),
            AdapterCommand::new(adapter).with_argument("--profile=definitely-invalid"),
        ))
        .map_err(debug_error)?;

    match RunningAdapter::launch(
        registry
            .get(&registration_id)
            .ok_or_else(|| "bootstrap-failure registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.bootstrap.failure.instance"),
    ) {
        Ok(running) => {
            let _ = running.shutdown();
            Err("invalid adapter profile unexpectedly bootstrapped".to_owned())
        }
        Err(_) => Ok(()),
    }
}

pub fn prove_execute_response_loss_has_no_replay_authority() -> Result<(), String> {
    let policy = RunningAdapter::response_loss_policy(AdapterTransportMethod::ExecutePlan);
    if policy.replay() != ReplayDisposition::TransportMustNotReplayExecution
        || RunningAdapter::execution_response_loss_allows_transport_replay()
    {
        return Err("execute response loss unexpectedly authorizes transport replay".to_owned());
    }
    Ok(())
}

fn selection_request(profile: RuntimeContractProfile) -> AdapterSelectionRequest {
    REQUIRED_CAPABILITIES.iter().fold(
        AdapterSelectionRequest::for_profile(BackendTarget::new(TARGET), profile),
        |request, capability| request.require(BackendCapability::new(*capability)),
    )
}

fn debug_error(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

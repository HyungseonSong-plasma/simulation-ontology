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

/// Supported generic Adapter Runtime 0.2 consumer surface.
///
/// `request` is a consumer envelope, not a new SOL protocol. It contains:
/// `{ "target": string, "required_capabilities": [string...], "plan_request": <AP0.2 request> }`.
/// The target/capabilities are selection inputs only; `plan_request` is passed to the
/// existing Adapter Protocol 0.2 parsers unchanged. Live `describe_adapter` evidence
/// remains authoritative for compatibility and capability selection.
pub fn run_consumer_v02(adapter: &Path, request: &Value) -> Result<Value, String> {
    let target = request
        .get("target")
        .and_then(Value::as_str)
        .ok_or_else(|| "consumer request requires string target".to_owned())?;
    let capabilities = request
        .get("required_capabilities")
        .and_then(Value::as_array)
        .ok_or_else(|| "consumer request requires required_capabilities array".to_owned())?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "required_capabilities entries must be strings".to_owned())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let plan_request = request
        .get("plan_request")
        .ok_or_else(|| "consumer request requires plan_request".to_owned())?;
    let plan_json = serde_json::to_string(plan_request).map_err(|error| error.to_string())?;

    let mut registry = AdapterRegistry::new();
    let registration_id = AdapterRegistrationId::new("external.runtime.v02.consumer");
    registry
        .register(AdapterRegistration::new(
            registration_id.clone(),
            AdapterCommand::new(adapter),
        ))
        .map_err(debug_error)?;
    let mut running = RunningAdapter::launch(
        registry
            .get(&registration_id)
            .ok_or_else(|| "runtime registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.v02.consumer.instance"),
    )
    .map_err(debug_error)?;

    let result = (|| {
        let profile = RuntimeContractProfile::realization_v02();
        let projection =
            AdapterRuntimeProjection::from_running_for(&running, profile).map_err(debug_error)?;
        let selection = capabilities.iter().fold(
            AdapterSelectionRequest::for_profile(BackendTarget::new(target), profile),
            |request, capability| request.require(BackendCapability::new(capability)),
        );
        let selected = match select_adapter(&selection, std::slice::from_ref(&projection)) {
            AdapterSelectionOutcome::Selected(candidate) => candidate,
            AdapterSelectionOutcome::NoCompatibleCandidate => {
                return Err("NO_COMPATIBLE_CANDIDATE".to_owned())
            }
            AdapterSelectionOutcome::Ambiguous(candidates) => {
                return Err(format!("AMBIGUOUS_CANDIDATES:{}", candidates.len()))
            }
        };

        let validate_request =
            ValidatePlanRequestV02::from_json(&plan_json).map_err(debug_error)?;
        let execute_request = ExecutePlanRequestV02::from_json(&plan_json).map_err(debug_error)?;
        let validate_response = match running
            .validate_plan_v02(&validate_request)
            .map_err(debug_error)?
        {
            AdapterOperationResult::Success(value) => value,
            AdapterOperationResult::ProtocolFailure(value) => {
                return Err(format!("PROTOCOL_FAILURE_VALIDATE:{value:?}"))
            }
        };
        if validate_response.preflight != PreflightOutcome::Accepted {
            return Err(format!(
                "VALIDATION_REJECTED:{:?}",
                validate_response.preflight
            ));
        }

        let execute_response = match running
            .execute_plan_v02(&execute_request)
            .map_err(debug_error)?
        {
            AdapterOperationResult::Success(value) => value,
            AdapterOperationResult::ProtocolFailure(value) => {
                return Err(format!("PROTOCOL_FAILURE_EXECUTE:{value:?}"))
            }
        };
        if execute_response.execution != ExecutionOutcome::Completed {
            return Err(format!(
                "EXECUTION_NOT_COMPLETED:{:?}",
                execute_response.execution
            ));
        }

        let no_replay = RunningAdapter::response_loss_policy(AdapterTransportMethod::ExecutePlan)
            .replay()
            == ReplayDisposition::TransportMustNotReplayExecution
            && !RunningAdapter::execution_response_loss_allows_transport_replay();

        Ok(json!({
            "status": "completed",
            "adapter_id": selected.adapter_id(),
            "adapter_protocol_version": selected.profile().adapter_protocol_version().to_string(),
            "public_contract_version": selected.profile().public_contract_version().to_string(),
            "target": selected.target().as_str(),
            "required_capabilities": capabilities,
            "selection": "selected",
            "validate": "accepted",
            "execute": "completed",
            "execute_response_loss_replay": if no_replay {
                "forbidden"
            } else {
                "unexpectedly_allowed"
            }
        }))
    })();

    let exit = running.shutdown().map_err(debug_error)?;
    if !exit.success {
        return Err(format!("SHUTDOWN_FAILURE:{:?}", exit.code));
    }

    result.map(|mut value| {
        value["shutdown"] = json!("success");
        value
    })
}

pub fn run_external_v02_flow(adapter: &Path, fixture: &Path) -> Result<Value, String> {
    let fixture_json = fs::read_to_string(fixture).map_err(|error| error.to_string())?;
    let plan_request: Value =
        serde_json::from_str(&fixture_json).map_err(|error| error.to_string())?;
    run_consumer_v02(
        adapter,
        &json!({
            "target": TARGET,
            "required_capabilities": REQUIRED_CAPABILITIES,
            "plan_request": plan_request
        }),
    )
}

pub fn prove_v01_not_eligible_for_v02(adapter: &Path) -> Result<(), String> {
    let mut registry = AdapterRegistry::new();
    let id = AdapterRegistrationId::new("external.runtime.v01");
    registry
        .register(AdapterRegistration::new(
            id.clone(),
            AdapterCommand::new(adapter),
        ))
        .map_err(debug_error)?;
    let running = RunningAdapter::launch(
        registry
            .get(&id)
            .ok_or_else(|| "v0.1 registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.v01.instance"),
    )
    .map_err(debug_error)?;

    let profile = RuntimeContractProfile::realization_v02();
    let projection =
        AdapterRuntimeProjection::from_running_for(&running, profile).map_err(debug_error)?;
    let outcome = select_adapter(&selection_request(profile), &[projection]);
    let exit = running.shutdown().map_err(debug_error)?;
    if !exit.success {
        return Err("v0.1 adapter failed shutdown".into());
    }
    match outcome {
        AdapterSelectionOutcome::NoCompatibleCandidate => Ok(()),
        other => Err(format!("v0.1 unexpectedly eligible: {other:?}")),
    }
}

pub fn prove_multiple_v02_candidates_are_ambiguous(adapter: &Path) -> Result<(), String> {
    let launch = |name: &str| -> Result<RunningAdapter, String> {
        let mut registry = AdapterRegistry::new();
        let id = AdapterRegistrationId::new(name);
        registry
            .register(AdapterRegistration::new(
                id.clone(),
                AdapterCommand::new(adapter),
            ))
            .map_err(debug_error)?;
        RunningAdapter::launch(
            registry
                .get(&id)
                .ok_or_else(|| "registration disappeared".to_owned())?,
            AdapterInstanceId::new(format!("{name}.instance")),
        )
        .map_err(debug_error)
    };

    let first = launch("external.runtime.v02.first")?;
    let second = launch("external.runtime.v02.second")?;
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
        return Err("ambiguity adapters failed shutdown".into());
    }

    match outcome {
        AdapterSelectionOutcome::Ambiguous(candidates) if candidates.len() == 2 => Ok(()),
        other => Err(format!("two candidates not ambiguous: {other:?}")),
    }
}

pub fn prove_bootstrap_failure_is_runtime_error(adapter: &Path) -> Result<(), String> {
    let mut registry = AdapterRegistry::new();
    let id = AdapterRegistrationId::new("external.runtime.bootstrap.failure");
    registry
        .register(AdapterRegistration::new(
            id.clone(),
            AdapterCommand::new(adapter).with_argument("--profile=definitely-invalid"),
        ))
        .map_err(debug_error)?;

    match RunningAdapter::launch(
        registry
            .get(&id)
            .ok_or_else(|| "registration disappeared".to_owned())?,
        AdapterInstanceId::new("external.runtime.bootstrap.failure.instance"),
    ) {
        Ok(running) => {
            let _ = running.shutdown();
            Err("invalid profile unexpectedly bootstrapped".into())
        }
        Err(_) => Ok(()),
    }
}

pub fn prove_execute_response_loss_has_no_replay_authority() -> Result<(), String> {
    let policy = RunningAdapter::response_loss_policy(AdapterTransportMethod::ExecutePlan);
    if policy.replay() != ReplayDisposition::TransportMustNotReplayExecution
        || RunningAdapter::execution_response_loss_allows_transport_replay()
    {
        Err("execute response loss unexpectedly authorizes replay".into())
    } else {
        Ok(())
    }
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

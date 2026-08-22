use sol_adapter_protocol::{
    AdapterDescription, ExecutePlanRequest, ExecutePlanResponse, ValidatePlanRequest,
    ValidatePlanResponse,
};
use sol_adapter_transport::{
    response_loss_recovery, AdapterOperationResult, AdapterProcessCommand, AdapterProcessExit,
    AdapterProcessSession, AdapterSessionError, AdapterSessionState, AdapterTransportMethod,
    ReplayDisposition, ResponseLossRecovery,
};

use crate::{
    AdapterInstance, AdapterInstanceId, AdapterRegistration, AdapterRegistrationId,
    AdapterRegistryEntry,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterRuntimeError {
    RegistrationDisabled(AdapterRegistrationId),
    RegistrationMismatch {
        running: AdapterRegistrationId,
        requested: AdapterRegistrationId,
    },
    Transport(AdapterSessionError),
}

impl From<AdapterSessionError> for AdapterRuntimeError {
    fn from(error: AdapterSessionError) -> Self {
        Self::Transport(error)
    }
}

/// One live external adapter process associated with one local registration.
///
/// The embedded `AdapterInstance` is operational bookkeeping only. Canonical
/// backend semantics remain outside this type. Protocol failures remain normal
/// `AdapterOperationResult` values while transport/process failures remain
/// `AdapterRuntimeError::Transport`.
pub struct RunningAdapter {
    registration: AdapterRegistration,
    instance: AdapterInstance,
    session: AdapterProcessSession,
}

impl RunningAdapter {
    /// Launch and bootstrap one enabled explicit registration.
    ///
    /// `AdapterProcessSession::spawn` performs the mandatory `describe_adapter`
    /// bootstrap before returning a ready typed session.
    pub fn launch(
        entry: &AdapterRegistryEntry,
        instance_id: AdapterInstanceId,
    ) -> Result<Self, AdapterRuntimeError> {
        if !entry.is_enabled() {
            return Err(AdapterRuntimeError::RegistrationDisabled(
                entry.registration().id().clone(),
            ));
        }

        let registration = entry.registration().clone();
        let command = transport_command(registration.command());
        let session = AdapterProcessSession::spawn(command)?;
        let instance = AdapterInstance::new(
            instance_id,
            registration.id().clone(),
            session.process_id(),
        );

        Ok(Self {
            registration,
            instance,
            session,
        })
    }

    pub fn instance(&self) -> &AdapterInstance {
        &self.instance
    }

    pub fn state(&self) -> AdapterSessionState {
        self.session.state()
    }

    /// Live bootstrap evidence returned by the published `describe_adapter`
    /// operation. Phase 2 exposes it but does not yet interpret compatibility.
    pub fn description(&self) -> Option<&AdapterDescription> {
        self.session.description()
    }

    pub fn validate_plan(
        &mut self,
        request: &ValidatePlanRequest,
    ) -> Result<AdapterOperationResult<ValidatePlanResponse>, AdapterRuntimeError> {
        self.session.validate_plan(request).map_err(Into::into)
    }

    pub fn execute_plan(
        &mut self,
        request: &ExecutePlanRequest,
    ) -> Result<AdapterOperationResult<ExecutePlanResponse>, AdapterRuntimeError> {
        self.session.execute_plan(request).map_err(Into::into)
    }

    /// Close stdin, wait for process exit, and return transport-owned exit/stderr
    /// evidence without converting it into canonical SOL lifecycle semantics.
    pub fn shutdown(self) -> Result<AdapterProcessExit, AdapterRuntimeError> {
        self.session.shutdown().map_err(Into::into)
    }

    /// Explicitly replace the current session with a freshly bootstrapped process.
    ///
    /// Reconnect never assumes continuity with the previous process. A new local
    /// `AdapterInstanceId` is required, the previous process is shut down first,
    /// and the registration must still match and be enabled.
    pub fn reconnect(
        self,
        entry: &AdapterRegistryEntry,
        new_instance_id: AdapterInstanceId,
    ) -> Result<(AdapterProcessExit, Self), AdapterRuntimeError> {
        let running_id = self.registration.id().clone();
        let requested_id = entry.registration().id().clone();
        if running_id != requested_id {
            return Err(AdapterRuntimeError::RegistrationMismatch {
                running: running_id,
                requested: requested_id,
            });
        }

        let exit = self.shutdown()?;
        let replacement = Self::launch(entry, new_instance_id)?;
        Ok((exit, replacement))
    }

    /// M0.5 response-loss policy remains authoritative for runtime callers.
    pub const fn response_loss_policy(operation: AdapterTransportMethod) -> ResponseLossRecovery {
        response_loss_recovery(operation)
    }

    /// Convenience invariant for callers deciding whether execution may be
    /// automatically retried after response loss. It is always false for the
    /// published execute operation under Protocol/Transport 0.1 semantics.
    pub const fn execution_response_loss_allows_transport_replay() -> bool {
        matches!(
            response_loss_recovery(AdapterTransportMethod::ExecutePlan).replay(),
            ReplayDisposition::CallerMayReissueDescription
                | ReplayDisposition::CallerMayReissueEquivalentValidation
        )
    }
}

fn transport_command(command: &crate::AdapterCommand) -> AdapterProcessCommand {
    let mut transport = AdapterProcessCommand::new(command.program().as_os_str());
    for argument in command.arguments() {
        transport = transport.arg(argument.to_os_string());
    }
    if let Some(directory) = command.working_directory() {
        transport = transport.current_dir(directory.to_path_buf());
    }
    transport
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use sol_adapter_transport::{
        AdapterSessionState, AdapterTransportMethod, ReplayDisposition, SideEffectEvidence,
    };

    use crate::{
        AdapterCommand, AdapterInstanceId, AdapterRegistration, AdapterRegistrationId,
        AdapterRegistry,
    };

    use super::{transport_command, AdapterRuntimeError, RunningAdapter};

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
        assert!(
            path.is_file(),
            "workspace CI/build must produce mock adapter fixture at {}",
            path.display()
        );
        path
    }

    fn register_mock(registry: &mut AdapterRegistry, id: &str) -> AdapterRegistrationId {
        let id = AdapterRegistrationId::new(id);
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("runtime crate must live under workspace/crates");
        registry
            .register(AdapterRegistration::new(
                id.clone(),
                AdapterCommand::new(mock_adapter_binary()).with_working_directory(workspace),
            ))
            .unwrap();
        id
    }

    #[test]
    fn runtime_command_preserves_arguments_and_working_directory() {
        let command = AdapterCommand::new("adapter")
            .with_argument("--stdio")
            .with_argument("--profile=test")
            .with_working_directory("/tmp/adapter-runtime");
        let transport = transport_command(&command);

        assert_eq!(transport.program(), std::ffi::OsStr::new("adapter"));
        assert_eq!(
            transport.args(),
            [
                std::ffi::OsString::from("--stdio"),
                std::ffi::OsString::from("--profile=test")
            ]
        );
        assert_eq!(
            transport.working_directory(),
            Some(Path::new("/tmp/adapter-runtime"))
        );
    }

    #[test]
    fn disabled_registration_is_not_launched() {
        let mut registry = AdapterRegistry::new();
        let id = AdapterRegistrationId::new("disabled");
        registry
            .register(AdapterRegistration::new(
                id.clone(),
                AdapterCommand::new("not-invoked"),
            ))
            .unwrap();
        registry.set_enabled(&id, false).unwrap();

        assert!(matches!(
            RunningAdapter::launch(
                registry.get(&id).unwrap(),
                AdapterInstanceId::new("instance.disabled")
            ),
            Err(AdapterRuntimeError::RegistrationDisabled(actual)) if actual == id
        ));
    }

    #[test]
    fn registered_external_adapter_launches_bootstraps_and_reconnects_as_fresh_session() {
        let mut registry = AdapterRegistry::new();
        let id = register_mock(&mut registry, "mock.runtime");
        let entry = registry.get(&id).unwrap();

        let running = RunningAdapter::launch(entry, AdapterInstanceId::new("instance.1")).unwrap();
        assert_eq!(running.state(), AdapterSessionState::Ready);
        assert!(running.description().is_some());
        let first_pid = running.instance().process_id();
        assert!(first_pid > 0);

        let (first_exit, replacement) = running
            .reconnect(entry, AdapterInstanceId::new("instance.2"))
            .unwrap();
        assert!(first_exit.success);
        assert_eq!(replacement.state(), AdapterSessionState::Ready);
        assert!(replacement.description().is_some());
        assert_eq!(replacement.instance().id().as_str(), "instance.2");
        assert_ne!(replacement.instance().process_id(), first_pid);

        let second_exit = replacement.shutdown().unwrap();
        assert!(second_exit.success);
    }

    #[test]
    fn execute_response_loss_policy_never_authorizes_transport_replay() {
        let recovery = RunningAdapter::response_loss_policy(AdapterTransportMethod::ExecutePlan);
        assert_eq!(
            recovery.replay(),
            ReplayDisposition::TransportMustNotReplayExecution
        );
        assert_eq!(
            recovery.conservative_side_effects(),
            SideEffectEvidence::MayHaveOccurred
        );
        assert!(!RunningAdapter::execution_response_loss_allows_transport_replay());
    }
}

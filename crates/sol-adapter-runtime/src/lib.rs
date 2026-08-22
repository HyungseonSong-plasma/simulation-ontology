#![forbid(unsafe_code)]

//! Core-local runtime model for external adapter registration and live instance state.
//!
//! These types are deliberately not Public Contract DTOs and do not represent
//! canonical SOL semantic identity. Canonical backend-target meaning remains in
//! `sol-target-resolver`; Protocol compatibility and capability evidence remains
//! a later live-bootstrap concern derived from `describe_adapter`.

mod registry;

pub use registry::*;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Local identifier for one adapter registration.
///
/// This identifier is only meaningful inside the SOL runtime/registry. It MUST
/// NOT be used as canonical simulation, backend-target, or solver-native identity.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdapterRegistrationId(String);

impl AdapterRegistrationId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Local command configuration used to locate and invoke an external adapter.
///
/// This structure intentionally carries no Adapter Protocol compatibility,
/// Public Contract compatibility, backend target, or capability declarations.
/// Those claims are not trusted until obtained from the live adapter bootstrap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterCommand {
    program: PathBuf,
    arguments: Vec<OsString>,
    working_directory: Option<PathBuf>,
}

impl AdapterCommand {
    pub fn new(program: impl Into<PathBuf>) -> Self {
        Self {
            program: program.into(),
            arguments: Vec::new(),
            working_directory: None,
        }
    }

    pub fn with_argument(mut self, argument: impl Into<OsString>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn with_working_directory(mut self, directory: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(directory.into());
        self
    }

    pub fn program(&self) -> &Path {
        &self.program
    }

    pub fn arguments(&self) -> impl Iterator<Item = &OsStr> {
        self.arguments.iter().map(OsString::as_os_str)
    }

    pub fn working_directory(&self) -> Option<&Path> {
        self.working_directory.as_deref()
    }
}

/// Core-local registration of an external adapter command.
///
/// A registration is a locator/configuration record only. It does not establish
/// that the command supports any target, capability, Adapter Protocol version,
/// or Public Contract version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRegistration {
    id: AdapterRegistrationId,
    command: AdapterCommand,
}

impl AdapterRegistration {
    pub fn new(id: AdapterRegistrationId, command: AdapterCommand) -> Self {
        Self { id, command }
    }

    pub fn id(&self) -> &AdapterRegistrationId {
        &self.id
    }

    pub fn command(&self) -> &AdapterCommand {
        &self.command
    }
}

/// Local identifier for one live operational adapter instance/session.
///
/// Equality of this value is runtime-local bookkeeping only. It has no
/// canonical SOL semantic meaning.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdapterInstanceId(String);

impl AdapterInstanceId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Operational facts identifying one live adapter process/session.
///
/// The instance points back to the local registration that launched it. Its
/// process/session identifiers are operational evidence only and MUST NOT be
/// used for canonical semantic identity or equality.
#[derive(Debug, Clone)]
pub struct AdapterInstance {
    id: AdapterInstanceId,
    registration_id: AdapterRegistrationId,
    process_id: u32,
}

impl AdapterInstance {
    pub fn new(
        id: AdapterInstanceId,
        registration_id: AdapterRegistrationId,
        process_id: u32,
    ) -> Self {
        Self {
            id,
            registration_id,
            process_id,
        }
    }

    pub fn id(&self) -> &AdapterInstanceId {
        &self.id
    }

    pub fn registration_id(&self) -> &AdapterRegistrationId {
        &self.registration_id
    }

    pub fn process_id(&self) -> u32 {
        self.process_id
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use std::path::Path;

    use sol_target_resolver::BackendTarget;

    use super::{
        AdapterCommand, AdapterInstance, AdapterInstanceId, AdapterRegistration,
        AdapterRegistrationId,
    };

    #[test]
    fn canonical_target_registration_and_instance_are_distinct_rust_types() {
        assert_ne!(
            TypeId::of::<BackendTarget>(),
            TypeId::of::<AdapterRegistration>()
        );
        assert_ne!(
            TypeId::of::<AdapterRegistration>(),
            TypeId::of::<AdapterInstance>()
        );
        assert_ne!(
            TypeId::of::<BackendTarget>(),
            TypeId::of::<AdapterInstance>()
        );
    }

    #[test]
    fn registration_contains_only_local_invocation_configuration() {
        let registration = AdapterRegistration::new(
            AdapterRegistrationId::new("local.thermal.adapter"),
            AdapterCommand::new("/opt/sol/adapters/thermal")
                .with_argument("--stdio")
                .with_working_directory("/tmp/sol-adapter"),
        );

        assert_eq!(registration.id().as_str(), "local.thermal.adapter");
        assert_eq!(
            registration.command().program(),
            Path::new("/opt/sol/adapters/thermal")
        );
        assert_eq!(
            registration.command().arguments().collect::<Vec<_>>(),
            vec![std::ffi::OsStr::new("--stdio")]
        );
        assert_eq!(
            registration.command().working_directory(),
            Some(Path::new("/tmp/sol-adapter"))
        );
    }

    #[test]
    fn instance_process_identity_remains_local_operational_state() {
        let instance = AdapterInstance::new(
            AdapterInstanceId::new("instance.42"),
            AdapterRegistrationId::new("local.thermal.adapter"),
            4242,
        );

        assert_eq!(instance.id().as_str(), "instance.42");
        assert_eq!(instance.registration_id().as_str(), "local.thermal.adapter");
        assert_eq!(instance.process_id(), 4242);
    }
}

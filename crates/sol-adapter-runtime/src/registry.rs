use std::collections::BTreeMap;

use crate::{AdapterRegistration, AdapterRegistrationId};

/// Runtime-local registry error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterRegistryError {
    DuplicateRegistration(AdapterRegistrationId),
    UnknownRegistration(AdapterRegistrationId),
}

/// One explicitly registered adapter command plus local enable/disable state.
///
/// Enablement controls local runtime eligibility only. It does not establish
/// Adapter Protocol/Public Contract compatibility or backend capability truth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterRegistryEntry {
    registration: AdapterRegistration,
    enabled: bool,
}

impl AdapterRegistryEntry {
    pub fn registration(&self) -> &AdapterRegistration {
        &self.registration
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Deterministic in-memory registry of explicitly configured adapter commands.
///
/// M0.7 Phase 1 intentionally provides no automatic discovery, filesystem scan,
/// package installation, remote registry, signing, marketplace, or auto-update
/// behavior. Persistence format is also deliberately not part of this API.
#[derive(Debug, Default)]
pub struct AdapterRegistry {
    entries: BTreeMap<AdapterRegistrationId, AdapterRegistryEntry>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one explicitly supplied adapter command.
    ///
    /// Registrations are enabled by default. Duplicate local registration IDs
    /// are rejected rather than silently replacing the existing configuration.
    pub fn register(
        &mut self,
        registration: AdapterRegistration,
    ) -> Result<(), AdapterRegistryError> {
        let id = registration.id().clone();
        if self.entries.contains_key(&id) {
            return Err(AdapterRegistryError::DuplicateRegistration(id));
        }

        self.entries.insert(
            id,
            AdapterRegistryEntry {
                registration,
                enabled: true,
            },
        );
        Ok(())
    }

    pub fn get(&self, id: &AdapterRegistrationId) -> Option<&AdapterRegistryEntry> {
        self.entries.get(id)
    }

    /// Iterate registrations in deterministic local registration-ID order.
    pub fn entries(&self) -> impl Iterator<Item = &AdapterRegistryEntry> {
        self.entries.values()
    }

    pub fn set_enabled(
        &mut self,
        id: &AdapterRegistrationId,
        enabled: bool,
    ) -> Result<(), AdapterRegistryError> {
        let entry = self
            .entries
            .get_mut(id)
            .ok_or_else(|| AdapterRegistryError::UnknownRegistration(id.clone()))?;
        entry.enabled = enabled;
        Ok(())
    }

    pub fn remove(&mut self, id: &AdapterRegistrationId) -> Option<AdapterRegistration> {
        self.entries.remove(id).map(|entry| entry.registration)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AdapterCommand, AdapterRegistration, AdapterRegistrationId};

    use super::{AdapterRegistry, AdapterRegistryError};

    fn registration(id: &str, program: &str) -> AdapterRegistration {
        AdapterRegistration::new(
            AdapterRegistrationId::new(id),
            AdapterCommand::new(program).with_argument("--stdio"),
        )
    }

    #[test]
    fn explicit_registration_can_be_added_read_disabled_enabled_and_removed() {
        let mut registry = AdapterRegistry::new();
        let id = AdapterRegistrationId::new("local.adapter");

        registry
            .register(registration("local.adapter", "/opt/adapter"))
            .unwrap();
        assert_eq!(registry.len(), 1);
        assert_eq!(
            registry
                .get(&id)
                .unwrap()
                .registration()
                .command()
                .program(),
            std::path::Path::new("/opt/adapter")
        );
        assert!(registry.get(&id).unwrap().is_enabled());

        registry.set_enabled(&id, false).unwrap();
        assert!(!registry.get(&id).unwrap().is_enabled());
        registry.set_enabled(&id, true).unwrap();
        assert!(registry.get(&id).unwrap().is_enabled());

        let removed = registry.remove(&id).unwrap();
        assert_eq!(removed.id(), &id);
        assert!(registry.is_empty());
    }

    #[test]
    fn duplicate_registration_id_is_rejected_without_replacement() {
        let mut registry = AdapterRegistry::new();
        let id = AdapterRegistrationId::new("local.adapter");
        registry
            .register(registration("local.adapter", "/opt/original"))
            .unwrap();

        assert_eq!(
            registry.register(registration("local.adapter", "/opt/replacement")),
            Err(AdapterRegistryError::DuplicateRegistration(id.clone()))
        );
        assert_eq!(
            registry
                .get(&id)
                .unwrap()
                .registration()
                .command()
                .program(),
            std::path::Path::new("/opt/original")
        );
    }

    #[test]
    fn registration_listing_is_deterministic_by_local_id() {
        let mut registry = AdapterRegistry::new();
        for id in ["zeta", "alpha", "middle"] {
            registry
                .register(registration(id, &format!("/opt/{id}")))
                .unwrap();
        }

        let ids: Vec<_> = registry
            .entries()
            .map(|entry| entry.registration().id().as_str())
            .collect();
        assert_eq!(ids, vec!["alpha", "middle", "zeta"]);
    }

    #[test]
    fn enablement_of_unknown_registration_is_explicit_error() {
        let mut registry = AdapterRegistry::new();
        let id = AdapterRegistrationId::new("missing");

        assert_eq!(
            registry.set_enabled(&id, false),
            Err(AdapterRegistryError::UnknownRegistration(id))
        );
    }
}

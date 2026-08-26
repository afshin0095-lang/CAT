#![allow(clippy::module_name_repetitions)]

use std::collections::BTreeMap;

use crate::{EventBusError, EventBusResult};

/// Compatibility policy for a version transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compatibility {
    Backward,
    Forward,
    Full,
    Breaking,
}

/// Immutable registration describing one event contract version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventContract {
    pub event_type: String,
    pub version: u16,
    pub schema_id: String,
    pub compatibility: Compatibility,
}

impl EventContract {
    pub fn new(
        event_type: impl Into<String>,
        version: u16,
        schema_id: impl Into<String>,
        compatibility: Compatibility,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            version,
            schema_id: schema_id.into(),
            compatibility,
        }
    }
}

/// In-process registry used as the authoritative contract lookup for the bus layer.
#[derive(Debug, Default, Clone)]
pub struct EventRegistry {
    contracts: BTreeMap<(String, u16), EventContract>,
}

impl EventRegistry {
    pub fn register(&mut self, contract: EventContract) -> EventBusResult<()> {
        let key = (contract.event_type.clone(), contract.version);
        if self.contracts.contains_key(&key) {
            return Err(EventBusError::ContractConflict {
                event_type: contract.event_type,
                version: contract.version,
            });
        }
        self.contracts.insert(key, contract);
        Ok(())
    }

    pub fn get(&self, event_type: &str, version: u16) -> Option<&EventContract> {
        self.contracts.get(&(event_type.to_owned(), version))
    }

    pub fn require(&self, event_type: &str, version: u16) -> EventBusResult<&EventContract> {
        self.get(event_type, version)
            .ok_or_else(|| EventBusError::UnknownContract {
                event_type: event_type.to_owned(),
                version,
            })
    }

    /// Returns the highest registered version for an event type.
    pub fn latest_version(&self, event_type: &str) -> Option<u16> {
        self.contracts
            .keys()
            .filter_map(|(registered_type, version)| {
                (registered_type == event_type).then_some(*version)
            })
            .max()
    }

    /// Lists all registered versions for an event type in ascending order.
    pub fn versions(&self, event_type: &str) -> Vec<u16> {
        self.contracts
            .keys()
            .filter_map(|(registered_type, version)| {
                (registered_type == event_type).then_some(*version)
            })
            .collect()
    }

    /// Checks whether a consumer targeting `consumer_version` can consume a
    /// producer contract. Breaking contracts are never auto-compatible.
    pub fn is_compatible(
        &self,
        event_type: &str,
        producer_version: u16,
        consumer_version: u16,
    ) -> EventBusResult<bool> {
        let producer = self.require(event_type, producer_version)?;

        if producer_version == consumer_version {
            return Ok(true);
        }

        Ok(match producer.compatibility {
            Compatibility::Full => true,
            Compatibility::Backward => consumer_version >= producer_version,
            Compatibility::Forward => consumer_version <= producer_version,
            Compatibility::Breaking => false,
        })
    }

    /// Resolves a consumer request to the newest compatible registered version.
    pub fn resolve_compatible(
        &self,
        event_type: &str,
        consumer_version: u16,
    ) -> EventBusResult<&EventContract> {
        self.versions(event_type)
            .into_iter()
            .rev()
            .find_map(|version| {
                self.is_compatible(event_type, version, consumer_version)
                    .ok()
                    .filter(|compatible| *compatible)
                    .and_then(|_| self.get(event_type, version))
            })
            .ok_or_else(|| EventBusError::UnknownContract {
                event_type: event_type.to_owned(),
                version: consumer_version,
            })
    }

    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }
}

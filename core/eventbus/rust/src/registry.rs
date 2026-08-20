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

    pub fn len(&self) -> usize {
        self.contracts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contracts.is_empty()
    }
}

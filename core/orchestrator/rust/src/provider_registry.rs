use std::collections::BTreeMap;
use std::sync::Arc;

use crate::ProviderExecutionAdapter;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ProviderCapability {
    Idempotency,
    ExecutionLookup,
    AsyncCompletion,
    Cancellation,
}

pub struct ProviderRegistration {
    pub name: String,
    pub capabilities: Vec<ProviderCapability>,
    pub adapter: Arc<dyn ProviderExecutionAdapter>,
}

#[derive(Default)]
pub struct ProviderAdapterRegistry {
    providers: BTreeMap<String, ProviderRegistration>,
}

impl ProviderAdapterRegistry {
    pub fn register(&mut self, registration: ProviderRegistration) -> Result<(), String> {
        if registration.name.trim().is_empty() {
            return Err("provider name must not be empty".into());
        }
        if self.providers.contains_key(&registration.name) {
            return Err(format!("provider already registered: {}", registration.name));
        }
        self.providers.insert(registration.name.clone(), registration);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ProviderRegistration> {
        self.providers.get(name)
    }

    pub fn supports(&self, name: &str, required: &[ProviderCapability]) -> bool {
        let Some(provider) = self.get(name) else { return false; };
        required.iter().all(|capability| provider.capabilities.contains(capability))
    }

    pub fn eligible(&self, required: &[ProviderCapability]) -> Vec<&ProviderRegistration> {
        self.providers
            .values()
            .filter(|provider| required.iter().all(|capability| provider.capabilities.contains(capability)))
            .collect()
    }

    pub fn names(&self) -> Vec<&str> {
        self.providers.keys().map(String::as_str).collect()
    }
}

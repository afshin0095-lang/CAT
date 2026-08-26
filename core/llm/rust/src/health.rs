use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::ProviderId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderHealthState {
    Healthy,
    Open,
    HalfOpen,
}

#[derive(Clone, Debug)]
pub struct ProviderHealthConfig {
    pub failure_threshold: u32,
    pub cooldown: Duration,
}

impl Default for ProviderHealthConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            cooldown: Duration::from_secs(30),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ProviderHealthSnapshot {
    pub provider: ProviderId,
    pub state: ProviderHealthState,
    pub consecutive_failures: u32,
}

#[derive(Clone, Debug)]
struct ProviderHealthEntry {
    consecutive_failures: u32,
    opened_at: Option<Instant>,
    probe_in_flight: bool,
}

impl ProviderHealthEntry {
    fn new() -> Self {
        Self {
            consecutive_failures: 0,
            opened_at: None,
            probe_in_flight: false,
        }
    }

    fn state(&self, cooldown: Duration, now: Instant) -> ProviderHealthState {
        match self.opened_at {
            None => ProviderHealthState::Healthy,
            Some(opened_at) if now.duration_since(opened_at) >= cooldown => {
                if self.probe_in_flight {
                    ProviderHealthState::HalfOpen
                } else {
                    ProviderHealthState::HalfOpen
                }
            }
            Some(_) => ProviderHealthState::Open,
        }
    }
}

/// Thread-safe provider circuit state used by the LLM router.
///
/// The registry does not choose models or rewrite policy. It only suppresses
/// repeated calls to a failing provider and permits one controlled recovery
/// probe after the cooldown interval.
#[derive(Clone, Debug)]
pub struct ProviderHealthRegistry {
    config: ProviderHealthConfig,
    entries: Arc<Mutex<HashMap<ProviderId, ProviderHealthEntry>>>,
}

impl ProviderHealthRegistry {
    pub fn new(config: ProviderHealthConfig) -> Self {
        Self {
            config,
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn config(&self) -> &ProviderHealthConfig {
        &self.config
    }

    pub fn is_available(&self, provider: &ProviderId) -> bool {
        let mut entries = self.entries.lock().expect("provider health mutex poisoned");
        let entry = entries.entry(provider.clone()).or_insert_with(ProviderHealthEntry::new);
        match entry.state(self.config.cooldown, Instant::now()) {
            ProviderHealthState::Healthy => true,
            ProviderHealthState::Open => false,
            ProviderHealthState::HalfOpen => {
                if entry.probe_in_flight {
                    false
                } else {
                    entry.probe_in_flight = true;
                    true
                }
            }
        }
    }

    pub fn record_success(&self, provider: &ProviderId) {
        let mut entries = self.entries.lock().expect("provider health mutex poisoned");
        let entry = entries.entry(provider.clone()).or_insert_with(ProviderHealthEntry::new);
        entry.consecutive_failures = 0;
        entry.opened_at = None;
        entry.probe_in_flight = false;
    }

    pub fn record_failure(&self, provider: &ProviderId) {
        let mut entries = self.entries.lock().expect("provider health mutex poisoned");
        let entry = entries.entry(provider.clone()).or_insert_with(ProviderHealthEntry::new);
        entry.consecutive_failures = entry.consecutive_failures.saturating_add(1);
        entry.probe_in_flight = false;
        if entry.consecutive_failures >= self.config.failure_threshold {
            entry.opened_at = Some(Instant::now());
        }
    }

    pub fn snapshot(&self, provider: &ProviderId) -> ProviderHealthSnapshot {
        let entries = self.entries.lock().expect("provider health mutex poisoned");
        let entry = entries.get(provider).cloned().unwrap_or_else(ProviderHealthEntry::new);
        ProviderHealthSnapshot {
            provider: provider.clone(),
            state: entry.state(self.config.cooldown, Instant::now()),
            consecutive_failures: entry.consecutive_failures,
        }
    }
}

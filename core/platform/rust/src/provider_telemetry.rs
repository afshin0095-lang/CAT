use crate::ProviderFailureClass;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProviderTelemetry {
    counts: BTreeMap<ProviderFailureClass, u64>,
}

impl ProviderTelemetry {
    pub fn record(&mut self, class: ProviderFailureClass) {
        *self.counts.entry(class).or_default() += 1;
    }

    pub fn count(&self, class: ProviderFailureClass) -> u64 { self.counts.get(&class).copied().unwrap_or(0) }

    pub fn snapshot(&self) -> BTreeMap<ProviderFailureClass, u64> { self.counts.clone() }

    pub fn retryable_failures(&self) -> u64 {
        self.counts.iter().filter(|(class, _)| class.retryable()).map(|(_, count)| *count).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telemetry_snapshot_is_deterministic() {
        let mut telemetry = ProviderTelemetry::default();
        telemetry.record(ProviderFailureClass::Server);
        telemetry.record(ProviderFailureClass::RateLimited);
        telemetry.record(ProviderFailureClass::Server);
        assert_eq!(telemetry.count(ProviderFailureClass::Server), 2);
        assert_eq!(telemetry.retryable_failures(), 3);
        assert_eq!(telemetry.snapshot().len(), 2);
    }
}

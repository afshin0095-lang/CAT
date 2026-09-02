use crate::{ProviderAdapterRegistry, ProviderCapability};

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderScore {
    pub provider: String,
    pub commission_score: f64,
    pub reliability_score: f64,
    pub conversion_score: f64,
    pub latency_score: f64,
    pub geographic_score: f64,
}

impl ProviderScore {
    pub fn total(&self) -> f64 {
        self.commission_score * 0.30
            + self.reliability_score * 0.25
            + self.conversion_score * 0.25
            + self.latency_score * 0.10
            + self.geographic_score * 0.10
    }
}

#[derive(Clone, Debug)]
pub struct ProviderSelectionRequest {
    pub required_capabilities: Vec<ProviderCapability>,
    pub scores: Vec<ProviderScore>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProviderSelection {
    pub provider: String,
    pub score: f64,
}

#[derive(Default)]
pub struct ProviderSelectionEngine;

impl ProviderSelectionEngine {
    pub fn select(
        &self,
        registry: &ProviderAdapterRegistry,
        request: ProviderSelectionRequest,
    ) -> Option<ProviderSelection> {
        request
            .scores
            .into_iter()
            .filter(|candidate| registry.supports(&candidate.provider, &request.required_capabilities))
            .max_by(|left, right| {
                left.total()
                    .partial_cmp(&right.total())
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| right.provider.cmp(&left.provider))
            })
            .map(|winner| ProviderSelection { provider: winner.provider, score: winner.total() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weighted_score_rewards_balanced_provider() {
        let value = ProviderScore {
            provider: "x".into(),
            commission_score: 1.0,
            reliability_score: 1.0,
            conversion_score: 1.0,
            latency_score: 1.0,
            geographic_score: 1.0,
        };
        assert_eq!(value.total(), 1.0);
    }
}

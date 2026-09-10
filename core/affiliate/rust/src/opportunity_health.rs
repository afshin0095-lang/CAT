//! Opportunity health: intentionally independent from lifecycle.
//!
//! Lifecycle answers "how old is the observation?" — health answers "can the
//! current observation still be trusted and acted on?". The two dimensions
//! are orthogonal by design:
//!
//! - `Active + Degraded` — fresh observation, but its source is struggling;
//! - `Stale + Healthy` — old observation from a source that is fine.
//!
//! Never merge these enums. Consumers that need both read both.

use serde::{Deserialize, Serialize};

use crate::discovery_source_health::{SourceHealthSnapshot, SourceHealthState};
use crate::opportunity_freshness::FreshnessEvaluation;
use crate::OpportunityRecord;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpportunityHealth {
    Healthy,
    /// The observation is usable but its source shows warning signs.
    Degraded,
    /// The observation cannot currently be acted on.
    Unavailable,
    /// Nothing is wrong with the source, but a refresh is pending or due.
    /// This is an aggregate label; `needs_revalidation` carries the flag
    /// itself so consumers never have to infer it from the enum.
    NeedsRevalidation,
}

impl OpportunityHealth {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
            Self::NeedsRevalidation => "needs_revalidation",
        }
    }
}

/// Why the assessment reached its conclusion. Deterministically ordered.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthConcern {
    NoObservations,
    BestSourceUnavailable { source: String },
    BestSourceDegraded { source: String },
    BestSourceUnknown { source: String },
    RevalidationPending,
}

/// Aggregate health assessment for one opportunity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityHealthAssessment {
    pub health: OpportunityHealth,
    pub needs_revalidation: bool,
    pub concerns: Vec<HealthConcern>,
}

/// Derives opportunity health from record, freshness, and optional source
/// health. Pure function: no I/O, no mutation, deterministic output.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpportunityHealthAssessor;

impl OpportunityHealthAssessor {
    /// Precedence (documented, deterministic):
    ///
    /// 1. no observations → `Unavailable`;
    /// 2. best-source snapshot `Unavailable` → `Unavailable`;
    /// 3. best-source snapshot `Degraded` → `Degraded`;
    /// 4. `revalidation_pending && otherwise Healthy` → `NeedsRevalidation`;
    /// 5. otherwise → `Healthy`.
    ///
    /// A missing (`None`) or `Unknown` snapshot never penalizes the
    /// opportunity (benefit of the doubt), but `Unknown` is reported as a
    /// concern for observability.
    pub fn assess(
        &self,
        record: &OpportunityRecord,
        _freshness: &FreshnessEvaluation,
        best_source_health: Option<&SourceHealthSnapshot>,
        revalidation_pending: bool,
    ) -> OpportunityHealthAssessment {
        let mut concerns = Vec::new();
        let mut health = OpportunityHealth::Healthy;

        if record.observations.is_empty() {
            concerns.push(HealthConcern::NoObservations);
            return OpportunityHealthAssessment {
                health: OpportunityHealth::Unavailable,
                needs_revalidation: revalidation_pending,
                concerns,
            };
        }

        match best_source_health.map(|snapshot| snapshot.state) {
            Some(SourceHealthState::Unavailable) => {
                health = OpportunityHealth::Unavailable;
                concerns.push(HealthConcern::BestSourceUnavailable {
                    source: record.best_source.clone(),
                });
            }
            Some(SourceHealthState::Degraded) => {
                health = OpportunityHealth::Degraded;
                concerns.push(HealthConcern::BestSourceDegraded {
                    source: record.best_source.clone(),
                });
            }
            Some(SourceHealthState::Unknown) => {
                concerns.push(HealthConcern::BestSourceUnknown {
                    source: record.best_source.clone(),
                });
            }
            Some(SourceHealthState::Healthy) | None => {}
        }

        if revalidation_pending && health == OpportunityHealth::Healthy {
            health = OpportunityHealth::NeedsRevalidation;
        }
        if revalidation_pending {
            concerns.push(HealthConcern::RevalidationPending);
        }

        OpportunityHealthAssessment {
            health,
            needs_revalidation: revalidation_pending,
            concerns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityStore};
    use uuid::Uuid;

    fn record(sources: &[&str]) -> OpportunityRecord {
        let mut store = InMemoryOpportunityStore::new();
        for (index, source) in sources.iter().enumerate() {
            let candidate = DiscoveryCandidate {
                source: (*source).into(),
                external_id: format!("sku-{index}"),
                merchant_name: "Acme".into(),
                product_name: "Widget".into(),
                canonical_key: canonical_key("Acme", "Widget"),
                category: None,
                destination_url: "https://example.test/widget".into(),
                currency: "EUR".into(),
                price_minor: Some(10_000),
                commission_bps: Some(500),
                demand_score: 8_000,
                competition_score: 2_000,
                freshness_score: 10_000,
                compliance_score: 10_000,
                observed_at_ms: 1_000,
            };
            let opportunity = DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score: 8_000, rank: 1 };
            store.upsert(opportunity).expect("valid opportunity");
        }
        store.list().into_iter().next().expect("record exists")
    }

    fn freshness_active() -> FreshnessEvaluation {
        FreshnessEvaluation { state: crate::FreshnessState::Active, age_ms: 10, is_fresh: true }
    }

    fn snapshot(source: &str, state: SourceHealthState) -> SourceHealthSnapshot {
        SourceHealthSnapshot {
            source: source.to_owned(),
            state,
            ..SourceHealthSnapshot::default()
        }
    }

    #[test]
    fn healthy_record_with_no_health_data_is_healthy() {
        let stored = record(&["network-a"]);
        let assessment = OpportunityHealthAssessor
            .assess(&stored, &freshness_active(), None, false);
        assert_eq!(assessment.health, OpportunityHealth::Healthy);
        assert!(!assessment.needs_revalidation);
        assert!(assessment.concerns.is_empty());
    }

    #[test]
    fn active_record_can_be_degraded_by_source_health() {
        let stored = record(&["network-a"]);
        let health = snapshot("network-a", SourceHealthState::Degraded);
        let assessment = OpportunityHealthAssessor.assess(&stored, &freshness_active(), Some(&health), false);
        assert_eq!(assessment.health, OpportunityHealth::Degraded);
        assert_eq!(
            assessment.concerns,
            vec![HealthConcern::BestSourceDegraded { source: "network-a".into() }]
        );
    }

    #[test]
    fn stale_record_can_stay_healthy() {
        // Staleness is a lifecycle dimension; health stays independent.
        let stored = record(&["network-a"]);
        let stale_freshness = FreshnessEvaluation {
            state: crate::FreshnessState::Stale,
            age_ms: 6_000,
            is_fresh: false,
        };
        let assessment = OpportunityHealthAssessor.assess(&stored, &stale_freshness, None, false);
        assert_eq!(assessment.health, OpportunityHealth::Healthy);
        assert!(!assessment.needs_revalidation);
    }

    #[test]
    fn unavailable_best_source_makes_the_opportunity_unavailable() {
        let stored = record(&["network-a"]);
        let health = snapshot("network-a", SourceHealthState::Unavailable);
        let assessment = OpportunityHealthAssessor.assess(&stored, &freshness_active(), Some(&health), true);
        assert_eq!(assessment.health, OpportunityHealth::Unavailable);
        assert!(assessment.needs_revalidation, "flag is preserved even when health is worse");
        assert!(matches!(
            assessment.concerns.as_slice(),
            [HealthConcern::BestSourceUnavailable { .. }, HealthConcern::RevalidationPending]
        ));
    }

    #[test]
    fn needs_revalidation_only_promotes_healthy_records() {
        let stored = record(&["network-a"]);
        let health = snapshot("network-a", SourceHealthState::Degraded);
        let assessment = OpportunityHealthAssessor.assess(&stored, &freshness_active(), Some(&health), true);
        assert_eq!(assessment.health, OpportunityHealth::Degraded, "revalidation never masks degradation");
        assert!(assessment.concerns.contains(&HealthConcern::RevalidationPending));
    }

    #[test]
    fn unknown_source_health_is_reported_but_not_penalized() {
        let stored = record(&["network-a"]);
        let health = snapshot("network-a", SourceHealthState::Unknown);
        let assessment = OpportunityHealthAssessor.assess(&stored, &freshness_active(), Some(&health), false);
        assert_eq!(assessment.health, OpportunityHealth::Healthy);
        assert_eq!(
            assessment.concerns,
            vec![HealthConcern::BestSourceUnknown { source: "network-a".into() }]
        );
    }

    #[test]
    fn records_without_observations_are_unavailable() {
        let mut stored = record(&["network-a"]);
        stored.observations.clear();
        let assessment = OpportunityHealthAssessor.assess(&stored, &freshness_active(), None, false);
        assert_eq!(assessment.health, OpportunityHealth::Unavailable);
        assert_eq!(assessment.concerns, vec![HealthConcern::NoObservations]);
    }

    #[test]
    fn needs_revalidation_flag_is_reported_alone() {
        let stored = record(&["network-a"]);
        let assessment = OpportunityHealthAssessor.assess(&stored, &freshness_active(), None, true);
        assert_eq!(assessment.health, OpportunityHealth::NeedsRevalidation);
        assert!(assessment.needs_revalidation);
        assert_eq!(assessment.concerns, vec![HealthConcern::RevalidationPending]);
    }
}

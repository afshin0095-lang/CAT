//! Opportunity status projection: a read model combining persisted facts,
//! freshness, revalidation planning, and health.
//!
//! This is a domain/read-model boundary. No UI code lives here; services and
//! future agents consume `OpportunityStatusView` to answer:
//!
//! - Is this opportunity currently active / stale / expired?
//! - Which source is best and how good is the best observation?
//! - How old is the observation and is it still fresh enough?
//! - Does it need revalidation, why, and against which sources?
//! - What is its revision (optimistic concurrency) and aggregate health?

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::OpportunityRecord;
use crate::discovery_source_health::SourceHealthSnapshot;
use crate::opportunity_freshness::{
    FreshnessEvaluation, FreshnessPolicy, FreshnessPolicyError, FreshnessState,
};
use crate::opportunity_health::{
    HealthConcern, OpportunityHealth, OpportunityHealthAssessment, OpportunityHealthAssessor,
};
use crate::opportunity_revalidation::RevalidationBlockReason;
use crate::opportunity_version::OpportunityRevision;
use crate::revalidation_planner::RevalidationPlanner;

/// The best (highest-scoring, deterministically tie-broken) observation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BestObservationView {
    pub source: String,
    pub external_id: String,
    pub destination_url: String,
    pub currency: String,
    pub price_minor: Option<i64>,
    pub commission_bps: Option<u32>,
    pub score: u32,
    pub observed_at_ms: u64,
}

/// Read model for one opportunity at one evaluation instant.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityStatusView {
    pub opportunity_id: Uuid,
    pub identity: String,
    pub merchant_name: String,
    pub product_name: String,
    pub category: Option<String>,
    /// Derived lifecycle state (see `opportunity_freshness`).
    pub lifecycle_state: FreshnessState,
    pub freshness: FreshnessEvaluation,
    pub last_observed_at_ms: u64,
    pub observation_age_ms: u64,
    pub best_source: String,
    pub best_score: u32,
    pub best_observation: Option<BestObservationView>,
    pub revision: OpportunityRevision,
    pub health: OpportunityHealth,
    pub health_concerns: Vec<HealthConcern>,
    pub needs_revalidation: bool,
    /// Sorted, de-duplicated reasons for the required revalidation.
    pub revalidation_reasons: Vec<String>,
    pub revalidation_blocked: Option<RevalidationBlockReason>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpportunityProjectionError {
    ClockBeforeObservation,
    /// A policy reconstructed from untrusted data (e.g. deserialization)
    /// violated the threshold ordering contract.
    InvalidPolicy,
}

impl std::fmt::Display for OpportunityProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClockBeforeObservation => {
                formatter.write_str("evaluation time cannot precede the last observation")
            }
            Self::InvalidPolicy => {
                formatter.write_str("freshness policy violates 0 <= active <= stale < expiration")
            }
        }
    }
}

impl std::error::Error for OpportunityProjectionError {}

impl From<FreshnessPolicyError> for OpportunityProjectionError {
    fn from(error: FreshnessPolicyError) -> Self {
        match error {
            FreshnessPolicyError::ClockBeforeObservation => Self::ClockBeforeObservation,
            FreshnessPolicyError::ActiveBeyondStale
            | FreshnessPolicyError::StaleBeyondExpiration
            | FreshnessPolicyError::ThresholdTooLarge => Self::InvalidPolicy,
        }
    }
}

/// Projects `OpportunityRecord`s into `OpportunityStatusView`s.
///
/// Deterministic: identical inputs always produce an identical view.
#[derive(Clone, Debug)]
pub struct OpportunityStatusProjector {
    policy: FreshnessPolicy,
    planner: RevalidationPlanner,
}

impl OpportunityStatusProjector {
    pub fn new(policy: FreshnessPolicy) -> Self {
        Self {
            policy,
            planner: RevalidationPlanner::new(policy),
        }
    }

    pub fn policy(&self) -> FreshnessPolicy {
        self.policy
    }

    /// Projects a record, assuming every observation source is reachable.
    pub fn project(
        &self,
        record: &OpportunityRecord,
        now_ms: u64,
    ) -> Result<OpportunityStatusView, OpportunityProjectionError> {
        // The convenience method promises the optimistic view that all
        // sources already persisted on the record are reachable. Callers that
        // have an explicit health snapshot must use
        // `project_with_availability` instead.
        let available_sources: Vec<String> = record.observations.keys().cloned().collect();
        self.project_with_availability(record, now_ms, &available_sources)
    }

    /// Projects a record with explicit source availability. Sources absent
    /// from `available_sources` are treated as unreachable when planning
    /// revalidation.
    pub fn project_with_availability(
        &self,
        record: &OpportunityRecord,
        now_ms: u64,
        available_sources: &[String],
    ) -> Result<OpportunityStatusView, OpportunityProjectionError> {
        let record_freshness = self.policy.evaluate(record, now_ms)?;
        let decision = self.planner.plan(
            record,
            record_freshness.evaluation,
            available_sources,
            now_ms,
        );

        // "Needs revalidation" is a freshness fact: anything past the active
        // freshness target needs a refresh even when planning is currently
        // blocked (the block reason explains why no request exists yet).
        let needs_revalidation = !record_freshness.evaluation.is_fresh;

        let assessor = OpportunityHealthAssessor;
        let best_source_health: Option<SourceHealthSnapshot> = None;
        let assessment: OpportunityHealthAssessment = assessor.assess(
            record,
            &record_freshness.evaluation,
            best_source_health.as_ref(),
            needs_revalidation,
        );

        let best_observation = record
            .best_observation()
            .map(|observation| BestObservationView {
                source: observation.source.clone(),
                external_id: observation.external_id.clone(),
                destination_url: observation.destination_url.clone(),
                currency: observation.currency.clone(),
                price_minor: observation.price_minor,
                commission_bps: observation.commission_bps,
                score: observation.score,
                observed_at_ms: observation.observed_at_ms,
            });

        Ok(OpportunityStatusView {
            opportunity_id: record.id,
            identity: record.identity.as_str().to_owned(),
            merchant_name: record.merchant_name.clone(),
            product_name: record.product_name.clone(),
            category: record.category.clone(),
            lifecycle_state: record_freshness.state(),
            freshness: record_freshness.evaluation,
            last_observed_at_ms: record_freshness.last_observed_at_ms,
            observation_age_ms: record_freshness.age_ms(),
            best_source: record.best_source.clone(),
            best_score: record.best_score,
            best_observation,
            revision: record.revision,
            health: assessment.health,
            health_concerns: assessment.concerns,
            needs_revalidation,
            revalidation_reasons: decision.reasons(),
            revalidation_blocked: decision.blocked,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityStore,
        canonical_key,
    };
    use uuid::Uuid;

    fn record(observed_at_ms: u64) -> OpportunityRecord {
        let candidate = DiscoveryCandidate {
            source: "network-a".into(),
            external_id: "sku-1".into(),
            merchant_name: "Acme".into(),
            product_name: "Widget".into(),
            canonical_key: canonical_key("Acme", "Widget"),
            category: Some("electronics".into()),
            destination_url: "https://example.test/widget".into(),
            currency: "EUR".into(),
            price_minor: Some(12_345),
            commission_bps: Some(750),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms,
        };
        let opportunity = DiscoveryOpportunity {
            id: Uuid::now_v7(),
            candidate,
            score: 8_800,
            rank: 1,
        };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store.list().into_iter().next().expect("record exists")
    }

    fn projector() -> OpportunityStatusProjector {
        OpportunityStatusProjector::new(
            FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
        )
    }

    #[test]
    fn view_answers_the_standard_questions() {
        let projector = projector();
        let stored = record(10_000);
        let view = projector
            .project(&stored, 10_500)
            .expect("valid projection");

        assert_eq!(view.identity, "acme:widget");
        assert_eq!(view.lifecycle_state, FreshnessState::Active);
        assert!(view.freshness.is_fresh);
        assert_eq!(view.observation_age_ms, 500);
        assert_eq!(view.best_source, "network-a");
        assert_eq!(view.best_score, 8_800);
        let observation = view.best_observation.expect("best observation");
        assert_eq!(observation.currency, "EUR");
        assert_eq!(observation.price_minor, Some(12_345));
        assert_eq!(observation.commission_bps, Some(750));
        assert!(!view.needs_revalidation);
        assert!(view.revalidation_reasons.is_empty());
        assert_eq!(view.health, OpportunityHealth::Healthy);
        assert_eq!(view.revision, OpportunityRevision::initial());
    }

    #[test]
    fn stale_views_explain_why_revalidation_is_needed() {
        let projector = projector();
        let stored = record(10_000);
        let view = projector
            .project(&stored, 16_000)
            .expect("valid projection");
        assert_eq!(view.lifecycle_state, FreshnessState::Stale);
        assert!(view.needs_revalidation);
        assert_eq!(view.revalidation_reasons, vec!["stale"]);
        assert_eq!(view.health, OpportunityHealth::NeedsRevalidation);
    }

    #[test]
    fn projection_is_deterministic() {
        let projector = projector();
        let stored = record(10_000);
        let first = projector
            .project(&stored, 12_000)
            .expect("valid projection");
        let second = projector
            .project(&stored, 12_000)
            .expect("valid projection");
        assert_eq!(first, second);
    }

    #[test]
    fn clock_regression_fails_closed() {
        let projector = projector();
        let stored = record(10_000);
        assert_eq!(
            projector.project(&stored, 9_999),
            Err(OpportunityProjectionError::ClockBeforeObservation)
        );
    }

    #[test]
    fn availability_changes_the_revalidation_story() {
        let projector = projector();
        let stored = record(10_000);
        let now = 13_000;
        let reachable = projector
            .project_with_availability(&stored, now, &["network-a".to_owned()])
            .expect("valid projection");
        assert!(reachable.needs_revalidation);
        assert!(reachable.revalidation_blocked.is_none());

        let unreachable = projector
            .project_with_availability(&stored, now, &[])
            .expect("valid projection");
        // Still stale, so it still *needs* revalidation — the block reason
        // explains why no request could be planned.
        assert!(unreachable.needs_revalidation);
        assert!(unreachable.revalidation_reasons.is_empty());
        assert_eq!(
            unreachable.revalidation_blocked,
            Some(RevalidationBlockReason::AllSourcesUnavailable)
        );
    }
}

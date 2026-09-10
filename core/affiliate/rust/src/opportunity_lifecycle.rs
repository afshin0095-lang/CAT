use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::opportunity_freshness::{FreshnessPolicy, FreshnessPolicyError, FreshnessState};
use crate::OpportunityRecord;

/// Lifecycle classification derived from the age of the most recent observation.
///
/// This is a thin lifecycle-facing view of [`FreshnessState`]; the derivation
/// logic itself lives in exactly one place (`opportunity_freshness`) so the
/// two vocabularies can never disagree.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpportunityLifecycleState {
    Active,
    Stale,
    Expired,
}

impl From<FreshnessState> for OpportunityLifecycleState {
    fn from(state: FreshnessState) -> Self {
        match state {
            FreshnessState::Active => Self::Active,
            FreshnessState::Stale => Self::Stale,
            FreshnessState::Expired => Self::Expired,
        }
    }
}

impl OpportunityLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }
}

/// Time policy for opportunity freshness.
///
/// Thresholds are expressed in milliseconds and must satisfy
/// `0 < stale_after_ms < expire_after_ms`. Validation and state derivation
/// delegate to [`FreshnessPolicy`] with
/// `active_threshold_ms == stale_after_ms`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityLifecyclePolicy {
    pub stale_after_ms: u64,
    pub expire_after_ms: u64,
}

impl OpportunityLifecyclePolicy {
    pub fn new(stale_after_ms: u64, expire_after_ms: u64) -> Result<Self, OpportunityLifecycleError> {
        if stale_after_ms == 0 || stale_after_ms >= expire_after_ms {
            return Err(OpportunityLifecycleError::InvalidPolicy);
        }
        Ok(Self {
            stale_after_ms,
            expire_after_ms,
        })
    }

    /// Canonical three-threshold policy behind this two-threshold view.
    pub fn to_freshness_policy(&self) -> FreshnessPolicy {
        FreshnessPolicy {
            active_threshold_ms: self.stale_after_ms,
            stale_threshold_ms: self.stale_after_ms,
            expiration_threshold_ms: self.expire_after_ms,
        }
    }
}

impl From<OpportunityLifecyclePolicy> for FreshnessPolicy {
    fn from(policy: OpportunityLifecyclePolicy) -> Self {
        // Valid by construction: `OpportunityLifecyclePolicy::new` already
        // rejected `stale_after_ms == 0` and `stale_after_ms >= expire_after_ms`.
        policy.to_freshness_policy()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityLifecycleSnapshot {
    pub state: OpportunityLifecycleState,
    pub age_ms: u64,
    pub last_observed_at_ms: u64,
}

/// Identity-bound lifecycle evaluation for one opportunity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LifecycleEvaluation {
    pub identity: String,
    pub opportunity_id: Uuid,
    pub snapshot: OpportunityLifecycleSnapshot,
}

/// Deterministic aggregate counts across a batch evaluation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LifecycleStateCounts {
    pub active: u64,
    pub stale: u64,
    pub expired: u64,
}

/// Deterministic lifecycle view for a batch of opportunities.
///
/// Evaluations are sorted by `OpportunityIdentity` so the output never
/// depends on storage iteration order.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LifecycleEvaluationBatch {
    pub evaluated_at_ms: u64,
    pub policy: OpportunityLifecyclePolicy,
    pub counts: LifecycleStateCounts,
    pub evaluations: Vec<LifecycleEvaluation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpportunityLifecycleError {
    InvalidPolicy,
    ClockBeforeObservation,
}

impl std::fmt::Display for OpportunityLifecycleError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPolicy => formatter.write_str("lifecycle policy must satisfy 0 < stale_after_ms < expire_after_ms"),
            Self::ClockBeforeObservation => formatter.write_str("evaluation time cannot precede the last observation"),
        }
    }
}

impl std::error::Error for OpportunityLifecycleError {}

impl From<FreshnessPolicyError> for OpportunityLifecycleError {
    fn from(error: FreshnessPolicyError) -> Self {
        match error {
            FreshnessPolicyError::ClockBeforeObservation => Self::ClockBeforeObservation,
            FreshnessPolicyError::ActiveBeyondStale
            | FreshnessPolicyError::StaleBeyondExpiration
            | FreshnessPolicyError::ThresholdTooLarge => Self::InvalidPolicy,
        }
    }
}

/// Deterministically evaluates freshness without mutating the persisted opportunity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityLifecycleEvaluator {
    policy: OpportunityLifecyclePolicy,
    freshness: FreshnessPolicy,
}

impl OpportunityLifecycleEvaluator {
    pub fn new(policy: OpportunityLifecyclePolicy) -> Self {
        Self {
            policy,
            freshness: policy.to_freshness_policy(),
        }
    }

    pub fn evaluate(&self, record: &OpportunityRecord, now_ms: u64) -> Result<OpportunityLifecycleSnapshot, OpportunityLifecycleError> {
        let evaluation = self.freshness.evaluate(record, now_ms)?;
        Ok(OpportunityLifecycleSnapshot {
            state: evaluation.state().into(),
            age_ms: evaluation.age_ms(),
            last_observed_at_ms: evaluation.last_observed_at_ms,
        })
    }

    pub fn policy(&self) -> OpportunityLifecyclePolicy {
        self.policy
    }
}

/// Produces a deterministic lifecycle view for a batch of opportunities.
pub fn evaluate_records(
    records: &[OpportunityRecord],
    evaluator: OpportunityLifecycleEvaluator,
    now_ms: u64,
) -> Result<Vec<(String, OpportunityLifecycleSnapshot)>, OpportunityLifecycleError> {
    Ok(evaluate_records_batch(records, evaluator, now_ms)?.pair_view())
}

/// Produces a deterministic, aggregated lifecycle batch for a set of records.
///
/// - output is sorted by identity (never storage order);
/// - the batch never mutates records;
/// - a clock-regressed record fails the whole batch (fail closed).
pub fn evaluate_records_batch(
    records: &[OpportunityRecord],
    evaluator: OpportunityLifecycleEvaluator,
    now_ms: u64,
) -> Result<LifecycleEvaluationBatch, OpportunityLifecycleError> {
    let mut evaluations = Vec::with_capacity(records.len());
    for record in records {
        let snapshot = evaluator.evaluate(record, now_ms)?;
        evaluations.push(LifecycleEvaluation {
            identity: record.identity.as_str().to_owned(),
            opportunity_id: record.id,
            snapshot,
        });
    }
    evaluations.sort_by(|left, right| left.identity.cmp(&right.identity));

    let mut counts = LifecycleStateCounts::default();
    for evaluation in &evaluations {
        match evaluation.snapshot.state {
            OpportunityLifecycleState::Active => counts.active = counts.active.saturating_add(1),
            OpportunityLifecycleState::Stale => counts.stale = counts.stale.saturating_add(1),
            OpportunityLifecycleState::Expired => counts.expired = counts.expired.saturating_add(1),
        }
    }

    Ok(LifecycleEvaluationBatch {
        evaluated_at_ms: now_ms,
        policy: evaluator.policy(),
        counts,
        evaluations,
    })
}

impl LifecycleEvaluationBatch {
    /// Compatibility view used by `evaluate_records`.
    fn pair_view(&self) -> Vec<(String, OpportunityLifecycleSnapshot)> {
        self.evaluations
            .iter()
            .map(|evaluation| (evaluation.identity.clone(), evaluation.snapshot))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityStore};
    use uuid::Uuid;

    fn record(observed_at_ms: u64, merchant: &str) -> OpportunityRecord {
        let candidate = DiscoveryCandidate {
            source: "network-a".into(),
            external_id: "sku-1".into(),
            merchant_name: merchant.into(),
            product_name: "Widget".into(),
            canonical_key: canonical_key(merchant, "Widget"),
            category: Some("electronics".into()),
            destination_url: "https://example.test/widget".into(),
            currency: "EUR".into(),
            price_minor: Some(10_000),
            commission_bps: Some(500),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms,
        };
        let opportunity = DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score: 8_000, rank: 1 };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store.list().into_iter().next().expect("record exists")
    }

    #[test]
    fn lifecycle_state_matches_freshness_state() {
        assert_eq!(OpportunityLifecycleState::from(FreshnessState::Active), OpportunityLifecycleState::Active);
        assert_eq!(OpportunityLifecycleState::from(FreshnessState::Stale), OpportunityLifecycleState::Stale);
        assert_eq!(OpportunityLifecycleState::from(FreshnessState::Expired), OpportunityLifecycleState::Expired);
    }

    #[test]
    fn delegate_policy_preserves_two_threshold_semantics() {
        let policy = OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy");
        let freshness = policy.to_freshness_policy();
        assert_eq!(freshness.active_threshold_ms, 1_000);
        assert_eq!(freshness.stale_threshold_ms, 1_000);
        assert_eq!(freshness.expiration_threshold_ms, 5_000);
        assert_eq!(freshness.evaluate_age(999).state, FreshnessState::Active);
        assert_eq!(freshness.evaluate_age(1_000).state, FreshnessState::Stale);
        assert_eq!(freshness.evaluate_age(5_000).state, FreshnessState::Expired);
    }

    #[test]
    fn batch_counts_and_ordering_are_deterministic() {
        let evaluator = OpportunityLifecycleEvaluator::new(
            OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy"),
        );
        let zeta = record(10_000, "Zeta");
        let alpha = record(10_000, "Alpha");
        let stale_one = record(4_000, "Beta");

        let batch = evaluate_records_batch(&[zeta, alpha, stale_one], evaluator, 10_500)
            .expect("valid batch");

        let identities: Vec<&str> = batch
            .evaluations
            .iter()
            .map(|evaluation| evaluation.identity.as_str())
            .collect();
        assert_eq!(identities, vec!["alpha:widget", "beta:widget", "zeta:widget"]);
        assert_eq!(batch.counts.active, 2);
        assert_eq!(batch.counts.stale, 1);
        assert_eq!(batch.counts.expired, 0);
        assert_eq!(batch.evaluated_at_ms, 10_500);
    }

    #[test]
    fn batch_fails_closed_on_clock_regression() {
        let evaluator = OpportunityLifecycleEvaluator::new(
            OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy"),
        );
        let good = record(10_000, "Alpha");
        let regressed = record(20_000, "Beta");
        assert_eq!(
            evaluate_records_batch(&[good, regressed], evaluator, 10_500),
            Err(OpportunityLifecycleError::ClockBeforeObservation)
        );
    }
}

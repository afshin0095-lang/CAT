//! Opportunity freshness policy: the canonical time-derived state contract.
//!
//! Freshness is a **derived projection** over durable observation facts. This
//! module owns the single implementation of that derivation;
//! `opportunity_lifecycle` delegates to it so no second threshold engine can
//! drift away.
//!
//! Design rules:
//! - integer milliseconds only, no floating point;
//! - no hidden system-clock dependency — evaluation time is always injected;
//! - checked arithmetic everywhere;
//! - clock regression fails closed instead of wrapping into a valid state.

use serde::{Deserialize, Serialize};

use crate::OpportunityRecord;

/// Large but finite cap on policy thresholds (≈317 years in milliseconds).
/// This rejects absurd configurations that could never expire anything while
/// remaining far above any realistic freshness window.
pub const MAX_THRESHOLD_MS: u64 = 10_000_000_000_000;

/// Derived freshness state of an opportunity.
///
/// Semantically aligned with `OpportunityLifecycleState`; that type remains a
/// lifecycle-facing alias so existing contracts keep their names.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    Active,
    Stale,
    Expired,
}

impl FreshnessState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }
}

/// Immutable freshness configuration.
///
/// Thresholds partition observation age:
///
/// | Age condition | State |
/// |---|---|
/// | `age < stale_threshold_ms` | Active |
/// | `stale_threshold_ms <= age < expiration_threshold_ms` | Stale |
/// | `age >= expiration_threshold_ms` | Expired |
///
/// `active_threshold_ms` is the *freshness target*: observations younger than
/// it need no revalidation at all. It must satisfy
/// `active_threshold_ms <= stale_threshold_ms` so a policy can never demand
/// refreshes inside its own active window.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FreshnessPolicy {
    pub active_threshold_ms: u64,
    pub stale_threshold_ms: u64,
    pub expiration_threshold_ms: u64,
}

impl FreshnessPolicy {
    /// Validates and constructs a policy.
    ///
    /// Rejected configurations (fail closed):
    /// - any threshold above [`MAX_THRESHOLD_MS`];
    /// - `active_threshold_ms > stale_threshold_ms`;
    /// - `stale_threshold_ms >= expiration_threshold_ms`;
    /// - `expiration_threshold_ms == 0` (implied by the strict inequality).
    ///
    /// `active_threshold_ms == stale_threshold_ms` is allowed: it means every
    /// observation past the active window is immediately eligible for refresh,
    /// which is exactly the mapping used by the two-threshold lifecycle policy.
    pub fn new(
        active_threshold_ms: u64,
        stale_threshold_ms: u64,
        expiration_threshold_ms: u64,
    ) -> Result<Self, FreshnessPolicyError> {
        if active_threshold_ms > MAX_THRESHOLD_MS
            || stale_threshold_ms > MAX_THRESHOLD_MS
            || expiration_threshold_ms > MAX_THRESHOLD_MS
        {
            return Err(FreshnessPolicyError::ThresholdTooLarge);
        }
        if active_threshold_ms > stale_threshold_ms {
            return Err(FreshnessPolicyError::ActiveBeyondStale);
        }
        if stale_threshold_ms >= expiration_threshold_ms {
            return Err(FreshnessPolicyError::StaleBeyondExpiration);
        }
        Ok(Self {
            active_threshold_ms,
            stale_threshold_ms,
            expiration_threshold_ms,
        })
    }

    /// Classifies an observation age against this policy.
    ///
    /// Deterministic and side-effect free. `age_ms` must be a checked
    /// difference (see [`FreshnessPolicy::evaluate`]); this function carries
    /// it into the returned [`FreshnessEvaluation`] unchanged.
    pub fn evaluate_age(&self, age_ms: u64) -> FreshnessEvaluation {
        let state = if age_ms >= self.expiration_threshold_ms {
            FreshnessState::Expired
        } else if age_ms >= self.stale_threshold_ms {
            FreshnessState::Stale
        } else {
            FreshnessState::Active
        };
        FreshnessEvaluation {
            state,
            age_ms,
            is_fresh: age_ms <= self.active_threshold_ms,
        }
    }

    /// Evaluates a persisted opportunity record at `evaluation_time_ms`.
    ///
    /// Clock regression (`evaluation_time_ms < last_observed_at_ms`) fails
    /// closed: it is never silently converted into a valid state.
    pub fn evaluate(
        &self,
        record: &OpportunityRecord,
        evaluation_time_ms: u64,
    ) -> Result<RecordFreshnessEvaluation, FreshnessPolicyError> {
        let age_ms = evaluation_time_ms
            .checked_sub(record.last_observed_at_ms)
            .ok_or(FreshnessPolicyError::ClockBeforeObservation)?;
        Ok(RecordFreshnessEvaluation {
            last_observed_at_ms: record.last_observed_at_ms,
            evaluation: self.evaluate_age(age_ms),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FreshnessPolicyError {
    ActiveBeyondStale,
    StaleBeyondExpiration,
    ThresholdTooLarge,
    ClockBeforeObservation,
}

impl std::fmt::Display for FreshnessPolicyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ActiveBeyondStale => {
                formatter.write_str("active threshold must not exceed the stale threshold")
            }
            Self::StaleBeyondExpiration => {
                formatter.write_str("stale threshold must be earlier than the expiration threshold")
            }
            Self::ThresholdTooLarge => {
                write!(
                    formatter,
                    "thresholds must not exceed {MAX_THRESHOLD_MS} ms"
                )
            }
            Self::ClockBeforeObservation => {
                formatter.write_str("evaluation time cannot precede the last observation")
            }
        }
    }
}

impl std::error::Error for FreshnessPolicyError {}

/// Age-only freshness evaluation (no record identity attached).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FreshnessEvaluation {
    pub state: FreshnessState,
    pub age_ms: u64,
    /// True when the observation is within the active freshness target and
    /// therefore not eligible for routine revalidation.
    pub is_fresh: bool,
}

/// Record-bound freshness evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RecordFreshnessEvaluation {
    pub last_observed_at_ms: u64,
    pub evaluation: FreshnessEvaluation,
}

impl RecordFreshnessEvaluation {
    pub fn state(&self) -> FreshnessState {
        self.evaluation.state
    }

    pub fn age_ms(&self) -> u64 {
        self.evaluation.age_ms
    }

    pub fn is_fresh(&self) -> bool {
        self.evaluation.is_fresh
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
            price_minor: Some(10_000),
            commission_bps: Some(500),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms,
        };
        let opportunity = DiscoveryOpportunity {
            id: Uuid::now_v7(),
            candidate,
            score: 8_000,
            rank: 1,
        };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store.list().into_iter().next().expect("record exists")
    }

    #[test]
    fn policy_boundaries_are_exact_and_deterministic() {
        let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
        assert_eq!(policy.evaluate_age(0).state, FreshnessState::Active);
        assert_eq!(policy.evaluate_age(4_999).state, FreshnessState::Active);
        assert_eq!(policy.evaluate_age(5_000).state, FreshnessState::Stale);
        assert_eq!(policy.evaluate_age(8_999).state, FreshnessState::Stale);
        assert_eq!(policy.evaluate_age(9_000).state, FreshnessState::Expired);
        assert_eq!(policy.evaluate_age(u64::MAX).state, FreshnessState::Expired);
    }

    #[test]
    fn active_target_marks_fresh_observations() {
        let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
        assert!(policy.evaluate_age(1_000).is_fresh);
        assert!(!policy.evaluate_age(1_001).is_fresh);
        assert!(policy.evaluate_age(0).is_fresh);
    }

    #[test]
    fn invalid_policies_fail_closed() {
        assert_eq!(
            FreshnessPolicy::new(6_000, 5_000, 9_000),
            Err(FreshnessPolicyError::ActiveBeyondStale)
        );
        assert_eq!(
            FreshnessPolicy::new(1_000, 9_000, 9_000),
            Err(FreshnessPolicyError::StaleBeyondExpiration)
        );
        assert_eq!(
            FreshnessPolicy::new(1_000, 5_000, 5_000),
            Err(FreshnessPolicyError::StaleBeyondExpiration)
        );
        assert_eq!(
            FreshnessPolicy::new(0, MAX_THRESHOLD_MS + 1, MAX_THRESHOLD_MS + 2),
            Err(FreshnessPolicyError::ThresholdTooLarge)
        );
        assert_eq!(
            FreshnessPolicy::new(0, 0, 1),
            Ok(FreshnessPolicy {
                active_threshold_ms: 0,
                stale_threshold_ms: 0,
                expiration_threshold_ms: 1,
            })
        );
    }

    #[test]
    fn clock_regression_fails_closed() {
        let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
        let stored = record(10_000);
        assert_eq!(
            policy.evaluate(&stored, 9_999),
            Err(FreshnessPolicyError::ClockBeforeObservation)
        );
        assert!(policy.evaluate(&stored, 10_000).is_ok());
    }

    #[test]
    fn record_evaluation_is_repeatable() {
        let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
        let stored = record(10_000);
        let first = policy.evaluate(&stored, 11_000).expect("valid evaluation");
        let second = policy.evaluate(&stored, 11_000).expect("valid evaluation");
        assert_eq!(first, second);
        assert_eq!(first.state(), FreshnessState::Stale);
        assert_eq!(first.age_ms(), 1_000);
        assert_eq!(first.last_observed_at_ms, 10_000);
        assert!(!first.is_fresh());
    }

    #[test]
    fn state_serialization_uses_snake_case_strings() {
        assert_eq!(
            serde_json::to_string(&FreshnessState::Active).unwrap(),
            "\"active\""
        );
        let decoded: FreshnessState = serde_json::from_str("\"expired\"").unwrap();
        assert_eq!(decoded, FreshnessState::Expired);
    }
}

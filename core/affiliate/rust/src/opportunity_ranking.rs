//! Deterministic opportunity ranking over integer basis-point arithmetic.
//!
//! Sprint 0 factor model (all factors are `0..=10_000`, weights are basis
//! points that must sum to exactly `10_000`):
//!
//! | Factor | Source | Weight (default) |
//! |---|---|---|
//! | `composite_score` | `OpportunityRecord::best_score` (discovery composite) | 6_000 |
//! | `freshness` | derived from freshness state/age | 2_000 |
//! | `economic_value` | best observation commission (basis points) | 1_200 |
//! | `source_reliability` | source health availability, unknown ⇒ 10_000 | 800 |
//!
//! The default weights encode an explicit, documented commercial position:
//! the discovery composite dominates, freshness materially protects users
//! from dead offers, economics and source trust fine-tune the ordering.
//! Nothing here is hidden: weights are constants on
//! [`OpportunityRankingProfile`] and every scored factor is returned in
//! [`RankedOpportunity::factors`].
//!
//! No floating point is used anywhere: all intermediate math is `u128`.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::discovery_source_health::SourceHealthSnapshot;
use crate::opportunity_freshness::{FreshnessPolicy, FreshnessPolicyError, FreshnessState};
use crate::OpportunityRecord;

pub const WEIGHT_SCALE: u32 = 10_000;

/// Default profile weights (see module docs).
pub const DEFAULT_COMPOSITE_SCORE_WEIGHT_BPS: u32 = 6_000;
pub const DEFAULT_FRESHNESS_WEIGHT_BPS: u32 = 2_000;
pub const DEFAULT_ECONOMIC_VALUE_WEIGHT_BPS: u32 = 1_200;
pub const DEFAULT_SOURCE_RELIABILITY_WEIGHT_BPS: u32 = 800;

/// Reliability credit granted when a source has no health observations yet:
/// new sources are not penalized for data that does not exist.
pub const UNKNOWN_SOURCE_RELIABILITY_BPS: u32 = 10_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityRankingProfile {
    pub composite_score_weight_bps: u32,
    pub freshness_weight_bps: u32,
    pub economic_value_weight_bps: u32,
    pub source_reliability_weight_bps: u32,
}

impl Default for OpportunityRankingProfile {
    fn default() -> Self {
        Self {
            composite_score_weight_bps: DEFAULT_COMPOSITE_SCORE_WEIGHT_BPS,
            freshness_weight_bps: DEFAULT_FRESHNESS_WEIGHT_BPS,
            economic_value_weight_bps: DEFAULT_ECONOMIC_VALUE_WEIGHT_BPS,
            source_reliability_weight_bps: DEFAULT_SOURCE_RELIABILITY_WEIGHT_BPS,
        }
    }
}

impl OpportunityRankingProfile {
    /// The default profile weights are valid by construction; explicit
    /// profiles must still be validated via [`OpportunityRanker::new`].
    pub fn validate(&self) -> Result<(), OpportunityRankingError> {
        let total = self
            .composite_score_weight_bps
            .saturating_add(self.freshness_weight_bps)
            .saturating_add(self.economic_value_weight_bps)
            .saturating_add(self.source_reliability_weight_bps);
        if total != WEIGHT_SCALE {
            return Err(OpportunityRankingError::InvalidProfile { total });
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RankingFactors {
    pub composite_score: u32,
    pub freshness: u32,
    pub economic_value: u32,
    pub source_reliability: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RankedOpportunity {
    pub identity: String,
    pub opportunity_id: Uuid,
    /// Composite ranking score, `0..=10_000`.
    pub score: u32,
    /// 1-based position in deterministic order (score desc, identity asc).
    pub rank: u32,
    pub factors: RankingFactors,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpportunityRankingError {
    InvalidProfile { total: u32 },
    ClockBeforeObservation,
}

impl std::fmt::Display for OpportunityRankingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProfile { total } => {
                write!(formatter, "ranking weights must sum to {WEIGHT_SCALE} basis points, got {total}")
            }
            Self::ClockBeforeObservation => {
                formatter.write_str("ranking time cannot precede an opportunity's last observation")
            }
        }
    }
}

impl std::error::Error for OpportunityRankingError {}

/// Deterministic ranker. Immutable profile; evaluation time and freshness
/// policy are supplied per call so ranking is replayable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityRanker {
    profile: OpportunityRankingProfile,
}

impl OpportunityRanker {
    pub fn new(profile: OpportunityRankingProfile) -> Result<Self, OpportunityRankingError> {
        profile.validate()?;
        Ok(Self { profile })
    }

    /// Ranker using the documented default profile.
    pub fn with_default_profile() -> Self {
        Self {
            profile: OpportunityRankingProfile::default(),
        }
    }

    pub fn profile(&self) -> OpportunityRankingProfile {
        self.profile
    }

    /// Ranks records deterministically: score descending, identity ascending.
    /// `source_health` may be empty; missing sources receive
    /// [`UNKNOWN_SOURCE_RELIABILITY_BPS`].
    pub fn rank(
        &self,
        records: &[OpportunityRecord],
        freshness_policy: &FreshnessPolicy,
        now_ms: u64,
        source_health: &BTreeMap<String, SourceHealthSnapshot>,
    ) -> Result<Vec<RankedOpportunity>, OpportunityRankingError> {
        let mut ranked: Vec<RankedOpportunity> = Vec::with_capacity(records.len());
        for record in records {
            let evaluation = freshness_policy
                .evaluate(record, now_ms)
                .map_err(|_| OpportunityRankingError::ClockBeforeObservation)?;
            let factors = self.factors(record, &evaluation.evaluation, freshness_policy, source_health);
            let score = self.weighted_score(&factors);
            ranked.push(RankedOpportunity {
                identity: record.identity.as_str().to_owned(),
                opportunity_id: record.id,
                score,
                rank: 0,
                factors,
            });
        }
        ranked.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.identity.cmp(&right.identity))
        });
        for (index, entry) in ranked.iter_mut().enumerate() {
            entry.rank = (index + 1) as u32;
        }
        Ok(ranked)
    }

    fn factors(
        &self,
        record: &OpportunityRecord,
        evaluation: &crate::opportunity_freshness::FreshnessEvaluation,
        policy: &FreshnessPolicy,
        source_health: &BTreeMap<String, SourceHealthSnapshot>,
    ) -> RankingFactors {
        let composite_score = record.best_score.min(WEIGHT_SCALE);
        let freshness = freshness_factor(evaluation, policy);
        let economic_value = record
            .best_observation()
            .and_then(|observation| observation.commission_bps)
            .map(|bps| bps.min(WEIGHT_SCALE))
            .unwrap_or(0);
        let source_reliability = source_health
            .get(&record.best_source)
            .and_then(|snapshot| snapshot.availability_bps)
            .map(u32::from)
            .unwrap_or(UNKNOWN_SOURCE_RELIABILITY_BPS)
            .min(WEIGHT_SCALE);
        RankingFactors {
            composite_score,
            freshness,
            economic_value,
            source_reliability,
        }
    }

    fn weighted_score(&self, factors: &RankingFactors) -> u32 {
        let weighted = u128::from(factors.composite_score)
            * u128::from(self.profile.composite_score_weight_bps)
            + u128::from(factors.freshness) * u128::from(self.profile.freshness_weight_bps)
            + u128::from(factors.economic_value) * u128::from(self.profile.economic_value_weight_bps)
            + u128::from(factors.source_reliability) * u128::from(self.profile.source_reliability_weight_bps);
        // Max input: 10_000 * 10_000 * 4 << u128::MAX; the division is exact
        // truncation of the weighted average, bounded by WEIGHT_SCALE.
        (weighted / u128::from(WEIGHT_SCALE)) as u32
    }
}

/// Freshness factor: Active stays full credit; Stale decays linearly from the
/// stale threshold to the expiration threshold; Expired is zero.
fn freshness_factor(evaluation: &crate::opportunity_freshness::FreshnessEvaluation, policy: &FreshnessPolicy) -> u32 {
    match evaluation.state {
        FreshnessState::Active => WEIGHT_SCALE,
        FreshnessState::Expired => 0,
        FreshnessState::Stale => {
            let window = policy.expiration_threshold_ms.saturating_sub(policy.stale_threshold_ms);
            if window == 0 {
                return 0;
            }
            let remaining = policy.expiration_threshold_ms.saturating_sub(evaluation.age_ms);
            let factor = u128::from(remaining) * u128::from(WEIGHT_SCALE) / u128::from(window);
            factor.min(u128::from(WEIGHT_SCALE)) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityStore};

    fn record(identity_product: &str, score: u32, commission_bps: Option<u32>, observed_at_ms: u64) -> OpportunityRecord {
        record_for(identity_product, score, observed_at_ms, commission_bps.unwrap_or(0))
    }

    fn record_for(product: &str, score: u32, observed_at_ms: u64, commission: u32) -> OpportunityRecord {
        let candidate = DiscoveryCandidate {
            source: "network-a".into(),
            external_id: "sku-1".into(),
            merchant_name: "Acme".into(),
            product_name: product.into(),
            canonical_key: canonical_key("Acme", product),
            category: None,
            destination_url: "https://example.test/item".into(),
            currency: "EUR".into(),
            price_minor: Some(9_999),
            commission_bps: Some(commission),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms,
        };
        let opportunity = DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score, rank: 1 };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store.list().into_iter().next().expect("record exists")
    }

    fn policy() -> FreshnessPolicy {
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy")
    }

    #[test]
    fn default_profile_is_valid_and_weights_sum_to_scale() {
        let profile = OpportunityRankingProfile::default();
        assert!(profile.validate().is_ok());
        assert_eq!(
            DEFAULT_COMPOSITE_SCORE_WEIGHT_BPS
                + DEFAULT_FRESHNESS_WEIGHT_BPS
                + DEFAULT_ECONOMIC_VALUE_WEIGHT_BPS
                + DEFAULT_SOURCE_RELIABILITY_WEIGHT_BPS,
            WEIGHT_SCALE
        );
    }

    #[test]
    fn invalid_profiles_are_rejected() {
        let profile = OpportunityRankingProfile {
            composite_score_weight_bps: 9_000,
            ..OpportunityRankingProfile::default()
        };
        assert_eq!(
            OpportunityRanker::new(profile),
            Err(OpportunityRankingError::InvalidProfile { total: 13_000 })
        );
    }

    #[test]
    fn ranking_is_deterministic_and_tie_breaks_by_identity() {
        let ranker = OpportunityRanker::with_default_profile();
        let records = vec![
            record("Zeta", 8_000, Some(5_000), 10_000),
            record("Alpha", 8_000, Some(5_000), 10_000),
            record("Mid", 9_000, Some(1_000), 10_000),
        ];
        let health = BTreeMap::new();
        let first = ranker.rank(&records, &policy(), 10_500, &health).expect("valid ranking");
        let second = ranker.rank(&records, &policy(), 10_500, &health).expect("valid ranking");
        assert_eq!(first, second);
        assert_eq!(first[0].identity, "acme:mid", "highest composite first");
        assert_eq!(first[0].rank, 1);
        assert_eq!(first[1].identity, "acme:alpha", "tie broken by identity");
        assert_eq!(first[2].identity, "acme:zeta");
    }

    #[test]
    fn freshness_reduces_scores_without_flipping_composite_outrank() {
        let ranker = OpportunityRanker::with_default_profile();
        let fresh = record("Fresh", 6_000, Some(10_000), 10_000);
        let stale = record("Stale", 6_000, Some(10_000), 5_500);

        let health = BTreeMap::new();
        let ranked = ranker
            .rank(&[stale, fresh], &policy(), 11_000, &health)
            .expect("valid ranking");
        assert_eq!(ranked[0].identity, "acme:fresh");
        assert!(ranked[0].score > ranked[1].score);
        assert!(
            ranked[1].factors.freshness < WEIGHT_SCALE,
            "stale freshness factor decays: {}",
            ranked[1].factors.freshness
        );
    }

    #[test]
    fn expired_opportunities_still_rank_but_score_lowest() {
        let ranker = OpportunityRanker::with_default_profile();
        // Equal composites and economics: freshness alone decides the order.
        let active = record_for("Active", 5_000, 20_000, 5_000);
        let expired = record_for("Expired", 5_000, 10_000, 5_000);

        let health = BTreeMap::new();
        let ranked = ranker.rank(&[active, expired], &policy(), 20_000, &health).expect("valid ranking");
        assert_eq!(ranked[0].identity, "acme:active");
        assert_eq!(ranked[1].factors.freshness, 0, "expired freshness factor is zero");
        assert!(ranked[1].score < ranked[0].score);
    }

    #[test]
    fn extreme_values_cannot_overflow() {
        let ranker = OpportunityRanker::with_default_profile();
        let mut top = record("Top", WEIGHT_SCALE, Some(WEIGHT_SCALE), 10_000);
        top.last_observed_at_ms = 0; // expired at now = u64::MAX / 2
        top.best_score = WEIGHT_SCALE;
        let health = BTreeMap::new();
        let ranked = ranker
            .rank(&[top], &policy(), u64::MAX / 2, &health)
            .expect("extreme ages must not overflow");
        assert_eq!(ranked[0].factors.freshness, 0);
        assert!(ranked[0].score <= WEIGHT_SCALE);
    }

    #[test]
    fn unknown_sources_receive_full_reliability_credit() {
        let ranker = OpportunityRanker::with_default_profile();
        let stored = record("Solo", 5_000, Some(5_000), 10_000);
        let health = BTreeMap::new();
        let ranked = ranker.rank(&[stored], &policy(), 10_500, &health).expect("valid ranking");
        assert_eq!(ranked[0].factors.source_reliability, UNKNOWN_SOURCE_RELIABILITY_BPS);
    }

    #[test]
    fn boundary_scores_are_exact() {
        let ranker = OpportunityRanker::with_default_profile();
        let perfect = record("Perfect", WEIGHT_SCALE, Some(WEIGHT_SCALE), 10_000);
        let health = BTreeMap::new();
        let ranked = ranker.rank(&[perfect], &policy(), 10_500, &health).expect("valid ranking");
        assert_eq!(ranked[0].score, WEIGHT_SCALE);

        // All-zero economic/composite/freshness factors with a fully
        // unavailable source produce the exact floor score.
        let zero = record("Zero", 0, Some(0), 10_000);
        let mut dead_health = BTreeMap::new();
        dead_health.insert(
            "network-a".to_string(),
            SourceHealthSnapshot {
                source: "network-a".to_string(),
                state: crate::SourceHealthState::Unavailable,
                availability_bps: Some(0),
                ..SourceHealthSnapshot::default()
            },
        );
        let ranked = ranker
            .rank(
                &[zero],
                &FreshnessPolicy::new(0, 0, 1).expect("valid policy"),
                10_001,
                &dead_health,
            )
            .expect("valid ranking");
        assert_eq!(ranked[0].score, 0);
    }

    #[test]
    fn clock_regression_fails_closed() {
        let ranker = OpportunityRanker::with_default_profile();
        let stored = record("Solo", 5_000, Some(5_000), 10_000);
        let health = BTreeMap::new();
        assert_eq!(
            ranker.rank(&[stored], &policy(), 9_999, &health),
            Err(OpportunityRankingError::ClockBeforeObservation)
        );
    }
}

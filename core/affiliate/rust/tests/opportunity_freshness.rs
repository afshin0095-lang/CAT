//! Integration coverage for the freshness policy contract.
//!
//! Property-style, table-driven: threshold boundaries, monotonicity, and
//! compatibility with the two-threshold lifecycle policy.

use cat_affiliate::{
    canonical_key, DiscoveryCandidate, DiscoveryOpportunity, FreshnessPolicy, FreshnessPolicyError,
    FreshnessState, InMemoryOpportunityStore, OpportunityLifecyclePolicy, OpportunityLifecycleState,
    OpportunityRecord, OpportunityStore,
};
use uuid::Uuid;

fn record(observed_at_ms: u64) -> OpportunityRecord {
    let candidate = DiscoveryCandidate {
        source: "network-a".into(),
        external_id: "sku-1".into(),
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
        observed_at_ms,
    };
    let opportunity = DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score: 8_000, rank: 1 };
    let mut store = InMemoryOpportunityStore::new();
    store.upsert(opportunity).expect("valid opportunity");
    store.list().into_iter().next().expect("record exists")
}

#[test]
fn table_driven_threshold_boundaries_are_exact() {
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let cases: &[(u64, FreshnessState, bool)] = &[
        (0, FreshnessState::Active, true),
        (1_000, FreshnessState::Active, true),
        (1_001, FreshnessState::Active, false),
        (4_999, FreshnessState::Active, false),
        (5_000, FreshnessState::Stale, false),
        (8_999, FreshnessState::Stale, false),
        (9_000, FreshnessState::Expired, false),
        (u64::MAX, FreshnessState::Expired, false),
    ];
    for &(age, expected_state, expected_fresh) in cases {
        let evaluation = policy.evaluate_age(age);
        assert_eq!(evaluation.state, expected_state, "state at age {age}");
        assert_eq!(evaluation.is_fresh, expected_fresh, "freshness at age {age}");
    }
}

#[test]
fn state_is_monotonic_in_age() {
    let policy = FreshnessPolicy::new(10, 100, 200).expect("valid policy");
    let order = [FreshnessState::Active, FreshnessState::Stale, FreshnessState::Expired];
    let mut previous_rank = 0;
    for age in 0..=300u64 {
        let state = policy.evaluate_age(age).state;
        let rank = order.iter().position(|candidate| *candidate == state).expect("known state");
        assert!(rank >= previous_rank, "freshness regressed at age {age}");
        previous_rank = rank;
    }
}

#[test]
fn every_record_evaluation_is_repeatable_and_side_effect_free() {
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let stored = record(1_000_000);
    let before = stored.clone();
    for evaluation_time in [1_000_000, 1_001_000, 1_005_000, 1_009_000] {
        let first = policy.evaluate(&stored, evaluation_time).expect("valid evaluation");
        let second = policy.evaluate(&stored, evaluation_time).expect("valid evaluation");
        assert_eq!(first, second, "evaluation at {evaluation_time} must be stable");
        assert_eq!(stored, before, "evaluation must never mutate the record");
    }
}

#[test]
fn clock_regression_is_always_rejected_regardless_of_thresholds() {
    for stale in [1u64, 1_000, 86_400_000] {
        let policy = FreshnessPolicy::new(stale, stale + 1, stale + 2).expect("valid policy");
        let stored = record(10_000);
        for evaluation_time in [0, 9_999] {
            assert_eq!(
                policy.evaluate(&stored, evaluation_time),
                Err(FreshnessPolicyError::ClockBeforeObservation),
                "regression at evaluation_time={evaluation_time} stale={stale}"
            );
        }
    }
}

#[test]
fn lifecycle_policy_is_the_two_threshold_view_of_freshness() {
    // Every valid two-threshold lifecycle policy maps to a valid freshness
    // policy with identical state semantics.
    for stale in [1u64, 60_000, 3_600_000] {
        for expire in [stale + 1, stale * 10, stale * 1_000] {
            let lifecycle = OpportunityLifecyclePolicy::new(stale, expire).expect("valid lifecycle policy");
            let freshness = FreshnessPolicy::from(lifecycle);
            let stored = record(10_000);
            let evaluator = OpportunityLifecyclePolicy::new(stale, expire).expect("valid policy");
            let evaluator = cat_affiliate::OpportunityLifecycleEvaluator::new(evaluator);

            for age in [0, stale.saturating_sub(1), stale, expire.saturating_sub(1), expire, expire * 2] {
                let lifecycle_state = evaluator
                    .evaluate(&stored, 10_000 + age)
                    .expect("valid evaluation")
                    .state;
                let freshness_state = freshness.evaluate_age(age).state;
                assert_eq!(
                    OpportunityLifecycleState::from(freshness_state),
                    lifecycle_state,
                    "vocabularies disagree at age {age} (stale={stale}, expire={expire})"
                );
            }
        }
    }
}

#[test]
fn policy_serialization_round_trips() {
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let encoded = serde_json::to_string(&policy).expect("serialize");
    let decoded: FreshnessPolicy = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, policy);
}

#[test]
fn impossible_policies_are_rejected() {
    assert_eq!(
        FreshnessPolicy::new(2_000, 1_000, 3_000),
        Err(FreshnessPolicyError::ActiveBeyondStale)
    );
    assert_eq!(
        FreshnessPolicy::new(1_000, 3_000, 2_000),
        Err(FreshnessPolicyError::StaleBeyondExpiration)
    );
    assert_eq!(
        FreshnessPolicy::new(1_000, 2_000, 2_000),
        Err(FreshnessPolicyError::StaleBeyondExpiration)
    );
    assert_eq!(
        FreshnessPolicy::new(0, cat_affiliate::opportunity_freshness::MAX_THRESHOLD_MS + 1, cat_affiliate::opportunity_freshness::MAX_THRESHOLD_MS + 2),
        Err(FreshnessPolicyError::ThresholdTooLarge)
    );
}

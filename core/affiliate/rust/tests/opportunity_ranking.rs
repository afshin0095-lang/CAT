//! Integration coverage for deterministic ranking over real store records.

use cat_affiliate::{
    canonical_key, DiscoveryCandidate, DiscoveryOpportunity, FreshnessPolicy, InMemoryOpportunityStore,
    OpportunityRanker, OpportunityRankingError, OpportunityRankingProfile, OpportunityStore, RankedOpportunity,
    SourceHealthSnapshot, SourceHealthState, WEIGHT_SCALE,
};
use std::collections::BTreeMap;
use uuid::Uuid;

fn record(product: &str, score: u32, observed_at_ms: u64) -> cat_affiliate::OpportunityRecord {
    record_with_commission(product, score, observed_at_ms, 5_000)
}

fn record_with_commission(product: &str, score: u32, observed_at_ms: u64, commission_bps: u32) -> cat_affiliate::OpportunityRecord {
    let candidate = DiscoveryCandidate {
        source: "network-a".into(),
        external_id: format!("sku-{product}"),
        merchant_name: "Acme".into(),
        product_name: product.into(),
        canonical_key: canonical_key("Acme", product),
        category: None,
        destination_url: "https://example.test/item".into(),
        currency: "EUR".into(),
        price_minor: Some(9_999),
        commission_bps: Some(commission_bps),
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

#[test]
fn ranking_orders_by_score_then_identity_and_never_reorders_on_repeat() {
    let ranker = OpportunityRanker::with_default_profile();
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let health = BTreeMap::new();
    let records = vec![
        record("Zulu", 7_000, 10_000),
        record("Bravo", 7_000, 10_000),
        record("Echo", 9_500, 10_000),
    ];

    let first = ranker.rank(&records, &policy, 10_500, &health).expect("valid");
    let second = ranker.rank(&records, &policy, 10_500, &health).expect("valid");
    assert_eq!(first, second, "ranking is deterministic");

    let identities: Vec<&str> = first.iter().map(|entry| entry.identity.as_str()).collect();
    assert_eq!(identities, vec!["acme:echo", "acme:bravo", "acme:zulu"]);
    let ranks: Vec<u32> = first.iter().map(|entry| entry.rank).collect();
    assert_eq!(ranks, vec![1, 2, 3]);
}

#[test]
fn source_reliability_reorders_equal_composites() {
    let ranker = OpportunityRanker::with_default_profile();
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");

    let mut healthy_source = record("Reliable", 7_000, 10_000);
    healthy_source.best_source = "reliable".into();
    let mut flaky_source = record("Flaky", 7_000, 10_000);
    flaky_source.best_source = "flaky".into();

    let mut health = BTreeMap::new();
    health.insert(
        "reliable".to_string(),
        SourceHealthSnapshot {
            source: "reliable".into(),
            state: SourceHealthState::Healthy,
            availability_bps: Some(10_000),
            ..SourceHealthSnapshot::default()
        },
    );
    health.insert(
        "flaky".to_string(),
        SourceHealthSnapshot {
            source: "flaky".into(),
            state: SourceHealthState::Unavailable,
            availability_bps: Some(0),
            ..SourceHealthSnapshot::default()
        },
    );

    let ranked = ranker
        .rank(&[flaky_source, healthy_source], &policy, 10_500, &health)
        .expect("valid");
    assert_eq!(ranked[0].identity, "acme:reliable");
    assert!(ranked[0].score > ranked[1].score);
}

#[test]
fn scores_stay_within_the_declared_scale_at_boundaries() {
    let ranker = OpportunityRanker::with_default_profile();
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let health = BTreeMap::new();

    let top = record_with_commission("Top", WEIGHT_SCALE, 10_000, WEIGHT_SCALE);
    let bottom = record_with_commission("Bottom", 0, 10_000, 0);
    let ranked = ranker.rank(&[top, bottom], &policy, 10_500, &health).expect("valid");
    for entry in &ranked {
        assert!(entry.score <= WEIGHT_SCALE, "score escaped the declared scale");
    }
    assert_eq!(ranked[0].score, WEIGHT_SCALE, "perfect inputs produce the maximum score");
}

#[test]
fn invalid_profiles_fail_closed_before_any_ranking() {
    let profile = OpportunityRankingProfile {
        composite_score_weight_bps: 10_000,
        ..OpportunityRankingProfile::default()
    };
    assert_eq!(
        OpportunityRanker::new(profile),
        Err(OpportunityRankingError::InvalidProfile { total: 18_000 })
    );
}

#[test]
fn ranked_entries_serialize_for_downstream_consumption() {
    let ranker = OpportunityRanker::with_default_profile();
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let health = BTreeMap::new();
    let records = vec![record("Solo", 8_000, 10_000)];
    let ranked: Vec<RankedOpportunity> = ranker.rank(&records, &policy, 10_500, &health).expect("valid");
    let encoded = serde_json::to_string(&ranked).expect("serialize");
    let decoded: Vec<RankedOpportunity> = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, ranked);
}

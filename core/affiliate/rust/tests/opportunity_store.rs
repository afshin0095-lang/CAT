use cat_affiliate::{
    DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityIdentity,
    OpportunityStore, canonical_key,
};
use uuid::Uuid;

fn opportunity(
    source: &str,
    external_id: &str,
    score: u32,
    observed_at_ms: u64,
) -> DiscoveryOpportunity {
    let candidate = DiscoveryCandidate {
        source: source.into(),
        external_id: external_id.into(),
        merchant_name: "Acme".into(),
        product_name: "Widget Pro".into(),
        canonical_key: canonical_key("Acme", "Widget Pro"),
        category: Some("electronics".into()),
        destination_url: format!("https://example.test/{external_id}"),
        currency: "EUR".into(),
        price_minor: Some(10_000),
        commission_bps: Some(500),
        demand_score: 8_000,
        competition_score: 3_000,
        freshness_score: 9_000,
        compliance_score: 10_000,
        observed_at_ms,
    };
    DiscoveryOpportunity {
        id: Uuid::now_v7(),
        candidate,
        score,
        rank: 1,
    }
}

#[test]
fn same_identity_from_two_sources_is_one_record_with_provenance() {
    let mut store = InMemoryOpportunityStore::new();
    assert!(
        store
            .upsert(opportunity("network-a", "a-1", 7_000, 100))
            .unwrap()
            .created
    );
    let result = store
        .upsert(opportunity("network-b", "b-9", 9_000, 200))
        .unwrap();
    assert!(!result.created);
    assert!(result.changed);

    let identity = OpportunityIdentity::new(&opportunity("network-c", "c-1", 1_000, 50).candidate);
    let record = store.get(&identity).unwrap();
    assert_eq!(store.list().len(), 1);
    assert_eq!(record.observations.len(), 2);
    assert_eq!(record.best_source, "network-b");
    assert_eq!(record.best_score, 9_000);
    assert_eq!(record.first_observed_at_ms, 100);
    assert_eq!(record.last_observed_at_ms, 200);
}

#[test]
fn repeated_identical_observation_is_idempotent() {
    let mut store = InMemoryOpportunityStore::new();
    let first = opportunity("network-a", "a-1", 7_000, 100);
    assert!(store.upsert(first.clone()).unwrap().created);
    let second = store.upsert(first).unwrap();
    assert!(!second.created);
    assert!(!second.changed);
    assert_eq!(store.list().len(), 1);
}

#[test]
fn equal_best_scores_use_deterministic_source_tiebreak() {
    let mut store = InMemoryOpportunityStore::new();
    store
        .upsert(opportunity("zeta", "z-1", 8_000, 100))
        .unwrap();
    store
        .upsert(opportunity("alpha", "a-1", 8_000, 200))
        .unwrap();
    let identity = OpportunityIdentity::new(&opportunity("x", "x-1", 1, 1).candidate);
    assert_eq!(store.get(&identity).unwrap().best_source, "alpha");
}

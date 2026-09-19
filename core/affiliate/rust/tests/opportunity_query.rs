//! Integration coverage for the persistence-neutral query boundary.

use cat_affiliate::{
    DiscoveryCandidate, DiscoveryOpportunity, FreshnessPolicy, FreshnessState,
    InMemoryOpportunityStore, OpportunityFilter, OpportunityQuery, OpportunityQueryService,
    OpportunitySort, OpportunitySortField, OpportunityStore, SortDirection, canonical_key,
};
use uuid::Uuid;

fn build(product: &str, merchant: &str, score: u32, observed_at_ms: u64) -> OpportunityFilterMatch {
    let candidate = DiscoveryCandidate {
        source: "network-a".into(),
        external_id: format!("sku-{product}"),
        merchant_name: merchant.into(),
        product_name: product.into(),
        canonical_key: canonical_key(merchant, product),
        category: Some("electronics".into()),
        destination_url: "https://example.test/item".into(),
        currency: "EUR".into(),
        price_minor: Some(5_000),
        commission_bps: Some(400),
        demand_score: 8_000,
        competition_score: 2_000,
        freshness_score: 10_000,
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

type OpportunityFilterMatch = cat_affiliate::DiscoveryOpportunity;

fn store_with(
    opportunities: Vec<cat_affiliate::DiscoveryOpportunity>,
) -> Vec<cat_affiliate::OpportunityRecord> {
    let mut store = InMemoryOpportunityStore::new();
    for opportunity in opportunities {
        store.upsert(opportunity).expect("valid opportunity");
    }
    store.list()
}

#[test]
fn end_to_end_filter_sort_and_paginate() {
    let records = store_with(vec![
        build("Alpha", "Acme", 9_000, 10_000),
        build("Beta", "Beta", 7_000, 10_000),
        build("Gamma", "Beta", 7_000, 12_000),
    ]);
    let service = OpportunityQueryService::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );

    let filter = OpportunityFilter {
        merchant: Some("Beta".into()),
        ..Default::default()
    };
    let query = OpportunityQuery::new(filter).with_limit(1);
    let page = service
        .execute(&records, &query, 12_500)
        .expect("valid query");
    assert_eq!(page.total_matched, 2);
    assert_eq!(page.items.len(), 1);
    assert!(page.has_more());
    // Score tie between Beta/Gamma resolved by identity ascending.
    assert_eq!(page.items[0].identity, "beta:beta");

    let filter = OpportunityFilter {
        merchant: Some("Beta".into()),
        ..Default::default()
    };
    let query = OpportunityQuery::new(filter)
        .with_limit(1)
        .with_offset(1)
        .with_sort(OpportunitySort {
            field: OpportunitySortField::LastObservedAt,
            direction: SortDirection::Descending,
        });
    let page = service
        .execute(&records, &query, 12_500)
        .expect("valid query");
    assert_eq!(
        page.items[0].identity,
        "beta:beta",
        "offset 1 selects the second item after the most recently observed first"
    );
}

#[test]
fn lifecycle_filter_uses_the_derived_state_not_storage() {
    let records = store_with(vec![
        build("Fresh", "Acme", 9_000, 12_000),
        build("Old", "Acme", 9_000, 10_000),
    ]);
    let service = OpportunityQueryService::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );

    // At 15_500: "Fresh" is 3_500ms old (Active), "Old" is 5_500ms old
    // (past the 5_000ms stale threshold).
    let filter = OpportunityFilter {
        lifecycle_state: Some(FreshnessState::Stale),
        ..Default::default()
    };
    let query = OpportunityQuery::new(filter).with_limit(10);
    let result = service
        .execute(&records, &query, 15_500)
        .expect("valid query");
    assert_eq!(result.total_matched, 1);
    assert_eq!(
        result.items[0].identity, "acme:old",
        "5.5s-old observation is stale with a 5s stale threshold"
    );
}

#[test]
fn empty_result_sets_are_stable() {
    let records = store_with(vec![build("Alpha", "Acme", 9_000, 10_000)]);
    let service = OpportunityQueryService::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );
    let filter = OpportunityFilter {
        merchant: Some("Nobody".into()),
        ..Default::default()
    };
    let query = OpportunityQuery::new(filter).with_limit(10);
    let result = service
        .execute(&records, &query, 10_500)
        .expect("valid query");
    assert_eq!(result.total_matched, 0);
    assert!(result.items.is_empty());
    assert!(!result.has_more());
}

//! Integration coverage: planner -> request -> store pipeline, plus the
//! Sprint 1 seam (requests are intents; no network calls happen here).

use cat_affiliate::{
    DiscoveryCandidate, DiscoveryOpportunity, FreshnessPolicy, InMemoryOpportunityStore,
    InMemoryRevalidationRequestStore, OpportunityRecord, OpportunityStore, RevalidationPlanner,
    RevalidationPriority, RevalidationReason, RevalidationRequestStore, RevalidationStatus,
    canonical_key,
};
use std::collections::BTreeMap;
use uuid::Uuid;

fn record(product: &str, sources: &[&str], observed_at_ms: u64) -> OpportunityRecord {
    let mut store = InMemoryOpportunityStore::new();
    for (index, source) in sources.iter().enumerate() {
        let candidate = DiscoveryCandidate {
            source: (*source).into(),
            external_id: format!("sku-{index}"),
            merchant_name: "Acme".into(),
            product_name: product.into(),
            canonical_key: canonical_key("Acme", product),
            category: None,
            destination_url: "https://example.test/item".into(),
            currency: "EUR".into(),
            price_minor: Some(9_999),
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
        store.upsert(opportunity).expect("valid opportunity");
    }
    store.list().into_iter().next().expect("record exists")
}

#[test]
fn planned_requests_persist_and_claim_deterministically() {
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let planner = RevalidationPlanner::new(policy);
    let mut store = InMemoryRevalidationRequestStore::new();

    // At now = 20_000: 6_000ms old => Stale (past 5_000ms stale threshold);
    // 12_000ms old => Expired (past 9_000ms expiration threshold).
    let stale = record("Stale Widget", &["network-a"], 14_000);
    let expired = record("Expired Widget", &["network-b"], 8_000);
    let stale_decision = planner
        .plan_for_record(&stale, 20_000, &["network-a".into()])
        .expect("plan");
    let expired_decision = planner
        .plan_for_record(&expired, 20_000, &["network-b".into()])
        .expect("plan");
    assert_eq!(
        stale_decision.requests[0].priority,
        RevalidationPriority::High
    );
    assert_eq!(
        expired_decision.requests[0].priority,
        RevalidationPriority::Critical
    );

    // Persist all requests; dedup keys converge across a second planning pass.
    let mut keys = Vec::new();
    for decision in [&stale_decision, &expired_decision] {
        for request in &decision.requests {
            let record = store.insert(request.clone()).expect("insert");
            assert_eq!(record.status, RevalidationStatus::Pending);
            keys.push(record.request.dedup_key.clone());
        }
    }
    // Re-planning produces the same keys (idempotency identity).
    let replay_stale = planner
        .plan_for_record(&stale, 20_000, &["network-a".into()])
        .expect("plan");
    for request in &replay_stale.requests {
        assert!(keys.contains(&request.dedup_key));
        assert!(matches!(
            store.insert(request.clone()),
            Err(cat_affiliate::RevalidationStoreError::DuplicateRequest { .. })
        ));
    }

    // Critical expires first when claiming.
    let claimed = store.claim_due(20_000, 10);
    assert_eq!(claimed.len(), 2);
    assert_eq!(claimed[0].request.reason, RevalidationReason::Expired);
    assert_eq!(claimed[0].status, RevalidationStatus::Claimed);
}

#[test]
fn batch_planning_is_stable_and_reports_global_counts() {
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let planner = RevalidationPlanner::new(policy);
    let mut availability = BTreeMap::new();
    availability.insert("network-a".to_string(), true);
    availability.insert("network-b".to_string(), false);

    let first = record("Alpha", &["network-a"], 10_000);
    let second = record("Beta", &["network-a"], 10_000);
    let records = vec![second.clone(), first.clone()];

    let outcome = cat_affiliate::RevalidationPlanningOutcome::plan_batch(
        &planner,
        &records,
        20_000,
        &availability,
    )
    .expect("valid batch");
    assert_eq!(outcome.decisions.len(), 2);
    assert!(outcome.decisions[0].identity < outcome.decisions[1].identity);
    assert_eq!(outcome.requested, 2);
    assert_eq!(outcome.blocked, 0);
    assert_eq!(outcome.unique_requests().len(), 2);

    // Input order never changes the output order.
    let flipped = cat_affiliate::RevalidationPlanningOutcome::plan_batch(
        &planner,
        &[first, second],
        20_000,
        &availability,
    )
    .expect("valid batch");
    assert_eq!(outcome.decisions, flipped.decisions);
}

#[test]
fn planning_never_touches_the_network_or_the_record() {
    // The planner's only inputs are domain values; this test pins the
    // contract that planning is pure: same inputs, same output, no side
    // effects on the record.
    let policy = FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy");
    let planner = RevalidationPlanner::new(policy);
    let stored = record("Widget", &["network-a"], 10_000);
    let before = stored.clone();

    let now = 20_000;
    let first = planner
        .plan_for_record(&stored, now, &["network-a".to_owned()])
        .expect("plan");
    let second = planner
        .plan_for_record(&stored, now, &["network-a".to_owned()])
        .expect("plan");
    assert_eq!(stored, before);
    assert_eq!(first.requests.len(), second.requests.len());
    for (left, right) in first.requests.iter().zip(second.requests.iter()) {
        assert_eq!(left.dedup_key, right.dedup_key);
        assert_eq!(left.target, right.target);
        assert_eq!(left.reason, right.reason);
        assert_eq!(left.priority, right.priority);
    }
}

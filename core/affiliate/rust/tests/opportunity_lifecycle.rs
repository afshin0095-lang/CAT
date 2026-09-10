use cat_affiliate::{
    canonical_key, DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore,
    OpportunityLifecycleEvaluator, OpportunityLifecyclePolicy, OpportunityLifecycleState,
    OpportunityStore,
};
use uuid::Uuid;

fn candidate(observed_at_ms: u64) -> DiscoveryCandidate {
    DiscoveryCandidate {
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
    }
}

fn record(observed_at_ms: u64) -> cat_affiliate::OpportunityRecord {
    let opportunity = DiscoveryOpportunity {
        id: Uuid::now_v7(),
        candidate: candidate(observed_at_ms),
        score: 8_000,
        rank: 1,
    };
    let mut store = InMemoryOpportunityStore::new();
    store.upsert(opportunity).expect("valid opportunity");
    store.list().into_iter().next().expect("record exists")
}

#[test]
fn lifecycle_transitions_at_exact_thresholds() {
    let policy = OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy");
    let evaluator = OpportunityLifecycleEvaluator::new(policy);
    let stored = record(10_000);

    assert_eq!(evaluator.evaluate(&stored, 10_999).unwrap().state, OpportunityLifecycleState::Active);
    assert_eq!(evaluator.evaluate(&stored, 11_000).unwrap().state, OpportunityLifecycleState::Stale);
    assert_eq!(evaluator.evaluate(&stored, 14_999).unwrap().state, OpportunityLifecycleState::Stale);
    assert_eq!(evaluator.evaluate(&stored, 15_000).unwrap().state, OpportunityLifecycleState::Expired);
}

#[test]
fn future_clock_is_rejected_instead_of_wrapping_age() {
    let policy = OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy");
    let evaluator = OpportunityLifecycleEvaluator::new(policy);
    let stored = record(10_000);

    assert!(matches!(
        evaluator.evaluate(&stored, 9_999),
        Err(cat_affiliate::OpportunityLifecycleError::ClockBeforeObservation)
    ));
}

#[test]
fn invalid_policy_is_rejected() {
    assert!(OpportunityLifecyclePolicy::new(0, 5_000).is_err());
    assert!(OpportunityLifecyclePolicy::new(5_000, 5_000).is_err());
    assert!(OpportunityLifecyclePolicy::new(6_000, 5_000).is_err());
}

#[test]
fn batch_evaluation_is_sorted_by_stable_identity() {
    let policy = OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy");
    let evaluator = OpportunityLifecycleEvaluator::new(policy);
    let first = record(10_000);
    let second = {
        let mut candidate = candidate(10_000);
        candidate.merchant_name = "Beta".into();
        candidate.canonical_key = canonical_key("Beta", "Widget");
        let opportunity = DiscoveryOpportunity {
            id: Uuid::now_v7(),
            candidate,
            score: 7_000,
            rank: 1,
        };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store.list().into_iter().next().expect("record exists")
    };

    let snapshots = cat_affiliate::opportunity_lifecycle::evaluate_records(
        &[second, first], evaluator, 10_500,
    )
    .expect("valid evaluation");

    assert_eq!(snapshots.len(), 2);
    assert!(snapshots[0].0 < snapshots[1].0);
}

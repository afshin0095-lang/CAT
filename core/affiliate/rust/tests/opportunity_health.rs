//! Integration coverage: health and lifecycle are independent dimensions.

use cat_affiliate::{
    DiscoveryCandidate, DiscoveryOpportunity, FreshnessEvaluation, FreshnessState, HealthConcern,
    InMemoryOpportunityStore, InMemorySourceHealthStore, OpportunityHealth,
    OpportunityHealthAssessment, OpportunityHealthAssessor, OpportunityRecord, OpportunityStore,
    SourceHealthState, SourceHealthStore, canonical_key,
};
use uuid::Uuid;

fn record() -> OpportunityRecord {
    let candidate = DiscoveryCandidate {
        source: "network-a".into(),
        external_id: "sku-1".into(),
        merchant_name: "Acme".into(),
        product_name: "Widget".into(),
        canonical_key: canonical_key("Acme", "Widget"),
        category: None,
        destination_url: "https://example.test/widget".into(),
        currency: "EUR".into(),
        price_minor: Some(1_000),
        commission_bps: Some(500),
        demand_score: 8_000,
        competition_score: 2_000,
        freshness_score: 10_000,
        compliance_score: 10_000,
        observed_at_ms: 1_000,
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

fn freshness(state: FreshnessState) -> FreshnessEvaluation {
    FreshnessEvaluation {
        state,
        age_ms: 100,
        is_fresh: state == FreshnessState::Active,
    }
}

#[test]
fn the_full_matrix_of_lifecycle_x_health_is_representable() {
    let assessor = OpportunityHealthAssessor;
    let stored = record();
    let mut health_store = InMemorySourceHealthStore::new();
    for _ in 0..cat_affiliate::DEGRADED_AFTER_CONSECUTIVE_FAILURES {
        health_store.record_failure("network-a", 2_000, Some("slow"));
    }
    let degraded = health_store.snapshot("network-a");
    assert_eq!(degraded.state, SourceHealthState::Degraded);

    // Active + Degraded: fresh observation, struggling source.
    let assessment: OpportunityHealthAssessment = assessor.assess(
        &stored,
        &freshness(FreshnessState::Active),
        Some(&degraded),
        false,
    );
    assert_eq!(assessment.health, OpportunityHealth::Degraded);
    assert!(matches!(
        assessment.concerns.as_slice(),
        [HealthConcern::BestSourceDegraded { .. }]
    ));

    // Stale + Healthy: old observation, perfectly fine source.
    let assessment = assessor.assess(&stored, &freshness(FreshnessState::Stale), None, false);
    assert_eq!(assessment.health, OpportunityHealth::Healthy);

    // Expired + Healthy: expired observation, healthy source (revalidation
    // will fix the age dimension, not the health dimension).
    let assessment = assessor.assess(&stored, &freshness(FreshnessState::Expired), None, true);
    assert_eq!(assessment.health, OpportunityHealth::NeedsRevalidation);
    assert!(assessment.needs_revalidation);
}

#[test]
fn health_assessments_serialize_for_observers() {
    let assessor = OpportunityHealthAssessor;
    let stored = record();
    let assessment = assessor.assess(&stored, &freshness(FreshnessState::Active), None, true);
    let encoded = serde_json::to_string(&assessment).expect("serialize");
    let decoded: OpportunityHealthAssessment = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, assessment);
}

#[test]
fn degraded_source_snapshots_come_from_the_health_store() {
    let mut health_store = InMemorySourceHealthStore::new();
    for at in 0..cat_affiliate::UNAVAILABLE_AFTER_CONSECUTIVE_FAILURES {
        health_store.record_failure("network-a", 1_000 + u64::from(at), None);
    }
    let snapshot = health_store.snapshot("network-a");
    assert_eq!(snapshot.state, SourceHealthState::Unavailable);

    let assessor = OpportunityHealthAssessor;
    let stored = record();
    let assessment = assessor.assess(
        &stored,
        &freshness(FreshnessState::Active),
        Some(&snapshot),
        false,
    );
    assert_eq!(assessment.health, OpportunityHealth::Unavailable);
}

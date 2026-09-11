//! Integration coverage: the status projection read model over persisted
//! records, including the health + lifecycle independence guarantees.

use cat_affiliate::{
    DiscoveryCandidate, DiscoveryOpportunity, FreshnessPolicy, FreshnessState,
    InMemoryOpportunityStore, OpportunityHealth, OpportunityRecord, OpportunityRevision,
    OpportunityStatusProjector, OpportunityStore, RevalidationBlockReason, canonical_key,
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
        price_minor: Some(12_345),
        commission_bps: Some(750),
        demand_score: 8_000,
        competition_score: 2_000,
        freshness_score: 10_000,
        compliance_score: 10_000,
        observed_at_ms,
    };
    let opportunity = DiscoveryOpportunity {
        id: Uuid::now_v7(),
        candidate,
        score: 8_800,
        rank: 1,
    };
    let mut store = InMemoryOpportunityStore::new();
    store.upsert(opportunity).expect("valid opportunity");
    store.list().into_iter().next().expect("record exists")
}

#[test]
fn view_tracks_lifecycle_transitions_end_to_end() {
    let projector = OpportunityStatusProjector::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );
    let stored = record(1_000_000);

    let active = projector.project(&stored, 1_000_500).expect("valid");
    assert_eq!(active.lifecycle_state, FreshnessState::Active);
    assert!(!active.needs_revalidation);
    assert_eq!(active.health, OpportunityHealth::Healthy);

    let stale = projector.project(&stored, 1_006_000).expect("valid");
    assert_eq!(stale.lifecycle_state, FreshnessState::Stale);
    assert!(stale.needs_revalidation);
    assert_eq!(stale.revalidation_reasons, vec!["stale"]);
    assert_eq!(stale.health, OpportunityHealth::NeedsRevalidation);

    let expired = projector.project(&stored, 1_010_000).expect("valid");
    assert_eq!(expired.lifecycle_state, FreshnessState::Expired);
    assert!(expired.needs_revalidation);
    assert_eq!(expired.revalidation_reasons, vec!["expired"]);
}

#[test]
fn view_exposes_revision_for_optimistic_concurrency() {
    let mut store = InMemoryOpportunityStore::new();
    let candidate = DiscoveryCandidate {
        source: "network-a".into(),
        external_id: "sku-1".into(),
        merchant_name: "Acme".into(),
        product_name: "Widget".into(),
        canonical_key: canonical_key("Acme", "Widget"),
        category: None,
        destination_url: "https://example.test/widget".into(),
        currency: "EUR".into(),
        price_minor: None,
        commission_bps: None,
        demand_score: 5_000,
        competition_score: 5_000,
        freshness_score: 5_000,
        compliance_score: 5_000,
        observed_at_ms: 1_000,
    };
    let opportunity = DiscoveryOpportunity {
        id: Uuid::now_v7(),
        candidate,
        score: 5_000,
        rank: 1,
    };
    let created = store.upsert(opportunity).expect("upsert");
    assert_eq!(created.revision, OpportunityRevision::initial());
    let stored = store.list().into_iter().next().expect("record");

    let projector = OpportunityStatusProjector::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );
    let view = projector.project(&stored, 1_500).expect("valid");
    assert_eq!(view.revision, OpportunityRevision::initial());
}

#[test]
fn blocked_revalidation_is_visible_in_the_view() {
    let projector = OpportunityStatusProjector::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );
    let stored = record(1_000_000);
    let view = projector
        .project_with_availability(&stored, 1_006_000, &[])
        .expect("valid");
    assert_eq!(view.lifecycle_state, FreshnessState::Stale);
    assert!(view.needs_revalidation, "stale always needs revalidation");
    assert!(
        view.revalidation_reasons.is_empty(),
        "no request could be planned"
    );
    assert_eq!(
        view.revalidation_blocked,
        Some(RevalidationBlockReason::AllSourcesUnavailable)
    );
}

#[test]
fn views_serialize_for_read_model_transport() {
    let projector = OpportunityStatusProjector::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );
    let stored = record(1_000_000);
    let view = projector.project(&stored, 1_001_000).expect("valid");
    let encoded = serde_json::to_string(&view).expect("serialize");
    let decoded: cat_affiliate::OpportunityStatusView =
        serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded, view);
}

#[test]
fn projection_fails_closed_on_clock_regression() {
    let projector = OpportunityStatusProjector::new(
        FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"),
    );
    let stored = record(1_000_000);
    assert_eq!(
        projector.project(&stored, 999_999),
        Err(cat_affiliate::OpportunityProjectionError::ClockBeforeObservation)
    );
}

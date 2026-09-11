//! Integration coverage: domain results convert into neutral metric samples
//! without coupling the domain to any telemetry backend.

use cat_affiliate::{
    DiscoveryCandidate, DiscoveryEngine, DiscoveryOpportunity, DiscoveryRequest, FreshnessPolicy,
    InMemoryMetricSink, InMemoryOpportunityStore, LifecycleStateCounts, MetricSink,
    OpportunityIngestionReport, OpportunityLifecycleEvaluator, OpportunityLifecyclePolicy,
    OpportunityStore, RevalidationStatus, ingestion_report_metrics, lifecycle_batch_metrics,
    metric_names, revalidation_status_metrics,
};
use uuid::Uuid;

fn sample_batch() -> cat_affiliate::LifecycleEvaluationBatch {
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
        observed_at_ms: 10_000,
    };
    let opportunity = DiscoveryOpportunity {
        id: Uuid::now_v7(),
        candidate,
        score: 8_000,
        rank: 1,
    };
    let mut store = InMemoryOpportunityStore::new();
    store.upsert(opportunity).expect("valid opportunity");
    let records = store.list();

    let policy = OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy");
    let evaluator = OpportunityLifecycleEvaluator::new(policy);
    cat_affiliate::evaluate_records_batch(&records, evaluator, 10_500).expect("valid batch")
}

fn canonical_key(merchant: &str, product: &str) -> String {
    cat_affiliate::canonical_key(merchant, product)
}

#[test]
fn a_full_ingestion_to_metrics_round_trip() {
    // 1. Discovery produces deterministic rejections and opportunities.
    let valid = DiscoveryCandidate {
        source: "network-a".into(),
        external_id: "good".into(),
        merchant_name: "Acme".into(),
        product_name: "Widget".into(),
        canonical_key: canonical_key("Acme", "Widget"),
        category: None,
        destination_url: "https://example.test/widget".into(),
        currency: "EUR".into(),
        price_minor: Some(1_000),
        commission_bps: Some(5_000),
        demand_score: 9_000,
        competition_score: 1_000,
        freshness_score: 9_000,
        compliance_score: 10_000,
        observed_at_ms: 10,
    };
    let mut invalid = valid.clone();
    invalid.destination_url = "not-a-url".into();

    let result = DiscoveryEngine
        .discover(DiscoveryRequest {
            candidates: vec![valid.clone(), invalid, valid],
            min_score: 0,
            limit: 10,
        })
        .expect("valid request");
    assert_eq!(result.rejected, 1);
    assert_eq!(result.opportunities.len(), 2);

    // 2. Persistence produces the deterministic report shape.
    let mut report = OpportunityIngestionReport::default();
    report.candidates_received = 3;
    report.candidates_rejected = 1;
    report.opportunities_discovered = 2;
    report.opportunities_created = 1;
    report.opportunities_changed = 2;

    // 3. The neutral sink aggregates everything by name.
    let mut sink = InMemoryMetricSink::new();
    for sample in ingestion_report_metrics(&report) {
        sink.record(sample);
    }
    let batch = sample_batch();
    for sample in lifecycle_batch_metrics(&batch) {
        sink.record(sample);
    }
    for sample in revalidation_status_metrics(RevalidationStatus::Succeeded, 3) {
        sink.record(sample);
    }

    let totals = sink.totals();
    assert_eq!(
        totals.get(metric_names::DISCOVERY_CANDIDATES_RECEIVED),
        Some(&3)
    );
    assert_eq!(
        totals.get(metric_names::DISCOVERY_CANDIDATES_REJECTED),
        Some(&1)
    );
    assert_eq!(
        totals.get(metric_names::DISCOVERY_OPPORTUNITIES_CREATED),
        Some(&1)
    );
    assert_eq!(
        totals.get(metric_names::DISCOVERY_OPPORTUNITIES_CHANGED),
        Some(&2)
    );
    assert!(totals.contains_key(metric_names::OPPORTUNITY_ACTIVE));
    assert!(totals.contains_key(metric_names::OPPORTUNITY_REVALIDATION_SUCCEEDED));
    assert_eq!(
        batch.counts,
        LifecycleStateCounts {
            active: 1,
            stale: 0,
            expired: 0
        }
    );
    assert_eq!(batch.evaluated_at_ms, 10_500);
    let policy = FreshnessPolicy::new(1_000, 1_000, 5_000).expect("valid policy");
    assert_eq!(
        policy.evaluate_age(2_000).state,
        cat_affiliate::FreshnessState::Stale
    );
}

//! Neutral observability contract for the Affiliate Opportunity platform.
//!
//! The domain never couples to Prometheus, OTLP, or any concrete telemetry
//! stack. It exposes:
//!
//! - canonical metric name constants (single source of truth for dashboards);
//! - [`MetricSample`], a minimal counter/gauge sample with labels;
//! - [`MetricSink`], a push boundary any backend can implement;
//! - conversion helpers from Sprint 0 domain results to metric samples.
//!
//! Emission is the responsibility of the boundary that *produces the fact*
//! (ingestion runs, revalidation execution, lifecycle detection). Read-only
//! derivations (e.g. a query that computes `Expired`) never emit lifecycle
//! metrics by themselves.

use std::collections::BTreeMap;

use serde::Serialize;

use crate::opportunity_ingestion::OpportunityIngestionReport;
use crate::opportunity_lifecycle::LifecycleEvaluationBatch;
use crate::opportunity_revalidation::RevalidationStatus;

/// Canonical Affiliate Opportunity metric names.
pub mod metric_names {
    pub const DISCOVERY_CANDIDATES_RECEIVED: &str = "affiliate.discovery.candidates_received";
    pub const DISCOVERY_CANDIDATES_REJECTED: &str = "affiliate.discovery.candidates_rejected";
    pub const DISCOVERY_OPPORTUNITIES_CREATED: &str = "affiliate.discovery.opportunities_created";
    pub const DISCOVERY_OPPORTUNITIES_CHANGED: &str = "affiliate.discovery.opportunities_changed";
    pub const DISCOVERY_SOURCE_FAILURES: &str = "affiliate.discovery.source_failures";
    pub const OPPORTUNITY_ACTIVE: &str = "affiliate.opportunity.active";
    pub const OPPORTUNITY_STALE: &str = "affiliate.opportunity.stale";
    pub const OPPORTUNITY_EXPIRED: &str = "affiliate.opportunity.expired";
    pub const OPPORTUNITY_REVALIDATION_REQUESTED: &str =
        "affiliate.opportunity.revalidation_requested";
    pub const OPPORTUNITY_REVALIDATION_SUCCEEDED: &str =
        "affiliate.opportunity.revalidation_succeeded";
    pub const OPPORTUNITY_REVALIDATION_FAILED: &str = "affiliate.opportunity.revalidation_failed";
}

/// Canonical label keys.
pub mod label_keys {
    pub const SOURCE: &str = "source";
    pub const BATCH_ID: &str = "batch_id";
}

/// One neutral metric sample. `name` references a [`metric_names`] constant.
/// Values are unsigned; counters and gauges are distinguished by name
/// convention, exactly like the metric registry of the target backend.
///
/// Samples serialize for forwarding but are not deserializable by design:
/// telemetry ingestion into the domain is not a Sprint 0 boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MetricSample {
    pub name: &'static str,
    pub value: u64,
    pub labels: Vec<(String, String)>,
}

impl MetricSample {
    pub fn counter(name: &'static str, value: u64) -> Self {
        Self {
            name,
            value,
            labels: Vec::new(),
        }
    }

    pub fn with_label(mut self, key: &str, value: impl Into<String>) -> Self {
        self.labels.push((key.to_owned(), value.into()));
        self
    }
}

/// Push boundary for metric backends. Implementations must never panic on
/// unknown metric names; they may drop or log them.
pub trait MetricSink {
    fn record(&mut self, sample: MetricSample);
}

/// In-memory sink for tests and single-process deployments.
#[derive(Clone, Debug, Default)]
pub struct InMemoryMetricSink {
    samples: Vec<MetricSample>,
}

impl InMemoryMetricSink {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.samples.len()
    }

    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    pub fn samples(&self) -> &[MetricSample] {
        &self.samples
    }

    /// Aggregated totals per metric name, ordered by name.
    pub fn totals(&self) -> BTreeMap<String, u64> {
        let mut totals = BTreeMap::new();
        for sample in &self.samples {
            *totals.entry(sample.name.to_owned()).or_insert(0) += sample.value;
        }
        totals
    }
}

impl MetricSink for InMemoryMetricSink {
    fn record(&mut self, sample: MetricSample) {
        self.samples.push(sample);
    }
}

/// Converts an ingestion report into discovery metric samples.
pub fn ingestion_report_metrics(report: &OpportunityIngestionReport) -> Vec<MetricSample> {
    let batch_label = report.batch_id.to_string();
    vec![
        MetricSample::counter(
            metric_names::DISCOVERY_CANDIDATES_RECEIVED,
            report.candidates_received as u64,
        )
        .with_label(label_keys::BATCH_ID, batch_label.clone()),
        MetricSample::counter(
            metric_names::DISCOVERY_CANDIDATES_REJECTED,
            report.candidates_rejected as u64,
        )
        .with_label(label_keys::BATCH_ID, batch_label.clone()),
        MetricSample::counter(
            metric_names::DISCOVERY_OPPORTUNITIES_CREATED,
            report.opportunities_created as u64,
        )
        .with_label(label_keys::BATCH_ID, batch_label.clone()),
        MetricSample::counter(
            metric_names::DISCOVERY_OPPORTUNITIES_CHANGED,
            report.opportunities_changed as u64,
        )
        .with_label(label_keys::BATCH_ID, batch_label.clone()),
        MetricSample::counter(
            metric_names::DISCOVERY_SOURCE_FAILURES,
            report.sources_failed as u64,
        )
        .with_label(label_keys::BATCH_ID, batch_label),
    ]
}

/// Converts a lifecycle batch evaluation into state gauge samples.
///
/// These are *gauge* observations of a derived projection at one instant;
/// they are deliberately distinct from lifecycle events, which only a
/// fact-producing boundary may emit.
pub fn lifecycle_batch_metrics(batch: &LifecycleEvaluationBatch) -> Vec<MetricSample> {
    let label = batch.evaluated_at_ms.to_string();
    vec![
        MetricSample::counter(metric_names::OPPORTUNITY_ACTIVE, batch.counts.active)
            .with_label("evaluated_at_ms", label.clone()),
        MetricSample::counter(metric_names::OPPORTUNITY_STALE, batch.counts.stale)
            .with_label("evaluated_at_ms", label.clone()),
        MetricSample::counter(metric_names::OPPORTUNITY_EXPIRED, batch.counts.expired)
            .with_label("evaluated_at_ms", label),
    ]
}

/// Converts a revalidation request status observation into metric samples.
///
/// Sprint 0 exposes the contract; Sprint 1's durable execution boundary
/// becomes the emission point when attempts actually run.
pub fn revalidation_status_metrics(status: RevalidationStatus, count: u64) -> Vec<MetricSample> {
    match status {
        RevalidationStatus::Pending | RevalidationStatus::Claimed | RevalidationStatus::Running => {
            vec![MetricSample::counter(
                metric_names::OPPORTUNITY_REVALIDATION_REQUESTED,
                count,
            )]
        }
        RevalidationStatus::Succeeded => {
            vec![MetricSample::counter(
                metric_names::OPPORTUNITY_REVALIDATION_SUCCEEDED,
                count,
            )]
        }
        RevalidationStatus::Failed | RevalidationStatus::DeadLettered => {
            vec![MetricSample::counter(
                metric_names::OPPORTUNITY_REVALIDATION_FAILED,
                count,
            )]
        }
        RevalidationStatus::Cancelled => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opportunity_freshness::{FreshnessPolicy, FreshnessState};
    use crate::opportunity_lifecycle::{
        LifecycleEvaluation, LifecycleStateCounts, OpportunityLifecyclePolicy,
        OpportunityLifecycleSnapshot,
    };
    use uuid::Uuid;

    #[test]
    fn metric_names_follow_the_documented_namespace() {
        assert!(metric_names::DISCOVERY_CANDIDATES_RECEIVED.starts_with("affiliate.discovery."));
        assert!(metric_names::OPPORTUNITY_ACTIVE.starts_with("affiliate.opportunity."));
        let names = [
            metric_names::DISCOVERY_CANDIDATES_RECEIVED,
            metric_names::DISCOVERY_CANDIDATES_REJECTED,
            metric_names::DISCOVERY_OPPORTUNITIES_CREATED,
            metric_names::DISCOVERY_OPPORTUNITIES_CHANGED,
            metric_names::DISCOVERY_SOURCE_FAILURES,
            metric_names::OPPORTUNITY_ACTIVE,
            metric_names::OPPORTUNITY_STALE,
            metric_names::OPPORTUNITY_EXPIRED,
            metric_names::OPPORTUNITY_REVALIDATION_REQUESTED,
            metric_names::OPPORTUNITY_REVALIDATION_SUCCEEDED,
            metric_names::OPPORTUNITY_REVALIDATION_FAILED,
        ];
        let unique: std::collections::HashSet<_> = names.iter().collect();
        assert_eq!(unique.len(), names.len(), "metric names must be unique");
    }

    #[test]
    fn ingestion_report_converts_into_all_discovery_metrics() {
        let mut report = OpportunityIngestionReport::default();
        report.candidates_received = 10;
        report.candidates_rejected = 2;
        report.opportunities_created = 5;
        report.opportunities_changed = 3;
        report.sources_failed = 1;
        let samples = ingestion_report_metrics(&report);
        assert_eq!(samples.len(), 5);
        assert!(samples.iter().all(|sample| {
            sample
                .labels
                .iter()
                .any(|(key, _)| key == label_keys::BATCH_ID)
        }));
    }

    #[test]
    fn lifecycle_batch_converts_into_state_gauges() {
        let batch = LifecycleEvaluationBatch {
            evaluated_at_ms: 42,
            policy: OpportunityLifecyclePolicy::new(1_000, 5_000).expect("valid policy"),
            counts: LifecycleStateCounts {
                active: 7,
                stale: 3,
                expired: 1,
            },
            evaluations: vec![LifecycleEvaluation {
                identity: "acme:widget".into(),
                opportunity_id: Uuid::now_v7(),
                snapshot: OpportunityLifecycleSnapshot {
                    state: crate::opportunity_lifecycle::OpportunityLifecycleState::Active,
                    age_ms: 0,
                    last_observed_at_ms: 42,
                },
            }],
        };
        let samples = lifecycle_batch_metrics(&batch);
        assert_eq!(samples.len(), 3);
        assert_eq!(samples[0].name, metric_names::OPPORTUNITY_ACTIVE);
        assert_eq!(samples[0].value, 7);
    }

    #[test]
    fn revalidation_statuses_map_to_the_correct_metric_family() {
        let requested = revalidation_status_metrics(RevalidationStatus::Pending, 4);
        assert_eq!(
            requested[0].name,
            metric_names::OPPORTUNITY_REVALIDATION_REQUESTED
        );
        assert_eq!(requested[0].value, 4);

        let succeeded = revalidation_status_metrics(RevalidationStatus::Succeeded, 2);
        assert_eq!(
            succeeded[0].name,
            metric_names::OPPORTUNITY_REVALIDATION_SUCCEEDED
        );

        let failed = revalidation_status_metrics(RevalidationStatus::DeadLettered, 1);
        assert_eq!(
            failed[0].name,
            metric_names::OPPORTUNITY_REVALIDATION_FAILED
        );

        assert!(revalidation_status_metrics(RevalidationStatus::Cancelled, 1).is_empty());
    }

    #[test]
    fn in_memory_sink_aggregates_totals_deterministically() {
        let mut sink = InMemoryMetricSink::new();
        sink.record(MetricSample::counter(
            metric_names::DISCOVERY_CANDIDATES_RECEIVED,
            3,
        ));
        sink.record(
            MetricSample::counter(metric_names::DISCOVERY_CANDIDATES_RECEIVED, 4)
                .with_label("source", "network-a"),
        );
        sink.record(MetricSample::counter(
            metric_names::DISCOVERY_SOURCE_FAILURES,
            1,
        ));
        let totals = sink.totals();
        assert_eq!(
            totals.get(metric_names::DISCOVERY_CANDIDATES_RECEIVED),
            Some(&7)
        );
        assert_eq!(
            totals.get(metric_names::DISCOVERY_SOURCE_FAILURES),
            Some(&1)
        );
        assert_eq!(sink.len(), 3);
        assert!(!sink.is_empty());
    }

    #[test]
    fn freshness_state_and_metric_names_stay_aligned() {
        // Guard against accidental vocabulary drift between metrics and domain.
        assert_eq!(FreshnessState::Active.as_str(), "active");
        assert_eq!(FreshnessState::Stale.as_str(), "stale");
        assert_eq!(FreshnessState::Expired.as_str(), "expired");
    }
}

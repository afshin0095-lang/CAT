//! Bridge between discovery-source batches and opportunity persistence.
//!
//! The source SPI remains transport-oriented, the discovery engine owns
//! validation/ranking, and the opportunity store owns deduplication/provenance.
//!
//! Isolation rules:
//! - a source failure is contained to that source and counted;
//! - a persistence failure is contained to its opportunity and counted;
//! - no path panics and no candidate is silently dropped: every candidate is
//!   either persisted or accounted as rejected/failed in the report.
//!
//! The report is deterministic for a deterministic input stream, and carries
//! `batch_id`, `started_at_ms`, and `completed_at_ms` for observability.

use uuid::Uuid;

use crate::clock::{Clock, SystemClock};
use crate::{
    DiscoveryEngine, DiscoveryIngestion, DiscoveryRequest, DiscoverySourceRegistry, OpportunityStore,
    OpportunityStoreError, OpportunityUpsertResult,
};

#[derive(Debug, thiserror::Error)]
pub enum OpportunityIngestionError {
    #[error("discovery engine failed: {0}")]
    Discovery(#[from] crate::DiscoveryError),
    #[error("opportunity persistence failed: {0}")]
    Persistence(#[from] OpportunityStoreError),
}

/// Aggregate result for one source-ingestion pass.
///
/// Counting invariants:
/// - `sources_attempted == sources_succeeded + sources_failed`;
/// - `candidates_received == candidates_rejected + opportunities_discovered`
///   per source (the remainder, if any, was filtered by score/limit and is
///   reported through `candidates_received - candidates_rejected -
///   opportunities_discovered`);
/// - `opportunities_discovered == opportunities_created + updates +
///   persistence_failures` in per-opportunity outcomes
///   (`opportunities_changed` counts changes, including creations).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpportunityIngestionReport {
    /// Unique identifier for this ingestion pass (UUID v7; nil when unset).
    pub batch_id: Uuid,
    pub sources_attempted: usize,
    pub sources_succeeded: usize,
    pub sources_failed: usize,
    pub candidates_received: usize,
    pub candidates_rejected: usize,
    pub opportunities_discovered: usize,
    pub opportunities_created: usize,
    pub opportunities_changed: usize,
    pub persistence_failures: usize,
    /// Discovery-engine rejections of the whole request (e.g. invalid
    /// `min_score`); contained per source and counted, never swallowed.
    pub discovery_failures: usize,
    pub started_at_ms: u64,
    pub completed_at_ms: u64,
}

impl Default for OpportunityIngestionReport {
    fn default() -> Self {
        Self {
            batch_id: Uuid::nil(),
            sources_attempted: 0,
            sources_succeeded: 0,
            sources_failed: 0,
            candidates_received: 0,
            candidates_rejected: 0,
            opportunities_discovered: 0,
            opportunities_created: 0,
            opportunities_changed: 0,
            persistence_failures: 0,
            discovery_failures: 0,
            started_at_ms: 0,
            completed_at_ms: 0,
        }
    }
}

impl OpportunityIngestionReport {
    fn record_persistence(&mut self, result: OpportunityUpsertResult) {
        if result.created {
            self.opportunities_created += 1;
        }
        if result.changed {
            self.opportunities_changed += 1;
        }
    }
}

/// Coordinates source collection, deterministic discovery, and persistence.
///
/// A source failure is isolated from other sources. A persistence failure is
/// also isolated to its opportunity so one malformed record cannot discard the
/// rest of the batch.
pub struct OpportunityIngestion<'a, S> {
    discovery: DiscoveryIngestion<'a>,
    store: S,
    engine: DiscoveryEngine,
    clock: Box<dyn Clock>,
}

impl<'a, S> OpportunityIngestion<'a, S>
where
    S: OpportunityStore,
{
    pub fn new(registry: &'a DiscoverySourceRegistry, store: S) -> Self {
        Self::with_clock(registry, store, SystemClock::new())
    }

    pub fn with_clock(registry: &'a DiscoverySourceRegistry, store: S, clock: impl Clock + 'static) -> Self {
        Self {
            discovery: DiscoveryIngestion::new(registry),
            store,
            engine: DiscoveryEngine,
            clock: Box::new(clock),
        }
    }

    pub fn store(&self) -> &S {
        &self.store
    }

    /// Collect and persist all valid opportunities returned by registered sources.
    ///
    /// The per-source limit prevents one provider from starving other providers;
    /// global deduplication still happens in the shared `OpportunityStore`.
    pub async fn ingest(
        &mut self,
        request: crate::DiscoverySourceRequest,
        min_score: u32,
        limit_per_source: usize,
    ) -> OpportunityIngestionReport {
        let mut report = OpportunityIngestionReport {
            batch_id: Uuid::now_v7(),
            started_at_ms: self.clock.now_ms(),
            ..OpportunityIngestionReport::default()
        };

        for result in self.discovery.collect(request).await {
            report.sources_attempted += 1;
            let (_source, batch) = match result {
                Ok(value) => value,
                Err(_) => {
                    report.sources_failed += 1;
                    continue;
                }
            };

            report.sources_succeeded += 1;
            report.candidates_received += batch.candidates.len();
            let discovery = match self.engine.discover(DiscoveryRequest {
                candidates: batch.candidates,
                min_score,
                limit: limit_per_source,
            }) {
                Ok(value) => value,
                Err(_) => {
                    report.discovery_failures += 1;
                    continue;
                }
            };

            report.opportunities_discovered += discovery.opportunities.len();
            report.candidates_rejected += discovery.rejected;

            for opportunity in discovery.opportunities {
                match self.store.upsert(opportunity) {
                    Ok(result) => report.record_persistence(result),
                    Err(_) => report.persistence_failures += 1,
                }
            }
        }

        report.completed_at_ms = self.clock.now_ms();
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        canonical_key, DiscoveryCandidate, DiscoverySource, DiscoverySourceBatch,
        DiscoverySourceError, DiscoverySourceFuture, DiscoverySourceId, DiscoverySourceInfo,
        DiscoverySourceKind, DiscoverySourceRequest, InMemoryOpportunityStore, OpportunityIdentity,
    };
    use crate::clock::FixedClock;
    use std::future::{ready, Future};
    use std::pin::Pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn block_on<F: Future>(mut future: F) -> F::Output {
        fn clone(_: *const ()) -> RawWaker { raw_waker() }
        fn wake(_: *const ()) {}
        fn wake_by_ref(_: *const ()) {}
        fn drop(_: *const ()) {}
        fn raw_waker() -> RawWaker {
            RawWaker::new(
                std::ptr::null(),
                &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
            )
        }
        let waker = unsafe { Waker::from_raw(raw_waker()) };
        let mut context = Context::from_waker(&waker);
        let mut future = unsafe { Pin::new_unchecked(&mut future) };
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => return value,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    struct StaticSource {
        info: DiscoverySourceInfo,
        batch: DiscoverySourceBatch,
        fail: bool,
    }

    impl DiscoverySource for StaticSource {
        fn info(&self) -> &DiscoverySourceInfo { &self.info }

        fn discover<'a>(
            &'a self,
            _request: DiscoverySourceRequest,
        ) -> DiscoverySourceFuture<'a, DiscoverySourceBatch> {
            if self.fail {
                Box::pin(ready(Err(DiscoverySourceError::Unavailable("provider down".into()))))
            } else {
                Box::pin(ready(Ok(self.batch.clone())))
            }
        }
    }

    fn source_info(id: &str, kind: DiscoverySourceKind) -> DiscoverySourceInfo {
        DiscoverySourceInfo {
            id: DiscoverySourceId(id.into()),
            name: id.into(),
            kind,
            capabilities: vec![],
        }
    }

    fn candidate(source: &str, product: &str, observed_at_ms: u64) -> DiscoveryCandidate {
        DiscoveryCandidate {
            source: source.into(),
            external_id: product.into(),
            merchant_name: "Acme".into(),
            product_name: product.into(),
            canonical_key: canonical_key("Acme", product),
            category: Some("electronics".into()),
            destination_url: "https://example.test/product".into(),
            currency: "EUR".into(),
            price_minor: Some(10_000),
            commission_bps: Some(10_000),
            demand_score: 9_000,
            competition_score: 1_000,
            freshness_score: 9_000,
            compliance_score: 10_000,
            observed_at_ms,
        }
    }

    #[test]
    fn ingestion_persists_and_deduplicates_across_sources() {
        let first = StaticSource {
            info: source_info("first", DiscoverySourceKind::AffiliateNetwork),
            batch: DiscoverySourceBatch {
                candidates: vec![candidate("first", "Widget", 10)],
                has_more: false,
                next_page: None,
            },
            fail: false,
        };
        let second = StaticSource {
            info: source_info("second", DiscoverySourceKind::MerchantCatalog),
            batch: DiscoverySourceBatch {
                candidates: vec![candidate("second", "Widget", 20)],
                has_more: false,
                next_page: None,
            },
            fail: false,
        };

        let mut registry = DiscoverySourceRegistry::new();
        registry.register(Box::new(first));
        registry.register(Box::new(second));
        let store = InMemoryOpportunityStore::new();
        let mut ingestion = OpportunityIngestion::with_clock(&registry, store, FixedClock::new(1_000));

        let report = block_on(ingestion.ingest(
            DiscoverySourceRequest {
                per_page: 10,
                ..Default::default()
            },
            0,
            10,
        ));

        assert!(!report.batch_id.is_nil());
        assert_eq!(report.sources_attempted, 2);
        assert_eq!(report.sources_succeeded, 2);
        assert_eq!(report.sources_failed, 0);
        assert_eq!(report.candidates_received, 2);
        assert_eq!(report.opportunities_discovered, 2);
        assert_eq!(report.opportunities_created, 1);
        assert_eq!(report.opportunities_changed, 2);
        assert_eq!(report.persistence_failures, 0);
        assert_eq!(report.started_at_ms, 1_000);
        assert_eq!(report.completed_at_ms, 1_000);
        let record = ingestion
            .store()
            .get(&OpportunityIdentity::new(&candidate("first", "Widget", 10)))
            .expect("deduplicated record");
        assert_eq!(record.observations.len(), 2);
        assert_eq!(record.best_source, "first");
    }

    #[test]
    fn source_failures_are_isolated_and_counted() {
        let healthy = StaticSource {
            info: source_info("healthy", DiscoverySourceKind::ProductFeed),
            batch: DiscoverySourceBatch {
                candidates: vec![candidate("healthy", "Widget", 10)],
                has_more: false,
                next_page: None,
            },
            fail: false,
        };
        let broken = StaticSource {
            info: source_info("broken", DiscoverySourceKind::Marketplace),
            batch: DiscoverySourceBatch::default(),
            fail: true,
        };

        let mut registry = DiscoverySourceRegistry::new();
        registry.register(Box::new(broken));
        registry.register(Box::new(healthy));
        let store = InMemoryOpportunityStore::new();
        let mut ingestion = OpportunityIngestion::new(&registry, store);

        let report = block_on(ingestion.ingest(
            DiscoverySourceRequest { per_page: 10, ..Default::default() },
            0,
            10,
        ));

        assert_eq!(report.sources_attempted, 2);
        assert_eq!(report.sources_succeeded, 1);
        assert_eq!(report.sources_failed, 1);
        assert_eq!(report.opportunities_created, 1);
        assert_eq!(report.persistence_failures, 0);
    }

    #[test]
    fn invalid_candidates_are_counted_never_dropped_silently() {
        let mut invalid = candidate("counting", "Bad", 10);
        invalid.destination_url.clear();
        let source = StaticSource {
            info: source_info("counting", DiscoverySourceKind::ProductFeed),
            batch: DiscoverySourceBatch {
                candidates: vec![invalid, candidate("counting", "Good", 10)],
                has_more: false,
                next_page: None,
            },
            fail: false,
        };

        let mut registry = DiscoverySourceRegistry::new();
        registry.register(Box::new(source));
        let store = InMemoryOpportunityStore::new();
        let mut ingestion = OpportunityIngestion::new(&registry, store);

        let report = block_on(ingestion.ingest(
            DiscoverySourceRequest { per_page: 10, ..Default::default() },
            0,
            10,
        ));

        assert_eq!(report.candidates_received, 2);
        assert_eq!(report.candidates_rejected, 1);
        assert_eq!(report.opportunities_discovered, 1);
        assert_eq!(report.opportunities_created, 1);
    }

    #[test]
    fn persistence_failures_are_counted_without_aborting_the_batch() {
        let source = StaticSource {
            info: source_info("persistence", DiscoverySourceKind::ProductFeed),
            batch: DiscoverySourceBatch {
                candidates: vec![candidate("persistence", "Fine", 10)],
                has_more: false,
                next_page: None,
            },
            fail: false,
        };

        let mut registry = DiscoverySourceRegistry::new();
        registry.register(Box::new(source));
        let store = FailingStore;
        let mut ingestion = OpportunityIngestion::new(&registry, store);

        let report = block_on(ingestion.ingest(
            DiscoverySourceRequest { per_page: 10, ..Default::default() },
            0,
            10,
        ));

        assert_eq!(report.opportunities_discovered, 1);
        assert_eq!(report.persistence_failures, 1);
        assert_eq!(report.opportunities_created, 0);
    }

    /// Store stub whose upsert always fails, isolating persistence accounting.
    struct FailingStore;

    impl OpportunityStore for FailingStore {
        fn upsert(&mut self, _: crate::DiscoveryOpportunity) -> Result<OpportunityUpsertResult, OpportunityStoreError> {
            Err(OpportunityStoreError::InvalidCandidate)
        }

        fn get(&self, _: &OpportunityIdentity) -> Result<crate::OpportunityRecord, OpportunityStoreError> {
            Err(OpportunityStoreError::NotFound)
        }

        fn list(&self) -> Vec<crate::OpportunityRecord> {
            Vec::new()
        }
    }
}

//! Bridge between discovery-source batches and durable opportunity persistence.
//!
//! The source SPI remains transport-oriented, while the discovery engine owns
//! validation/ranking and the opportunity store owns deduplication/provenance.

use async_trait::async_trait;

use crate::{
    DiscoveryEngine, DiscoveryIngestion, DiscoveryRequest, DiscoverySourceError,
    DiscoverySourceId, OpportunityStoreError, OpportunityUpsertResult,
};
use crate::opportunity_store::OpportunityStore;

#[derive(Debug, thiserror::Error)]
pub enum OpportunityIngestionError {
    #[error("discovery source {source:?} failed: {error}")]
    Source {
        source: DiscoverySourceId,
        error: DiscoverySourceError,
    },
    #[error("discovery engine failed: {0}")]
    Discovery(#[from] crate::DiscoveryError),
    #[error("opportunity persistence failed: {0}")]
    Persistence(#[from] OpportunityStoreError),
}

/// Aggregate result for one source-ingestion pass.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OpportunityIngestionReport {
    pub sources_succeeded: usize,
    pub sources_failed: usize,
    pub opportunities_discovered: usize,
    pub candidates_rejected: usize,
    pub opportunities_created: usize,
    pub opportunities_changed: usize,
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
/// Source failures are isolated to the failing source so a transient provider
/// does not discard valid opportunities discovered elsewhere in the same pass.
pub struct OpportunityIngestion<'a, S> {
    discovery: DiscoveryIngestion<'a>,
    store: S,
    engine: DiscoveryEngine,
}

impl<'a, S> OpportunityIngestion<'a, S>
where
    S: OpportunityStore,
{
    pub fn new(registry: &'a crate::DiscoverySourceRegistry, store: S) -> Self {
        Self {
            discovery: DiscoveryIngestion::new(registry),
            store,
            engine: DiscoveryEngine,
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
        &self,
        request: crate::DiscoverySourceRequest,
        min_score: u32,
        limit_per_source: usize,
    ) -> OpportunityIngestionReport {
        let mut report = OpportunityIngestionReport::default();

        for result in self.discovery.collect(request).await {
            let (source, batch) = match result {
                Ok(value) => value,
                Err(error) => {
                    report.sources_failed += 1;
                    let _ = OpportunityIngestionError::Source {
                        source: DiscoverySourceId("unknown".into()),
                        error,
                    };
                    continue;
                }
            };

            report.sources_succeeded += 1;
            let discovery = self.engine.discover(DiscoveryRequest {
                candidates: batch.candidates,
                min_score,
                limit: limit_per_source,
            });

            let discovery = match discovery {
                Ok(value) => value,
                Err(_) => continue,
            };

            report.opportunities_discovered += discovery.opportunities.len();
            report.candidates_rejected += discovery.rejected;

            for opportunity in discovery.opportunities {
                if let Ok(result) = self.store.upsert(&opportunity) {
                    report.record_persistence(result);
                }
            }

            let _ = source;
        }

        report
    }
}

/// Async persistence boundary for implementations that cannot use the
/// synchronous `OpportunityStore` contract (for example PostgreSQL).
#[async_trait]
pub trait AsyncOpportunitySink: Send + Sync {
    async fn persist(
        &self,
        opportunity: &crate::DiscoveryOpportunity,
    ) -> Result<OpportunityUpsertResult, OpportunityIngestionError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        canonical_key, DiscoveryCandidate, DiscoverySource, DiscoverySourceBatch,
        DiscoverySourceInfo, DiscoverySourceKind, DiscoverySourceRequest,
        InMemoryOpportunityStore,
    };
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
    }

    impl DiscoverySource for StaticSource {
        fn info(&self) -> &DiscoverySourceInfo { &self.info }

        fn discover<'a>(
            &'a self,
            _request: DiscoverySourceRequest,
        ) -> crate::DiscoverySourceFuture<'a, DiscoverySourceBatch> {
            Box::pin(ready(Ok(self.batch.clone())))
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
            info: DiscoverySourceInfo {
                id: DiscoverySourceId("first".into()),
                name: "First".into(),
                kind: DiscoverySourceKind::AffiliateNetwork,
                capabilities: vec![],
            },
            batch: DiscoverySourceBatch {
                candidates: vec![candidate("first", "Widget", 10)],
                has_more: false,
                next_page: None,
            },
        };
        let second = StaticSource {
            info: DiscoverySourceInfo {
                id: DiscoverySourceId("second".into()),
                name: "Second".into(),
                kind: DiscoverySourceKind::MerchantCatalog,
                capabilities: vec![],
            },
            batch: DiscoverySourceBatch {
                candidates: vec![candidate("second", "Widget", 20)],
                has_more: false,
                next_page: None,
            },
        };

        let mut registry = crate::DiscoverySourceRegistry::new();
        registry.register(Box::new(first));
        registry.register(Box::new(second));
        let store = InMemoryOpportunityStore::new();
        let ingestion = OpportunityIngestion::new(&registry, store);

        let report = block_on(ingestion.ingest(
            DiscoverySourceRequest {
                per_page: 10,
                ..Default::default()
            },
            0,
            10,
        ));

        assert_eq!(report.sources_succeeded, 2);
        assert_eq!(report.sources_failed, 0);
        assert_eq!(report.opportunities_discovered, 2);
        assert_eq!(report.opportunities_created, 1);
        assert_eq!(report.opportunities_changed, 2);
        let record = ingestion
            .store()
            .get(&crate::OpportunityIdentity::new(&candidate("first", "Widget", 10)))
            .expect("deduplicated record");
        assert_eq!(record.observations.len(), 2);
        assert_eq!(record.best_source, "first");
    }
}

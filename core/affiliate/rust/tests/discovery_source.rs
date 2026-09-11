use cat_affiliate::{
    DiscoveryCandidate, DiscoveryIngestion, DiscoverySource, DiscoverySourceBatch,
    DiscoverySourceCapability, DiscoverySourceFuture, DiscoverySourceId, DiscoverySourceInfo,
    DiscoverySourceKind, DiscoverySourceRegistry, DiscoverySourceRequest,
};
use std::future::ready;

struct TestSource {
    info: DiscoverySourceInfo,
    batch: DiscoverySourceBatch,
}

impl DiscoverySource for TestSource {
    fn info(&self) -> &DiscoverySourceInfo {
        &self.info
    }

    fn discover<'a>(
        &'a self,
        _request: DiscoverySourceRequest,
    ) -> DiscoverySourceFuture<'a, DiscoverySourceBatch> {
        Box::pin(ready(Ok(self.batch.clone())))
    }
}

fn candidate(source: &str, external_id: &str) -> DiscoveryCandidate {
    DiscoveryCandidate {
        source: source.into(),
        external_id: external_id.into(),
        merchant_name: "Merchant".into(),
        product_name: "Product".into(),
        canonical_key: format!("{source}:{external_id}"),
        category: Some("electronics".into()),
        destination_url: "https://example.com/product".into(),
        currency: "EUR".into(),
        price_minor: Some(1_000),
        commission_bps: Some(500),
        demand_score: 7_000,
        competition_score: 3_000,
        freshness_score: 9_000,
        compliance_score: 10_000,
        observed_at_ms: 1,
    }
}

#[tokio::test]
async fn ingestion_collects_registered_sources_in_order() {
    let mut registry = DiscoverySourceRegistry::new();
    for (source, external_id) in [("feed-a", "a-1"), ("feed-b", "b-1")] {
        registry.register(Box::new(TestSource {
            info: DiscoverySourceInfo {
                id: DiscoverySourceId(source.into()),
                name: source.into(),
                kind: DiscoverySourceKind::ProductFeed,
                capabilities: vec![DiscoverySourceCapability::Pagination],
            },
            batch: DiscoverySourceBatch {
                candidates: vec![candidate(source, external_id)],
                has_more: false,
                next_page: None,
            },
        }));
    }

    let results = DiscoveryIngestion::new(&registry)
        .collect(DiscoverySourceRequest {
            per_page: 25,
            ..Default::default()
        })
        .await;

    assert_eq!(results.len(), 2);
    assert_eq!(
        results[0].as_ref().unwrap().0,
        DiscoverySourceId("feed-a".into())
    );
    assert_eq!(results[0].as_ref().unwrap().1.candidates.len(), 1);
    assert_eq!(
        results[1].as_ref().unwrap().0,
        DiscoverySourceId("feed-b".into())
    );
}

#[tokio::test]
async fn invalid_request_is_rejected_before_source_execution() {
    let mut registry = DiscoverySourceRegistry::new();
    registry.register(Box::new(TestSource {
        info: DiscoverySourceInfo {
            id: DiscoverySourceId("feed".into()),
            name: "Feed".into(),
            kind: DiscoverySourceKind::ProductFeed,
            capabilities: vec![],
        },
        batch: DiscoverySourceBatch::default(),
    }));

    let results = DiscoveryIngestion::new(&registry)
        .collect(DiscoverySourceRequest {
            per_page: 0,
            ..Default::default()
        })
        .await;

    assert_eq!(results.len(), 1);
    assert!(results[0].is_err());
}

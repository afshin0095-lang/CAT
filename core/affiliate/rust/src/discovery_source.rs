//! Discovery-source SPI for affiliate opportunity ingestion.
//!
//! This boundary deliberately separates external discovery mechanisms from
//! CAT's normalized opportunity engine. A source may be an affiliate network,
//! merchant feed, product API, catalog export, or a future browser/agent
//! connector. Source-specific transport and authentication stay behind the
//! adapter boundary.

use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::discovery::DiscoveryCandidate;

pub type DiscoverySourceFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<T, DiscoverySourceError>> + Send + 'a>>;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DiscoverySourceId(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiscoverySourceKind {
    AffiliateNetwork,
    MerchantCatalog,
    ProductFeed,
    Marketplace,
    SearchIndex,
    AgentConnector,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiscoverySourceCapability {
    Pagination,
    IncrementalSync,
    CategoryFilter,
    GeographicFilter,
    CurrencyFilter,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoverySourceInfo {
    pub id: DiscoverySourceId,
    pub name: String,
    pub kind: DiscoverySourceKind,
    pub capabilities: Vec<DiscoverySourceCapability>,
}

impl DiscoverySourceInfo {
    pub fn supports(&self, capability: DiscoverySourceCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoverySourceRequest {
    pub query: Option<String>,
    pub page: u32,
    pub per_page: u32,
    pub category: Option<String>,
    pub geographic_market: Option<String>,
    pub currency: Option<String>,
}

impl DiscoverySourceRequest {
    pub fn validate(&self) -> Result<(), DiscoverySourceError> {
        if self.per_page == 0 {
            return Err(DiscoverySourceError::InvalidRequest("per_page must be greater than zero"));
        }
        if let Some(query) = &self.query {
            if query.trim().is_empty() {
                return Err(DiscoverySourceError::InvalidRequest("query must not be blank"));
            }
        }
        if let Some(category) = &self.category {
            if category.trim().is_empty() {
                return Err(DiscoverySourceError::InvalidRequest("category must not be blank"));
            }
        }
        if let Some(market) = &self.geographic_market {
            if market.trim().is_empty() {
                return Err(DiscoverySourceError::InvalidRequest("geographic_market must not be blank"));
            }
        }
        if let Some(currency) = &self.currency {
            if currency.trim().is_empty() {
                return Err(DiscoverySourceError::InvalidRequest("currency must not be blank"));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoverySourceBatch {
    pub candidates: Vec<DiscoveryCandidate>,
    pub has_more: bool,
    pub next_page: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiscoverySourceError {
    InvalidRequest(&'static str),
    Unavailable(String),
    AuthenticationFailed,
    RateLimited,
    InvalidResponse(String),
}

impl std::fmt::Display for DiscoverySourceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(message) => write!(formatter, "invalid discovery source request: {message}"),
            Self::Unavailable(message) => write!(formatter, "discovery source unavailable: {message}"),
            Self::AuthenticationFailed => formatter.write_str("discovery source authentication failed"),
            Self::RateLimited => formatter.write_str("discovery source rate limited"),
            Self::InvalidResponse(message) => write!(formatter, "invalid discovery source response: {message}"),
        }
    }
}

impl std::error::Error for DiscoverySourceError {}

/// Adapter contract for every external discovery mechanism.
pub trait DiscoverySource: Send + Sync {
    fn info(&self) -> &DiscoverySourceInfo;

    fn discover<'a>(&'a self, request: DiscoverySourceRequest) -> DiscoverySourceFuture<'a, DiscoverySourceBatch>;
}

/// Thin orchestration boundary over multiple sources.
///
/// Sources are queried in registration order. The engine does not rank,
/// mutate, or silently repair candidates; normalization and opportunity
/// scoring remain owned by `DiscoveryEngine`.
pub struct DiscoverySourceRegistry {
    sources: Vec<Box<dyn DiscoverySource>>,
}

impl DiscoverySourceRegistry {
    pub fn new() -> Self {
        Self { sources: Vec::new() }
    }

    pub fn register(&mut self, source: Box<dyn DiscoverySource>) {
        self.sources.push(source);
    }

    pub fn get(&self, source_id: &DiscoverySourceId) -> Option<&dyn DiscoverySource> {
        self.sources
            .iter()
            .find(|source| source.info().id == *source_id)
            .map(|source| source.as_ref())
    }

    pub fn all(&self) -> &[Box<dyn DiscoverySource>] {
        &self.sources
    }

    pub fn count(&self) -> usize {
        self.sources.len()
    }
}

impl Default for DiscoverySourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Collects source batches without coupling the source SPI to scoring.
pub struct DiscoveryIngestion<'a> {
    registry: &'a DiscoverySourceRegistry,
}

impl<'a> DiscoveryIngestion<'a> {
    pub fn new(registry: &'a DiscoverySourceRegistry) -> Self {
        Self { registry }
    }

    pub async fn collect(
        &self,
        request: DiscoverySourceRequest,
    ) -> Vec<Result<(DiscoverySourceId, DiscoverySourceBatch), DiscoverySourceError>> {
        let mut results = Vec::with_capacity(self.registry.count());
        for source in self.registry.all() {
            let source_id = source.info().id.clone();
            let result = match request.validate() {
                Ok(()) => source.discover(request.clone()).await.map(|batch| (source_id, batch)),
                Err(error) => Err(error),
            };
            results.push(result);
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::ready;

    struct StaticSource {
        info: DiscoverySourceInfo,
        batch: DiscoverySourceBatch,
    }

    impl DiscoverySource for StaticSource {
        fn info(&self) -> &DiscoverySourceInfo {
            &self.info
        }

        fn discover<'a>(&'a self, _request: DiscoverySourceRequest) -> DiscoverySourceFuture<'a, DiscoverySourceBatch> {
            Box::pin(ready(Ok(self.batch.clone())))
        }
    }

    #[test]
    fn request_rejects_zero_page_size() {
        let request = DiscoverySourceRequest { per_page: 0, ..Default::default() };
        assert!(matches!(request.validate(), Err(DiscoverySourceError::InvalidRequest(_))));
    }

    #[test]
    fn capabilities_are_explicit() {
        let info = DiscoverySourceInfo {
            id: DiscoverySourceId("test".into()),
            name: "Test".into(),
            kind: DiscoverySourceKind::ProductFeed,
            capabilities: vec![DiscoverySourceCapability::Pagination],
        };
        assert!(info.supports(DiscoverySourceCapability::Pagination));
        assert!(!info.supports(DiscoverySourceCapability::CurrencyFilter));
    }

    #[test]
    fn registry_preserves_registration_order_and_lookup() {
        let first = StaticSource {
            info: DiscoverySourceInfo {
                id: DiscoverySourceId("first".into()),
                name: "First".into(),
                kind: DiscoverySourceKind::ProductFeed,
                capabilities: vec![],
            },
            batch: DiscoverySourceBatch::default(),
        };
        let second = StaticSource {
            info: DiscoverySourceInfo {
                id: DiscoverySourceId("second".into()),
                name: "Second".into(),
                kind: DiscoverySourceKind::MerchantCatalog,
                capabilities: vec![],
            },
            batch: DiscoverySourceBatch::default(),
        };

        let mut registry = DiscoverySourceRegistry::new();
        registry.register(Box::new(first));
        registry.register(Box::new(second));

        assert_eq!(registry.count(), 2);
        assert_eq!(registry.all()[0].info().id, DiscoverySourceId("first".into()));
        assert!(registry.get(&DiscoverySourceId("second".into())).is_some());
    }
}

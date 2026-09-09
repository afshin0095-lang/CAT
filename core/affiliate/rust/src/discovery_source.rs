//! Discovery-source SPI for affiliate opportunity ingestion.
//!
//! External discovery mechanisms normalize into `DiscoveryCandidate` behind
//! this boundary. Transport, authentication, rate limiting, and vendor SDK
//! details remain outside the affiliate domain.

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
            return Err(DiscoverySourceError::InvalidRequest(
                "per_page must be greater than zero".into(),
            ));
        }
        validate_optional_text(self.query.as_deref(), "query")?;
        validate_optional_text(self.category.as_deref(), "category")?;
        validate_optional_text(self.geographic_market.as_deref(), "geographic_market")?;
        validate_optional_text(self.currency.as_deref(), "currency")?;
        Ok(())
    }
}

fn validate_optional_text(value: Option<&str>, field: &str) -> Result<(), DiscoverySourceError> {
    if value.is_some_and(|text| text.trim().is_empty()) {
        return Err(DiscoverySourceError::InvalidRequest(format!(
            "{field} must not be blank"
        )));
    }
    Ok(())
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoverySourceBatch {
    pub candidates: Vec<DiscoveryCandidate>,
    pub has_more: bool,
    pub next_page: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiscoverySourceError {
    InvalidRequest(String),
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

/// Registry of source adapters. Registration order is preserved.
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
        fn info(&self) -> &DiscoverySourceInfo { &self.info }

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
    fn request_rejects_blank_filters() {
        let request = DiscoverySourceRequest {
            per_page: 10,
            query: Some("  ".into()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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

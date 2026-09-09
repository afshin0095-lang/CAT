//! Normalizes affiliate-network programs into the source-neutral discovery contract.
use std::future::Future;
use std::pin::Pin;
use crate::discovery::{canonical_key, DiscoveryCandidate};
use crate::discovery_source::{DiscoverySource, DiscoverySourceBatch, DiscoverySourceCapability, DiscoverySourceError, DiscoverySourceFuture, DiscoverySourceId, DiscoverySourceInfo, DiscoverySourceKind, DiscoverySourceRequest};
use crate::network_adapter::{NetworkAdapter, NetworkProgram};

fn commission_to_bps(value: &str) -> Result<u32, DiscoverySourceError> {
    let value = value.trim();
    if value.is_empty() { return Err(DiscoverySourceError::InvalidResponse("commission_rate must not be blank".into())); }
    let percent = value.ends_with('%');
    let number = value.trim_end_matches('%').trim().parse::<f64>().map_err(|_| DiscoverySourceError::InvalidResponse(format!("invalid commission_rate: {value}")))?;
    let percentage = if percent { number } else if number <= 1.0 { number * 100.0 } else { number };
    if !percentage.is_finite() || !(0.0..=100.0).contains(&percentage) { return Err(DiscoverySourceError::InvalidResponse(format!("commission_rate out of range: {value}"))); }
    Ok((percentage * 100.0).round() as u32)
}

fn normalize_program(program: NetworkProgram, observed_at_ms: i64) -> Result<DiscoveryCandidate, DiscoverySourceError> {
    if program.external_id.trim().is_empty() || program.name.trim().is_empty() || program.merchant_name.trim().is_empty() || program.url.trim().is_empty() {
        return Err(DiscoverySourceError::InvalidResponse("network program contains a required blank field".into()));
    }
    Ok(DiscoveryCandidate {
        source: program.network_id.0,
        external_id: program.external_id,
        merchant_name: program.merchant_name.clone(),
        product_name: program.name.clone(),
        canonical_key: canonical_key(&program.merchant_name, &program.name),
        category: program.categories.first().cloned(),
        destination_url: program.url,
        currency: "UNKNOWN".into(),
        price_minor: None,
        commission_bps: Some(commission_to_bps(&program.commission_rate)?),
        demand_score: 5_000,
        competition_score: 5_000,
        freshness_score: 10_000,
        compliance_score: if program.accepting_applications { 10_000 } else { 5_000 },
        observed_at_ms,
    })
}

/// Adapts one existing `NetworkAdapter` without exposing network-specific APIs to discovery.
pub struct NetworkDiscoveryAdapter<'a> {
    network: &'a dyn NetworkAdapter,
    info: DiscoverySourceInfo,
    clock: Box<dyn Fn() -> i64 + Send + Sync + 'a>,
}

impl<'a> NetworkDiscoveryAdapter<'a> {
    pub fn new(network: &'a dyn NetworkAdapter) -> Self {
        let network_info = network.info();
        Self {
            network,
            info: DiscoverySourceInfo { id: DiscoverySourceId(format!("network:{}", network_info.id.0)), name: network_info.name.clone(), kind: DiscoverySourceKind::AffiliateNetwork, capabilities: vec![DiscoverySourceCapability::Pagination] },
            clock: Box::new(|| std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)),
        }
    }

    #[cfg(test)]
    fn with_clock<F>(network: &'a dyn NetworkAdapter, clock: F) -> Self where F: Fn() -> i64 + Send + Sync + 'a {
        let mut adapter = Self::new(network); adapter.clock = Box::new(clock); adapter
    }
}

impl<'a> DiscoverySource for NetworkDiscoveryAdapter<'a> {
    fn info(&self) -> &DiscoverySourceInfo { &self.info }

    fn discover<'b>(&'b self, request: DiscoverySourceRequest) -> DiscoverySourceFuture<'b, DiscoverySourceBatch> {
        Box::pin(async move {
            request.validate()?;
            if request.category.is_some() || request.geographic_market.is_some() || request.currency.is_some() {
                return Err(DiscoverySourceError::InvalidRequest("network adapter supports only query and pagination".into()));
            }
            let programs = self.network.list_programs(request.query.as_deref(), request.page, request.per_page).await.map_err(|e| DiscoverySourceError::Unavailable(e.to_string()))?;
            let observed_at_ms = (self.clock)();
            let mut candidates = Vec::with_capacity(programs.len());
            for program in programs { candidates.push(normalize_program(program, observed_at_ms)?); }
            let has_more = candidates.len() == request.per_page as usize;
            Ok(DiscoverySourceBatch { candidates, has_more, next_page: has_more.then_some(request.page.saturating_add(1)) })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AffiliateDomainResult, NetworkConversion, NetworkId, NetworkInfo};
    use std::future::ready;

    struct Stub { info: NetworkInfo, programs: Vec<NetworkProgram> }
    impl NetworkAdapter for Stub {
        fn info(&self) -> &NetworkInfo { &self.info }
        fn list_programs(&self, _: Option<&str>, _: u32, _: u32) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<Vec<NetworkProgram>>> + Send + '_>> { Box::pin(ready(Ok(self.programs.clone()))) }
        fn generate_link(&self, _: &str, _: &str, _: &str) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<String>> + Send + '_>> { Box::pin(ready(Ok("https://example.test/link".into()))) }
        fn fetch_conversions(&self, _: i64, _: i64) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<Vec<NetworkConversion>>> + Send + '_>> { Box::pin(ready(Ok(vec![]))) }
        fn validate_credentials(&self) -> Pin<Box<dyn Future<Output=AffiliateDomainResult<bool>> + Send + '_>> { Box::pin(ready(Ok(true))) }
    }
    fn network(programs: Vec<NetworkProgram>) -> Stub { Stub { info: NetworkInfo { id: NetworkId("test".into()), name: "Test".into(), base_url: "https://test.invalid".into(), supports_real_time_reporting: false, supports_deep_linking: true, default_cookie_days: 30 }, programs } }
    fn program(rate: &str) -> NetworkProgram { NetworkProgram { external_id: "p1".into(), network_id: NetworkId("test".into()), name: "Widget".into(), merchant_name: "Acme".into(), commission_rate: rate.into(), cookie_days: 30, categories: vec!["electronics".into()], url: "https://merchant.invalid/widget".into(), description: None, accepting_applications: true } }
    #[test] fn rates_are_normalized() { assert_eq!(commission_to_bps("5%").unwrap(), 500); assert_eq!(commission_to_bps("5.5%").unwrap(), 550); assert_eq!(commission_to_bps("0.05").unwrap(), 500); }
    #[test] fn invalid_rates_are_rejected() { assert!(commission_to_bps("bad").is_err()); assert!(commission_to_bps("101%").is_err()); }
    #[test] fn adapter_exposes_only_supported_capability() { let n=network(vec![]); let a=NetworkDiscoveryAdapter::with_clock(&n,||42); assert_eq!(a.info().id, DiscoverySourceId("network:test".into())); assert!(a.info().supports(DiscoverySourceCapability::Pagination)); assert!(!a.info().supports(DiscoverySourceCapability::CurrencyFilter)); }
    #[test] fn normalization_preserves_identity_and_timestamp() { let result=normalize_program(program("7.5%"),42).unwrap(); assert_eq!(result.source,"test"); assert_eq!(result.commission_bps,Some(750)); assert_eq!(result.canonical_key,"acme/widget"); assert_eq!(result.observed_at_ms,42); }
}

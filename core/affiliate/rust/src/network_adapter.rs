//! Network adapter boundary: trait-based abstraction over affiliate networks.
//!
//! Every affiliate network (Amazon Associates, CJ, ShareASale, Impact, etc.)
//! has a different API, auth mechanism, and data format. This module defines
//! the contract that all network adapters must implement, keeping provider-
//! specific SDK/API details behind adapter boundaries.
//!
//! Inspired by:
//! - CAT's existing `provider_adapter.rs` in cat-orchestrator
//! - OpenPartner's deployment-mode pattern
//! - NucleusLinks' smart routing to highest-EPC network
//! - Brambles.ai's 15+ network auto-selection

use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

use crate::AffiliateDomainResult;

// ---------------------------------------------------------------------------
// Network identity
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct NetworkId(pub String);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub id: NetworkId,
    pub name: String,
    pub base_url: String,
    pub supports_real_time_reporting: bool,
    pub supports_deep_linking: bool,
    pub default_cookie_days: u32,
}

// ---------------------------------------------------------------------------
// Program data from a network
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProgram {
    pub external_id: String,
    pub network_id: NetworkId,
    pub name: String,
    pub merchant_name: String,
    pub commission_rate: String,
    pub cookie_days: u32,
    pub categories: Vec<String>,
    pub url: String,
    pub description: Option<String>,
    pub accepting_applications: bool,
}

// ---------------------------------------------------------------------------
// Conversion report from a network
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConversion {
    pub external_id: String,
    pub network_id: NetworkId,
    pub click_ref: Option<String>,
    pub order_ref: Option<String>,
    pub sale_amount_minor: i64,
    pub commission_amount_minor: i64,
    pub currency: String,
    pub status: NetworkConversionStatus,
    pub timestamp_ms: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NetworkConversionStatus {
    Pending,
    Approved,
    Rejected,
    Paid,
}

// ---------------------------------------------------------------------------
// Network adapter trait
// ---------------------------------------------------------------------------

/// The contract all affiliate network adapters must implement.
pub trait NetworkAdapter: Send + Sync {
    /// Network metadata.
    fn info(&self) -> &NetworkInfo;

    /// Discover available programs (paginated).
    fn list_programs(
        &self,
        query: Option<&str>,
        page: u32,
        per_page: u32,
    ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<Vec<NetworkProgram>>> + Send + '_>>;

    /// Generate an affiliate link for a given program + destination URL.
    fn generate_link(
        &self,
        program_external_id: &str,
        destination_url: &str,
        click_ref: &str,
    ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<String>> + Send + '_>>;

    /// Fetch conversion reports for a date range.
    fn fetch_conversions(
        &self,
        from_ms: i64,
        to_ms: i64,
    ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<Vec<NetworkConversion>>> + Send + '_>>;

    /// Check if the adapter's credentials are valid.
    fn validate_credentials(
        &self,
    ) -> Pin<Box<dyn Future<Output = AffiliateDomainResult<bool>> + Send + '_>>;
}

// ---------------------------------------------------------------------------
// Network registry
// ---------------------------------------------------------------------------

/// Registry of all configured network adapters.
/// Provider selection will eventually use commission, reliability,
/// conversion rate, latency, geography, and learned performance to
/// route to the best network for each click.
pub struct NetworkRegistry {
    adapters: Vec<Box<dyn NetworkAdapter>>,
}

impl NetworkRegistry {
    pub fn new() -> Self {
        Self { adapters: vec![] }
    }

    pub fn register(&mut self, adapter: Box<dyn NetworkAdapter>) {
        self.adapters.push(adapter);
    }

    pub fn get(&self, network_id: &NetworkId) -> Option<&dyn NetworkAdapter> {
        self.adapters
            .iter()
            .find(|a| a.info().id == *network_id)
            .map(|a| a.as_ref())
    }

    pub fn all(&self) -> &[Box<dyn NetworkAdapter>] {
        &self.adapters
    }

    pub fn count(&self) -> usize {
        self.adapters.len()
    }
}

impl Default for NetworkRegistry {
    fn default() -> Self {
        Self::new()
    }
}

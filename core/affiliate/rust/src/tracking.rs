//! Click tracking, link management, and UTM parameter handling.
//!
//! Inspired by OpenPartner's click router: every affiliate link click
//! is recorded as an immutable event with IP hash, user-agent, referer,
//! UTM parameters, and an optional fraud flag. The click ID is propagated
//! via a first-party cookie (`_cref`) and query parameter so the identity
//! stitching layer can correlate anonymous clicks with authenticated users.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AffiliateId, OfferId, ProgramId};

// ---------------------------------------------------------------------------
// IDs
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ClickId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct LinkId(pub Uuid);

// ---------------------------------------------------------------------------
// Link — the affiliate's shareable URL
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub id: LinkId,
    pub link_key: String,
    pub affiliate_id: AffiliateId,
    pub program_id: ProgramId,
    pub offer_id: Option<OfferId>,
    /// Override of the program's default destination URL.
    /// `None` = inherit from Program.
    pub destination_url: Option<String>,
    pub created_at: i64,
}

// ---------------------------------------------------------------------------
// Click — immutable raw event (never updated after creation)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ClickFraudFlag {
    /// IP velocity exceeded threshold.
    Velocity,
    /// Manually flagged by operator.
    Manual,
    /// Partner was revoked; click still redirects but won't attribute.
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtmParams {
    pub source: Option<String>,
    pub medium: Option<String>,
    pub campaign: Option<String>,
    pub term: Option<String>,
    pub content: Option<String>,
}

impl UtmParams {
    pub fn empty() -> Self {
        Self { source: None, medium: None, campaign: None, term: None, content: None }
    }

    /// Build a query string fragment from non-None fields.
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        if let Some(ref v) = self.source { pairs.push(("utm_source".into(), v.clone())); }
        if let Some(ref v) = self.medium { pairs.push(("utm_medium".into(), v.clone())); }
        if let Some(ref v) = self.campaign { pairs.push(("utm_campaign".into(), v.clone())); }
        if let Some(ref v) = self.term { pairs.push(("utm_term".into(), v.clone())); }
        if let Some(ref v) = self.content { pairs.push(("utm_content".into(), v.clone())); }
        pairs
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Click {
    pub id: ClickId,
    pub link_id: Option<LinkId>,
    pub affiliate_id: AffiliateId,
    pub program_id: ProgramId,
    pub landing_url: String,
    /// SHA-256 truncated to 32 hex chars. Never store raw IPs.
    pub ip_hash: Option<String>,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
    pub utm: UtmParams,
    /// ISO 3166-1 alpha-2 lowercase (e.g. "de", "us").
    pub country: Option<String>,
    pub fraud_flag: Option<ClickFraudFlag>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: i64,
}

// ---------------------------------------------------------------------------
// Identity stitching — correlates anonymous clicks with authenticated users
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct IdentityId(pub Uuid);

/// An external user identifier (from the merchant's auth system).
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExternalUserId(pub String);

/// Links a `ClickId` to an authenticated `ExternalUserId`.
/// Written when the user signs up or logs in with an active `cref`
/// cookie/param. Immutable after creation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Identity {
    pub id: IdentityId,
    pub click_id: ClickId,
    pub user_id: ExternalUserId,
    pub stitched_at_ms: i64,
}

// ---------------------------------------------------------------------------
// Velocity check (in-memory, not persisted)
// ---------------------------------------------------------------------------

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

/// Simple sliding-window velocity checker.
/// Tracks click timestamps per IP hash and flags when the rate
/// exceeds `max_clicks` within `window_seconds`.
pub struct VelocityChecker {
    max_clicks: usize,
    window_seconds: u64,
    buckets: Mutex<HashMap<String, Vec<Instant>>>,
}

impl VelocityChecker {
    pub fn new(max_clicks: usize, window_seconds: u64) -> Self {
        Self {
            max_clicks,
            window_seconds,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Returns `true` if the IP has exceeded the velocity threshold.
    pub fn check_and_record(&self, ip_hash: &str) -> bool {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();
        let window = std::time::Duration::from_secs(self.window_seconds);

        let timestamps = buckets.entry(ip_hash.to_string()).or_default();
        timestamps.retain(|t| now.duration_since(*t) < window);
        timestamps.push(now);

        timestamps.len() > self.max_clicks
    }
}

impl Default for VelocityChecker {
    fn default() -> Self {
        // 30 clicks per 60 seconds per IP
        Self::new(30, 60)
    }
}

// ---------------------------------------------------------------------------
// UTM link builder
// ---------------------------------------------------------------------------

/// Generates a UTM-tagged affiliate link for a specific platform and content type.
pub fn build_utm_link(
    base_url: &str,
    product_slug: &str,
    platform: &str,
    content_type: &str,
    variant: Option<&str>,
) -> String {
    let campaign = format!("{}-{}", product_slug, platform);
    let content = match variant {
        Some(v) => format!("{}-{}", content_type, v),
        None => content_type.to_string(),
    };

    let separator = if base_url.contains('?') { "&" } else { "?" };
    format!(
        "{}{}utm_source={}&utm_medium={}&utm_campaign={}&utm_content={}",
        base_url, separator, platform, content_type, campaign, content,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utm_link_with_no_existing_params() {
        let url = build_utm_link("https://example.com/ref/abc", "heygen", "linkedin", "review", None);
        assert!(url.contains("utm_source=linkedin"));
        assert!(url.contains("utm_campaign=heygen-linkedin"));
        assert!(url.starts_with("https://example.com/ref/abc?"));
    }

    #[test]
    fn utm_link_with_existing_params() {
        let url = build_utm_link("https://example.com/ref/abc?id=123", "semrush", "twitter", "thread", Some("v2"));
        assert!(url.contains("&utm_source=twitter"));
        assert!(url.contains("utm_content=thread-v2"));
    }

    #[test]
    fn velocity_checker_flags_excess() {
        let checker = VelocityChecker::new(2, 60);
        let ip = "abcdef1234567890";
        assert!(!checker.check_and_record(ip));
        assert!(!checker.check_and_record(ip));
        assert!(checker.check_and_record(ip)); // 3rd click exceeds limit of 2
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SCORE_SCALE: u32 = 10_000;

/// Validation bounds. These bound storage and downstream serialization
/// exposure; they are intentionally generous and are *not* business rules.
pub const MAX_SOURCE_LEN: usize = 128;
pub const MAX_EXTERNAL_ID_LEN: usize = 256;
pub const MAX_NAME_LEN: usize = 256;
pub const MAX_CANONICAL_KEY_LEN: usize = 512;
pub const MAX_CATEGORY_LEN: usize = 128;
pub const MAX_CURRENCY_LEN: usize = 16;
pub const MAX_DESTINATION_URL_LEN: usize = 2_048;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryCandidate {
    pub source: String,
    pub external_id: String,
    pub merchant_name: String,
    pub product_name: String,
    pub canonical_key: String,
    pub category: Option<String>,
    pub destination_url: String,
    pub currency: String,
    pub price_minor: Option<i64>,
    pub commission_bps: Option<u32>,
    pub demand_score: u32,
    pub competition_score: u32,
    pub freshness_score: u32,
    pub compliance_score: u32,
    pub observed_at_ms: u64,
}

impl DiscoveryCandidate {
    pub fn validate(&self) -> Result<(), DiscoveryError> {
        validate_bounded("source", &self.source, MAX_SOURCE_LEN)?;
        validate_bounded("external_id", &self.external_id, MAX_EXTERNAL_ID_LEN)?;
        validate_bounded("merchant_name", &self.merchant_name, MAX_NAME_LEN)?;
        validate_bounded("product_name", &self.product_name, MAX_NAME_LEN)?;
        validate_bounded("canonical_key", &self.canonical_key, MAX_CANONICAL_KEY_LEN)?;
        // Degenerate identity guard (see `canonical_key` internationalization
        // notes): a fully non-ASCII name normalizes to the bare separator,
        // which would collide across unrelated products. Fail closed instead.
        if !self
            .canonical_key
            .chars()
            .any(|character| character.is_ascii_alphanumeric())
        {
            return Err(DiscoveryError::MissingField("canonical_key"));
        }
        if let Some(category) = &self.category {
            validate_bounded("category", category, MAX_CATEGORY_LEN)?;
        }
        validate_bounded("currency", &self.currency, MAX_CURRENCY_LEN)?;
        validate_destination_url(&self.destination_url)?;
        validate_score(self.demand_score, "demand_score")?;
        validate_score(self.competition_score, "competition_score")?;
        validate_score(self.freshness_score, "freshness_score")?;
        validate_score(self.compliance_score, "compliance_score")?;
        if self.price_minor.is_some_and(|value| value < 0) {
            return Err(DiscoveryError::NegativeValue("price_minor"));
        }
        if self.commission_bps.is_some_and(|value| value > 100_000) {
            return Err(DiscoveryError::OutOfRange("commission_bps"));
        }
        Ok(())
    }

    pub fn opportunity_score(&self) -> u32 {
        let economic = self.commission_bps.unwrap_or(0).min(10_000);
        let competition = SCORE_SCALE - self.competition_score;
        let weighted = economic as u64 * 35
            + self.demand_score as u64 * 25
            + competition as u64 * 15
            + self.freshness_score as u64 * 15
            + self.compliance_score as u64 * 10;
        (weighted / 100).min(SCORE_SCALE as u64) as u32
    }
}

fn validate_bounded(
    field: &'static str,
    value: &str,
    max_len: usize,
) -> Result<(), DiscoveryError> {
    if value.trim().is_empty() {
        return Err(DiscoveryError::MissingField(field));
    }
    if value.chars().count() > max_len {
        return Err(DiscoveryError::TooLong {
            field,
            max: max_len,
        });
    }
    if value.chars().any(|character| character.is_control()) {
        return Err(DiscoveryError::InvalidCharacters(field));
    }
    Ok(())
}

/// Syntactic URL validation (no network I/O, no normalization that could
/// change identity). Requirements: `http`/`https` scheme (case-insensitive),
/// non-empty host, no whitespace or control characters, bounded length.
fn validate_destination_url(url: &str) -> Result<(), DiscoveryError> {
    if url.trim().is_empty() {
        return Err(DiscoveryError::MissingField("destination_url"));
    }
    if url.len() > MAX_DESTINATION_URL_LEN {
        return Err(DiscoveryError::TooLong {
            field: "destination_url",
            max: MAX_DESTINATION_URL_LEN,
        });
    }
    let Some((scheme, remainder)) = url.split_once("://") else {
        return Err(DiscoveryError::InvalidUrl("missing scheme separator"));
    };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return Err(DiscoveryError::InvalidUrl(
            "only http and https schemes are accepted",
        ));
    }
    let host = remainder.split(['/', '?', '#']).next().unwrap_or_default();
    if host.is_empty() {
        return Err(DiscoveryError::InvalidUrl("missing host"));
    }
    if host
        .chars()
        .any(|character| character.is_whitespace() || character.is_control())
    {
        return Err(DiscoveryError::InvalidUrl(
            "host must not contain whitespace or control characters",
        ));
    }
    if url.chars().any(|character| character.is_control()) {
        return Err(DiscoveryError::InvalidUrl(
            "control characters are not allowed",
        ));
    }
    Ok(())
}

fn validate_score(value: u32, field: &'static str) -> Result<(), DiscoveryError> {
    if value > SCORE_SCALE {
        return Err(DiscoveryError::OutOfRange(field));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryOpportunity {
    pub id: Uuid,
    pub candidate: DiscoveryCandidate,
    pub score: u32,
    pub rank: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryRequest {
    pub candidates: Vec<DiscoveryCandidate>,
    pub min_score: u32,
    pub limit: usize,
}

impl DiscoveryRequest {
    pub fn validate(&self) -> Result<(), DiscoveryError> {
        validate_score(self.min_score, "min_score")?;
        if self.limit == 0 {
            return Err(DiscoveryError::InvalidLimit);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DiscoveryResult {
    pub opportunities: Vec<DiscoveryOpportunity>,
    pub rejected: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DiscoveryError {
    MissingField(&'static str),
    NegativeValue(&'static str),
    OutOfRange(&'static str),
    InvalidLimit,
    TooLong { field: &'static str, max: usize },
    InvalidCharacters(&'static str),
    InvalidUrl(&'static str),
}

impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(formatter, "missing required field: {field}"),
            Self::NegativeValue(field) => {
                write!(formatter, "negative value is not allowed: {field}")
            }
            Self::OutOfRange(field) => write!(formatter, "value is out of range: {field}"),
            Self::InvalidLimit => formatter.write_str("discovery limit must be greater than zero"),
            Self::TooLong { field, max } => {
                write!(formatter, "field {field} exceeds {max} characters")
            }
            Self::InvalidCharacters(field) => {
                write!(formatter, "field {field} contains forbidden characters")
            }
            Self::InvalidUrl(reason) => write!(formatter, "destination_url is invalid: {reason}"),
        }
    }
}

impl std::error::Error for DiscoveryError {}

#[derive(Clone, Copy, Debug, Default)]
pub struct DiscoveryEngine;

impl DiscoveryEngine {
    pub fn discover(&self, request: DiscoveryRequest) -> Result<DiscoveryResult, DiscoveryError> {
        request.validate()?;

        let mut valid = Vec::with_capacity(request.candidates.len());
        let mut rejected = 0usize;

        for candidate in request.candidates {
            if candidate.validate().is_err() {
                rejected += 1;
                continue;
            }
            valid.push(candidate);
        }

        valid.sort_by(|left, right| {
            right
                .opportunity_score()
                .cmp(&left.opportunity_score())
                .then_with(|| left.canonical_key.cmp(&right.canonical_key))
                .then_with(|| left.source.cmp(&right.source))
                .then_with(|| left.external_id.cmp(&right.external_id))
        });

        let opportunities = valid
            .into_iter()
            .filter_map(|candidate| {
                let score = candidate.opportunity_score();
                (score >= request.min_score).then_some(candidate)
            })
            .take(request.limit)
            .enumerate()
            .map(|(index, candidate)| DiscoveryOpportunity {
                id: Uuid::now_v7(),
                score: candidate.opportunity_score(),
                rank: (index + 1) as u32,
                candidate,
            })
            .collect();

        Ok(DiscoveryResult {
            opportunities,
            rejected,
        })
    }
}

/// Deterministic canonical identity for a merchant/product pair.
///
/// # Internationalization limitation (documented, deliberate)
///
/// The current normalization is **ASCII-oriented**: lowercase mapping applies
/// to Unicode, but only ASCII alphanumerics and whitespace survive — every
/// other character is dropped. Consequences:
///
/// - `"Café"` normalizes to `"caf"`; `"北京"` normalizes to `""`;
/// - a fully non-ASCII name collapses toward `":"`, which candidate
///   validation then rejects (fail closed) — non-ASCII merchants cannot be
///   ingested today instead of being silently merged with unrelated products.
///
/// This preserves compatibility with every stored identity: changing the
/// normalization would rewrite canonical keys and break deduplication for
/// existing data. International-merchant support must arrive as a
/// deliberate, versioned identity scheme (e.g. a `canonical_key_v2` with
/// Unicode normalization and an explicit migration), never as a silent
/// change to this function.
pub fn canonical_key(merchant_name: &str, product_name: &str) -> String {
    format!(
        "{}:{}",
        normalize_key_part(merchant_name),
        normalize_key_part(product_name)
    )
}

/// ASCII-oriented normalization: see [`canonical_key`] for the documented
/// internationalization limitation.
fn normalize_key_part(value: &str) -> String {
    value
        .trim()
        .chars()
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_ascii_alphanumeric() || character.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(name: &str, score: u32) -> DiscoveryCandidate {
        DiscoveryCandidate {
            source: "test-source".into(),
            external_id: name.into(),
            merchant_name: "Acme".into(),
            product_name: name.into(),
            canonical_key: canonical_key("Acme", name),
            category: Some("electronics".into()),
            destination_url: "https://example.test/product".into(),
            currency: "EUR".into(),
            price_minor: Some(10_000),
            commission_bps: Some(10_000),
            demand_score: score,
            competition_score: SCORE_SCALE - score,
            freshness_score: score,
            compliance_score: SCORE_SCALE,
            observed_at_ms: 1,
        }
    }

    #[test]
    fn canonical_key_is_deterministic_and_normalized() {
        assert_eq!(
            canonical_key(" ACME ", "Noise Cancelling Headphones!"),
            "acme:noise-cancelling-headphones"
        );
    }

    #[test]
    fn canonical_key_ascii_limitation_is_documented_and_fail_closed() {
        // Documented limitation: non-ASCII letters are dropped today.
        assert_eq!(canonical_key("Café", "Crème"), "caf:crm");
        // Fully non-ASCII names collapse to the bare separator...
        assert_eq!(canonical_key("北京", "产品"), ":");
        // ...and candidate validation then rejects the blank identity
        // (fail closed) instead of merging unrelated products.
        let mut candidate = candidate("non-ascii", 5_000);
        candidate.canonical_key = canonical_key("北京", "产品");
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::MissingField("canonical_key"))
        ));
    }

    #[test]
    fn destination_urls_must_be_absolute_http_or_https() {
        let mut candidate = candidate("url-check", 5_000);

        candidate.destination_url = "ftp://example.test/file".into();
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::InvalidUrl(_))
        ));

        candidate.destination_url = "example.test/product".into();
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::InvalidUrl(_))
        ));

        candidate.destination_url = "https:///no-host".into();
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::InvalidUrl(_))
        ));

        candidate.destination_url = "https://exa mple.test/product".into();
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::InvalidUrl(_))
        ));

        candidate.destination_url = format!(
            "https://example.test/{}",
            "a".repeat(MAX_DESTINATION_URL_LEN)
        );
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::TooLong { .. })
        ));

        candidate.destination_url = "HTTPS://Example.Test/Product".into();
        assert!(
            candidate.validate().is_ok(),
            "scheme case is accepted without normalization"
        );
    }

    #[test]
    fn oversized_and_control_character_values_are_rejected() {
        let mut candidate = candidate("bounds", 5_000);
        candidate.source = "s".repeat(MAX_SOURCE_LEN + 1);
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::TooLong {
                field: "source",
                ..
            })
        ));

        let mut candidate = candidate("bounds", 5_000);
        candidate.external_id = "id\u{7f}ent".into();
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::InvalidCharacters("external_id"))
        ));

        let mut candidate = candidate("bounds", 5_000);
        candidate.currency = "EURURURURURURURUR".into(); // 17 characters
        assert!(matches!(
            candidate.validate(),
            Err(DiscoveryError::TooLong {
                field: "currency",
                ..
            })
        ));
    }

    #[test]
    fn invalid_candidates_are_rejected_without_failing_the_batch() {
        let mut invalid = candidate("invalid", 5_000);
        invalid.destination_url.clear();
        let result = DiscoveryEngine
            .discover(DiscoveryRequest {
                candidates: vec![invalid, candidate("valid", 9_000)],
                min_score: 0,
                limit: 10,
            })
            .expect("valid request");
        assert_eq!(result.rejected, 1);
        assert_eq!(result.opportunities.len(), 1);
        assert_eq!(result.opportunities[0].candidate.external_id, "valid");
    }

    #[test]
    fn ranking_is_deterministic_for_equal_scores() {
        let result = DiscoveryEngine
            .discover(DiscoveryRequest {
                candidates: vec![candidate("zeta", 8_000), candidate("alpha", 8_000)],
                min_score: 0,
                limit: 10,
            })
            .expect("valid request");
        assert_eq!(result.opportunities[0].candidate.external_id, "alpha");
        assert_eq!(result.opportunities[1].candidate.external_id, "zeta");
        assert_eq!(result.opportunities[0].rank, 1);
        assert_eq!(result.opportunities[1].rank, 2);
    }

    #[test]
    fn minimum_score_and_limit_are_enforced() {
        let result = DiscoveryEngine
            .discover(DiscoveryRequest {
                candidates: vec![candidate("high", 9_000), candidate("low", 2_000)],
                min_score: 7_000,
                limit: 1,
            })
            .expect("valid request");
        assert_eq!(result.opportunities.len(), 1);
        assert_eq!(result.opportunities[0].candidate.external_id, "high");
    }

    #[test]
    fn zero_limit_is_invalid() {
        let error = DiscoveryEngine
            .discover(DiscoveryRequest {
                candidates: Vec::new(),
                min_score: 0,
                limit: 0,
            })
            .expect_err("zero limit must fail");
        assert_eq!(error, DiscoveryError::InvalidLimit);
    }
}

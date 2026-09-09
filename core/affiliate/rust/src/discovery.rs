use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SCORE_SCALE: u32 = 10_000;

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
        if self.source.trim().is_empty() {
            return Err(DiscoveryError::MissingField("source"));
        }
        if self.external_id.trim().is_empty() {
            return Err(DiscoveryError::MissingField("external_id"));
        }
        if self.merchant_name.trim().is_empty() {
            return Err(DiscoveryError::MissingField("merchant_name"));
        }
        if self.product_name.trim().is_empty() {
            return Err(DiscoveryError::MissingField("product_name"));
        }
        if self.canonical_key.trim().is_empty() {
            return Err(DiscoveryError::MissingField("canonical_key"));
        }
        if self.destination_url.trim().is_empty() {
            return Err(DiscoveryError::MissingField("destination_url"));
        }
        if self.currency.trim().is_empty() {
            return Err(DiscoveryError::MissingField("currency"));
        }
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
}

impl std::fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingField(field) => write!(formatter, "missing required field: {field}"),
            Self::NegativeValue(field) => write!(formatter, "negative value is not allowed: {field}"),
            Self::OutOfRange(field) => write!(formatter, "value is out of range: {field}"),
            Self::InvalidLimit => formatter.write_str("discovery limit must be greater than zero"),
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

        Ok(DiscoveryResult { opportunities, rejected })
    }
}

pub fn canonical_key(merchant_name: &str, product_name: &str) -> String {
    format!(
        "{}:{}",
        normalize_key_part(merchant_name),
        normalize_key_part(product_name)
    )
}

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
        assert_eq!(canonical_key(" ACME ", "Noise Cancelling Headphones!"), "acme:noise-cancelling-headphones");
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

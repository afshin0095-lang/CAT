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
        for (value, field) in [
            (&self.source, "source"),
            (&self.external_id, "external_id"),
            (&self.merchant_name, "merchant_name"),
            (&self.product_name, "product_name"),
            (&self.canonical_key, "canonical_key"),
            (&self.destination_url, "destination_url"),
            (&self.currency, "currency"),
        ] {
            if value.trim().is_empty() {
                return Err(DiscoveryError::MissingField(field));
            }
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
            right.opportunity_score().cmp(&left.opportunity_score())
                .then_with(|| left.canonical_key.cmp(&right.canonical_key))
                .then_with(|| left.source.cmp(&right.source))
                .then_with(|| left.external_id.cmp(&right.external_id))
        });
        let opportunities = valid.into_iter()
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
    format!("{}:{}", normalize_key_part(merchant_name), normalize_key_part(product_name))
}

fn normalize_key_part(value: &str) -> String {
    value.trim().chars()
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_ascii_alphanumeric() || character.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

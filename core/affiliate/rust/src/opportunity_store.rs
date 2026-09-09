use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity};

/// Stable identity used to deduplicate the same merchant/product across sources.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct OpportunityIdentity(String);

impl OpportunityIdentity {
    pub fn new(candidate: &DiscoveryCandidate) -> Self {
        Self(canonical_key(&candidate.merchant_name, &candidate.product_name))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

/// One source's observation of a commercially discoverable opportunity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityObservation {
    pub source: String,
    pub external_id: String,
    pub destination_url: String,
    pub currency: String,
    pub price_minor: Option<i64>,
    pub commission_bps: Option<u32>,
    pub score: u32,
    pub observed_at_ms: u64,
}

impl OpportunityObservation {
    pub fn from_opportunity(opportunity: &DiscoveryOpportunity) -> Self {
        let candidate = &opportunity.candidate;
        Self {
            source: candidate.source.clone(),
            external_id: candidate.external_id.clone(),
            destination_url: candidate.destination_url.clone(),
            currency: candidate.currency.clone(),
            price_minor: candidate.price_minor,
            commission_bps: candidate.commission_bps,
            score: opportunity.score,
            observed_at_ms: candidate.observed_at_ms,
        }
    }
}

/// Persistence record retaining the aggregate opportunity and all source provenance.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityRecord {
    pub id: Uuid,
    pub identity: OpportunityIdentity,
    pub merchant_name: String,
    pub product_name: String,
    pub category: Option<String>,
    pub observations: BTreeMap<String, OpportunityObservation>,
    pub best_source: String,
    pub best_score: u32,
    pub first_observed_at_ms: u64,
    pub last_observed_at_ms: u64,
}

impl OpportunityRecord {
    pub fn from_opportunity(opportunity: &DiscoveryOpportunity) -> Self {
        let candidate = &opportunity.candidate;
        let observation = OpportunityObservation::from_opportunity(opportunity);
        let source = candidate.source.clone();
        let observed_at_ms = candidate.observed_at_ms;
        let mut observations = BTreeMap::new();
        observations.insert(source.clone(), observation);
        Self {
            id: opportunity.id,
            identity: OpportunityIdentity::new(candidate),
            merchant_name: candidate.merchant_name.clone(),
            product_name: candidate.product_name.clone(),
            category: candidate.category.clone(),
            observations,
            best_source: source,
            best_score: opportunity.score,
            first_observed_at_ms: observed_at_ms,
            last_observed_at_ms: observed_at_ms,
        }
    }

    fn merge(&mut self, opportunity: &DiscoveryOpportunity) -> OpportunityUpsertResult {
        let candidate = &opportunity.candidate;
        let identity = OpportunityIdentity::new(candidate);
        debug_assert_eq!(self.identity, identity);
        let source = candidate.source.clone();
        let observation = OpportunityObservation::from_opportunity(opportunity);
        let changed = self.observations.get(&source) != Some(&observation);
        self.observations.insert(source.clone(), observation);
        self.first_observed_at_ms = self.first_observed_at_ms.min(candidate.observed_at_ms);
        self.last_observed_at_ms = self.last_observed_at_ms.max(candidate.observed_at_ms);
        if opportunity.score > self.best_score
            || (opportunity.score == self.best_score && source < self.best_source)
        {
            self.best_score = opportunity.score;
            self.best_source = source;
        }
        if self.category.is_none() {
            self.category = candidate.category.clone();
        }
        OpportunityUpsertResult { created: false, changed }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityUpsertResult {
    pub created: bool,
    pub changed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpportunityStoreError {
    InvalidCandidate,
    IdentityConflict,
    NotFound,
}

impl std::fmt::Display for OpportunityStoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCandidate => formatter.write_str("opportunity candidate is invalid"),
            Self::IdentityConflict => formatter.write_str("opportunity identity conflicts with stored record"),
            Self::NotFound => formatter.write_str("opportunity was not found"),
        }
    }
}

impl std::error::Error for OpportunityStoreError {}

/// Persistence-neutral contract. SQL, document stores and event-sourced adapters implement this boundary.
pub trait OpportunityStore {
    fn upsert(&mut self, opportunity: DiscoveryOpportunity) -> Result<OpportunityUpsertResult, OpportunityStoreError>;
    fn get(&self, identity: &OpportunityIdentity) -> Result<OpportunityRecord, OpportunityStoreError>;
    fn list(&self) -> Vec<OpportunityRecord>;
}

#[derive(Default)]
pub struct InMemoryOpportunityStore {
    records: BTreeMap<OpportunityIdentity, OpportunityRecord>,
}

impl InMemoryOpportunityStore {
    pub fn new() -> Self { Self::default() }
}

impl OpportunityStore for InMemoryOpportunityStore {
    fn upsert(&mut self, opportunity: DiscoveryOpportunity) -> Result<OpportunityUpsertResult, OpportunityStoreError> {
        opportunity.candidate.validate().map_err(|_| OpportunityStoreError::InvalidCandidate)?;
        let identity = OpportunityIdentity::new(&opportunity.candidate);
        match self.records.get_mut(&identity) {
            Some(record) => Ok(record.merge(&opportunity)),
            None => {
                self.records.insert(identity, OpportunityRecord::from_opportunity(&opportunity));
                Ok(OpportunityUpsertResult { created: true, changed: true })
            }
        }
    }

    fn get(&self, identity: &OpportunityIdentity) -> Result<OpportunityRecord, OpportunityStoreError> {
        self.records.get(identity).cloned().ok_or(OpportunityStoreError::NotFound)
    }

    fn list(&self) -> Vec<OpportunityRecord> {
        self.records.values().cloned().collect()
    }
}

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity, OpportunityRevision, OpportunityRevisionError};

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
///
/// `revision` is the write counter for this aggregate (see
/// [`OpportunityRevision`]). It is bumped on every persisted *change* and is
/// the basis for optimistic concurrency in adapters; it is never derived from
/// lifecycle state.
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
    pub revision: OpportunityRevision,
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
            revision: OpportunityRevision::initial(),
        }
    }

    fn merge(&mut self, opportunity: &DiscoveryOpportunity) -> Result<OpportunityUpsertResult, OpportunityStoreError> {
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
        let revision = if changed {
            self.revision.next().map_err(OpportunityStoreError::RevisionOverflow)?
        } else {
            self.revision
        };
        Ok(OpportunityUpsertResult { created: false, changed, revision })
    }

    /// Returns the best-scoring observation, or `None` when the record has no
    /// observations (possible only for hand-built records, never for records
    /// produced by a store).
    pub fn best_observation(&self) -> Option<&OpportunityObservation> {
        self.observations.get(&self.best_source)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpportunityUpsertResult {
    pub created: bool,
    pub changed: bool,
    /// Aggregate revision *after* this upsert.
    pub revision: OpportunityRevision,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpportunityStoreError {
    InvalidCandidate,
    IdentityConflict,
    NotFound,
    /// Checked revision increment refused (never silently wraps).
    RevisionOverflow(OpportunityRevisionError),
}

impl std::fmt::Display for OpportunityStoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCandidate => formatter.write_str("opportunity candidate is invalid"),
            Self::IdentityConflict => formatter.write_str("opportunity identity conflicts with stored record"),
            Self::NotFound => formatter.write_str("opportunity was not found"),
            Self::RevisionOverflow(error) => write!(formatter, "opportunity revision update failed: {error}"),
        }
    }
}

impl std::error::Error for OpportunityStoreError {}

/// Persistence-neutral contract. SQL, document stores and event-sourced adapters implement this boundary.
///
/// Idempotency: upserting the same logical observation repeatedly converges
/// to the same record state; an unchanged re-observation reports
/// `changed == false` and leaves the revision untouched.
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
            Some(record) => record.merge(&opportunity),
            None => {
                let revision = OpportunityRevision::initial();
                self.records.insert(identity, OpportunityRecord::from_opportunity(&opportunity));
                Ok(OpportunityUpsertResult { created: true, changed: true, revision })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn opportunity(score: u32, observed_at_ms: u64) -> DiscoveryOpportunity {
        let candidate = DiscoveryCandidate {
            source: "network-a".into(),
            external_id: "a-1".into(),
            merchant_name: "Acme".into(),
            product_name: "Widget".into(),
            canonical_key: canonical_key("Acme", "Widget"),
            category: Some("electronics".into()),
            destination_url: "https://example.test/widget".into(),
            currency: "EUR".into(),
            price_minor: Some(10_000),
            commission_bps: Some(500),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms,
        };
        DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score, rank: 1 }
    }

    #[test]
    fn created_records_start_at_initial_revision() {
        let mut store = InMemoryOpportunityStore::new();
        let result = store.upsert(opportunity(7_000, 100)).expect("valid upsert");
        assert!(result.created);
        assert_eq!(result.revision, OpportunityRevision::initial());
    }

    #[test]
    fn revision_increments_only_on_change() {
        let mut store = InMemoryOpportunityStore::new();
        let first = store.upsert(opportunity(7_000, 100)).expect("valid upsert");
        assert_eq!(first.revision.get(), 1);

        // Identical re-observation: no change, no revision bump (idempotent).
        let repeat = store.upsert(opportunity(7_000, 100)).expect("valid upsert");
        assert!(!repeat.changed);
        assert_eq!(repeat.revision.get(), 1);

        // Changed observation: revision increments deterministically.
        let changed = store.upsert(opportunity(9_000, 200)).expect("valid upsert");
        assert!(changed.changed);
        assert_eq!(changed.revision.get(), 2);
    }

    #[test]
    fn best_observation_resolves_the_best_source() {
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity(9_000, 100)).expect("valid upsert");
        let record = store.list().into_iter().next().expect("record exists");
        let observation = record.best_observation().expect("best observation");
        assert_eq!(observation.source, record.best_source);
        assert_eq!(observation.score, 9_000);
    }
}

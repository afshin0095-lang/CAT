//! Persistence-neutral opportunity query boundary.
//!
//! `OpportunityQuery` composes filters, sorting, and pagination as a domain
//! object. It never builds SQL strings: adapters translate the query into
//! their own storage primitives. Evaluation over an `OpportunityStore` is
//! deterministic and stable.
//!
//! Pagination contract: `offset + limit` windows over the deterministic
//! sorted match set; `total_matched` reports the unpaginated match count so
//! clients never need a second query. Offsets saturate instead of wrapping.

use serde::{Deserialize, Serialize};

use crate::opportunity_freshness::{FreshnessPolicy, FreshnessState};
use crate::opportunity_projection::{OpportunityProjectionError, OpportunityStatusProjector, OpportunityStatusView};
use crate::OpportunityRecord;

/// Hard upper bound on a single page; keeps accidental `limit = u32::MAX`
/// queries from materializing unbounded views.
pub const MAX_QUERY_LIMIT: u32 = 1_000;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityFilter {
    pub merchant: Option<String>,
    pub source: Option<String>,
    pub category: Option<String>,
    pub min_score: Option<u32>,
    pub lifecycle_state: Option<FreshnessState>,
    pub observed_after_ms: Option<u64>,
    pub observed_before_ms: Option<u64>,
    pub currency: Option<String>,
    pub price_min_minor: Option<i64>,
    pub price_max_minor: Option<i64>,
    pub commission_min_bps: Option<u32>,
    pub commission_max_bps: Option<u32>,
}

impl OpportunityFilter {
    pub fn matches(&self, record: &OpportunityRecord, lifecycle_state: FreshnessState) -> bool {
        if let Some(merchant) = &self.merchant {
            if record.merchant_name != *merchant {
                return false;
            }
        }
        if let Some(category) = &self.category {
            if record.category.as_deref() != Some(category.as_str()) {
                return false;
            }
        }
        if let Some(min_score) = self.min_score {
            if record.best_score < min_score {
                return false;
            }
        }
        if let Some(expected) = self.lifecycle_state {
            if lifecycle_state != expected {
                return false;
            }
        }
        if let Some(after) = self.observed_after_ms {
            if record.last_observed_at_ms <= after {
                return false;
            }
        }
        if let Some(before) = self.observed_before_ms {
            if record.last_observed_at_ms >= before {
                return false;
            }
        }
        if let Some(source) = &self.source {
            if !record.observations.contains_key(source.as_str()) {
                return false;
            }
        }
        if let Some(currency) = &self.currency {
            if !record
                .observations
                .values()
                .any(|observation| observation.currency == *currency)
            {
                return false;
            }
        }
        self.matches_ranges(record)
    }

    fn matches_ranges(&self, record: &OpportunityRecord) -> bool {
        let price = (self.price_min_minor, self.price_max_minor);
        let commission = (self.commission_min_bps, self.commission_max_bps);
        if price == (None, None) && commission == (None, None) {
            return true;
        }
        record.observations.values().any(|observation| {
            let price_ok = match (self.price_min_minor, self.price_max_minor) {
                (None, None) => true,
                (min, max) => observation
                    .price_minor
                    .is_some_and(|value| min.is_none_or(|bound| value >= bound) && max.is_none_or(|bound| value <= bound)),
            };
            let commission_ok = match (self.commission_min_bps, self.commission_max_bps) {
                (None, None) => true,
                (min, max) => observation
                    .commission_bps
                    .is_some_and(|value| min.is_none_or(|bound| value >= bound) && max.is_none_or(|bound| value <= bound)),
            };
            price_ok && commission_ok
        })
    }

    pub fn validate(&self) -> Result<(), OpportunityQueryError> {
        if let (Some(after), Some(before)) = (self.observed_after_ms, self.observed_before_ms) {
            if after > before {
                return Err(OpportunityQueryError::InvalidFilter(
                    "observed_after_ms must not exceed observed_before_ms".into(),
                ));
            }
        }
        if let (Some(min), Some(max)) = (self.price_min_minor, self.price_max_minor) {
            if min > max {
                return Err(OpportunityQueryError::InvalidFilter("price_min_minor must not exceed price_max_minor".into()));
            }
        }
        if let (Some(min), Some(max)) = (self.commission_min_bps, self.commission_max_bps) {
            if min > max {
                return Err(OpportunityQueryError::InvalidFilter(
                    "commission_min_bps must not exceed commission_max_bps".into(),
                ));
            }
        }
        if let Some(min_score) = self.min_score {
            if min_score > 10_000 {
                return Err(OpportunityQueryError::InvalidFilter("min_score must be within 0..=10000".into()));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpportunitySortField {
    Score,
    LastObservedAt,
    Identity,
    MerchantName,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Ascending,
    Descending,
}

/// Deterministic sort specification. `OpportunityIdentity` ascending is
/// always appended as the final tie-breaker so output ordering never depends
/// on storage iteration order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunitySort {
    pub field: OpportunitySortField,
    pub direction: SortDirection,
}

impl Default for OpportunitySort {
    fn default() -> Self {
        Self {
            field: OpportunitySortField::Score,
            direction: SortDirection::Descending,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OpportunityQuery {
    pub filter: OpportunityFilter,
    pub sort: OpportunitySort,
    /// Page size; must be within `1..=MAX_QUERY_LIMIT`.
    pub limit: u32,
    /// Offset into the deterministic match set; saturates at its end.
    pub offset: u64,
}

impl OpportunityQuery {
    pub fn new(filter: OpportunityFilter) -> Self {
        Self {
            filter,
            sort: OpportunitySort::default(),
            limit: 50,
            offset: 0,
        }
    }

    pub fn with_sort(mut self, sort: OpportunitySort) -> Self {
        self.sort = sort;
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }

    pub fn validate(&self) -> Result<(), OpportunityQueryError> {
        self.filter.validate()?;
        if self.limit == 0 || self.limit > MAX_QUERY_LIMIT {
            return Err(OpportunityQueryError::InvalidLimit);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpportunityQueryResult {
    pub items: Vec<OpportunityStatusView>,
    /// Count of all matching opportunities before pagination.
    pub total_matched: u64,
    pub offset: u64,
    pub limit: u32,
}

impl OpportunityQueryResult {
    pub fn has_more(&self) -> bool {
        self.offset + (self.items.len() as u64) < self.total_matched
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpportunityQueryError {
    InvalidFilter(String),
    InvalidLimit,
    ClockBeforeObservation,
}

impl std::fmt::Display for OpportunityQueryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFilter(message) => write!(formatter, "invalid opportunity filter: {message}"),
            Self::InvalidLimit => write!(
                formatter,
                "opportunity query limit must be within 1..={MAX_QUERY_LIMIT}"
            ),
            Self::ClockBeforeObservation => {
                formatter.write_str("query time cannot precede an opportunity's last observation")
            }
        }
    }
}

impl std::error::Error for OpportunityQueryError {}

impl From<OpportunityProjectionError> for OpportunityQueryError {
    fn from(error: OpportunityProjectionError) -> Self {
        match error {
            OpportunityProjectionError::ClockBeforeObservation => Self::ClockBeforeObservation,
            OpportunityProjectionError::InvalidPolicy => Self::InvalidFilter(
                "freshness policy violates 0 <= active <= stale < expiration".into(),
            ),
        }
    }
}

/// Executes `OpportunityQuery`s over in-memory record slices. Storage
/// adapters implement the same semantics natively; this reference
/// implementation defines the expected behavior.
pub struct OpportunityQueryService {
    projector: OpportunityStatusProjector,
}

impl OpportunityQueryService {
    pub fn new(policy: FreshnessPolicy) -> Self {
        Self {
            projector: OpportunityStatusProjector::new(policy),
        }
    }

    /// Runs the query against `records` at evaluation time `now_ms`.
    ///
    /// Only the returned page is projected into full status views; filtering
    /// and sorting operate on cheap record fields, matching what a database
    /// adapter can push down.
    pub fn execute(
        &self,
        records: &[OpportunityRecord],
        query: &OpportunityQuery,
        now_ms: u64,
    ) -> Result<OpportunityQueryResult, OpportunityQueryError> {
        query.validate()?;
        let policy = self.projector.policy();

        let mut matched: Vec<&OpportunityRecord> = Vec::with_capacity(records.len());
        for record in records {
            // Fail closed if any record's clock is ahead of the query time.
            let lifecycle_state = policy
                .evaluate(record, now_ms)
                .map_err(|_| OpportunityQueryError::ClockBeforeObservation)?
                .state();
            if query.filter.matches(record, lifecycle_state) {
                matched.push(record);
            }
        }

        let total_matched = matched.len() as u64;
        matched.sort_by(|left, right| self.compare(left, right, query.sort));

        let offset = query.offset.min(total_matched);
        let end = offset.saturating_add(u64::from(query.limit)).min(total_matched);
        let mut items = Vec::with_capacity((end - offset) as usize);
        for record in &matched[offset as usize..end as usize] {
            items.push(self.projector.project(record, now_ms)?);
        }

        Ok(OpportunityQueryResult {
            items,
            total_matched,
            offset,
            limit: query.limit,
        })
    }

    fn compare(&self, left: &OpportunityRecord, right: &OpportunityRecord, sort: OpportunitySort) -> std::cmp::Ordering {
        use OpportunitySortField::*;
        use std::cmp::Ordering;
        let primary = match sort.field {
            Score => left.best_score.cmp(&right.best_score),
            LastObservedAt => left.last_observed_at_ms.cmp(&right.last_observed_at_ms),
            Identity => left.identity.cmp(&right.identity),
            MerchantName => left.merchant_name.cmp(&right.merchant_name),
        };
        let directed = match sort.direction {
            SortDirection::Ascending => primary,
            SortDirection::Descending => primary.reverse(),
        };
        directed.then_with(|| left.identity.cmp(&right.identity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{canonical_key, DiscoveryCandidate, DiscoveryOpportunity, InMemoryOpportunityStore, OpportunityStore};
    use uuid::Uuid;

    fn build_record(merchant: &str, product: &str, score: u32, observed_at_ms: u64) -> OpportunityRecord {
        let candidate = DiscoveryCandidate {
            source: "network-a".into(),
            external_id: "sku-1".into(),
            merchant_name: merchant.into(),
            product_name: product.into(),
            canonical_key: canonical_key(merchant, product),
            category: Some("electronics".into()),
            destination_url: "https://example.test/item".into(),
            currency: "EUR".into(),
            price_minor: Some(9_999),
            commission_bps: Some(600),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms,
        };
        let opportunity = DiscoveryOpportunity { id: Uuid::now_v7(), candidate, score, rank: 1 };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store.list().into_iter().next().expect("record exists")
    }

    fn service() -> OpportunityQueryService {
        OpportunityQueryService::new(FreshnessPolicy::new(1_000, 5_000, 9_000).expect("valid policy"))
    }

    fn default_query() -> OpportunityQuery {
        OpportunityQuery::new(OpportunityFilter::default()).with_limit(10)
    }

    #[test]
    fn query_is_invalid_without_limit_or_with_excess_limit() {
        assert_eq!(default_query().with_limit(0).validate(), Err(OpportunityQueryError::InvalidLimit));
        assert_eq!(
            default_query().with_limit(MAX_QUERY_LIMIT + 1).validate(),
            Err(OpportunityQueryError::InvalidLimit)
        );
        assert_eq!(default_query().with_limit(MAX_QUERY_LIMIT).validate(), Ok(()));
    }

    #[test]
    fn invalid_filter_bounds_are_rejected() {
        let mut filter = OpportunityFilter::default();
        filter.observed_after_ms = Some(100);
        filter.observed_before_ms = Some(50);
        assert!(matches!(
            OpportunityQuery::new(filter).validate(),
            Err(OpportunityQueryError::InvalidFilter(_))
        ));

        let mut filter = OpportunityFilter::default();
        filter.commission_min_bps = Some(700);
        filter.commission_max_bps = Some(300);
        assert!(matches!(
            OpportunityQuery::new(filter).validate(),
            Err(OpportunityQueryError::InvalidFilter(_))
        ));

        let mut filter = OpportunityFilter::default();
        filter.min_score = Some(10_001);
        assert!(matches!(
            OpportunityQuery::new(filter).validate(),
            Err(OpportunityQueryError::InvalidFilter(_))
        ));
    }

    #[test]
    fn filters_compose() {
        let service = service();
        let records = vec![
            build_record("Acme", "Widget", 9_000, 10_000),
            build_record("Beta", "Gadget", 7_000, 10_000),
            build_record("Beta", "Doohickey", 5_000, 10_000),
        ];

        let mut filter = OpportunityFilter::default();
        filter.merchant = Some("Beta".into());
        filter.min_score = Some(6_000);
        let query = OpportunityQuery::new(filter).with_limit(10);
        let result = service.execute(&records, &query, 10_500).expect("valid query");
        assert_eq!(result.total_matched, 1);
        assert_eq!(result.items[0].product_name, "Gadget");

        let mut filter = OpportunityFilter::default();
        filter.lifecycle_state = Some(FreshnessState::Active);
        let query = OpportunityQuery::new(filter).with_limit(10);
        let result = service.execute(&records, &query, 10_500).expect("valid query");
        assert_eq!(result.total_matched, 3);
    }

    #[test]
    fn observation_filters_match_any_source_observation() {
        let service = service();
        let stored = build_record("Acme", "Widget", 9_000, 10_000);
        // Give the record a second observation from another source.
        let candidate = DiscoveryCandidate {
            source: "network-b".into(),
            external_id: "sku-9".into(),
            merchant_name: "Acme".into(),
            product_name: "Widget".into(),
            canonical_key: canonical_key("Acme", "Widget"),
            category: Some("electronics".into()),
            destination_url: "https://example.test/widget-b".into(),
            currency: "USD".into(),
            price_minor: Some(19_999),
            commission_bps: Some(200),
            demand_score: 8_000,
            competition_score: 2_000,
            freshness_score: 10_000,
            compliance_score: 10_000,
            observed_at_ms: 10_000,
        };
        let opportunity = DiscoveryOpportunity { id: stored.id, candidate, score: 9_500, rank: 1 };
        let mut store = InMemoryOpportunityStore::new();
        store.upsert(opportunity).expect("valid opportunity");
        store
            .upsert(crate::DiscoveryOpportunity {
                id: stored.id,
                candidate: {
                    let mut base = candidate;
                    base.source = "network-a".into();
                    base.external_id = "sku-1".into();
                    base.destination_url = "https://example.test/item".into();
                    base.currency = "EUR".into();
                    base.price_minor = Some(9_999);
                    base.commission_bps = Some(600);
                    base
                },
                score: 9_000,
                rank: 1,
            })
            .expect("valid opportunity");
        let stored = store.list().into_iter().next().expect("record exists");

        let mut filter = OpportunityFilter::default();
        filter.currency = Some("USD".into());
        let query = OpportunityQuery::new(filter).with_limit(10);
        let result = service.execute(std::slice::from_ref(&stored), &query, 10_500).expect("valid query");
        assert_eq!(result.total_matched, 1);

        let mut filter = OpportunityFilter::default();
        filter.commission_min_bps = Some(500);
        filter.commission_max_bps = Some(700);
        let query = OpportunityQuery::new(filter).with_limit(10);
        let result = service.execute(std::slice::from_ref(&stored), &query, 10_500).expect("valid query");
        assert_eq!(result.total_matched, 1, "network-a observation has 600 bps");

        let mut filter = OpportunityFilter::default();
        filter.price_min_minor = Some(15_000);
        let query = OpportunityQuery::new(filter).with_limit(10);
        let result = service.execute(std::slice::from_ref(&stored), &query, 10_500).expect("valid query");
        assert_eq!(result.total_matched, 1, "network-b observation has 19999 minor units");
    }

    #[test]
    fn sorting_is_deterministic_with_identity_tiebreak() {
        let service = service();
        let records = vec![
            build_record("Acme", "Widget", 7_000, 10_000),
            build_record("Beta", "Gadget", 9_000, 9_000),
            build_record("Acme", "Gizmo", 7_000, 11_000),
        ];

        let query = default_query(); // Score descending
        let result = service.execute(&records, &query, 11_500).expect("valid query");
        let identities: Vec<&str> = result
            .items
            .iter()
            .map(|view| view.identity.as_str())
            .collect();
        assert_eq!(
            identities,
            vec!["beta:gadget", "acme:gizmo", "acme:widget"],
            "score desc (9000, 7000, 7000), then identity asc tiebreak"
        );

        let query = default_query().with_sort(OpportunitySort {
            field: OpportunitySortField::LastObservedAt,
            direction: SortDirection::Ascending,
        });
        let result = service.execute(&records, &query, 11_500).expect("valid query");
        assert_eq!(result.items[0].identity, "beta:gadget", "oldest observation first");
    }

    #[test]
    fn pagination_windows_are_stable_and_report_total() {
        let service = service();
        let records: Vec<OpportunityRecord> = (0..7)
            .map(|index| build_record("Acme", &format!("Item {index}"), 5_000 + index as u32 * 100, 10_000))
            .collect();

        let page_one = service
            .execute(&records, &default_query().with_limit(3), 10_500)
            .expect("valid query");
        assert_eq!(page_one.items.len(), 3);
        assert_eq!(page_one.total_matched, 7);
        assert!(page_one.has_more());

        let page_three = service
            .execute(&records, &default_query().with_limit(3).with_offset(6), 10_500)
            .expect("valid query");
        assert_eq!(page_three.items.len(), 1);
        assert!(!page_three.has_more());

        let beyond = service
            .execute(&records, &default_query().with_limit(3).with_offset(100), 10_500)
            .expect("valid query");
        assert!(beyond.items.is_empty());
        assert_eq!(beyond.offset, 7, "offset saturates at the match set size");
        assert_eq!(beyond.total_matched, 7);
    }

    #[test]
    fn query_clock_regression_fails_closed() {
        let service = service();
        let records = vec![build_record("Acme", "Widget", 9_000, 10_000)];
        let result = service.execute(&records, &default_query(), 9_999);
        assert_eq!(result, Err(OpportunityQueryError::ClockBeforeObservation));
    }
}

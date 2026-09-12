//! Opportunity revalidation contracts (persistence-neutral).
//!
//! This layer **decides** what should be revalidated, why, with what
//! priority, and against which source. It never performs the revalidation:
//! external execution stays behind the existing `DiscoverySource` /
//! `NetworkAdapter` boundaries (Sprint 1 maps requests into the Orchestrator).
//!
//! Persistence model discipline:
//! - a **request** is the intent to refresh (this module);
//! - an **attempt** is one execution of a request (Sprint 1 / Orchestrator);
//! - a **result** is the observed outcome merged back into opportunity facts.
//! These are never conflated.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Deduplication window for revalidation requests (one hour).
///
/// Two planning passes produce the same `dedup_key` when they request the
/// same (identity, source, reason) within the same window bucket, so repeated
/// planning converges instead of stacking duplicate requests.
pub const REVALIDATION_DEDUP_WINDOW_MS: u64 = 60 * 60 * 1000;

/// Why a revalidation is being requested.
///
/// Serialization is forward compatible: unknown reason strings from a newer
/// writer deserialize into [`RevalidationReason::Unknown`] instead of failing,
/// so older readers never reject messages they do not understand. Strict
/// parsing (rejecting unknown values, fail closed) is available through
/// [`RevalidationReason::parse_strict`].
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum RevalidationReason {
    Stale,
    Expired,
    PriceChanged,
    CommissionChanged,
    DestinationChanged,
    ProviderUnavailable,
    ManualReview,
    PeriodicRefresh,
    /// A reason introduced by a newer schema version.
    Unknown(String),
}

impl RevalidationReason {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::PriceChanged => "price_changed",
            Self::CommissionChanged => "commission_changed",
            Self::DestinationChanged => "destination_changed",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::ManualReview => "manual_review",
            Self::PeriodicRefresh => "periodic_refresh",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Strict parsing: unknown strings are rejected (fail closed). Use this
    /// on write paths; use the serde deserializer on read paths.
    pub fn parse_strict(value: &str) -> Option<Self> {
        match value {
            "stale" => Some(Self::Stale),
            "expired" => Some(Self::Expired),
            "price_changed" => Some(Self::PriceChanged),
            "commission_changed" => Some(Self::CommissionChanged),
            "destination_changed" => Some(Self::DestinationChanged),
            "provider_unavailable" => Some(Self::ProviderUnavailable),
            "manual_review" => Some(Self::ManualReview),
            "periodic_refresh" => Some(Self::PeriodicRefresh),
            _ => None,
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

impl std::fmt::Display for RevalidationReason {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for RevalidationReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RevalidationReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Ok(Self::parse_strict(&value).unwrap_or(Self::Unknown(value)))
    }
}

/// Urgency of a revalidation request. Ordering is explicit:
/// `Critical > High > Normal > Low`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl RevalidationPriority {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }

    pub fn parse_strict(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "normal" => Some(Self::Normal),
            "high" => Some(Self::High),
            "critical" => Some(Self::Critical),
            _ => None,
        }
    }
}

impl std::fmt::Display for RevalidationPriority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// What should be revalidated, and against which source.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RevalidationTarget {
    pub identity: String,
    pub source: String,
}

impl RevalidationTarget {
    pub fn new(identity: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            identity: identity.into(),
            source: source.into(),
        }
    }
}

/// A durable intent to revalidate one opportunity against one source.
///
/// Identity and idempotency:
/// - `request_id` is unique per construction (UUID v7) and is *not* the
///   idempotency identity;
/// - `dedup_key` is the idempotency identity: `(identity, source, reason)`
///   bucketed into [`REVALIDATION_DEDUP_WINDOW_MS`] windows of
///   `scheduled_for_ms`. Repeated planning of the same fact within a window
///   yields the same key; stores use it to suppress duplicates.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RevalidationRequest {
    pub request_id: Uuid,
    pub opportunity_id: Uuid,
    pub target: RevalidationTarget,
    pub reason: RevalidationReason,
    pub priority: RevalidationPriority,
    pub requested_at_ms: u64,
    pub scheduled_for_ms: u64,
    pub dedup_key: String,
}

impl RevalidationRequest {
    /// Constructs a request with the default deduplication window.
    pub fn new(
        opportunity_id: Uuid,
        target: RevalidationTarget,
        reason: RevalidationReason,
        priority: RevalidationPriority,
        requested_at_ms: u64,
        scheduled_for_ms: u64,
    ) -> Self {
        Self::with_dedup_window(
            opportunity_id,
            target,
            reason,
            priority,
            requested_at_ms,
            scheduled_for_ms,
            REVALIDATION_DEDUP_WINDOW_MS,
        )
    }

    /// Constructs a request with an explicit deduplication window.
    /// `window_ms == 0` disables bucketing (each plan creates a distinct key).
    pub fn with_dedup_window(
        opportunity_id: Uuid,
        target: RevalidationTarget,
        reason: RevalidationReason,
        priority: RevalidationPriority,
        requested_at_ms: u64,
        scheduled_for_ms: u64,
        window_ms: u64,
    ) -> Self {
        let bucket = if window_ms == 0 {
            scheduled_for_ms
        } else {
            scheduled_for_ms.saturating_sub(scheduled_for_ms % window_ms)
        };
        let request_id = Uuid::now_v7();
        Self {
            dedup_key: format!(
                "opp-revalidation:{}:{}:{}:{}",
                target.identity,
                target.source,
                reason.as_str(),
                bucket
            ),
            request_id,
            opportunity_id,
            target,
            reason,
            priority,
            requested_at_ms,
            scheduled_for_ms,
        }
    }
}

/// Why a planned revalidation could not produce any request at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationBlockReason {
    /// The record carries no observation, so no source can be targeted.
    NoKnownSources,
    /// Every known source is currently unavailable.
    AllSourcesUnavailable,
}

impl RevalidationBlockReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoKnownSources => "no_known_sources",
            Self::AllSourcesUnavailable => "all_sources_unavailable",
        }
    }
}

/// Why one source was skipped while planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationSkipReason {
    SourceUnavailable,
}

/// Deterministic planning output for one opportunity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevalidationDecision {
    pub identity: String,
    pub opportunity_id: Uuid,
    pub evaluated_at_ms: u64,
    pub requests: Vec<RevalidationRequest>,
    pub skipped: Vec<(String, RevalidationSkipReason)>,
    pub blocked: Option<RevalidationBlockReason>,
}

impl RevalidationDecision {
    pub fn requires_revalidation(&self) -> bool {
        !self.requests.is_empty()
    }

    /// Sorted, de-duplicated reasons across all produced requests.
    pub fn reasons(&self) -> Vec<String> {
        let mut reasons: Vec<String> = self
            .requests
            .iter()
            .map(|request| request.reason.as_str().to_owned())
            .collect();
        reasons.sort();
        reasons.dedup();
        reasons
    }
}

/// Lifecycle of a persisted revalidation request.
///
/// Legal transitions (everything else is rejected, fail closed):
///
/// ```text
/// Pending   -> Claimed | Running | Cancelled
/// Claimed   -> Running | Cancelled
/// Running   -> Succeeded | Failed | DeadLettered | Cancelled
/// Failed    -> Pending (retry) | DeadLettered
/// Succeeded | Cancelled | DeadLettered  (terminal)
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevalidationStatus {
    Pending,
    Claimed,
    Running,
    Succeeded,
    Failed,
    Cancelled,
    DeadLettered,
}

impl RevalidationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Claimed => "claimed",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::DeadLettered => "dead_lettered",
        }
    }

    pub fn parse_strict(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "claimed" => Some(Self::Claimed),
            "running" => Some(Self::Running),
            "succeeded" => Some(Self::Succeeded),
            "failed" => Some(Self::Failed),
            "cancelled" => Some(Self::Cancelled),
            "dead_lettered" => Some(Self::DeadLettered),
            _ => None,
        }
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Cancelled | Self::DeadLettered)
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        use RevalidationStatus::*;
        matches!(
            (self, next),
            (Pending, Claimed)
                | (Pending, Running)
                | (Pending, Cancelled)
                | (Claimed, Running)
                | (Claimed, Cancelled)
                | (Running, Succeeded)
                | (Running, Failed)
                | (Running, DeadLettered)
                | (Running, Cancelled)
                | (Failed, Pending)
                | (Failed, DeadLettered)
        )
    }
}

impl std::fmt::Display for RevalidationStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Persisted revalidation request state: the REQUEST plus its bookkeeping.
/// Attempts and results remain separate concerns (Sprint 1).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RevalidationRequestRecord {
    pub request: RevalidationRequest,
    pub status: RevalidationStatus,
    pub attempt: u32,
    pub created_at_ms: u64,
    pub started_at_ms: Option<u64>,
    pub completed_at_ms: Option<u64>,
    /// Bounded, sanitized failure note; never a raw provider payload.
    pub last_error: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RevalidationStoreError {
    /// A request with the same `dedup_key` is already active.
    DuplicateRequest {
        dedup_key: String,
    },
    NotFound,
    InvalidTransition {
        from: RevalidationStatus,
        to: RevalidationStatus,
    },
    /// Persisted reason/status strings that this version cannot understand.
    UnknownEnumValue {
        field: &'static str,
        value: String,
    },
    InvalidRequest(String),
}

impl std::fmt::Display for RevalidationStoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateRequest { dedup_key } => {
                write!(
                    formatter,
                    "revalidation request already active: {dedup_key}"
                )
            }
            Self::NotFound => formatter.write_str("revalidation request was not found"),
            Self::InvalidTransition { from, to } => {
                write!(formatter, "illegal revalidation transition: {from} -> {to}")
            }
            Self::UnknownEnumValue { field, value } => {
                write!(formatter, "unknown {field} value: {value}")
            }
            Self::InvalidRequest(message) => {
                write!(formatter, "invalid revalidation request: {message}")
            }
        }
    }
}

impl std::error::Error for RevalidationStoreError {}

/// Persistence-neutral contract for revalidation requests.
///
/// Implementations must be idempotent on `dedup_key`: inserting a request
/// whose key matches an active (non-terminal) request is a
/// [`RevalidationStoreError::DuplicateRequest`], not a new row.
pub trait RevalidationRequestStore {
    fn insert(
        &mut self,
        request: RevalidationRequest,
    ) -> Result<RevalidationRequestRecord, RevalidationStoreError>;
    fn get(&self, request_id: Uuid) -> Result<RevalidationRequestRecord, RevalidationStoreError>;
    fn find_by_dedup_key(&self, dedup_key: &str) -> Option<RevalidationRequestRecord>;
    /// Applies a checked status transition; terminal statuses reject further
    /// transitions. `attempt` increments on the first `Running` transition.
    fn transition(
        &mut self,
        request_id: Uuid,
        to: RevalidationStatus,
        at_ms: u64,
        error: Option<String>,
    ) -> Result<RevalidationRequestRecord, RevalidationStoreError>;
    /// Deterministically claims due pending requests:
    /// `scheduled_for_ms <= now_ms`, ordered by `(priority desc, scheduled_for_ms asc, identity asc, request_id asc)`.
    fn claim_due(&mut self, now_ms: u64, limit: usize) -> Vec<RevalidationRequestRecord>;
    /// All records, deterministically ordered by `(created_at_ms, request_id)`.
    fn list(&self) -> Vec<RevalidationRequestRecord>;
}

/// In-memory implementation used by tests and single-process deployments.
#[derive(Default)]
pub struct InMemoryRevalidationRequestStore {
    records: std::collections::BTreeMap<Uuid, RevalidationRequestRecord>,
    dedup_index: std::collections::BTreeMap<String, Uuid>,
}

impl InMemoryRevalidationRequestStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RevalidationRequestStore for InMemoryRevalidationRequestStore {
    fn insert(
        &mut self,
        request: RevalidationRequest,
    ) -> Result<RevalidationRequestRecord, RevalidationStoreError> {
        if request.target.identity.trim().is_empty() || request.target.source.trim().is_empty() {
            return Err(RevalidationStoreError::InvalidRequest(
                "revalidation target requires an identity and a source".into(),
            ));
        }
        if let Some(existing) = self.find_by_dedup_key(&request.dedup_key) {
            if !existing.status.is_terminal() {
                return Err(RevalidationStoreError::DuplicateRequest {
                    dedup_key: request.dedup_key,
                });
            }
        }
        let record = RevalidationRequestRecord {
            status: RevalidationStatus::Pending,
            attempt: 0,
            // created_at_ms is a fact supplied by the planner, not wall clock.
            created_at_ms: request.requested_at_ms,
            started_at_ms: None,
            completed_at_ms: None,
            last_error: None,
            request,
        };
        self.dedup_index
            .insert(record.request.dedup_key.clone(), record.request.request_id);
        self.records
            .insert(record.request.request_id, record.clone());
        Ok(record)
    }

    fn get(&self, request_id: Uuid) -> Result<RevalidationRequestRecord, RevalidationStoreError> {
        self.records
            .get(&request_id)
            .cloned()
            .ok_or(RevalidationStoreError::NotFound)
    }

    fn find_by_dedup_key(&self, dedup_key: &str) -> Option<RevalidationRequestRecord> {
        let request_id = self.dedup_index.get(dedup_key).copied()?;
        self.records.get(&request_id).cloned()
    }

    fn transition(
        &mut self,
        request_id: Uuid,
        to: RevalidationStatus,
        at_ms: u64,
        error: Option<String>,
    ) -> Result<RevalidationRequestRecord, RevalidationStoreError> {
        let record = self
            .records
            .get_mut(&request_id)
            .ok_or(RevalidationStoreError::NotFound)?;
        if !record.status.can_transition_to(to) {
            return Err(RevalidationStoreError::InvalidTransition {
                from: record.status,
                to,
            });
        }
        if to == RevalidationStatus::Running {
            record.attempt = record.attempt.saturating_add(1);
            if record.started_at_ms.is_none() {
                record.started_at_ms = Some(at_ms);
            }
        }
        if to.is_terminal() {
            record.completed_at_ms = Some(at_ms);
        }
        record.status = to;
        if let Some(text) = error {
            record.last_error = Some(bound_error_text(&text));
        }
        Ok(record.clone())
    }

    fn claim_due(&mut self, now_ms: u64, limit: usize) -> Vec<RevalidationRequestRecord> {
        if limit == 0 {
            return Vec::new();
        }
        // Collect candidates first and release the immutable borrow before
        // transitioning (which requires &mut self).
        let mut due: Vec<RevalidationRequestRecord> = self
            .records
            .values()
            .filter(|record| {
                record.status == RevalidationStatus::Pending
                    && record.request.scheduled_for_ms <= now_ms
            })
            .cloned()
            .collect();
        due.sort_by(|left, right| {
            right
                .request
                .priority
                .cmp(&left.request.priority)
                .then_with(|| {
                    left.request
                        .scheduled_for_ms
                        .cmp(&right.request.scheduled_for_ms)
                })
                .then_with(|| {
                    left.request
                        .target
                        .identity
                        .cmp(&right.request.target.identity)
                })
                .then_with(|| left.request.request_id.cmp(&right.request.request_id))
        });
        due.truncate(limit);
        let mut claimed = Vec::with_capacity(due.len());
        for record in due {
            if let Ok(claimed_record) = self.transition(
                record.request.request_id,
                RevalidationStatus::Claimed,
                now_ms,
                None,
            ) {
                claimed.push(claimed_record);
            }
        }
        claimed
    }

    fn list(&self) -> Vec<RevalidationRequestRecord> {
        let mut records: Vec<RevalidationRequestRecord> = self.records.values().cloned().collect();
        records.sort_by(|left, right| {
            left.created_at_ms
                .cmp(&right.created_at_ms)
                .then_with(|| left.request.request_id.cmp(&right.request.request_id))
        });
        records
    }
}

/// Bounds and sanitizes error text so raw provider payloads never persist.
fn bound_error_text(value: &str) -> String {
    const MAX_ERROR_CHARS: usize = 512;
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .take(MAX_ERROR_CHARS)
        .collect();
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(scheduled_for_ms: u64, priority: RevalidationPriority) -> RevalidationRequest {
        RevalidationRequest::new(
            Uuid::now_v7(),
            RevalidationTarget::new("acme:widget", "network-a"),
            RevalidationReason::Stale,
            priority,
            1_000,
            scheduled_for_ms,
        )
    }

    #[test]
    fn reason_serialization_is_forward_compatible() {
        assert_eq!(
            serde_json::to_string(&RevalidationReason::PriceChanged).unwrap(),
            "\"price_changed\""
        );
        // A future writer introduces a reason this version does not know.
        let decoded: RevalidationReason = serde_json::from_str("\"voice_search_boost\"").unwrap();
        assert_eq!(
            decoded,
            RevalidationReason::Unknown("voice_search_boost".into())
        );
        assert!(decoded.is_unknown());
        assert_eq!(decoded.as_str(), "voice_search_boost");
        // Strict parsing rejects the same value (write paths fail closed).
        assert!(RevalidationReason::parse_strict("voice_search_boost").is_none());
        // Known values round-trip exactly.
        let round_trip: RevalidationReason = serde_json::from_str("\"stale\"").unwrap();
        assert_eq!(round_trip, RevalidationReason::Stale);
    }

    #[test]
    fn priority_orders_critical_above_low() {
        assert!(RevalidationPriority::Critical > RevalidationPriority::High);
        assert!(RevalidationPriority::High > RevalidationPriority::Normal);
        assert!(RevalidationPriority::Normal > RevalidationPriority::Low);
        assert_eq!(
            RevalidationPriority::parse_strict("critical"),
            Some(RevalidationPriority::Critical)
        );
        assert_eq!(RevalidationPriority::parse_strict("urgent"), None);
    }

    #[test]
    fn dedup_key_is_stable_within_window_and_distinct_across_windows() {
        let base = RevalidationRequest::with_dedup_window(
            Uuid::now_v7(),
            RevalidationTarget::new("acme:widget", "network-a"),
            RevalidationReason::Stale,
            RevalidationPriority::High,
            1_000,
            3_600_123,
            3_600_000,
        );
        let repeat = RevalidationRequest::with_dedup_window(
            Uuid::now_v7(),
            RevalidationTarget::new("acme:widget", "network-a"),
            RevalidationReason::Stale,
            RevalidationPriority::High,
            2_000,
            3_600_456,
            3_600_000,
        );
        assert_eq!(base.dedup_key, repeat.dedup_key);

        let next_window = RevalidationRequest::with_dedup_window(
            Uuid::now_v7(),
            RevalidationTarget::new("acme:widget", "network-a"),
            RevalidationReason::Stale,
            RevalidationPriority::High,
            2_000,
            3_600_000 + 3_600_001,
            3_600_000,
        );
        assert_ne!(base.dedup_key, next_window.dedup_key);
    }

    #[test]
    fn status_transitions_are_strict() {
        assert!(RevalidationStatus::Pending.can_transition_to(RevalidationStatus::Running));
        assert!(RevalidationStatus::Running.can_transition_to(RevalidationStatus::Failed));
        assert!(RevalidationStatus::Failed.can_transition_to(RevalidationStatus::Pending));
        assert!(!RevalidationStatus::Succeeded.can_transition_to(RevalidationStatus::Pending));
        assert!(!RevalidationStatus::Pending.can_transition_to(RevalidationStatus::Succeeded));
        assert!(!RevalidationStatus::DeadLettered.can_transition_to(RevalidationStatus::Running));
        assert!(RevalidationStatus::Succeeded.is_terminal());
        assert!(!RevalidationStatus::Failed.is_terminal());
    }

    #[test]
    fn insert_rejects_blank_targets() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let mut request = request(1_000, RevalidationPriority::High);
        request.target.source = "  ".into();
        assert!(matches!(
            store.insert(request),
            Err(RevalidationStoreError::InvalidRequest(_))
        ));
    }

    #[test]
    fn duplicate_active_request_is_rejected_but_terminal_allows_reissue() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let first = request(1_000, RevalidationPriority::High);
        let dedup_key = first.dedup_key.clone();
        store.insert(first).expect("first insert");
        let duplicate = RevalidationRequest {
            request_id: Uuid::now_v7(),
            ..request(1_000, RevalidationPriority::High)
        };
        assert!(matches!(
            store.insert(duplicate),
            Err(RevalidationStoreError::DuplicateRequest { .. })
        ));

        let record = store
            .find_by_dedup_key(&dedup_key)
            .expect("existing request");
        store
            .transition(
                record.request.request_id,
                RevalidationStatus::Running,
                2_000,
                None,
            )
            .expect("start");
        store
            .transition(
                record.request.request_id,
                RevalidationStatus::Succeeded,
                3_000,
                None,
            )
            .expect("complete");
        assert!(
            store
                .find_by_dedup_key(&dedup_key)
                .expect("terminal record")
                .status
                .is_terminal()
        );

        // Terminal requests stop suppressing dedup: a fresh plan may re-issue.
        let reissued = RevalidationRequest {
            request_id: Uuid::now_v7(),
            ..request(1_000, RevalidationPriority::High)
        };
        assert!(
            store.insert(reissued).is_ok(),
            "terminal dedup entry must not block re-issue"
        );
        assert_eq!(store.list().len(), 2);
    }

    #[test]
    fn transition_sets_attempt_started_and_completed_times() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let inserted = store
            .insert(request(1_000, RevalidationPriority::High))
            .expect("insert");
        let id = inserted.request.request_id;

        assert_eq!(inserted.status, RevalidationStatus::Pending);
        assert_eq!(inserted.attempt, 0);
        assert_eq!(inserted.created_at_ms, 1_000);

        let running = store
            .transition(id, RevalidationStatus::Running, 2_000, None)
            .expect("start");
        assert_eq!(running.attempt, 1);
        assert_eq!(running.started_at_ms, Some(2_000));

        let failed = store
            .transition(
                id,
                RevalidationStatus::Failed,
                3_000,
                Some("provider timeout".into()),
            )
            .expect("fail");
        assert_eq!(failed.completed_at_ms, Some(3_000));
        assert_eq!(failed.last_error.as_deref(), Some("provider timeout"));

        // Retry path: Failed -> Pending, then run again with attempt 2.
        let requeued = store
            .transition(id, RevalidationStatus::Pending, 4_000, None)
            .expect("requeue");
        assert_eq!(requeued.attempt, 1);
        let running_again = store
            .transition(id, RevalidationStatus::Running, 5_000, None)
            .expect("start again");
        assert_eq!(running_again.attempt, 2);
        assert_eq!(
            running_again.started_at_ms,
            Some(2_000),
            "started_at stays at first start"
        );
    }

    #[test]
    fn illegal_transitions_are_rejected() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let inserted = store
            .insert(request(1_000, RevalidationPriority::High))
            .expect("insert");
        assert_eq!(
            store.transition(
                inserted.request.request_id,
                RevalidationStatus::Succeeded,
                2_000,
                None
            ),
            Err(RevalidationStoreError::InvalidTransition {
                from: RevalidationStatus::Pending,
                to: RevalidationStatus::Succeeded,
            })
        );
        assert_eq!(
            store.transition(Uuid::now_v7(), RevalidationStatus::Running, 2_000, None),
            Err(RevalidationStoreError::NotFound)
        );
    }

    #[test]
    fn claim_due_is_deterministic_and_marks_claimed() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let low = store
            .insert(request(1_000, RevalidationPriority::Low))
            .expect("insert low");
        let critical = store
            .insert(request(500, RevalidationPriority::Critical))
            .expect("insert critical");
        let high_later = store
            .insert(request(2_000, RevalidationPriority::High))
            .expect("insert high");
        let not_due = store
            .insert(request(9_999, RevalidationPriority::Critical))
            .expect("insert future");

        let claimed = store.claim_due(2_000, 10);
        let ids: Vec<Uuid> = claimed
            .iter()
            .map(|record| record.request.request_id)
            .collect();
        assert_eq!(
            ids,
            vec![
                critical.request.request_id,
                high_later.request.request_id,
                low.request.request_id
            ]
        );
        assert!(
            claimed
                .iter()
                .all(|record| record.status == RevalidationStatus::Claimed)
        );
        assert!(store.claim_due(2_000, 10).is_empty(), "already claimed");
        assert_eq!(
            store
                .get(not_due.request.request_id)
                .expect("future request")
                .status,
            RevalidationStatus::Pending
        );
        assert!(
            store.claim_due(2_000, 0).is_empty(),
            "zero limit claims nothing"
        );
    }

    #[test]
    fn error_text_is_bounded_and_sanitized() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let inserted = store
            .insert(request(1_000, RevalidationPriority::High))
            .expect("insert");
        let payload = "secret".to_string() + &"x".repeat(2_000);
        store
            .transition(
                inserted.request.request_id,
                RevalidationStatus::Running,
                2_000,
                Some(format!("failed\n{}\r", payload)),
            )
            .expect("fail");
        let record = store.get(inserted.request.request_id).expect("record");
        let text = record.last_error.expect("error text");
        assert!(text.len() <= 512);
        assert!(!text.contains('\n'));
        assert!(!text.contains('\r'));
    }

    #[test]
    fn list_is_ordered_by_creation_time_then_id() {
        let mut store = InMemoryRevalidationRequestStore::new();
        let later = store
            .insert(request(5_000, RevalidationPriority::Low))
            .expect("insert");
        let earlier = store
            .insert(request(1_000, RevalidationPriority::Low))
            .expect("insert");
        let records = store.list();
        assert_eq!(records[0].request.request_id, earlier.request.request_id);
        assert_eq!(records[1].request.request_id, later.request.request_id);
    }
}

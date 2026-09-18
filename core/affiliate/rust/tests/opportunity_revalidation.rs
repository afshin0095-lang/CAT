//! Integration coverage for the revalidation request domain: identity,
//! deduplication, persistence-neutral store semantics, and forward-compatible
//! serialization.

use cat_affiliate::{
    InMemoryRevalidationRequestStore, REVALIDATION_DEDUP_WINDOW_MS, RevalidationDecision,
    RevalidationPriority, RevalidationReason, RevalidationRequest, RevalidationRequestStore,
    RevalidationStatus, RevalidationStoreError, RevalidationTarget,
};
use uuid::Uuid;

fn request(
    identity: &str,
    source: &str,
    reason: RevalidationReason,
    scheduled_for_ms: u64,
) -> RevalidationRequest {
    RevalidationRequest::new(
        Uuid::now_v7(),
        RevalidationTarget::new(identity, source),
        reason,
        RevalidationPriority::Normal,
        1_000,
        scheduled_for_ms,
    )
}

#[test]
fn default_dedup_window_is_one_hour() {
    assert_eq!(REVALIDATION_DEDUP_WINDOW_MS, 3_600_000);
}

#[test]
fn requests_serialize_round_trip_with_forward_compatible_reasons() {
    let request = request(
        "acme:widget",
        "network-a",
        RevalidationReason::CommissionChanged,
        5_000,
    );
    let encoded = serde_json::to_string(&request).expect("serialize");
    let decoded: RevalidationRequest = serde_json::from_str(&encoded).expect("deserialize");
    assert_eq!(decoded.dedup_key, request.dedup_key);
    assert_eq!(decoded.target, request.target);

    // A future reason string stays readable on the wire.
    let patched = encoded.replace("\"commission_changed\"", "\"ai_agent_review\"");
    let decoded: RevalidationRequest =
        serde_json::from_str(&patched).expect("unknown reasons decode");
    assert!(decoded.reason.is_unknown());
    assert_eq!(decoded.reason.as_str(), "ai_agent_review");
}

#[test]
fn store_semantics_match_the_documented_contract() {
    let mut store = InMemoryRevalidationRequestStore::new();

    // Insert -> Pending, dedup-visible, gettable.
    let inserted_request =
        request("acme:widget", "network-a", RevalidationReason::Stale, 1_000);
    let inserted = store.insert(inserted_request).expect("insert");
    assert_eq!(inserted.status, RevalidationStatus::Pending);
    assert_eq!(
        store.get(inserted.request.request_id).expect("get").request,
        inserted.request
    );
    assert!(
        store
            .find_by_dedup_key(&inserted.request.dedup_key)
            .is_some()
    );

    // Active duplicate suppressed.
    let duplicate = request("acme:widget", "network-a", RevalidationReason::Stale, 1_000);
    assert_eq!(
        store.insert(duplicate).unwrap_err(),
        RevalidationStoreError::DuplicateRequest {
            dedup_key: inserted.request.dedup_key.clone(),
        }
    );

    // Full lifecycle: Pending -> Running -> Succeeded.
    let running = store
        .transition(
            inserted.request.request_id,
            RevalidationStatus::Running,
            2_000,
            None,
        )
        .expect("start");
    assert_eq!(running.attempt, 1);
    let done = store
        .transition(
            inserted.request.request_id,
            RevalidationStatus::Succeeded,
            3_000,
            None,
        )
        .expect("complete");
    assert_eq!(done.status, RevalidationStatus::Succeeded);
    assert_eq!(done.completed_at_ms, Some(3_000));

    // Terminal statuses reject further transitions.
    assert_eq!(
        store.transition(
            inserted.request.request_id,
            RevalidationStatus::Running,
            4_000,
            None
        ),
        Err(RevalidationStoreError::InvalidTransition {
            from: RevalidationStatus::Succeeded,
            to: RevalidationStatus::Running,
        })
    );
}

#[test]
fn failure_transitions_preserve_bounded_diagnostics() {
    let mut store = InMemoryRevalidationRequestStore::new();
    let inserted = store
        .insert(request(
            "acme:widget",
            "network-a",
            RevalidationReason::Expired,
            1_000,
        ))
        .expect("insert");
    store
        .transition(
            inserted.request.request_id,
            RevalidationStatus::Running,
            2_000,
            None,
        )
        .expect("start");
    let failed = store
        .transition(
            inserted.request.request_id,
            RevalidationStatus::Failed,
            3_000,
            Some("provider returned 503 for /api".into()),
        )
        .expect("fail");
    assert_eq!(
        failed.last_error.as_deref(),
        Some("provider returned 503 for /api")
    );

    // Failed -> DeadLettered ends the line.
    let dead = store
        .transition(
            inserted.request.request_id,
            RevalidationStatus::DeadLettered,
            4_000,
            None,
        )
        .expect("dead letter");
    assert!(dead.status.is_terminal());
}

#[test]
fn decision_helpers_summarize_requests_deterministically() {
    let mut decision = RevalidationDecision {
        identity: "acme:widget".into(),
        opportunity_id: Uuid::now_v7(),
        evaluated_at_ms: 1_000,
        requests: vec![
            request(
                "acme:widget",
                "network-b",
                RevalidationReason::Expired,
                1_000,
            ),
            request(
                "acme:widget",
                "network-a",
                RevalidationReason::Expired,
                1_000,
            ),
            request(
                "acme:widget",
                "network-a",
                RevalidationReason::Expired,
                1_000,
            ),
        ],
        skipped: Vec::new(),
        blocked: None,
    };
    // Sort for a deterministic reasons view (planner output is pre-sorted;
    // this exercises the helper against arbitrary input order).
    decision
        .requests
        .sort_by(|left, right| left.target.source.cmp(&right.target.source));
    assert!(decision.requires_revalidation());
    assert_eq!(decision.reasons(), vec!["expired".to_string()]);
}

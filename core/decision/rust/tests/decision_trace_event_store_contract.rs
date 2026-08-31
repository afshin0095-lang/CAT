use cat_decision::{
    Alternative, DecisionOutcome, DecisionStatus, DecisionTrace, DecisionTraceEventStore,
};
use cat_kernel::{
    CorrelationId, ExpectedVersion, FixedClock, IdempotencyKey, TenantId, TimestampMs,
};
use serde_json::json;
use uuid::Uuid;

fn outcome() -> DecisionOutcome {
    DecisionOutcome {
        decision_id: Uuid::now_v7(),
        selected: Some(Alternative {
            id: "safe".into(),
            label: "Safe".into(),
            rationale: "policy compliant".into(),
            expected_value: 1.0,
            confidence: 0.9,
            constraints_satisfied: true,
            metadata: json!({}),
        }),
        status: DecisionStatus::Proposed,
        confidence: 0.9,
        policy_reasons: vec!["advisory".into()],
        advisory_only: true,
        trace_id: Uuid::now_v7(),
    }
}

#[test]
fn persisted_trace_hydrates_and_replays_without_execution_authority() {
    let outcome = outcome();
    let mut trace = DecisionTrace {
        trace_id: outcome.trace_id,
        decision_id: outcome.decision_id,
        steps: Vec::new(),
    };
    trace.push("policy", "evaluated policy constraints");

    let mut store = DecisionTraceEventStore::default();
    let clock = FixedClock::new(TimestampMs::new(1).unwrap());

    store
        .persist(
            trace,
            TenantId::new(),
            CorrelationId::new(),
            ExpectedVersion::Empty,
            IdempotencyKey::new("decision-trace-1").unwrap(),
            &clock,
        )
        .unwrap();

    assert_eq!(store.event_count(), 1);
    assert_eq!(store.hydrate().unwrap().len(), 1);

    let replay = store.replay(outcome.trace_id, &outcome).unwrap();
    assert!(replay.advisory_only);
    assert_eq!(replay.selected_id.as_deref(), Some("safe"));
}

#[test]
fn stale_expected_version_fails_closed() {
    let outcome = outcome();
    let trace = DecisionTrace {
        trace_id: outcome.trace_id,
        decision_id: outcome.decision_id,
        steps: Vec::new(),
    };
    let mut store = DecisionTraceEventStore::default();
    let clock = FixedClock::new(TimestampMs::new(1).unwrap());

    store
        .persist(
            trace.clone(),
            TenantId::new(),
            CorrelationId::new(),
            ExpectedVersion::Empty,
            IdempotencyKey::new("trace-a").unwrap(),
            &clock,
        )
        .unwrap();

    assert!(
        store
            .persist(
                trace,
                TenantId::new(),
                CorrelationId::new(),
                ExpectedVersion::Empty,
                IdempotencyKey::new("trace-b").unwrap(),
                &clock,
            )
            .is_err()
    );
}

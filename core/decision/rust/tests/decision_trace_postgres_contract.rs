use cat_decision::{Alternative, DecisionOutcome, DecisionStatus, DecisionTrace, DurableDecisionTraceEventStore};
use cat_eventstore_postgres::PostgresEventStore;
use cat_kernel::{CorrelationId, EntityId, ExpectedVersion, FixedClock, IdempotencyKey, TenantId, TimestampMs};
use serde_json::json;
use uuid::Uuid;

fn outcome() -> DecisionOutcome {
    DecisionOutcome {
        decision_id: Uuid::now_v7(),
        selected: Some(Alternative { id: "safe".into(), label: "Safe".into(), rationale: "policy compliant".into(), expected_value: 1.0, confidence: 0.9, constraints_satisfied: true, metadata: json!({}) }),
        status: DecisionStatus::Proposed,
        confidence: 0.9,
        policy_reasons: vec!["advisory".into()],
        advisory_only: true,
        trace_id: Uuid::now_v7(),
    }
}

async fn store() -> Option<PostgresEventStore> {
    let url = std::env::var("DATABASE_URL").ok()?;
    let store = PostgresEventStore::connect(&url).await.ok()?;
    store.ensure_schema().await.ok()?;
    Some(store)
}

#[tokio::test]
async fn durable_trace_round_trip_preserves_advisory_replay() {
    let Some(events) = store().await else { return };
    let outcome = outcome();
    let mut trace = DecisionTrace { trace_id: outcome.trace_id, decision_id: outcome.decision_id, steps: Vec::new() };
    trace.push("policy", "evaluated policy constraints");

    let adapter = DurableDecisionTraceEventStore::new(EntityId::new(), events);
    let clock = FixedClock::new(TimestampMs::new(1));
    adapter.persist(trace, TenantId::new(), CorrelationId::new(), ExpectedVersion::Empty, &IdempotencyKey::new(format!("decision-durable-{}", Uuid::now_v7())).unwrap(), &clock).await.unwrap();

    assert_eq!(adapter.current_version().await.unwrap().as_u64(), 1);
    assert_eq!(adapter.hydrate().await.unwrap().len(), 1);
    let replay = adapter.replay(outcome.trace_id, &outcome).await.unwrap();
    assert!(replay.advisory_only);
    assert_eq!(replay.selected_id.as_deref(), Some("safe"));
}

#[tokio::test]
async fn durable_trace_rejects_stale_version() {
    let Some(events) = store().await else { return };
    let outcome = outcome();
    let trace = DecisionTrace { trace_id: outcome.trace_id, decision_id: outcome.decision_id, steps: Vec::new() };
    let adapter = DurableDecisionTraceEventStore::new(EntityId::new(), events);
    let clock = FixedClock::new(TimestampMs::new(1));

    adapter.persist(trace.clone(), TenantId::new(), CorrelationId::new(), ExpectedVersion::Empty, &IdempotencyKey::new(format!("decision-durable-a-{}", Uuid::now_v7())).unwrap(), &clock).await.unwrap();
    assert!(adapter.persist(trace, TenantId::new(), CorrelationId::new(), ExpectedVersion::Empty, &IdempotencyKey::new(format!("decision-durable-b-{}", Uuid::now_v7())).unwrap(), &clock).await.is_err());
}

use cat_eventstore_postgres::{PostgresEventStore, PostgresEventStoreError};
use cat_kernel::{CorrelationId, EntityId, EventEnvelope, ExpectedVersion, IdempotencyKey, SequenceNumber, TenantId, TimestampMs};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct TestPayload {
    value: String,
}

fn envelope(sequence: u64, tenant: TenantId, correlation: CorrelationId) -> EventEnvelope<TestPayload> {
    EventEnvelope::new(
        "test.event",
        1,
        tenant,
        correlation,
        None,
        EntityId::new(),
        TimestampMs::new(sequence),
        SequenceNumber::new(sequence),
        TestPayload { value: format!("value-{sequence}") },
    )
    .expect("test envelope must be valid")
}

async fn store() -> Option<PostgresEventStore> {
    let url = std::env::var("DATABASE_URL").ok()?;
    let store = PostgresEventStore::connect(&url).await.ok()?;
    store.ensure_schema().await.ok()?;
    Some(store)
}

#[tokio::test]
async fn append_is_idempotent_and_stream_order_is_preserved() {
    let Some(store) = store().await else { return };
    let stream = Uuid::now_v7();
    let tenant = TenantId::new();
    let correlation = CorrelationId::new();

    let first = envelope(1, tenant, correlation);
    let key = IdempotencyKey::new(format!("contract-test-{stream}" )).unwrap();
    let first_receipt = store
        .append(stream, ExpectedVersion::Empty, &key, &first)
        .await
        .unwrap();

    let replay = store
        .append(stream, ExpectedVersion::Any, &key, &first)
        .await
        .unwrap();

    assert_eq!(replay.event_id, first_receipt.event_id);
    assert_eq!(replay.sequence, first_receipt.sequence);
    assert!(replay.idempotent_replay);
    assert_eq!(store.current_version(stream).await.unwrap().as_u64(), 1);

    let second = envelope(2, tenant, correlation);
    let second_key = IdempotencyKey::new(format!("contract-test-{stream}-2")).unwrap();
    store
        .append(stream, ExpectedVersion::Exact(SequenceNumber::new(1)), &second_key, &second)
        .await
        .unwrap();

    let events = store.read_stream::<TestPayload>(stream).await.unwrap();
    assert_eq!(events.len(), 2);
    assert_eq!(events[0].sequence.as_u64(), 1);
    assert_eq!(events[1].sequence.as_u64(), 2);
    assert_eq!(events[1].payload.value, "value-2");
}

#[tokio::test]
async fn optimistic_concurrency_and_sequence_contracts_are_enforced() {
    let Some(store) = store().await else { return; };
    let stream = Uuid::now_v7();
    let tenant = TenantId::new();
    let correlation = CorrelationId::new();
    let first = envelope(1, tenant, correlation);
    let key = IdempotencyKey::new(format!("contract-test-{stream}-1")).unwrap();
    store.append(stream, ExpectedVersion::Empty, &key, &first).await.unwrap();

    let wrong_version = envelope(2, tenant, correlation);
    let wrong_key = IdempotencyKey::new(format!("contract-test-{stream}-wrong-version")).unwrap();
    assert!(matches!(
        store.append(stream, ExpectedVersion::Empty, &wrong_key, &wrong_version).await,
        Err(PostgresEventStoreError::ConcurrencyConflict { expected: 0, actual: 1 })
    ));

    let wrong_sequence = envelope(3, tenant, correlation);
    let sequence_key = IdempotencyKey::new(format!("contract-test-{stream}-wrong-sequence")).unwrap();
    assert!(matches!(
        store.append(stream, ExpectedVersion::Exact(SequenceNumber::new(1)), &sequence_key, &wrong_sequence).await,
        Err(PostgresEventStoreError::SequenceConflict { expected: 2, actual: 3 })
    ));
}

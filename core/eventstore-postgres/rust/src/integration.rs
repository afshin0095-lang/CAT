use cat_eventbus::EventEnvelope as BusEventEnvelope;
use cat_kernel::{EventEnvelope, ExpectedVersion, IdempotencyKey, SequenceNumber};
use serde::Serialize;
use sqlx::{PgPool, Row};

use crate::{DurableAppendReceipt, PostgresEventStoreError, PostgresEventStoreResult};

/// Atomic event-store + outbox boundary.
///
/// The canonical kernel event and its EventBus publication record are committed
/// in the same PostgreSQL transaction. A crash can therefore leave the system in
/// only one of two durable states: neither the event nor its publication exists,
/// or both exist and the publication worker can resume later.
#[derive(Clone, Debug)]
pub struct TransactionalEventPublisher {
    pool: PgPool,
}

impl TransactionalEventPublisher {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn append_and_enqueue<T: Serialize>(
        &self,
        stream_id: uuid::Uuid,
        expected: ExpectedVersion,
        idempotency_key: &IdempotencyKey,
        kernel_event: &EventEnvelope<T>,
        bus_event: &BusEventEnvelope,
    ) -> PostgresEventStoreResult<DurableAppendReceipt> {
        if kernel_event.event_id.as_uuid() != bus_event.event_id {
            return Err(PostgresEventStoreError::PublicationIdentityMismatch);
        }

        let mut tx = self.pool.begin().await?;

        if let Some(row) = sqlx::query(
            "SELECT event_id, sequence FROM cat_event_idempotency WHERE idempotency_key = $1",
        )
        .bind(idempotency_key.as_str())
        .fetch_optional(&mut *tx)
        .await?
        {
            let event_id: uuid::Uuid = row.try_get("event_id")?;
            let sequence: i64 = row.try_get("sequence")?;
            return Ok(DurableAppendReceipt {
                event_id,
                sequence: SequenceNumber::new(sequence as u64),
                idempotent_replay: true,
            });
        }

        sqlx::query(
            "INSERT INTO cat_event_streams (stream_id, current_sequence) VALUES ($1, 0) ON CONFLICT (stream_id) DO NOTHING",
        )
        .bind(stream_id)
        .execute(&mut *tx)
        .await?;

        let row = sqlx::query(
            "SELECT current_sequence FROM cat_event_streams WHERE stream_id = $1 FOR UPDATE",
        )
        .bind(stream_id)
        .fetch_one(&mut *tx)
        .await?;
        let current = row.try_get::<i64, _>("current_sequence")? as u64;

        match expected {
            ExpectedVersion::Any => {}
            ExpectedVersion::Empty if current != 0 => {
                return Err(PostgresEventStoreError::ConcurrencyConflict {
                    expected: 0,
                    actual: current,
                });
            }
            ExpectedVersion::Empty => {}
            ExpectedVersion::Exact(version) if current != version.as_u64() => {
                return Err(PostgresEventStoreError::ConcurrencyConflict {
                    expected: version.as_u64(),
                    actual: current,
                });
            }
            ExpectedVersion::Exact(_) => {}
        }

        let next = current
            .checked_add(1)
            .ok_or(PostgresEventStoreError::SequenceOverflow)?;
        if kernel_event.sequence.as_u64() != next {
            return Err(PostgresEventStoreError::SequenceConflict {
                expected: next,
                actual: kernel_event.sequence.as_u64(),
            });
        }

        let event_json = serde_json::to_value(kernel_event)?;
        sqlx::query(
            "INSERT INTO cat_events (stream_id, sequence, event_id, event_type, event_version, tenant_id, correlation_id, causation_id, actor_id, occurred_at_ms, envelope) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)",
        )
        .bind(stream_id)
        .bind(next as i64)
        .bind(kernel_event.event_id.as_uuid())
        .bind(&kernel_event.event_type)
        .bind(kernel_event.event_version as i32)
        .bind(kernel_event.tenant_id.as_uuid())
        .bind(kernel_event.correlation_id.as_uuid())
        .bind(kernel_event.causation_id.map(|id| id.as_uuid()))
        .bind(kernel_event.actor_id.as_uuid())
        .bind(kernel_event.occurred_at.as_i64())
        .bind(event_json)
        .execute(&mut *tx)
        .await?;

        sqlx::query("UPDATE cat_event_streams SET current_sequence = $2 WHERE stream_id = $1")
            .bind(stream_id)
            .bind(next as i64)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO cat_event_idempotency (idempotency_key, stream_id, event_id, sequence) VALUES ($1,$2,$3,$4)",
        )
        .bind(idempotency_key.as_str())
        .bind(stream_id)
        .bind(kernel_event.event_id.as_uuid())
        .bind(next as i64)
        .execute(&mut *tx)
        .await?;

        let bus_json = serde_json::to_value(bus_event)?;
        sqlx::query(
            "INSERT INTO cat_event_outbox (event_id, stream_id, sequence, event_type, envelope) VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(bus_event.event_id)
        .bind(stream_id)
        .bind(next as i64)
        .bind(&bus_event.event_type)
        .bind(bus_json)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(DurableAppendReceipt {
            event_id: kernel_event.event_id.as_uuid(),
            sequence: SequenceNumber::new(next),
            idempotent_replay: false,
        })
    }
}

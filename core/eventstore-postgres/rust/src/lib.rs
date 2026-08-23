use cat_kernel::{EventEnvelope, ExpectedVersion, IdempotencyKey, SequenceNumber};
use serde::{de::DeserializeOwned, Serialize};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, PgPool, Row};
use thiserror::Error;

mod checkpoint;
mod inbox;
mod integration;
mod outbox;

pub use checkpoint::{CheckpointError, CheckpointResult, PostgresProjectionCheckpointStore};
pub use inbox::PostgresInbox;
pub use integration::TransactionalEventPublisher;
pub use outbox::{OutboxRecord, PostgresOutbox};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Debug, Error)]
pub enum PostgresEventStoreError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("optimistic concurrency conflict: expected {expected}, actual {actual}")]
    ConcurrencyConflict { expected: u64, actual: u64 },
    #[error("sequence conflict: expected {expected}, actual {actual}")]
    SequenceConflict { expected: u64, actual: u64 },
    #[error("event bus publication id does not match the canonical event id")]
    PublicationIdentityMismatch,
    #[error("sequence number overflow")]
    SequenceOverflow,
}

pub type PostgresEventStoreResult<T> = Result<T, PostgresEventStoreError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableAppendReceipt {
    pub event_id: uuid::Uuid,
    pub sequence: SequenceNumber,
    pub idempotent_replay: bool,
}

#[derive(Clone, Debug)]
pub struct PostgresEventStore {
    pool: PgPool,
}

impl PostgresEventStore {
    pub async fn connect(database_url: &str) -> PostgresEventStoreResult<Self> {
        let pool = PgPoolOptions::new().max_connections(10).connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn from_pool(pool: PgPool) -> Self { Self { pool } }

    pub fn pool(&self) -> &PgPool { &self.pool }

    /// Applies every checked-in migration in order.
    ///
    /// The previous implementation executed a small hand-maintained subset of migration
    /// files, which made later schema additions easy to omit. The embedded migrator is now
    /// the single source of truth: adding a migration under `migrations/` automatically
    /// extends the schema bootstrap path without another code edit.
    pub async fn ensure_schema(&self) -> PostgresEventStoreResult<()> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }

    pub async fn append<T: Serialize>(
        &self,
        stream_id: uuid::Uuid,
        expected: ExpectedVersion,
        idempotency_key: &IdempotencyKey,
        envelope: &EventEnvelope<T>,
    ) -> PostgresEventStoreResult<DurableAppendReceipt> {
        let mut tx = self.pool.begin().await?;
        if let Some(row) = sqlx::query("SELECT event_id, sequence FROM cat_event_idempotency WHERE idempotency_key = $1")
            .bind(idempotency_key.as_str()).fetch_optional(&mut *tx).await? {
            return Ok(DurableAppendReceipt {
                event_id: row.try_get("event_id")?,
                sequence: SequenceNumber::new(row.try_get::<i64, _>("sequence")? as u64),
                idempotent_replay: true,
            });
        }

        sqlx::query("INSERT INTO cat_event_streams (stream_id, current_sequence) VALUES ($1, 0) ON CONFLICT (stream_id) DO NOTHING")
            .bind(stream_id).execute(&mut *tx).await?;
        let row = sqlx::query("SELECT current_sequence FROM cat_event_streams WHERE stream_id = $1 FOR UPDATE")
            .bind(stream_id).fetch_one(&mut *tx).await?;
        let current: u64 = row.try_get::<i64, _>("current_sequence")? as u64;

        match expected {
            ExpectedVersion::Any => {}
            ExpectedVersion::Empty if current != 0 => return Err(PostgresEventStoreError::ConcurrencyConflict { expected: 0, actual: current }),
            ExpectedVersion::Empty => {}
            ExpectedVersion::Exact(version) if current != version.as_u64() => return Err(PostgresEventStoreError::ConcurrencyConflict { expected: version.as_u64(), actual: current }),
            ExpectedVersion::Exact(_) => {}
        }

        let next = current.checked_add(1).ok_or(PostgresEventStoreError::SequenceOverflow)?;
        if envelope.sequence.as_u64() != next {
            return Err(PostgresEventStoreError::SequenceConflict { expected: next, actual: envelope.sequence.as_u64() });
        }

        sqlx::query("INSERT INTO cat_events (stream_id, sequence, event_id, event_type, event_version, tenant_id, correlation_id, causation_id, actor_id, occurred_at_ms, envelope) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)")
            .bind(stream_id).bind(next as i64).bind(envelope.event_id.as_uuid()).bind(&envelope.event_type)
            .bind(envelope.event_version as i32).bind(envelope.tenant_id.as_uuid()).bind(envelope.correlation_id.as_uuid())
            .bind(envelope.causation_id.map(|id| id.as_uuid())).bind(envelope.actor_id.as_uuid()).bind(envelope.occurred_at.as_i64())
            .bind(serde_json::to_value(envelope)?).execute(&mut *tx).await?;
        sqlx::query("UPDATE cat_event_streams SET current_sequence = $2 WHERE stream_id = $1")
            .bind(stream_id).bind(next as i64).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO cat_event_idempotency (idempotency_key, stream_id, event_id, sequence) VALUES ($1,$2,$3,$4)")
            .bind(idempotency_key.as_str()).bind(stream_id).bind(envelope.event_id.as_uuid()).bind(next as i64)
            .execute(&mut *tx).await?;
        tx.commit().await?;

        Ok(DurableAppendReceipt { event_id: envelope.event_id.as_uuid(), sequence: SequenceNumber::new(next), idempotent_replay: false })
    }

    pub async fn read_stream<T: DeserializeOwned>(&self, stream_id: uuid::Uuid) -> PostgresEventStoreResult<Vec<EventEnvelope<T>>> {
        let rows = sqlx::query("SELECT envelope FROM cat_events WHERE stream_id = $1 ORDER BY sequence ASC")
            .bind(stream_id).fetch_all(&self.pool).await?;
        rows.into_iter().map(|row| Ok(serde_json::from_value(row.try_get("envelope")?)?)).collect()
    }

    pub async fn current_version(&self, stream_id: uuid::Uuid) -> PostgresEventStoreResult<SequenceNumber> {
        let row = sqlx::query("SELECT current_sequence FROM cat_event_streams WHERE stream_id = $1")
            .bind(stream_id).fetch_optional(&self.pool).await?;
        Ok(row.map(|row| SequenceNumber::new(row.try_get::<i64, _>("current_sequence").unwrap_or(0) as u64)).unwrap_or(SequenceNumber::ZERO))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn durable_adapter_has_a_stable_contract_name() {
        assert_eq!(env!("CARGO_PKG_NAME"), "cat-eventstore-postgres");
    }

    #[test]
    fn embedded_migration_catalog_is_not_empty() {
        assert!(!super::MIGRATOR.migrations.is_empty());
    }
}
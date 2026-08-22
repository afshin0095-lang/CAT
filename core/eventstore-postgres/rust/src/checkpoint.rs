use cat_kernel::{EntityId, EventId, ProjectionCheckpoint, ProjectionPhase, SequenceNumber};
use sqlx::{PgPool, Row};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckpointError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("invalid checkpoint state: {0}")]
    InvalidState(String),
}

pub type CheckpointResult<T> = Result<T, CheckpointError>;

/// PostgreSQL persistence for projection checkpoints.
///
/// The table is intentionally separate from the event log: checkpoints are
/// derived execution state and can be deleted/rebuilt without changing the
/// canonical event history.
#[derive(Clone, Debug)]
pub struct PostgresProjectionCheckpointStore {
    pool: PgPool,
}

impl PostgresProjectionCheckpointStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn load(
        &self,
        projection_id: &str,
        stream_id: EntityId,
    ) -> CheckpointResult<Option<ProjectionCheckpoint>> {
        let row = sqlx::query(
            "SELECT sequence, event_id, phase FROM cat_projection_checkpoints WHERE projection_id = $1 AND stream_id = $2",
        )
        .bind(projection_id)
        .bind(stream_id.as_uuid())
        .fetch_optional(&self.pool)
        .await?;

        row.map(|row| {
            let sequence: i64 = row.try_get("sequence")?;
            let phase: String = row.try_get("phase")?;
            let phase = match phase.as_str() {
                "catchup" => ProjectionPhase::Catchup,
                "live" => ProjectionPhase::Live,
                other => return Err(CheckpointError::InvalidState(format!("unknown phase: {other}"))),
            };
            let event_id = EventId::from_uuid(row.try_get::<uuid::Uuid, _>("event_id")?);
            ProjectionCheckpoint::new(
                projection_id,
                stream_id,
                SequenceNumber::new(sequence as u64),
                event_id,
                phase,
            )
            .map_err(|error| CheckpointError::InvalidState(error.to_string()))
        }).transpose()
    }

    /// Saves a checkpoint only when it advances the stored sequence.
    pub async fn save(&self, checkpoint: &ProjectionCheckpoint) -> CheckpointResult<()> {
        let phase = match checkpoint.phase {
            ProjectionPhase::Catchup => "catchup",
            ProjectionPhase::Live => "live",
        };

        sqlx::query(
            "INSERT INTO cat_projection_checkpoints (projection_id, stream_id, sequence, event_id, phase) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (projection_id, stream_id) DO UPDATE SET sequence = EXCLUDED.sequence, event_id = EXCLUDED.event_id, phase = EXCLUDED.phase, updated_at = NOW() WHERE cat_projection_checkpoints.sequence <= EXCLUDED.sequence",
        )
        .bind(&checkpoint.projection_id)
        .bind(checkpoint.stream_id.as_uuid())
        .bind(checkpoint.sequence.as_u64() as i64)
        .bind(checkpoint.event_id.as_uuid())
        .bind(phase)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

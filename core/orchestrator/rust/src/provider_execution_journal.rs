use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    OrchestratorError, OrchestratorResult, PostgresExecutionStore, ProviderOutcomeState,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderExecutionJournalEvent {
    Submitted,
    Observed,
}

impl ProviderExecutionJournalEvent {
    fn as_str(self) -> &'static str {
        match self {
            Self::Submitted => "submitted",
            Self::Observed => "observed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderExecutionJournalEntry {
    pub journal_sequence: i64,
    pub journal_id: Uuid,
    pub execution_id: Uuid,
    pub event_key: String,
    pub event: ProviderExecutionJournalEvent,
    pub provider: String,
    pub provider_execution_id: String,
    pub request_hash: String,
    pub outcome: Option<ProviderOutcomeState>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub recorded_at_ms: u64,
}

pub(crate) fn journal_event_key(
    event: ProviderExecutionJournalEvent,
    provider_execution_id: &str,
    outcome: Option<ProviderOutcomeState>,
    recorded_at_ms: u64,
) -> String {
    match event {
        ProviderExecutionJournalEvent::Submitted => {
            format!("submission:{provider_execution_id}")
        }
        ProviderExecutionJournalEvent::Observed => format!(
            "observation:{provider_execution_id}:{}:{}",
            outcome.map(ProviderOutcomeState::as_str).unwrap_or("none"),
            recorded_at_ms
        ),
    }
}

pub(crate) async fn insert_provider_journal_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    execution_id: Uuid,
    event: ProviderExecutionJournalEvent,
    provider: &str,
    provider_execution_id: &str,
    request_hash: &str,
    outcome: Option<ProviderOutcomeState>,
    result: Option<serde_json::Value>,
    error: Option<&str>,
    recorded_at_ms: u64,
) -> OrchestratorResult<ProviderExecutionJournalEntry> {
    if execution_id.is_nil()
        || provider.trim().is_empty()
        || provider_execution_id.trim().is_empty()
        || request_hash.trim().is_empty()
    {
        return Err(OrchestratorError::Serialization(
            "invalid provider journal identity".into(),
        ));
    }

    let event_key =
        journal_event_key(event, provider_execution_id, outcome, recorded_at_ms);
    let journal_id = Uuid::now_v7();

    let inserted = sqlx::query(
        "INSERT INTO cat_provider_execution_journal
         (journal_id, execution_id, event_key, event_type, provider,
          provider_execution_id, request_hash, outcome_state, result, error, recorded_at)
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,TO_TIMESTAMP($11 / 1000.0))
         ON CONFLICT (event_key) DO NOTHING
         RETURNING journal_sequence",
    )
    .bind(journal_id)
    .bind(execution_id)
    .bind(&event_key)
    .bind(event.as_str())
    .bind(provider)
    .bind(provider_execution_id)
    .bind(request_hash)
    .bind(outcome.map(ProviderOutcomeState::as_str))
    .bind(&result)
    .bind(error)
    .bind(recorded_at_ms as f64)
    .fetch_optional(&mut **tx)
    .await
    .map_err(db_error)?;

    let (journal_id, journal_sequence) = if let Some(row) = inserted {
        (
            journal_id,
            row.try_get::<i64, _>("journal_sequence")
                .map_err(row_error)?,
        )
    } else {
        let row = sqlx::query(
            "SELECT journal_sequence, journal_id, execution_id, event_key, event_type,
                    provider, provider_execution_id, request_hash, outcome_state, result, error,
                    EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms
             FROM cat_provider_execution_journal
             WHERE event_key = $1",
        )
        .bind(&event_key)
        .fetch_one(&mut **tx)
        .await
        .map_err(db_error)?;

        let existing_execution: Uuid = row.try_get("execution_id").map_err(row_error)?;
        let existing_event_type: String = row.try_get("event_type").map_err(row_error)?;
        let existing_provider: String = row.try_get("provider").map_err(row_error)?;
        let existing_provider_execution_id: String =
            row.try_get("provider_execution_id").map_err(row_error)?;
        let existing_request_hash: String =
            row.try_get("request_hash").map_err(row_error)?;
        let existing_outcome: Option<String> =
            row.try_get("outcome_state").map_err(row_error)?;
        let existing_result: Option<serde_json::Value> =
            row.try_get("result").map_err(row_error)?;
        let existing_error: Option<String> =
            row.try_get("error").map_err(row_error)?;

        if existing_execution != execution_id
            || existing_event_type != event.as_str()
            || existing_provider != provider
            || existing_provider_execution_id != provider_execution_id
            || existing_request_hash != request_hash
            || existing_outcome.as_deref() != outcome.map(ProviderOutcomeState::as_str)
            || existing_result != result
            || existing_error.as_deref() != error
        {
            return Err(OrchestratorError::Serialization(
                "provider journal event identity conflict".into(),
            ));
        }

        (
            row.try_get("journal_id").map_err(row_error)?,
            row.try_get("journal_sequence").map_err(row_error)?,
        )
    };

    Ok(ProviderExecutionJournalEntry {
        journal_sequence,
        journal_id,
        execution_id,
        event_key,
        event,
        provider: provider.to_owned(),
        provider_execution_id: provider_execution_id.to_owned(),
        request_hash: request_hash.to_owned(),
        outcome,
        result,
        error: error.map(str::to_owned),
        recorded_at_ms,
    })
}

pub async fn _unused_guard() {}

impl PostgresExecutionStore {
    async fn append_provider_journal(
        &self,
        execution_id: Uuid,
        event: ProviderExecutionJournalEvent,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        outcome: Option<ProviderOutcomeState>,
        result: Option<serde_json::Value>,
        error: Option<&str>,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ProviderExecutionJournalEntry> {
        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let entry = insert_provider_journal_tx(
            &mut tx,
            execution_id,
            event,
            provider,
            provider_execution_id,
            request_hash,
            outcome,
            result,
            error,
            recorded_at_ms,
        )
        .await?;
        tx.commit().await.map_err(db_error)?;
        Ok(entry)
    }
}

#[async_trait]
pub trait ProviderExecutionJournalStore: Send + Sync {
    async fn append_submission(
        &self,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ProviderExecutionJournalEntry>;

    async fn append_observation(
        &self,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        outcome: ProviderOutcomeState,
        result: Option<serde_json::Value>,
        error: Option<&str>,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ProviderExecutionJournalEntry>;

    async fn list_execution_journal(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Vec<ProviderExecutionJournalEntry>>;
}

#[async_trait]
impl ProviderExecutionJournalStore for PostgresExecutionStore {
    async fn append_submission(
        &self,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ProviderExecutionJournalEntry> {
        self.append_provider_journal(
            execution_id,
            ProviderExecutionJournalEvent::Submitted,
            provider,
            provider_execution_id,
            request_hash,
            None,
            None,
            None,
            recorded_at_ms,
        )
        .await
    }

    async fn append_observation(
        &self,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        outcome: ProviderOutcomeState,
        result: Option<serde_json::Value>,
        error: Option<&str>,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<ProviderExecutionJournalEntry> {
        self.append_provider_journal(
            execution_id,
            ProviderExecutionJournalEvent::Observed,
            provider,
            provider_execution_id,
            request_hash,
            Some(outcome),
            result,
            error,
            recorded_at_ms,
        )
        .await
    }

    async fn list_execution_journal(
        &self,
        execution_id: Uuid,
    ) -> OrchestratorResult<Vec<ProviderExecutionJournalEntry>> {
        let rows = sqlx::query(
            "SELECT journal_sequence, journal_id, execution_id, event_key, event_type,
                    provider, provider_execution_id, request_hash, outcome_state, result, error,
                    EXTRACT(EPOCH FROM recorded_at) * 1000 AS recorded_at_ms
             FROM cat_provider_execution_journal
             WHERE execution_id = $1
             ORDER BY journal_sequence ASC",
        )
        .bind(execution_id)
        .fetch_all(self.pool())
        .await
        .map_err(db_error)?;

        rows.into_iter()
            .map(decode_journal)
            .collect::<OrchestratorResult<Vec<_>>>()
    }
}

fn decode_journal(
    row: sqlx::postgres::PgRow,
) -> OrchestratorResult<ProviderExecutionJournalEntry> {
    let event_type: String = row.try_get("event_type").map_err(row_error)?;
    let event = match event_type.as_str() {
        "submitted" => ProviderExecutionJournalEvent::Submitted,
        "observed" => ProviderExecutionJournalEvent::Observed,
        other => {
            return Err(OrchestratorError::Serialization(format!(
                "unknown provider journal event type: {other}"
            )));
        }
    };
    let outcome = row
        .try_get::<Option<String>, _>("outcome_state")
        .map_err(row_error)?
        .as_deref()
        .map(parse_outcome)
        .transpose()?;

    Ok(ProviderExecutionJournalEntry {
        journal_sequence: row.try_get("journal_sequence").map_err(row_error)?,
        journal_id: row.try_get("journal_id").map_err(row_error)?,
        execution_id: row.try_get("execution_id").map_err(row_error)?,
        event_key: row.try_get("event_key").map_err(row_error)?,
        event,
        provider: row.try_get("provider").map_err(row_error)?,
        provider_execution_id: row.try_get("provider_execution_id").map_err(row_error)?,
        request_hash: row.try_get("request_hash").map_err(row_error)?,
        outcome,
        result: row.try_get("result").map_err(row_error)?,
        error: row.try_get("error").map_err(row_error)?,
        recorded_at_ms: row
            .try_get::<f64, _>("recorded_at_ms")
            .map_err(row_error)?
            .max(0.0) as u64,
    })
}

fn parse_outcome(value: &str) -> OrchestratorResult<ProviderOutcomeState> {
    match value {
        "succeeded" => Ok(ProviderOutcomeState::Succeeded),
        "failed" => Ok(ProviderOutcomeState::Failed),
        "unknown" => Ok(ProviderOutcomeState::Unknown),
        other => Err(OrchestratorError::Serialization(format!(
            "unknown provider outcome state: {other}"
        ))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql provider journal error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!(
        "postgresql provider journal row error: {error}"
    ))
}

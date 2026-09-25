use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::provider_execution_journal::{insert_provider_journal_tx, ProviderExecutionJournalEvent};
use crate::{OrchestratorError, OrchestratorResult, PostgresExecutionStore, ProviderOutcomeState};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCallbackCorrelationState {
    Unmatched,
    Correlated,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCallbackReplayDisposition {
    Correlated,
    StillUnmatched,
    Rejected,
    AlreadyHandled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderCallbackReplayResult {
    pub callback_id: Uuid,
    pub provider_execution_id: String,
    pub disposition: ProviderCallbackReplayDisposition,
    pub execution_id: Option<Uuid>,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderCallback {
    pub callback_id: Uuid,
    pub provider: String,
    pub provider_execution_id: String,
    pub request_hash: Option<String>,
    pub outcome: ProviderOutcomeState,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
    pub received_at_ms: u64,
}

impl ProviderCallback {
    pub fn validate(&self) -> OrchestratorResult<()> {
        if self.callback_id.is_nil()
            || self.provider.trim().is_empty()
            || self.provider.len() > 128
            || self.provider_execution_id.trim().is_empty()
            || self.provider_execution_id.len() > 512
            || self.received_at_ms == 0
        {
            return Err(OrchestratorError::Serialization(
                "invalid provider callback identity or timestamp".into(),
            ));
        }

        if self
            .request_hash
            .as_ref()
            .is_some_and(|value| value.trim().is_empty())
        {
            return Err(OrchestratorError::Serialization(
                "provider callback request hash must not be empty when supplied".into(),
            ));
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderCallbackRecord {
    pub callback_sequence: i64,
    pub callback: ProviderCallback,
    pub execution_id: Option<Uuid>,
    pub correlation_state: ProviderCallbackCorrelationState,
    pub correlated_at_ms: Option<u64>,
    pub correlation_error: Option<String>,
}

pub(crate) fn callback_event_key(provider: &str, callback_id: Uuid) -> String {
    format!("callback:{provider}:{callback_id}")
}

#[async_trait]
pub trait ProviderCallbackStore: Send + Sync {
    async fn ingest_callback(
        &self,
        callback: ProviderCallback,
    ) -> OrchestratorResult<ProviderCallbackRecord>;

    async fn list_unmatched_callbacks(
        &self,
        provider: &str,
        limit: u32,
    ) -> OrchestratorResult<Vec<ProviderCallbackRecord>>;

    async fn reconcile_unmatched_callback(
        &self,
        callback_id: Uuid,
        reconciled_at_ms: u64,
    ) -> OrchestratorResult<ProviderCallbackReplayResult>;
}

impl PostgresExecutionStore {
    async fn ingest_provider_callback(
        &self,
        callback: ProviderCallback,
    ) -> OrchestratorResult<ProviderCallbackRecord> {
        callback.validate()?;

        let event_key = callback_event_key(&callback.provider, callback.callback_id);
        let received_at_ms = callback.received_at_ms;
        let mut tx = self.pool().begin().await.map_err(db_error)?;

        let inserted = sqlx::query(
            "INSERT INTO cat_provider_execution_callbacks
             (callback_id, event_key, provider, provider_execution_id, request_hash,
              outcome_state, result, error, received_at)
             VALUES ($1,$2,$3,$4,$5,$6,$7,$8,TO_TIMESTAMP($9 / 1000.0))
             ON CONFLICT (callback_id) DO NOTHING
             RETURNING callback_sequence",
        )
        .bind(callback.callback_id)
        .bind(&event_key)
        .bind(&callback.provider)
        .bind(&callback.provider_execution_id)
        .bind(&callback.request_hash)
        .bind(callback.outcome.as_str())
        .bind(&callback.result)
        .bind(&callback.error)
        .bind(callback.received_at_ms as f64)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        if inserted.is_none() {
            let existing = sqlx::query(
                "SELECT callback_sequence, callback_id, provider, provider_execution_id,
                        request_hash, outcome_state, result, error,
                        EXTRACT(EPOCH FROM received_at) * 1000 AS received_at_ms,
                        execution_id, correlation_state, correlation_error,
                        EXTRACT(EPOCH FROM correlated_at) * 1000 AS correlated_at_ms
                 FROM cat_provider_execution_callbacks
                 WHERE callback_id = $1",
            )
            .bind(callback.callback_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db_error)?;

            let existing_record = decode_callback(existing)?;
            if existing_record.callback.provider != callback.provider
                || existing_record.callback.provider_execution_id != callback.provider_execution_id
                || existing_record.callback.request_hash != callback.request_hash
                || existing_record.callback.outcome != callback.outcome
                || existing_record.callback.result != callback.result
                || existing_record.callback.error != callback.error
                || existing_record.callback.received_at_ms != callback.received_at_ms
            {
                return Err(OrchestratorError::Serialization(
                    "provider callback identity conflict".into(),
                ));
            }

            tx.commit().await.map_err(db_error)?;
            return Ok(existing_record);
        }

        let provider_row = sqlx::query(
            "SELECT execution_id, request_hash, outcome_state, provider_execution_id, result, error
             FROM cat_provider_execution_results
             WHERE provider = $1 AND provider_execution_id = $2
             FOR UPDATE",
        )
        .bind(&callback.provider)
        .bind(&callback.provider_execution_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        let Some(provider_row) = provider_row else {
            tx.commit().await.map_err(db_error)?;
            let callback_sequence = inserted
                .ok_or_else(|| OrchestratorError::Serialization("callback insert result missing".into()))?
                .try_get("callback_sequence")
                .map_err(row_error)?;
            return Ok(ProviderCallbackRecord {
                callback_sequence,
                callback,
                execution_id: None,
                correlation_state: ProviderCallbackCorrelationState::Unmatched,
                correlated_at_ms: None,
                correlation_error: None,
            });
        };

        let execution_id: Uuid = provider_row.try_get("execution_id").map_err(row_error)?;
        let stored_request_hash: String = provider_row.try_get("request_hash").map_err(row_error)?;
        if callback.request_hash.as_ref().is_some_and(|value| value != &stored_request_hash) {
            return Err(OrchestratorError::Serialization(
                "provider callback request hash does not match submitted execution".into(),
            ));
        }

        let stored_outcome: Option<String> = provider_row.try_get("outcome_state").map_err(row_error)?;
        let stored_result: Option<serde_json::Value> = provider_row.try_get("result").map_err(row_error)?;
        let stored_error: Option<String> = provider_row.try_get("error").map_err(row_error)?;
        if let Some(existing_outcome) = stored_outcome {
            if existing_outcome != callback.outcome.as_str()
                || stored_result != callback.result
                || stored_error != callback.error
            {
                return Err(OrchestratorError::Serialization(
                    "provider callback conflicts with an already recorded terminal outcome".into(),
                ));
            }
        }

        let callback_sequence = inserted
            .ok_or_else(|| OrchestratorError::Serialization("callback insert result missing".into()))?
            .try_get("callback_sequence")
            .map_err(row_error)?;

        self.correlate_callback_tx(
            &mut tx,
            callback.callback_id,
            execution_id,
            &callback.provider,
            &callback.provider_execution_id,
            &stored_request_hash,
            callback.outcome,
            callback.result.clone(),
            callback.error.as_deref(),
            callback.received_at_ms,
        )
        .await?;

        tx.commit().await.map_err(db_error)?;

        Ok(ProviderCallbackRecord {
            callback_sequence,
            callback,
            execution_id: Some(execution_id),
            correlation_state: ProviderCallbackCorrelationState::Correlated,
            correlated_at_ms: Some(received_at_ms),
            correlation_error: None,
        })
    }

    async fn list_unmatched_provider_callbacks(
        &self,
        provider: &str,
        limit: u32,
    ) -> OrchestratorResult<Vec<ProviderCallbackRecord>> {
        if provider.trim().is_empty() {
            return Err(OrchestratorError::Serialization(
                "provider name must not be empty".into(),
            ));
        }
        let limit = limit.clamp(1, 500) as i64;
        let rows = sqlx::query(
            "SELECT callback_sequence, callback_id, provider, provider_execution_id,
                    request_hash, outcome_state, result, error,
                    EXTRACT(EPOCH FROM received_at) * 1000 AS received_at_ms,
                    execution_id, correlation_state, correlation_error,
                    EXTRACT(EPOCH FROM correlated_at) * 1000 AS correlated_at_ms
             FROM cat_provider_execution_callbacks
             WHERE provider = $1 AND correlation_state = 'unmatched'
             ORDER BY callback_sequence ASC
             LIMIT $2",
        )
        .bind(provider)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(db_error)?;

        rows.into_iter().map(decode_callback).collect()
    }

    async fn reconcile_unmatched_provider_callback(
        &self,
        callback_id: Uuid,
        reconciled_at_ms: u64,
    ) -> OrchestratorResult<ProviderCallbackReplayResult> {
        if callback_id.is_nil() || reconciled_at_ms == 0 {
            return Err(OrchestratorError::Serialization(
                "invalid callback reconciliation identity or timestamp".into(),
            ));
        }

        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let row = sqlx::query(
            "SELECT callback_id, provider, provider_execution_id, request_hash, outcome_state,
                    result, error, EXTRACT(EPOCH FROM received_at) * 1000 AS received_at_ms,
                    execution_id, correlation_state, correlation_error
             FROM cat_provider_execution_callbacks
             WHERE callback_id = $1
             FOR UPDATE",
        )
        .bind(callback_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        let Some(row) = row else {
            return Err(OrchestratorError::Serialization(format!(
                "provider callback not found: {callback_id}"
            )));
        };

        let state: String = row.try_get("correlation_state").map_err(row_error)?;
        if state != "unmatched" {
            let provider_execution_id: String = row.try_get("provider_execution_id").map_err(row_error)?;
            let execution_id: Option<Uuid> = row.try_get("execution_id").map_err(row_error)?;
            tx.commit().await.map_err(db_error)?;
            return Ok(ProviderCallbackReplayResult {
                callback_id,
                provider_execution_id,
                disposition: ProviderCallbackReplayDisposition::AlreadyHandled,
                execution_id,
                reason: row.try_get("correlation_error").map_err(row_error)?,
            });
        }

        let provider: String = row.try_get("provider").map_err(row_error)?;
        let provider_execution_id: String = row.try_get("provider_execution_id").map_err(row_error)?;
        let callback_request_hash: Option<String> = row.try_get("request_hash").map_err(row_error)?;
        let outcome = parse_outcome(&row.try_get::<String, _>("outcome_state").map_err(row_error)?)?;
        let result: Option<serde_json::Value> = row.try_get("result").map_err(row_error)?;
        let error: Option<String> = row.try_get("error").map_err(row_error)?;
        let received_at_ms: f64 = row.try_get("received_at_ms").map_err(row_error)?;

        let provider_row = sqlx::query(
            "SELECT execution_id, request_hash, outcome_state, result, error
             FROM cat_provider_execution_results
             WHERE provider = $1 AND provider_execution_id = $2
             FOR UPDATE",
        )
        .bind(&provider)
        .bind(&provider_execution_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;

        let Some(provider_row) = provider_row else {
            tx.commit().await.map_err(db_error)?;
            return Ok(ProviderCallbackReplayResult {
                callback_id,
                provider_execution_id,
                disposition: ProviderCallbackReplayDisposition::StillUnmatched,
                execution_id: None,
                reason: None,
            });
        };

        let execution_id: Uuid = provider_row.try_get("execution_id").map_err(row_error)?;
        let request_hash: String = provider_row.try_get("request_hash").map_err(row_error)?;
        if callback_request_hash.as_ref().is_some_and(|value| value != &request_hash) {
            let reason = "provider callback request hash does not match submitted execution".to_owned();
            sqlx::query(
                "UPDATE cat_provider_execution_callbacks
                 SET correlation_state = 'rejected', correlation_error = $1, correlated_at = NULL
                 WHERE callback_id = $2",
            )
            .bind(&reason)
            .bind(callback_id)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
            tx.commit().await.map_err(db_error)?;
            return Ok(ProviderCallbackReplayResult {
                callback_id,
                provider_execution_id,
                disposition: ProviderCallbackReplayDisposition::Rejected,
                execution_id: Some(execution_id),
                reason: Some(reason),
            });
        }

        let stored_outcome: Option<String> = provider_row.try_get("outcome_state").map_err(row_error)?;
        let stored_result: Option<serde_json::Value> = provider_row.try_get("result").map_err(row_error)?;
        let stored_error: Option<String> = provider_row.try_get("error").map_err(row_error)?;
        if stored_outcome.as_deref().is_some_and(|value| value != outcome.as_str())
            || stored_outcome.is_some() && (stored_result != result || stored_error != error)
        {
            let reason = "provider callback conflicts with an already recorded terminal outcome".to_owned();
            sqlx::query(
                "UPDATE cat_provider_execution_callbacks
                 SET correlation_state = 'rejected', correlation_error = $1, correlated_at = NULL
                 WHERE callback_id = $2",
            )
            .bind(&reason)
            .bind(callback_id)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?;
            tx.commit().await.map_err(db_error)?;
            return Ok(ProviderCallbackReplayResult {
                callback_id,
                provider_execution_id,
                disposition: ProviderCallbackReplayDisposition::Rejected,
                execution_id: Some(execution_id),
                reason: Some(reason),
            });
        }

        self.correlate_callback_tx(
            &mut tx,
            callback_id(callback_id),
            execution_id,
            &provider,
            &provider_execution_id,
            &request_hash,
            outcome,
            result,
            error.as_deref(),
            received_at_ms.max(0.0) as u64,
        )
        .await?;

        tx.commit().await.map_err(db_error)?;
        Ok(ProviderCallbackReplayResult {
            callback_id,
            provider_execution_id,
            disposition: ProviderCallbackReplayDisposition::Correlated,
            execution_id: Some(execution_id),
            reason: None,
        })
    }

    async fn correlate_callback_tx(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        callback_id: Uuid,
        execution_id: Uuid,
        provider: &str,
        provider_execution_id: &str,
        request_hash: &str,
        outcome: ProviderOutcomeState,
        result: Option<serde_json::Value>,
        error: Option<&str>,
        recorded_at_ms: u64,
    ) -> OrchestratorResult<()> {
        sqlx::query(
            "UPDATE cat_provider_execution_callbacks
             SET execution_id = $1, correlation_state = 'correlated',
                 correlation_error = NULL, correlated_at = TO_TIMESTAMP($2 / 1000.0)
             WHERE callback_id = $3",
        )
        .bind(execution_id)
        .bind(recorded_at_ms as f64)
        .bind(callback_id)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        sqlx::query(
            "UPDATE cat_provider_execution_results
             SET outcome_state = $1, observed_at = TO_TIMESTAMP($2 / 1000.0),
                 result = $3, error = $4
             WHERE execution_id = $5 AND provider_execution_id = $6",
        )
        .bind(outcome.as_str())
        .bind(recorded_at_ms as f64)
        .bind(&result)
        .bind(error)
        .bind(execution_id)
        .bind(provider_execution_id)
        .execute(&mut *tx)
        .await
        .map_err(db_error)?;

        insert_provider_journal_tx(
            tx,
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
        .await?;

        Ok(())
    }
}

#[async_trait]
impl ProviderCallbackStore for PostgresExecutionStore {
    async fn ingest_callback(
        &self,
        callback: ProviderCallback,
    ) -> OrchestratorResult<ProviderCallbackRecord> {
        self.ingest_provider_callback(callback).await
    }

    async fn list_unmatched_callbacks(
        &self,
        provider: &str,
        limit: u32,
    ) -> OrchestratorResult<Vec<ProviderCallbackRecord>> {
        self.list_unmatched_provider_callbacks(provider, limit).await
    }

    async fn reconcile_unmatched_callback(
        &self,
        callback_id: Uuid,
        reconciled_at_ms: u64,
    ) -> OrchestratorResult<ProviderCallbackReplayResult> {
        self.reconcile_unmatched_provider_callback(callback_id, reconciled_at_ms)
            .await
    }
}

fn decode_callback(row: sqlx::postgres::PgRow) -> OrchestratorResult<ProviderCallbackRecord> {
    let outcome = parse_outcome(&row.try_get::<String, _>("outcome_state").map_err(row_error)?)?;
    let correlation_state = match row.try_get::<String, _>("correlation_state").map_err(row_error)?.as_str() {
        "unmatched" => ProviderCallbackCorrelationState::Unmatched,
        "correlated" => ProviderCallbackCorrelationState::Correlated,
        "rejected" => ProviderCallbackCorrelationState::Rejected,
        other => return Err(OrchestratorError::Serialization(format!("unknown provider callback correlation state: {other}"))),
    };
    let received_at_ms: f64 = row.try_get("received_at_ms").map_err(row_error)?;
    let correlated_at_ms: Option<f64> = row.try_get("correlated_at_ms").map_err(row_error)?;
    Ok(ProviderCallbackRecord {
        callback_sequence: row.try_get("callback_sequence").map_err(row_error)?,
        callback: ProviderCallback {
            callback_id: row.try_get("callback_id").map_err(row_error)?,
            provider: row.try_get("provider").map_err(row_error)?,
            provider_execution_id: row.try_get("provider_execution_id").map_err(row_error)?,
            request_hash: row.try_get("request_hash").map_err(row_error)?,
            outcome,
            result: row.try_get("result").map_err(row_error)?,
            error: row.try_get("error").map_err(row_error)?,
            received_at_ms: received_at_ms.max(0.0) as u64,
        },
        execution_id: row.try_get("execution_id").map_err(row_error)?,
        correlation_state,
        correlated_at_ms: correlated_at_ms.map(|value| value.max(0.0) as u64),
        correlation_error: row.try_get("correlation_error").map_err(row_error)?,
    })
}

fn parse_outcome(value: &str) -> OrchestratorResult<ProviderOutcomeState> {
    match value {
        "succeeded" => Ok(ProviderOutcomeState::Succeeded),
        "failed" => Ok(ProviderOutcomeState::Failed),
        "unknown" => Ok(ProviderOutcomeState::Unknown),
        other => Err(OrchestratorError::Serialization(format!("unknown provider outcome state: {other}"))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql provider callback error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql provider callback row error: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callback_event_key_is_provider_namespaced() {
        let id = Uuid::new_v4();
        assert!(callback_event_key("provider-a", id).contains("provider-a"));
        assert_ne!(callback_event_key("provider-a", id), callback_event_key("provider-b", id));
    }

    #[test]
    fn callback_validation_requires_identity_and_timestamp() {
        let callback = ProviderCallback {
            callback_id: Uuid::nil(),
            provider: "provider".into(),
            provider_execution_id: "remote-1".into(),
            request_hash: None,
            outcome: ProviderOutcomeState::Succeeded,
            result: None,
            error: None,
            received_at_ms: 1,
        };
        assert!(callback.validate().is_err());
    }
}
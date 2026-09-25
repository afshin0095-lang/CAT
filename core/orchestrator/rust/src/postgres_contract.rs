use cat_eventbus::EventEnvelope;
use uuid::Uuid;

use crate::{OrchestratorError, OrchestratorResult, WorkflowInstance};

/// PostgreSQL persistence contract. This module defines semantics without coupling the
/// Orchestrator crate to a particular async SQL client or connection pool.
pub trait PostgresDurableExecutor {
    fn load_workflow(&mut self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance>;

    /// The implementation must commit the workflow revision and all outbox rows in one
    /// database transaction and reject a stale `expected_revision`.
    fn commit_workflow_and_outbox(
        &mut self,
        workflow: &WorkflowInstance,
        expected_revision: u64,
        events: &[EventEnvelope],
    ) -> OrchestratorResult<()>;

    /// Acquire a resource lease and return its fencing token. The token must monotonically
    /// increase for each resource and become invalid as soon as another owner takes over.
    fn acquire_fenced_lease(
        &mut self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<u64>;
}

/// Canonical PostgreSQL table names reserved for the durable execution subsystem.
pub struct PostgresSchemaV1;

impl PostgresSchemaV1 {
    pub const WORKFLOWS: &'static str = "cat_workflows";
    pub const OUTBOX: &'static str = "cat_workflow_outbox";
    pub const LEASES: &'static str = "cat_execution_leases";
    pub const AUTHORIZATIONS: &'static str = "cat_execution_authorizations";
    pub const PROVIDER_JOURNAL: &'static str = "cat_provider_execution_journal";
    pub const PROVIDER_CALLBACKS: &'static str = "cat_provider_execution_callbacks";
    pub const AUDIT_EVENTS: &'static str = "cat_execution_audit_events";
    pub const AUDIT_READ_MODEL: &'static str = "cat_execution_audit_read_model";

    pub const CREATE_SQL: &'static str = r#"
CREATE TABLE IF NOT EXISTS cat_workflows (
    id UUID PRIMARY KEY,
    workflow_type TEXT NOT NULL,
    workflow_version INTEGER NOT NULL,
    state JSONB NOT NULL,
    revision BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cat_workflow_outbox (
    event_id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES cat_workflows(id),
    event_type TEXT NOT NULL,
    version INTEGER NOT NULL,
    occurred_at_ms BIGINT NOT NULL,
    payload JSONB NOT NULL,
    attempt INTEGER NOT NULL DEFAULT 0,
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    claimed_by TEXT,
    claimed_until TIMESTAMPTZ,
    last_error TEXT
);

CREATE TABLE IF NOT EXISTS cat_execution_leases (
    resource TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    fencing_token BIGINT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cat_execution_attempts (
    execution_id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES cat_workflows(id),
    step_id TEXT NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'failed', 'cancelled')),
    owner TEXT NOT NULL,
    fencing_token BIGINT NOT NULL CHECK (fencing_token >= 0),
    started_at TIMESTAMPTZ NOT NULL,
    heartbeat_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ,
    result JSONB,
    error TEXT,
    UNIQUE (workflow_id, step_id, attempt)
);

CREATE TABLE IF NOT EXISTS cat_execution_authorizations (
    execution_id UUID PRIMARY KEY REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    invocation_id UUID NOT NULL,
    agent_id UUID NOT NULL,
    capability_id TEXT NOT NULL,
    requested_side_effect TEXT NOT NULL CHECK (
        requested_side_effect IN ('S0', 'S1', 'S2', 'S3')
    ),
    required_policies JSONB NOT NULL,
    approval_reference TEXT,
    idempotency_key TEXT NOT NULL,
    correlation_id UUID NOT NULL,
    admitted_at TIMESTAMPTZ NOT NULL,
    UNIQUE (execution_id, idempotency_key)
);

CREATE TABLE IF NOT EXISTS cat_provider_execution_journal (
    journal_sequence BIGSERIAL PRIMARY KEY,
    journal_id UUID NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    event_key TEXT NOT NULL UNIQUE,
    event_type TEXT NOT NULL CHECK (event_type IN ('submitted', 'observed')),
    provider TEXT NOT NULL,
    provider_execution_id TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    outcome_state TEXT CHECK (outcome_state IN ('succeeded', 'failed', 'unknown')),
    result JSONB,
    error TEXT,
    recorded_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE IF NOT EXISTS cat_provider_execution_callbacks (
    callback_sequence BIGSERIAL PRIMARY KEY,
    callback_id UUID NOT NULL UNIQUE,
    event_key TEXT NOT NULL UNIQUE,
    provider TEXT NOT NULL,
    provider_execution_id TEXT NOT NULL,
    request_hash TEXT,
    outcome_state TEXT NOT NULL CHECK (outcome_state IN ('succeeded', 'failed', 'unknown')),
    result JSONB,
    error TEXT,
    received_at TIMESTAMPTZ NOT NULL,
    execution_id UUID,
    correlation_state TEXT NOT NULL CHECK (correlation_state IN ('unmatched', 'correlated', 'rejected')),
    correlation_error TEXT,
    correlated_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS cat_execution_audit_events (
    audit_sequence BIGSERIAL PRIMARY KEY,
    audit_id UUID NOT NULL UNIQUE,
    event_key TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    workflow_id UUID NOT NULL,
    step_id TEXT NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    action TEXT NOT NULL CHECK (
        action IN ('noop', 'continue', 'confirm_success', 'confirm_failure', 'manual_review')
    ),
    agent_id UUID NOT NULL,
    capability_id TEXT NOT NULL,
    requested_side_effect TEXT NOT NULL CHECK (
        requested_side_effect IN ('S0', 'S1', 'S2', 'S3')
    ),
    approval_reference TEXT,
    correlation_id UUID NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    evidence JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS cat_execution_audit_read_model (
    execution_id UUID PRIMARY KEY REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    event_key TEXT NOT NULL,
    workflow_id UUID NOT NULL,
    step_id TEXT NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'failed', 'cancelled')),
    action TEXT NOT NULL CHECK (
        action IN ('noop', 'continue', 'confirm_success', 'confirm_failure', 'manual_review')
    ),
    agent_id UUID NOT NULL,
    capability_id TEXT NOT NULL,
    requested_side_effect TEXT NOT NULL CHECK (
        requested_side_effect IN ('S0', 'S1', 'S2', 'S3')
    ),
    approval_reference TEXT,
    correlation_id UUID NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    source_audit_sequence BIGINT NOT NULL,
    evidence JSONB NOT NULL
);
"#;

    pub fn validate_identifier(value: &str) -> OrchestratorResult<()> {
        if value.is_empty()
            || !value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte == b'.'
            })
        {
            return Err(OrchestratorError::Serialization(
                "invalid PostgreSQL resource identifier".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_has_expected_tables() {
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::WORKFLOWS));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::OUTBOX));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::LEASES));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::AUTHORIZATIONS));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::PROVIDER_JOURNAL));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::PROVIDER_CALLBACKS));
        assert!(PostgresSchemaV1::CREATE_SQL.contains("correlation_error"));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::AUDIT_EVENTS));
        assert!(PostgresSchemaV1::CREATE_SQL.contains(PostgresSchemaV1::AUDIT_READ_MODEL));
    }

    #[test]
    fn identifiers_are_restricted_to_safe_characters() {
        PostgresSchemaV1::validate_identifier("workflow/1").unwrap_err();
        PostgresSchemaV1::validate_identifier("cat_workflows").unwrap();
        PostgresSchemaV1::validate_identifier("schema.table").unwrap();
    }
}

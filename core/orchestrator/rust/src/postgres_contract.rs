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
"#;

    pub fn validate_identifier(value: &str) -> OrchestratorResult<()> {
        if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-' || byte == b'.') {
            return Err(OrchestratorError::Serialization("invalid PostgreSQL resource identifier".into()));
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
    }

    #[test]
    fn identifiers_are_restricted_to_safe_characters() {
        PostgresSchemaV1::validate_identifier("workflow/1").unwrap_err();
        PostgresSchemaV1::validate_identifier("cat_workflows").unwrap();
        PostgresSchemaV1::validate_identifier("schema.table").unwrap();
    }
}

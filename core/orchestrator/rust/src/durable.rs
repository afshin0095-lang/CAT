use std::collections::HashMap;

use uuid::Uuid;

use crate::{Lease, OrchestratorError, OrchestratorResult, WorkflowInstance};

/// Persistence boundary for workflow instances.
///
/// Implementations may be backed by Postgres, SQLite, a replicated KV store, or another
/// durable system. The orchestrator only relies on optimistic revision checks.
pub trait WorkflowRepository {
    fn load(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance>;
    fn save(&mut self, workflow: WorkflowInstance, expected_revision: u64) -> OrchestratorResult<()>;
}

/// Event publication boundary for durable execution.
pub trait ExecutionEventSink {
    fn publish(&mut self, event: cat_eventbus::EventEnvelope) -> OrchestratorResult<()>;
}

/// Lease acquisition boundary. External implementations should use a distributed lease store.
pub trait LeaseProvider {
    fn acquire(&mut self, resource: &str, owner: &str, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<Lease>;
}

/// Small deterministic repository for unit tests and local development.
#[derive(Default)]
pub struct InMemoryWorkflowRepository {
    workflows: HashMap<Uuid, WorkflowInstance>,
}

impl InMemoryWorkflowRepository {
    pub fn insert(&mut self, workflow: WorkflowInstance) {
        self.workflows.insert(workflow.id, workflow);
    }
}

impl WorkflowRepository for InMemoryWorkflowRepository {
    fn load(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance> {
        self.workflows
            .get(&workflow_id)
            .cloned()
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(workflow_id.to_string()))
    }

    fn save(&mut self, workflow: WorkflowInstance, expected_revision: u64) -> OrchestratorResult<()> {
        match self.workflows.get(&workflow.id) {
            Some(current) if current.revision == expected_revision => {
                self.workflows.insert(workflow.id, workflow);
                Ok(())
            }
            Some(current) => Err(OrchestratorError::RevisionConflict {
                workflow_id: workflow.id.to_string(),
                expected: expected_revision,
                actual: current.revision,
            }),
            None => Err(OrchestratorError::WorkflowNotFound(workflow.id.to_string())),
        }
    }
}

#[derive(Default)]
pub struct InMemoryLeaseProvider {
    leases: HashMap<String, Lease>,
}

impl LeaseProvider for InMemoryLeaseProvider {
    fn acquire(&mut self, resource: &str, owner: &str, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<Lease> {
        if let Some(existing) = self.leases.get(resource) {
            if existing.expires_at_ms > now_ms && existing.owner != owner {
                return Err(OrchestratorError::LeaseUnavailable { resource: resource.to_owned() });
            }
        }

        let lease = Lease::acquire(resource, owner, now_ms, ttl_ms);
        self.leases.insert(resource.to_owned(), lease.clone());
        Ok(lease)
    }
}

/// In-memory sink that preserves event order for deterministic tests.
#[derive(Default)]
pub struct RecordingExecutionEventSink {
    events: Vec<cat_eventbus::EventEnvelope>,
}

impl RecordingExecutionEventSink {
    pub fn events(&self) -> &[cat_eventbus::EventEnvelope] {
        &self.events
    }
}

impl ExecutionEventSink for RecordingExecutionEventSink {
    fn publish(&mut self, event: cat_eventbus::EventEnvelope) -> OrchestratorResult<()> {
        self.events.push(event);
        Ok(())
    }
}

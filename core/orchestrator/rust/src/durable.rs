use std::collections::HashMap;

use cat_eventbus::{EventBus, EventEnvelope};
use uuid::Uuid;

use crate::{Lease, OrchestratorError, OrchestratorResult, WorkflowInstance};

pub trait DurableWorkflowStore {
    fn load(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance>;
    fn commit(
        &mut self,
        workflow: WorkflowInstance,
        expected_revision: u64,
        events: &[EventEnvelope],
    ) -> OrchestratorResult<()>;
}

pub trait ExecutionEventSink {
    fn publish(&mut self, event: EventEnvelope) -> OrchestratorResult<()>;
}

pub struct EventBusExecutionEventSink<'a> {
    bus: &'a EventBus,
}

impl<'a> EventBusExecutionEventSink<'a> {
    pub fn new(bus: &'a EventBus) -> Self {
        Self { bus }
    }
}

impl ExecutionEventSink for EventBusExecutionEventSink<'_> {
    fn publish(&mut self, event: EventEnvelope) -> OrchestratorResult<()> {
        self.bus
            .publish(event)
            .map(|_| ())
            .map_err(|error| OrchestratorError::Serialization(error.to_string()))
    }
}

#[derive(Default)]
pub struct RecordingExecutionEventSink {
    events: Vec<EventEnvelope>,
}

impl RecordingExecutionEventSink {
    pub fn events(&self) -> &[EventEnvelope] {
        &self.events
    }
}

impl ExecutionEventSink for RecordingExecutionEventSink {
    fn publish(&mut self, event: EventEnvelope) -> OrchestratorResult<()> {
        self.events.push(event);
        Ok(())
    }
}

pub trait LeaseProvider {
    fn acquire(
        &mut self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<Lease>;
}

#[derive(Default)]
pub struct InMemoryDurableWorkflowStore {
    workflows: HashMap<Uuid, WorkflowInstance>,
    outbox: Vec<EventEnvelope>,
}

impl InMemoryDurableWorkflowStore {
    pub fn insert(&mut self, workflow: WorkflowInstance) {
        self.workflows.insert(workflow.id, workflow);
    }
    pub fn outbox(&self) -> &[EventEnvelope] {
        &self.outbox
    }
}

impl DurableWorkflowStore for InMemoryDurableWorkflowStore {
    fn load(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowInstance> {
        self.workflows
            .get(&workflow_id)
            .cloned()
            .ok_or_else(|| OrchestratorError::WorkflowNotFound(workflow_id.to_string()))
    }

    fn commit(
        &mut self,
        workflow: WorkflowInstance,
        expected_revision: u64,
        events: &[EventEnvelope],
    ) -> OrchestratorResult<()> {
        match self.workflows.get(&workflow.id) {
            Some(current) if current.revision == expected_revision => {
                self.workflows.insert(workflow.id, workflow);
                self.outbox.extend_from_slice(events);
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
    fn acquire(
        &mut self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<Lease> {
        if let Some(existing) = self.leases.get(resource) {
            if existing.expires_at_ms > now_ms && existing.owner != owner {
                return Err(OrchestratorError::LeaseUnavailable {
                    resource: resource.to_owned(),
                });
            }
        }
        let lease = Lease::acquire(resource, owner, now_ms, ttl_ms);
        self.leases.insert(resource.to_owned(), lease.clone());
        Ok(lease)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{StepState, WorkflowDefinition, WorkflowState, WorkflowStep};

    fn workflow() -> WorkflowInstance {
        WorkflowInstance {
            id: Uuid::now_v7(),
            definition: WorkflowDefinition {
                workflow_type: "test".into(),
                version: 1,
                steps: vec![WorkflowStep {
                    id: "work".into(),
                    capability_id: cat_kernel::CapabilityId::new("cat.capability.test.work.v1").unwrap(),
                    dependencies: vec![],
                    state: StepState::Ready,
                    attempt: 0,
                    max_attempts: 3,
                    compensation_step: None,
                }],
            },
            state: WorkflowState::Running,
            revision: 0,
        }
    }

    #[test]
    fn commit_persists_state_and_outbox() {
        let mut store = InMemoryDurableWorkflowStore::default();
        let workflow = workflow();
        let id = workflow.id;
        store.insert(workflow.clone());
        let event = EventEnvelope {
            event_id: Uuid::now_v7(),
            event_type: "test.event".into(),
            version: 1,
            kind: cat_eventbus::EventKind::Domain,
            occurred_at_ms: 10,
            producer: "test".into(),
            correlation_id: None,
            causation_id: None,
            subject_id: Some(id),
            payload: serde_json::json!({}),
        };
        let mut committed = workflow;
        committed.revision = 1;
        store.commit(committed, 0, &[event]).unwrap();
        assert_eq!(store.load(id).unwrap().revision, 1);
        assert_eq!(store.outbox().len(), 1);
    }

    #[test]
    fn stale_revision_is_rejected() {
        let mut store = InMemoryDurableWorkflowStore::default();
        let workflow = workflow();
        let id = workflow.id;
        store.insert(workflow.clone());
        let mut committed = workflow;
        committed.revision = 1;
        store.commit(committed, 0, &[]).unwrap();
        let stale = store.load(id).unwrap();
        assert!(matches!(
            store.commit(stale, 0, &[]),
            Err(OrchestratorError::RevisionConflict { .. })
        ));
    }

    #[test]
    fn lease_is_exclusive_until_expiry() {
        let mut provider = InMemoryLeaseProvider::default();
        provider.acquire("workflow/1", "worker-a", 100, 50).unwrap();
        assert!(matches!(
            provider.acquire("workflow/1", "worker-b", 120, 50),
            Err(OrchestratorError::LeaseUnavailable { .. })
        ));
        provider.acquire("workflow/1", "worker-b", 150, 50).unwrap();
    }
}

use cat_eventbus::{CatEvent, EventEnvelope, EventBusResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{StepState, WorkflowState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStarted {
    pub workflow_id: Uuid,
    pub workflow_type: String,
    pub workflow_version: u16,
}

impl CatEvent for WorkflowStarted {
    const TYPE: &'static str = "orchestrator.workflow.started";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStepStateChanged {
    pub workflow_id: Uuid,
    pub step_id: String,
    pub state: StepState,
    pub attempt: u32,
    pub revision: u64,
}

impl CatEvent for WorkflowStepStateChanged {
    const TYPE: &'static str = "orchestrator.workflow.step.state_changed";
    const VERSION: u16 = 1;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowCompleted {
    pub workflow_id: Uuid,
    pub state: WorkflowState,
    pub revision: u64,
}

impl CatEvent for WorkflowCompleted {
    const TYPE: &'static str = "orchestrator.workflow.completed";
    const VERSION: u16 = 1;
}

pub struct WorkflowEventFactory;

impl WorkflowEventFactory {
    pub fn started(
        workflow_id: Uuid,
        workflow_type: impl Into<String>,
        workflow_version: u16,
        producer: impl Into<String>,
    ) -> EventBusResult<EventEnvelope> {
        EventEnvelope::from_typed(
            WorkflowStarted {
                workflow_id,
                workflow_type: workflow_type.into(),
                workflow_version,
            },
            producer,
        )
    }

    pub fn step_state_changed(
        workflow_id: Uuid,
        step_id: impl Into<String>,
        state: StepState,
        attempt: u32,
        revision: u64,
        producer: impl Into<String>,
    ) -> EventBusResult<EventEnvelope> {
        EventEnvelope::from_typed(
            WorkflowStepStateChanged {
                workflow_id,
                step_id: step_id.into(),
                state,
                attempt,
                revision,
            },
            producer,
        )
    }

    pub fn completed(
        workflow_id: Uuid,
        state: WorkflowState,
        revision: u64,
        producer: impl Into<String>,
    ) -> EventBusResult<EventEnvelope> {
        EventEnvelope::from_typed(
            WorkflowCompleted {
                workflow_id,
                state,
                revision,
            },
            producer,
        )
    }
}

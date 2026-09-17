use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowState {
    Pending,
    Running,
    Waiting,
    Compensating,
    Succeeded,
    Failed,
    Cancelled,
}

impl WorkflowState {
    pub fn terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepState {
    Pending,
    Ready,
    Running,
    Waiting,
    Succeeded,
    Failed,
    Compensated,
    Skipped,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,
    pub dependencies: Vec<String>,
    pub state: StepState,
    pub attempt: u32,
    pub max_attempts: u32,
    pub compensation_step: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    pub workflow_type: String,
    pub version: u16,
    pub steps: Vec<WorkflowStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowInstance {
    pub id: Uuid,
    pub definition: WorkflowDefinition,
    pub state: WorkflowState,
    pub revision: u64,
}

impl WorkflowInstance {
    pub fn new(definition: WorkflowDefinition) -> Self {
        Self {
            id: Uuid::now_v7(),
            definition,
            state: WorkflowState::Pending,
            revision: 0,
        }
    }
}

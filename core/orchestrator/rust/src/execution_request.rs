use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub workflow_id: Uuid,
    pub step_id: String,
    pub attempt: u32,
    pub requested_at_ms: u64,
}

impl ExecutionRequest {
    pub fn new(
        workflow_id: Uuid,
        step_id: impl Into<String>,
        attempt: u32,
        requested_at_ms: u64,
    ) -> Self {
        Self {
            workflow_id,
            step_id: step_id.into(),
            attempt,
            requested_at_ms,
        }
    }
}

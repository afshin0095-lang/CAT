use crate::{ReasoningResult, ReasoningStep};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningTrace {
    pub trace_id: Uuid,
    pub reasoning_id: Uuid,
    pub request_id: Uuid,
    pub steps: Vec<ReasoningStep>,
    pub created_at_ms: u64,
}

impl ReasoningTrace {
    pub fn from_result(result: &ReasoningResult, created_at_ms: u64) -> Self {
        Self {
            trace_id: Uuid::now_v7(),
            reasoning_id: result.reasoning_id,
            request_id: result.request_id,
            steps: result.steps.clone(),
            created_at_ms,
        }
    }
}

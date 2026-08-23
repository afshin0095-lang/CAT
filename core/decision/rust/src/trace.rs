use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionTraceStep {
    pub step_id: Uuid,
    pub stage: String,
    pub description: String,
    pub evidence: Vec<Uuid>,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionTrace {
    pub trace_id: Uuid,
    pub decision_id: Uuid,
    pub steps: Vec<DecisionTraceStep>,
}

impl DecisionTrace {
    pub fn new(decision_id: Uuid) -> Self {
        Self { trace_id: Uuid::now_v7(), decision_id, steps: Vec::new() }
    }

    pub fn push(&mut self, stage: impl Into<String>, description: impl Into<String>) {
        self.steps.push(DecisionTraceStep {
            step_id: Uuid::now_v7(),
            stage: stage.into(),
            description: description.into(),
            evidence: Vec::new(),
            metadata: Value::Null,
        });
    }

    pub fn is_immutable_view(&self) -> bool { true }
}

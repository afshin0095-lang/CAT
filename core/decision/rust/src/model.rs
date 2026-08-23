use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Alternative {
    pub id: String,
    pub label: String,
    pub rationale: String,
    pub expected_value: f64,
    pub confidence: f64,
    pub constraints_satisfied: bool,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionRequest {
    pub decision_id: Uuid,
    pub objective: String,
    pub alternatives: Vec<Alternative>,
    pub required_confidence: f64,
    pub require_human_approval: bool,
    pub context: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DecisionStatus {
    Proposed,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionOutcome {
    pub decision_id: Uuid,
    pub selected: Option<Alternative>,
    pub status: DecisionStatus,
    pub confidence: f64,
    pub policy_reasons: Vec<String>,
    pub advisory_only: bool,
    pub trace_id: Uuid,
}

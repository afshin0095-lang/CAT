use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionClass {
    Advisory,
    Operational,
    HighImpact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionState {
    Proposed,
    Approved,
    Rejected,
    Executed,
    Superseded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequirement {
    None,
    Policy,
    Human,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionCandidate {
    pub id: Uuid,
    pub policy_id: String,
    pub class: DecisionClass,
    pub title: String,
    pub rationale: String,
    pub evidence: Vec<String>,
    pub confidence_bps: u16,
    pub expected_value: Option<f64>,
    pub risk_score_bps: u16,
    pub proposed_action: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub decision_id: Uuid,
    pub candidate_id: Uuid,
    pub state: DecisionState,
    pub approval: ApprovalRequirement,
    pub policy_id: String,
    pub actor: String,
    pub reason: String,
    pub action: Value,
}

impl DecisionCandidate {
    pub fn new(policy_id: impl Into<String>, class: DecisionClass, title: impl Into<String>, action: Value) -> Self {
        Self {
            id: Uuid::now_v7(),
            policy_id: policy_id.into(),
            class,
            title: title.into(),
            rationale: String::new(),
            evidence: Vec::new(),
            confidence_bps: 0,
            expected_value: None,
            risk_score_bps: 0,
            proposed_action: action,
        }
    }
}

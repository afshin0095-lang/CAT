use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningMode {
    Deterministic,
    Assisted,
    Llm,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningRequest {
    pub request_id: Uuid,
    pub objective: String,
    pub context: Value,
    pub evidence: Vec<Evidence>,
    pub mode: ReasoningMode,
    pub max_steps: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_id: Uuid,
    pub source: String,
    pub statement: String,
    pub confidence: f32,
    pub authoritative: bool,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hypothesis {
    pub hypothesis_id: Uuid,
    pub statement: String,
    pub support: f32,
    pub contradiction: f32,
    pub evidence_ids: Vec<Uuid>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_id: Uuid,
    pub sequence: u16,
    pub operation: String,
    pub input_ids: Vec<Uuid>,
    pub output: String,
    pub confidence: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub reasoning_id: Uuid,
    pub request_id: Uuid,
    pub conclusion: String,
    pub confidence: f32,
    pub hypotheses: Vec<Hypothesis>,
    pub steps: Vec<ReasoningStep>,
    pub evidence_used: Vec<Uuid>,
    pub assumptions: Vec<String>,
    pub advisory_only: bool,
}

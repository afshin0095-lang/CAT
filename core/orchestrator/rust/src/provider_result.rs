use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderOutcomeState {
    Succeeded,
    Failed,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReconciliationAction {
    Noop,
    Continue,
    ConfirmSuccess,
    ConfirmFailure,
    ManualReview,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderExecutionRecord {
    pub execution_id: Uuid,
    pub provider: String,
    pub provider_execution_id: String,
    pub request_hash: String,
    pub submitted_at_ms: u64,
    pub outcome: Option<ProviderOutcomeState>,
    pub observed_at_ms: Option<u64>,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl ProviderExecutionRecord {
    pub fn action(&self) -> ReconciliationAction {
        match self.outcome {
            None => ReconciliationAction::Continue,
            Some(ProviderOutcomeState::Succeeded) => ReconciliationAction::ConfirmSuccess,
            Some(ProviderOutcomeState::Failed) => ReconciliationAction::ConfirmFailure,
            Some(ProviderOutcomeState::Unknown) => ReconciliationAction::ManualReview,
        }
    }

    pub fn is_observed(&self) -> bool {
        self.outcome.is_some() && self.observed_at_ms.is_some()
    }
}

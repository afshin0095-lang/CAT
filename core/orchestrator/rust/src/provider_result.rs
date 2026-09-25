use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderOutcomeState {
    Succeeded,
    Failed,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl ReconciliationAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Noop => "noop",
            Self::Continue => "continue",
            Self::ConfirmSuccess => "confirm_success",
            Self::ConfirmFailure => "confirm_failure",
            Self::ManualReview => "manual_review",
        }
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "noop" => Some(Self::Noop),
            "continue" => Some(Self::Continue),
            "confirm_success" => Some(Self::ConfirmSuccess),
            "confirm_failure" => Some(Self::ConfirmFailure),
            "manual_review" => Some(Self::ManualReview),
            _ => None,
        }
    }
}

impl ProviderOutcomeState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }
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

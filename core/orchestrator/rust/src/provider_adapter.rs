use async_trait::async_trait;
use serde_json::Value;

use crate::{OrchestratorResult, ProviderExecutionRecord, ProviderOutcomeState};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderExecutionRequest {
    pub provider: String,
    pub operation: String,
    pub idempotency_key: String,
    pub request_hash: String,
    pub payload: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderExecutionSubmission {
    pub provider_execution_id: String,
    pub accepted_at_ms: u64,
}

#[async_trait]
pub trait ProviderExecutionAdapter: Send + Sync {
    fn provider_name(&self) -> &str;

    async fn submit(
        &self,
        request: ProviderExecutionRequest,
    ) -> OrchestratorResult<ProviderExecutionSubmission>;

    async fn lookup(
        &self,
        provider_execution_id: &str,
    ) -> OrchestratorResult<Option<ProviderExecutionRecord>>;
}

pub fn idempotency_key(execution_id: &str, request_hash: &str) -> String {
    format!("cat:{}:{}", execution_id, request_hash)
}

pub fn normalize_provider_outcome(
    record: Option<ProviderExecutionRecord>,
) -> Option<ProviderOutcomeState> {
    record.and_then(|value| value.outcome)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_is_stable_for_same_execution_and_request() {
        assert_eq!(idempotency_key("exec-1", "hash-a"), idempotency_key("exec-1", "hash-a"));
        assert_ne!(idempotency_key("exec-1", "hash-a"), idempotency_key("exec-2", "hash-a"));
    }
}

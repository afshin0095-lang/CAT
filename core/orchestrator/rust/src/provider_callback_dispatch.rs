use std::collections::BTreeMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    OrchestratorError, OrchestratorResult, ProviderCallback, ProviderCallbackRecord,
    ProviderCallbackVerificationEvidence,
    ProviderCallbackStore, ProviderOutcomeState,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderCallbackIngress {
    pub callback_id: Uuid,
    pub provider: String,
    pub headers: BTreeMap<String, String>,
    pub body: serde_json::Value,
    pub received_at_ms: u64,
}

impl ProviderCallbackIngress {
    pub fn validate(&self) -> OrchestratorResult<()> {
        if self.callback_id.is_nil()
            || self.provider.trim().is_empty()
            || self.provider.len() > 128
            || self.received_at_ms == 0
        {
            return Err(OrchestratorError::Serialization(
                "invalid provider callback ingress".into(),
            ));
        }
        Ok(())
    }
}

#[async_trait]
pub trait ProviderCallbackVerifier: Send + Sync {
    fn provider_name(&self) -> &str;

    /// Verify provider-specific authenticity and normalize the external payload.
    ///
    /// Implementations may use signed headers, provider SDK verification, or another
    /// provider-owned mechanism. Raw credentials remain outside CAT domain contracts.
    async fn verify(
        &self,
        ingress: &ProviderCallbackIngress,
    ) -> OrchestratorResult<ProviderCallback>;
}

#[derive(Default, Clone)]
pub struct ProviderCallbackVerifierRegistry {
    verifiers: BTreeMap<String, Arc<dyn ProviderCallbackVerifier>>,
}

impl ProviderCallbackVerifierRegistry {
    pub fn register(
        &mut self,
        verifier: Arc<dyn ProviderCallbackVerifier>,
    ) -> OrchestratorResult<()> {
        let provider = verifier.provider_name().trim();
        if provider.is_empty() || provider.len() > 128 {
            return Err(OrchestratorError::Serialization(
                "provider verifier name is invalid".into(),
            ));
        }
        if self.verifiers.contains_key(provider) {
            return Err(OrchestratorError::Serialization(format!(
                "provider callback verifier already registered: {provider}"
            )));
        }
        self.verifiers.insert(provider.to_owned(), verifier);
        Ok(())
    }

    pub fn get(&self, provider: &str) -> Option<Arc<dyn ProviderCallbackVerifier>> {
        self.verifiers.get(provider).cloned()
    }

    pub fn names(&self) -> Vec<&str> {
        self.verifiers.keys().map(String::as_str).collect()
    }
}

pub struct ProviderCallbackDispatcher<S> {
    pub store: S,
    pub verifiers: ProviderCallbackVerifierRegistry,
}

impl<S> ProviderCallbackDispatcher<S>
where
    S: ProviderCallbackStore,
{
    pub fn new(store: S, verifiers: ProviderCallbackVerifierRegistry) -> Self {
        Self { store, verifiers }
    }

    pub async fn dispatch(
        &self,
        ingress: ProviderCallbackIngress,
    ) -> OrchestratorResult<ProviderCallbackDispatchResult> {
        ingress.validate()?;

        let verifier = self.verifiers.get(&ingress.provider).ok_or_else(|| {
            OrchestratorError::InvalidAuthorizationInput(format!(
                "no callback verifier is registered for provider {}",
                ingress.provider
            ))
        })?;

        if verifier.provider_name() != ingress.provider {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "callback verifier/provider routing identity mismatch".into(),
            ));
        }

        let callback = verifier.verify(&ingress).await?;
        callback.validate()?;

        if callback.callback_id != ingress.callback_id
            || callback.provider != ingress.provider
            || callback.received_at_ms != ingress.received_at_ms
        {
            return Err(OrchestratorError::Serialization(
                "provider verifier returned callback identity mismatch".into(),
            ));
        }

        let record = self.store.ingest_callback(callback).await?;
        Ok(ProviderCallbackDispatchResult::from_record(&record))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCallbackDispatchDisposition {
    Correlated,
    Unmatched,
    Rejected,
    AlreadyHandled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderCallbackDispatchResult {
    pub callback_id: Uuid,
    pub provider: String,
    pub provider_execution_id: String,
    pub disposition: ProviderCallbackDispatchDisposition,
    pub execution_id: Option<Uuid>,
    pub reason: Option<String>,
}

impl ProviderCallbackDispatchResult {
    fn from_record(record: &ProviderCallbackRecord) -> Self {
        let disposition = match record.correlation_state {
            crate::ProviderCallbackCorrelationState::Correlated =>
                ProviderCallbackDispatchDisposition::Correlated,
            crate::ProviderCallbackCorrelationState::Unmatched =>
                ProviderCallbackDispatchDisposition::Unmatched,
            crate::ProviderCallbackCorrelationState::Rejected =>
                ProviderCallbackDispatchDisposition::Rejected,
        };

        Self {
            callback_id: record.callback.callback_id,
            provider: record.callback.provider.clone(),
            provider_execution_id: record.callback.provider_execution_id.clone(),
            disposition,
            execution_id: record.execution_id,
            reason: record.correlation_error.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct TestVerifier;

    #[async_trait]
    impl ProviderCallbackVerifier for TestVerifier {
        fn provider_name(&self) -> &str {
            "test-provider"
        }

        async fn verify(
            &self,
            ingress: &ProviderCallbackIngress,
        ) -> OrchestratorResult<ProviderCallback> {
            let execution_id = ingress
                .body
                .get("provider_execution_id")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(|| OrchestratorError::Serialization("missing provider execution id".into()))?;
            Ok(ProviderCallback {
                callback_id: ingress.callback_id,
                provider: ingress.provider.clone(),
                provider_execution_id: execution_id.to_owned(),
                request_hash: None,
                outcome: ProviderOutcomeState::Succeeded,
                result: Some(ingress.body.clone()),
                error: None,
                received_at_ms: ingress.received_at_ms,
            })
        }
    }

    #[test]
    fn registry_rejects_duplicate_verifier() {
        let mut registry = ProviderCallbackVerifierRegistry::default();
        registry.register(Arc::new(TestVerifier)).unwrap();
        assert!(registry.register(Arc::new(TestVerifier)).is_err());
        assert_eq!(registry.names(), vec!["test-provider"]);
    }

    #[test]
    fn ingress_validation_rejects_missing_identity() {
        let ingress = ProviderCallbackIngress {
            callback_id: Uuid::nil(),
            provider: "test-provider".into(),
            headers: BTreeMap::new(),
            body: serde_json::json!({}),
            received_at_ms: 1,
        };
        assert!(ingress.validate().is_err());
    }
}
use crate::{AdapterRequest, AdapterResponse, IntegrationTarget, PlatformError, PlatformResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn new(value: impl Into<String>) -> PlatformResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(PlatformError::InvalidCommand("provider id cannot be empty".into()));
        }
        if value.chars().any(|c| c.is_whitespace()) {
            return Err(PlatformError::InvalidCommand("provider id cannot contain whitespace".into()));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderHealth {
    Unknown,
    Ready,
    Degraded,
    Unavailable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    pub provider_id: ProviderId,
    pub target: IntegrationTarget,
    pub operations: Vec<String>,
    pub health: ProviderHealth,
}

pub trait ExternalProviderAdapter: Send + Sync {
    fn capabilities(&self) -> ProviderCapabilities;
    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse>;
}

#[derive(Default)]
pub struct ProviderAdapterRegistry {
    adapters: BTreeMap<ProviderId, Arc<dyn ExternalProviderAdapter>>,
}

impl ProviderAdapterRegistry {
    pub fn register(&mut self, adapter: Arc<dyn ExternalProviderAdapter>) -> PlatformResult<()> {
        let capabilities = adapter.capabilities();
        if capabilities.operations.is_empty() {
            return Err(PlatformError::InvalidCommand(format!(
                "provider {} must declare at least one operation",
                capabilities.provider_id.as_str()
            )));
        }
        if self.adapters.contains_key(&capabilities.provider_id) {
            return Err(PlatformError::InvalidCommand(format!(
                "provider already registered: {}",
                capabilities.provider_id.as_str()
            )));
        }
        self.adapters.insert(capabilities.provider_id, adapter);
        Ok(())
    }

    pub fn execute(&self, provider: &ProviderId, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        let adapter = self
            .adapters
            .get(provider)
            .ok_or_else(|| PlatformError::AdapterNotFound(provider.as_str().to_owned()))?;
        let capabilities = adapter.capabilities();
        if capabilities.target != request.target {
            return Err(PlatformError::TargetMismatch {
                expected: capabilities.target,
                actual: request.target,
            });
        }
        if !capabilities.operations.iter().any(|operation| operation == &request.operation) {
            return Err(PlatformError::InvalidCommand(format!(
                "provider {} does not support operation {}",
                provider.as_str(), request.operation
            )));
        }
        if capabilities.health == ProviderHealth::Unavailable {
            return Err(PlatformError::TransportUnavailable(format!(
                "provider {} is unavailable",
                provider.as_str()
            )));
        }
        adapter.execute(request)
    }

    pub fn capabilities(&self) -> Vec<ProviderCapabilities> {
        self.adapters.values().map(|adapter| adapter.capabilities()).collect()
    }

    pub fn len(&self) -> usize { self.adapters.len() }
    pub fn is_empty(&self) -> bool { self.adapters.is_empty() }
}

/// Deterministic adapter used for local integration tests and provider-contract probes.
#[derive(Clone)]
pub struct DeterministicProviderAdapter {
    capabilities: ProviderCapabilities,
}

impl DeterministicProviderAdapter {
    pub fn new(
        provider_id: impl Into<String>,
        target: IntegrationTarget,
        operations: impl IntoIterator<Item = impl Into<String>>,
    ) -> PlatformResult<Self> {
        Ok(Self {
            capabilities: ProviderCapabilities {
                provider_id: ProviderId::new(provider_id)?,
                target,
                operations: operations.into_iter().map(Into::into).collect(),
                health: ProviderHealth::Ready,
            },
        })
    }
}

impl ExternalProviderAdapter for DeterministicProviderAdapter {
    fn capabilities(&self) -> ProviderCapabilities { self.capabilities.clone() }

    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        Ok(AdapterResponse {
            request_id: request.context.request_id,
            target: request.target,
            operation: request.operation.clone(),
            accepted: true,
            payload: Value::from(serde_json::json!({
                "provider": self.capabilities.provider_id.as_str(),
                "operation": request.operation,
                "request_id": request.context.request_id,
                "payload": request.payload,
            })),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AdapterRequest, IntegrationCommand, IntegrationContext};

    fn request(target: IntegrationTarget, operation: &str) -> AdapterRequest {
        AdapterRequest::from_command(
            IntegrationCommand::new(target, operation, IntegrationContext::new("provider-test")),
            serde_json::json!({"probe": true}),
        )
    }

    #[test]
    fn provider_id_rejects_empty_and_whitespace_values() {
        assert!(ProviderId::new("").is_err());
        assert!(ProviderId::new("bad provider").is_err());
        assert_eq!(ProviderId::new("openai").unwrap().as_str(), "openai");
    }

    #[test]
    fn registry_enforces_target_and_operation_contracts() {
        let mut registry = ProviderAdapterRegistry::default();
        let adapter = Arc::new(
            DeterministicProviderAdapter::new(
                "retrieval-local",
                IntegrationTarget::Retrieval,
                ["search", "health"],
            )
            .unwrap(),
        );
        registry.register(adapter).unwrap();
        let provider = ProviderId::new("retrieval-local").unwrap();

        let response = registry.execute(&provider, &request(IntegrationTarget::Retrieval, "search")).unwrap();
        assert!(response.accepted);
        assert_eq!(response.payload["provider"], "retrieval-local");

        assert!(matches!(
            registry.execute(&provider, &request(IntegrationTarget::Retrieval, "delete")),
            Err(PlatformError::InvalidCommand(_))
        ));
        assert!(matches!(
            registry.execute(&provider, &request(IntegrationTarget::Decision, "search")),
            Err(PlatformError::TargetMismatch { .. })
        ));
    }

    #[test]
    fn capabilities_are_exported_deterministically() {
        let mut registry = ProviderAdapterRegistry::default();
        registry
            .register(Arc::new(DeterministicProviderAdapter::new(
                "b-provider", IntegrationTarget::Llm, ["generate"],
            ).unwrap()))
            .unwrap();
        registry
            .register(Arc::new(DeterministicProviderAdapter::new(
                "a-provider", IntegrationTarget::Llm, ["generate"],
            ).unwrap()))
            .unwrap();
        let capabilities = registry.capabilities();
        assert_eq!(capabilities[0].provider_id.as_str(), "a-provider");
        assert_eq!(capabilities[1].provider_id.as_str(), "b-provider");
    }

    #[test]
    fn uuid_request_context_survives_provider_execution() {
        let request_id = Uuid::now_v7();
        let command = IntegrationCommand {
            command_id: Uuid::now_v7(),
            target: IntegrationTarget::Llm,
            operation: "generate".into(),
            context: IntegrationContext::new("provider-test"),
        };
        let mut request = AdapterRequest::from_command(command, serde_json::json!({"x": 1}));
        request.context.request_id = request_id;

        let adapter = DeterministicProviderAdapter::new("local", IntegrationTarget::Llm, ["generate"]).unwrap();
        let response = adapter.execute(&request).unwrap();
        assert_eq!(response.request_id, request_id);
        assert_eq!(response.payload["request_id"], request_id);
    }
}

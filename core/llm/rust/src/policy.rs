use std::sync::Arc;

use crate::{GenerationRequest, GenerationResponse, LlmError, LlmProvider, ModelId, ProviderId, SafetyClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRoute {
    pub provider: ProviderId,
    pub model: ModelId,
    pub allowed_safety: SafetyClass,
    pub enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutingPolicy {
    pub routes: Vec<ModelRoute>,
    pub default_provider: ProviderId,
    pub default_model: ModelId,
}

impl RoutingPolicy {
    pub fn resolve(&self, requested: Option<&ModelId>, safety: SafetyClass) -> Option<&ModelRoute> {
        let model = requested.unwrap_or(&self.default_model);
        self.routes.iter().find(|route| {
            route.enabled && &route.model == model && safety_allowed(route.allowed_safety, safety)
        })
    }

    pub fn default_route(&self, safety: SafetyClass) -> Option<&ModelRoute> {
        self.routes.iter().find(|route| {
            route.enabled
                && route.provider == self.default_provider
                && route.model == self.default_model
                && safety_allowed(route.allowed_safety, safety)
        })
    }
}

/// Provider-aware execution boundary for LLM generation.
///
/// Routing policy remains declarative: the executor never invents a provider,
/// silently bypasses a disabled route, or downgrades a safety class. A provider
/// must be explicitly registered and must match the provider selected by policy.
pub struct LlmRouter {
    policy: RoutingPolicy,
    providers: Vec<Arc<dyn LlmProvider>>,
}

impl LlmRouter {
    pub fn new(policy: RoutingPolicy) -> Self {
        Self {
            policy,
            providers: Vec::new(),
        }
    }

    pub fn policy(&self) -> &RoutingPolicy {
        &self.policy
    }

    pub fn register_provider<P>(&mut self, provider: P)
    where
        P: LlmProvider + 'static,
    {
        self.providers.push(Arc::new(provider));
    }

    pub fn register_shared_provider(&mut self, provider: Arc<dyn LlmProvider>) {
        self.providers.push(provider);
    }

    pub fn provider_count(&self) -> usize {
        self.providers.len()
    }

    pub fn provider_ids(&self) -> Vec<ProviderId> {
        self.providers.iter().map(|provider| provider.id()).collect()
    }

    pub async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let route = self
            .policy
            .resolve(Some(&request.model), request.safety)
            .ok_or(LlmError::NoRoute)?;

        let provider = self
            .providers
            .iter()
            .find(|provider| provider.id() == route.provider)
            .ok_or_else(|| {
                LlmError::ProviderFailure(format!(
                    "provider {} is required by the selected route but is not registered",
                    route.provider.0
                ))
            })?;

        provider.generate(request).await
    }

    pub async fn generate_default(
        &self,
        mut request: GenerationRequest,
    ) -> Result<GenerationResponse, LlmError> {
        let route = self
            .policy
            .default_route(request.safety)
            .ok_or(LlmError::NoRoute)?;
        request.model = route.model.clone();
        self.generate(request).await
    }
}

fn safety_allowed(route: SafetyClass, requested: SafetyClass) -> bool {
    let rank = |value| match value {
        SafetyClass::Unclassified => 0,
        SafetyClass::Standard => 1,
        SafetyClass::Sensitive => 2,
        SafetyClass::Restricted => 3,
    };
    rank(requested) <= rank(route)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeterministicProvider, Message};

    fn policy() -> RoutingPolicy {
        RoutingPolicy {
            routes: vec![ModelRoute {
                provider: ProviderId::new("local.deterministic"),
                model: ModelId::new("deterministic-v1"),
                allowed_safety: SafetyClass::Sensitive,
                enabled: true,
            }],
            default_provider: ProviderId::new("local.deterministic"),
            default_model: ModelId::new("deterministic-v1"),
        }
    }

    fn request(safety: SafetyClass) -> GenerationRequest {
        let mut request = GenerationRequest::new(
            ModelId::new("deterministic-v1"),
            vec![Message::user("route me")],
        );
        request.safety = safety;
        request
    }

    #[tokio::test]
    async fn router_executes_only_through_the_policy_selected_provider() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);

        let response = router.generate(request(SafetyClass::Standard)).await.unwrap();
        assert_eq!(response.provider, ProviderId::new("local.deterministic"));
        assert_eq!(response.model, ModelId::new("deterministic-v1"));
        assert_eq!(response.content, "CAT deterministic provider: route me");
    }

    #[tokio::test]
    async fn router_rejects_safety_class_above_route_allowance() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);

        let error = router.generate(request(SafetyClass::Restricted)).await.unwrap_err();
        assert!(matches!(error, LlmError::NoRoute));
    }

    #[tokio::test]
    async fn router_rejects_missing_provider_registration() {
        let router = LlmRouter::new(policy());
        let error = router.generate(request(SafetyClass::Standard)).await.unwrap_err();
        assert!(matches!(error, LlmError::ProviderFailure(_)));
    }

    #[tokio::test]
    async fn default_generation_selects_the_declared_default_route() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);

        let mut request = GenerationRequest::new(
            ModelId::new("ignored-by-default"),
            vec![Message::user("default route")],
        );
        request.safety = SafetyClass::Standard;

        let response = router.generate_default(request).await.unwrap();
        assert_eq!(response.model, ModelId::new("deterministic-v1"));
    }
}

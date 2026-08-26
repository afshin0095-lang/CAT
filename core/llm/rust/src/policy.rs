use std::sync::Arc;

use futures_util::StreamExt;

use crate::{
    GenerationRequest, GenerationResponse, LlmError, LlmGenerationStream, LlmProvider, ModelId,
    ProviderHealthConfig, ProviderHealthRegistry, ProviderId, SafetyClass,
};

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

pub struct LlmRouter {
    policy: RoutingPolicy,
    providers: Vec<Arc<dyn LlmProvider>>,
    health: ProviderHealthRegistry,
}

impl LlmRouter {
    pub fn new(policy: RoutingPolicy) -> Self { Self::with_health_config(policy, ProviderHealthConfig::default()) }

    pub fn with_health_config(policy: RoutingPolicy, config: ProviderHealthConfig) -> Self {
        Self { policy, providers: Vec::new(), health: ProviderHealthRegistry::new(config) }
    }

    pub fn policy(&self) -> &RoutingPolicy { &self.policy }
    pub fn health(&self) -> &ProviderHealthRegistry { &self.health }

    pub fn register_provider<P>(&mut self, provider: P)
    where P: LlmProvider + 'static { self.providers.push(Arc::new(provider)); }

    pub fn register_shared_provider(&mut self, provider: Arc<dyn LlmProvider>) { self.providers.push(provider); }
    pub fn provider_count(&self) -> usize { self.providers.len() }
    pub fn provider_ids(&self) -> Vec<ProviderId> { self.providers.iter().map(|provider| provider.id()).collect() }

    fn provider_for(&self, provider_id: &ProviderId) -> Result<&Arc<dyn LlmProvider>, LlmError> {
        self.providers.iter().find(|provider| provider.id() == *provider_id).ok_or_else(|| {
            LlmError::ProviderFailure(format!("provider {} is required by the selected route but is not registered", provider_id.0))
        })
    }

    pub async fn generate(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let route = self.policy.resolve(Some(&request.model), request.safety).ok_or(LlmError::NoRoute)?;
        self.provider_for(&route.provider)?.generate(request).await
    }

    pub async fn generate_resilient(&self, request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let routes: Vec<ModelRoute> = self.policy.routes.iter().filter(|route| {
            route.enabled && route.model == request.model && safety_allowed(route.allowed_safety, request.safety)
        }).cloned().collect();
        if routes.is_empty() { return Err(LlmError::NoRoute); }
        let mut last_error = None;
        for route in routes {
            if !self.health.is_available(&route.provider) { continue; }
            let provider = match self.provider_for(&route.provider) {
                Ok(provider) => provider,
                Err(error) => { self.health.record_failure(&route.provider); last_error = Some(error); continue; }
            };
            match provider.generate(request.clone()).await {
                Ok(response) => { self.health.record_success(&route.provider); return Ok(response); }
                Err(error) => { self.health.record_failure(&route.provider); last_error = Some(error); }
            }
        }
        Err(last_error.unwrap_or(LlmError::NoRoute))
    }

    pub async fn generate_stream(&self, request: GenerationRequest) -> Result<LlmGenerationStream, LlmError> {
        let route = self.policy.resolve(Some(&request.model), request.safety).ok_or(LlmError::NoRoute)?;
        self.provider_for(&route.provider)?.generate_stream(request).await
    }

    pub async fn generate_stream_resilient(&self, request: GenerationRequest) -> Result<LlmGenerationStream, LlmError> {
        let routes: Vec<ModelRoute> = self.policy.routes.iter().filter(|route| {
            route.enabled && route.model == request.model && safety_allowed(route.allowed_safety, request.safety)
        }).cloned().collect();
        if routes.is_empty() { return Err(LlmError::NoRoute); }
        let mut last_error = None;
        for route in routes {
            if !self.health.is_available(&route.provider) { continue; }
            let provider = match self.provider_for(&route.provider) {
                Ok(provider) => provider,
                Err(error) => { self.health.record_failure(&route.provider); last_error = Some(error); continue; }
            };
            match provider.generate_stream(request.clone()).await {
                Ok(stream) => {
                    let provider_id = route.provider.clone();
                    let health = self.health.clone();
                    let guarded = stream.map(move |item| match item {
                        Ok(chunk) => {
                            if chunk.finish_reason.is_some() { health.record_success(&provider_id); }
                            Ok(chunk)
                        }
                        Err(error) => {
                            health.record_failure(&provider_id);
                            Err(error)
                        }
                    });
                    return Ok(Box::pin(guarded));
                }
                Err(error) => { self.health.record_failure(&route.provider); last_error = Some(error); }
            }
        }
        Err(last_error.unwrap_or(LlmError::NoRoute))
    }

    pub async fn generate_default(&self, mut request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let route = self.policy.default_route(request.safety).ok_or(LlmError::NoRoute)?;
        request.model = route.model.clone();
        self.generate(request).await
    }

    pub async fn generate_default_resilient(&self, mut request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        let route = self.policy.default_route(request.safety).ok_or(LlmError::NoRoute)?;
        request.model = route.model.clone();
        self.generate_resilient(request).await
    }

    pub async fn generate_default_stream(&self, mut request: GenerationRequest) -> Result<LlmGenerationStream, LlmError> {
        let route = self.policy.default_route(request.safety).ok_or(LlmError::NoRoute)?;
        request.model = route.model.clone();
        self.generate_stream(request).await
    }

    pub async fn generate_default_stream_resilient(&self, mut request: GenerationRequest) -> Result<LlmGenerationStream, LlmError> {
        let route = self.policy.default_route(request.safety).ok_or(LlmError::NoRoute)?;
        request.model = route.model.clone();
        self.generate_stream_resilient(request).await
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
    use crate::{collect_stream, DeterministicProvider, Message};

    fn policy() -> RoutingPolicy {
        RoutingPolicy {
            routes: vec![ModelRoute { provider: ProviderId::new("local.deterministic"), model: ModelId::new("deterministic-v1"), allowed_safety: SafetyClass::Sensitive, enabled: true }],
            default_provider: ProviderId::new("local.deterministic"),
            default_model: ModelId::new("deterministic-v1"),
        }
    }

    fn request(safety: SafetyClass) -> GenerationRequest {
        let mut request = GenerationRequest::new(ModelId::new("deterministic-v1"), vec![Message::user("route me")]);
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
    }

    #[tokio::test]
    async fn router_rejects_safety_class_above_route_allowance() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);
        assert!(matches!(router.generate(request(SafetyClass::Restricted)).await.unwrap_err(), LlmError::NoRoute));
    }

    #[tokio::test]
    async fn router_rejects_missing_provider_registration() {
        let router = LlmRouter::new(policy());
        assert!(matches!(router.generate(request(SafetyClass::Standard)).await.unwrap_err(), LlmError::ProviderFailure(_)));
    }

    #[tokio::test]
    async fn default_generation_selects_declared_default_route() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);
        let mut request = GenerationRequest::new(ModelId::new("ignored-by-default"), vec![Message::user("default route")]);
        request.safety = SafetyClass::Standard;
        let response = router.generate_default(request).await.unwrap();
        assert_eq!(response.model, ModelId::new("deterministic-v1"));
    }

    #[tokio::test]
    async fn streaming_router_preserves_complete_deterministic_output() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);
        let request = request(SafetyClass::Standard);
        let expected = router.generate(request.clone()).await.unwrap().content;
        let stream = router.generate_stream(request).await.unwrap();
        let actual = collect_stream(stream).await.unwrap();
        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn resilient_streaming_marks_provider_healthy_only_at_terminal_chunk() {
        let mut router = LlmRouter::new(policy());
        router.register_provider(DeterministicProvider);
        let provider = ProviderId::new("local.deterministic");
        let stream = router.generate_stream_resilient(request(SafetyClass::Standard)).await.unwrap();
        assert_eq!(router.health().snapshot(&provider).state, crate::ProviderHealthState::Healthy);
        let _ = collect_stream(stream).await.unwrap();
        assert_eq!(router.health().snapshot(&provider).state, crate::ProviderHealthState::Healthy);
    }
}

use async_trait::async_trait;
use cat_llm::{
    DeterministicProvider, GenerationRequest, GenerationResponse, LlmError, LlmProvider, LlmRouter,
    Message, ModelId, ModelRoute, ProviderHealthConfig, ProviderHealthState, ProviderId,
    RoutingPolicy, SafetyClass,
};
use std::time::Duration;

#[derive(Clone, Debug)]
struct FailingProvider;

#[async_trait]
impl LlmProvider for FailingProvider {
    fn id(&self) -> ProviderId {
        ProviderId::new("mock.failing")
    }

    async fn generate(&self, _request: GenerationRequest) -> Result<GenerationResponse, LlmError> {
        Err(LlmError::ProviderFailure(
            "synthetic provider failure".to_owned(),
        ))
    }
}

fn policy() -> RoutingPolicy {
    RoutingPolicy {
        routes: vec![
            ModelRoute {
                provider: ProviderId::new("mock.failing"),
                model: ModelId::new("deterministic-v1"),
                allowed_safety: SafetyClass::Sensitive,
                enabled: true,
            },
            ModelRoute {
                provider: ProviderId::new("local.deterministic"),
                model: ModelId::new("deterministic-v1"),
                allowed_safety: SafetyClass::Sensitive,
                enabled: true,
            },
        ],
        default_provider: ProviderId::new("mock.failing"),
        default_model: ModelId::new("deterministic-v1"),
    }
}

fn request() -> GenerationRequest {
    GenerationRequest::new(
        ModelId::new("deterministic-v1"),
        vec![Message::user("resilient routing")],
    )
}

#[tokio::test]
async fn resilient_router_falls_back_only_to_declared_route() {
    let mut router = LlmRouter::with_health_config(
        policy(),
        ProviderHealthConfig {
            failure_threshold: 1,
            cooldown: Duration::from_secs(60),
        },
    );
    router.register_provider(FailingProvider);
    router.register_provider(DeterministicProvider);

    let response = router.generate_resilient(request()).await.unwrap();

    assert_eq!(response.provider, ProviderId::new("local.deterministic"));
    assert_eq!(response.model, ModelId::new("deterministic-v1"));
    assert_eq!(
        router
            .health()
            .snapshot(&ProviderId::new("mock.failing"))
            .state,
        ProviderHealthState::Open
    );
    assert_eq!(
        router
            .health()
            .snapshot(&ProviderId::new("mock.failing"))
            .consecutive_failures,
        1
    );
}

#[tokio::test]
async fn resilient_router_skips_open_provider_without_retrying_it() {
    let mut router = LlmRouter::with_health_config(
        policy(),
        ProviderHealthConfig {
            failure_threshold: 1,
            cooldown: Duration::from_secs(60),
        },
    );
    router.register_provider(FailingProvider);
    router.register_provider(DeterministicProvider);

    let first = router.generate_resilient(request()).await.unwrap();
    let second = router.generate_resilient(request()).await.unwrap();

    assert_eq!(first.provider, ProviderId::new("local.deterministic"));
    assert_eq!(second.provider, ProviderId::new("local.deterministic"));
    assert_eq!(
        router
            .health()
            .snapshot(&ProviderId::new("mock.failing"))
            .consecutive_failures,
        1
    );
}

#[test]
fn provider_health_defaults_are_conservative() {
    let config = ProviderHealthConfig::default();
    assert_eq!(config.failure_threshold, 3);
    assert_eq!(config.cooldown, Duration::from_secs(30));
}

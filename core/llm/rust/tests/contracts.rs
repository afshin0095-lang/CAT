use cat_llm::{DeterministicProvider, GenerationRequest, LlmProvider, ModelId, ModelRoute, ProviderId, RoutingPolicy, SafetyClass};

#[test]
fn routing_respects_safety_ceiling() {
    let policy = RoutingPolicy {
        routes: vec![ModelRoute {
            provider: ProviderId::new("local"),
            model: ModelId::new("test-model"),
            allowed_safety: SafetyClass::Standard,
            enabled: true,
        }],
        default_provider: ProviderId::new("local"),
        default_model: ModelId::new("test-model"),
    };

    assert!(policy.resolve(None, SafetyClass::Standard).is_some());
    assert!(policy.resolve(None, SafetyClass::Sensitive).is_none());
}

#[tokio::test]
async fn deterministic_provider_is_replayable() {
    let provider = DeterministicProvider;
    let request = GenerationRequest::new(ModelId::new("test-model"), vec![cat_llm::Message::user("hello cat")]);
    let response = provider.generate(request.clone()).await.unwrap();
    assert_eq!(response.model, request.model);
    assert_eq!(response.content, "CAT deterministic provider: hello cat");
    assert_eq!(response.usage.total_tokens, response.usage.input_tokens + response.usage.output_tokens);
}

#[test]
fn generation_request_has_stable_defaults() {
    let request = GenerationRequest::new(ModelId::new("test"), vec![]);
    assert_eq!(request.temperature, 0.0);
    assert_eq!(request.max_output_tokens, 1024);
    assert_eq!(request.safety, SafetyClass::Standard);
}

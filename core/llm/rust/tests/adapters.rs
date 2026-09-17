use cat_llm::{
    AnthropicProvider, GenerationRequest, LlmProvider, Message, ModelId, OpenAiCompatibleProvider,
    ProviderId, Role,
};

#[test]
fn provider_ids_are_stable() {
    let openai = OpenAiCompatibleProvider::new("https://example.invalid/v1", "test-key").unwrap();
    let anthropic = AnthropicProvider::new("https://example.invalid", "test-key").unwrap();
    assert_eq!(openai.id(), ProviderId::new("openai.compatible"));
    assert_eq!(anthropic.id(), ProviderId::new("anthropic"));
}

#[test]
fn provider_construction_rejects_empty_credentials() {
    assert!(OpenAiCompatibleProvider::new("https://example.invalid/v1", "").is_err());
    assert!(AnthropicProvider::new("", "test-key").is_err());
}

#[test]
fn generation_request_preserves_message_roles() {
    let request = GenerationRequest::new(
        ModelId::new("example-model"),
        vec![
            Message::system("policy"),
            Message::user("question"),
            Message::assistant("answer"),
        ],
    );
    assert_eq!(request.messages[0].role, Role::System);
    assert_eq!(request.messages[1].role, Role::User);
    assert_eq!(request.messages[2].role, Role::Assistant);
}

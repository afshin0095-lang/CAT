use cat_llm::{
    ProviderId, ProviderToolAuthorizationRegistry, ToolCall, ToolId, ToolVersion,
};

#[test]
fn provider_authorization_contract_requires_an_explicit_exact_grant() {
    let provider = ProviderId::new("local.deterministic");
    let call = ToolCall {
        call_id: "provider-boundary-1".to_owned(),
        tool_id: ToolId::new("cat.lookup"),
        tool_version: ToolVersion(1),
        input: serde_json::json!({"query":"CAT"}),
    };

    let registry = ProviderToolAuthorizationRegistry::default();
    assert!(registry.authorize(&provider, &call).is_err());

    let mut registry = registry;
    registry
        .grant(provider.clone(), ToolId::new("cat.lookup"), ToolVersion(1))
        .unwrap();
    assert!(registry.authorize(&provider, &call).is_ok());
}

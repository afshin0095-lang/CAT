use std::collections::BTreeSet;

use cat_llm::{
    PolicyId, PolicyVersion, PromptId, PromptToolPolicy, PromptVersion, SafetyClass, ToolCall,
    ToolId, ToolVersion,
};

fn policy() -> PromptToolPolicy {
    PromptToolPolicy {
        id: PolicyId::new("cat.contract.external"),
        version: PolicyVersion(1),
        enabled: true,
        max_safety: SafetyClass::Standard,
        allowed_prompts: BTreeSet::from([(PromptId::new("cat.external"), PromptVersion(1))]),
        allowed_tools: BTreeSet::from([(ToolId::new("cat.lookup"), ToolVersion(1))]),
        max_prompt_bytes: 256,
        max_tool_input_bytes: 256,
    }
}

#[test]
fn authorization_contract_fails_closed_for_unknown_prompt_and_tool_versions() {
    let policy = policy();

    assert!(
        policy
            .authorize_prompt(
                &PromptId::new("cat.external"),
                PromptVersion(2),
                SafetyClass::Standard,
                "hello",
            )
            .is_err()
    );

    let call = ToolCall {
        call_id: "contract-1".to_owned(),
        tool_id: ToolId::new("cat.lookup"),
        tool_version: ToolVersion(2),
        input: serde_json::json!({"query": "CAT"}),
    };
    assert!(policy.authorize_tool(&call).is_err());
}

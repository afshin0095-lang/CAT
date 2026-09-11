use std::collections::BTreeSet;

use cat_llm::{
    ExecutionAuthorizationRequest, ExecutionPolicyGate, LlmError, PolicyId, PolicyVersion,
    PromptToolPolicy, ProviderId, ProviderToolAuthorizationRegistry, SafetyClass, ToolCall,
    ToolExecutionStatus, ToolExecutor, ToolExecutorRegistry, ToolId, ToolVersion,
};

struct CountingExecutor;

#[async_trait::async_trait]
impl ToolExecutor for CountingExecutor {
    fn tool_id(&self) -> &ToolId {
        static ID: std::sync::OnceLock<ToolId> = std::sync::OnceLock::new();
        ID.get_or_init(|| ToolId::new("cat.lookup"))
    }

    fn tool_version(&self) -> ToolVersion {
        ToolVersion(1)
    }

    async fn execute(&self, call: &ToolCall) -> Result<cat_llm::ToolExecution, LlmError> {
        Ok(cat_llm::ToolExecution {
            call_id: call.call_id.clone(),
            tool_id: call.tool_id.clone(),
            tool_version: call.tool_version,
            status: ToolExecutionStatus::Succeeded,
            output: Some(serde_json::json!({"accepted": true})),
            error: None,
            provenance: Default::default(),
        })
    }
}

fn request() -> ExecutionAuthorizationRequest {
    ExecutionAuthorizationRequest {
        provider_id: ProviderId::new("provider.contract"),
        subject: "agent:contract".to_owned(),
        policy: PromptToolPolicy {
            id: PolicyId::new("cat.execution.contract"),
            version: PolicyVersion(1),
            enabled: true,
            max_safety: SafetyClass::Sensitive,
            allowed_prompts: BTreeSet::new(),
            allowed_tools: BTreeSet::from([(ToolId::new("cat.lookup"), ToolVersion(1))]),
            max_prompt_bytes: 1024,
            max_tool_input_bytes: 1024,
        },
        call: ToolCall {
            call_id: "execution-contract-1".to_owned(),
            tool_id: ToolId::new("cat.lookup"),
            tool_version: ToolVersion(1),
            input: serde_json::json!({"q": "CAT"}),
        },
    }
}

#[tokio::test]
async fn authorization_and_execution_preserve_phase_boundary() {
    let request = request();
    let mut grants = ProviderToolAuthorizationRegistry::default();
    grants
        .grant(
            request.provider_id.clone(),
            request.call.tool_id.clone(),
            request.call.tool_version,
        )
        .unwrap();

    let mut executors = ToolExecutorRegistry::default();
    executors.register(Box::new(CountingExecutor)).unwrap();

    let result = ExecutionPolicyGate::new(&grants)
        .authorize_and_execute(&request, &executors)
        .await
        .unwrap();

    assert!(result.authorization.decision.allowed);
    assert_eq!(result.execution.status, ToolExecutionStatus::Succeeded);
    assert_eq!(
        result.execution.output,
        Some(serde_json::json!({"accepted": true}))
    );
}

#[tokio::test]
async fn provider_denial_prevents_execution_lookup() {
    let request = request();
    let grants = ProviderToolAuthorizationRegistry::default();
    let executors = ToolExecutorRegistry::default();

    assert!(matches!(
        ExecutionPolicyGate::new(&grants)
            .authorize_and_execute(&request, &executors)
            .await,
        Err(LlmError::ProviderPolicyDenied(_))
    ));
}

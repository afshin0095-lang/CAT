use std::collections::BTreeMap;

use async_trait::async_trait;
use cat_llm::{
    ToolCall, ToolExecution, ToolExecutionStatus, ToolExecutor, ToolExecutorRegistry, ToolId,
    ToolVersion,
};

struct EchoExecutor {
    id: ToolId,
}

#[async_trait]
impl ToolExecutor for EchoExecutor {
    fn tool_id(&self) -> &ToolId { &self.id }
    fn tool_version(&self) -> ToolVersion { ToolVersion(1) }

    async fn execute(&self, call: &ToolCall) -> Result<ToolExecution, cat_llm::LlmError> {
        Ok(ToolExecution {
            call_id: call.call_id.clone(),
            tool_id: call.tool_id.clone(),
            tool_version: call.tool_version,
            status: ToolExecutionStatus::Succeeded,
            output: Some(call.input.clone()),
            error: None,
            provenance: BTreeMap::from([("executor".to_owned(), "echo.v1".to_owned())]),
        })
    }
}

#[tokio::test]
async fn registered_executor_returns_valid_derived_evidence() {
    let mut registry = ToolExecutorRegistry::default();
    registry
        .register(Box::new(EchoExecutor { id: ToolId::new("cat.echo") }))
        .unwrap();

    let call = ToolCall {
        call_id: "call-1".to_owned(),
        tool_id: ToolId::new("cat.echo"),
        tool_version: ToolVersion(1),
        input: serde_json::json!({"message":"derived"}),
    };

    let result = registry.execute(&call).await.unwrap();
    assert_eq!(result.status, ToolExecutionStatus::Succeeded);
    assert_eq!(result.output, Some(serde_json::json!({"message":"derived"})));
    assert_eq!(result.provenance.get("executor").map(String::as_str), Some("echo.v1"));
}

#[tokio::test]
async fn unknown_executor_is_rejected_without_fallback_execution() {
    let registry = ToolExecutorRegistry::default();
    let call = ToolCall {
        call_id: "call-2".to_owned(),
        tool_id: ToolId::new("cat.unknown"),
        tool_version: ToolVersion(1),
        input: serde_json::json!({}),
    };

    assert!(registry.execute(&call).await.is_err());
}

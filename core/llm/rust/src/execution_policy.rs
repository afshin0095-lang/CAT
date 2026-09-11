use serde::{Deserialize, Serialize};

use crate::{
    LlmError, PolicyDecision, PromptToolPolicy, ProviderId, ProviderToolAuthorizationRegistry,
    ToolCall, ToolExecution, ToolExecutionStatus, ToolExecutorRegistry,
};

/// Execution-time authorization input. Routing chooses the provider elsewhere;
/// this boundary evaluates only whether the already-selected execution may proceed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionAuthorizationRequest {
    pub provider_id: ProviderId,
    pub subject: String,
    pub policy: PromptToolPolicy,
    pub call: ToolCall,
}

/// Immutable authorization result suitable for audit/event publication.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExecutionAuthorizationResult {
    pub decision: PolicyDecision,
    pub provider_id: ProviderId,
}

/// Combined execution result. Authorization and tool execution remain distinct
/// phases so callers can persist an admission decision even when execution
/// later fails.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorizedToolExecution {
    pub authorization: ExecutionAuthorizationResult,
    pub execution: ToolExecution,
}

pub struct ExecutionPolicyGate<'a> {
    provider_registry: &'a ProviderToolAuthorizationRegistry,
}

impl<'a> ExecutionPolicyGate<'a> {
    pub fn new(provider_registry: &'a ProviderToolAuthorizationRegistry) -> Self {
        Self { provider_registry }
    }

    /// Fail-closed composition of domain policy and provider capability grants.
    /// Both boundaries must independently allow execution.
    pub fn authorize(
        &self,
        request: &ExecutionAuthorizationRequest,
    ) -> Result<ExecutionAuthorizationResult, LlmError> {
        if request.subject.trim().is_empty() {
            return Err(LlmError::InvalidPolicy(
                "execution subject must not be empty".to_owned(),
            ));
        }

        request.policy.authorize_tool(&request.call)?;
        self.provider_registry
            .authorize(&request.provider_id, &request.call)?;

        Ok(ExecutionAuthorizationResult {
            decision: PolicyDecision {
                policy_id: request.policy.id.clone(),
                policy_version: request.policy.version,
                subject: request.subject.clone(),
                allowed: true,
                reason: "policy and provider grant admitted execution".to_owned(),
            },
            provider_id: request.provider_id.clone(),
        })
    }

    /// Authorize and execute through an explicitly registered executor.
    ///
    /// The authorization gate deliberately runs before executor lookup or
    /// invocation. A denied request therefore produces no tool-side effect.
    /// Executor failures are returned as execution evidence only when the
    /// executor itself returns valid derived evidence; malformed evidence is
    /// rejected by the registry.
    pub async fn authorize_and_execute(
        &self,
        request: &ExecutionAuthorizationRequest,
        executors: &ToolExecutorRegistry,
    ) -> Result<AuthorizedToolExecution, LlmError> {
        let authorization = self.authorize(request)?;
        let execution = executors.execute(&request.call).await?;

        if execution.status == ToolExecutionStatus::Rejected {
            return Err(LlmError::InvalidTool(
                "authorized executor returned a rejected execution result".to_owned(),
            ));
        }

        Ok(AuthorizedToolExecution {
            authorization,
            execution,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use async_trait::async_trait;

    use super::*;
    use crate::{PolicyId, PolicyVersion, SafetyClass, ToolExecutor, ToolId, ToolVersion};

    fn request() -> ExecutionAuthorizationRequest {
        ExecutionAuthorizationRequest {
            provider_id: ProviderId::new("provider.alpha"),
            subject: "agent:planner".to_owned(),
            policy: PromptToolPolicy {
                id: PolicyId::new("cat.execution.default"),
                version: PolicyVersion(1),
                enabled: true,
                max_safety: SafetyClass::Sensitive,
                allowed_prompts: BTreeSet::new(),
                allowed_tools: BTreeSet::from([(ToolId::new("cat.search"), ToolVersion(1))]),
                max_prompt_bytes: 1024,
                max_tool_input_bytes: 1024,
            },
            call: ToolCall {
                call_id: "exec-1".to_owned(),
                tool_id: ToolId::new("cat.search"),
                tool_version: ToolVersion(1),
                input: serde_json::json!({"query": "CAT"}),
            },
        }
    }

    struct EchoExecutor;

    #[async_trait]
    impl ToolExecutor for EchoExecutor {
        fn tool_id(&self) -> &ToolId {
            static ID: std::sync::OnceLock<ToolId> = std::sync::OnceLock::new();
            ID.get_or_init(|| ToolId::new("cat.search"))
        }

        fn tool_version(&self) -> ToolVersion {
            ToolVersion(1)
        }

        async fn execute(&self, call: &ToolCall) -> Result<ToolExecution, LlmError> {
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

    #[test]
    fn execution_requires_policy_and_provider_grant() {
        let request = request();
        let mut providers = ProviderToolAuthorizationRegistry::default();
        providers
            .grant(
                request.provider_id.clone(),
                request.call.tool_id.clone(),
                request.call.tool_version,
            )
            .unwrap();

        let gate = ExecutionPolicyGate::new(&providers);
        assert!(gate.authorize(&request).unwrap().decision.allowed);
    }

    #[test]
    fn execution_fails_closed_when_provider_grant_is_missing() {
        let request = request();
        let providers = ProviderToolAuthorizationRegistry::default();
        let gate = ExecutionPolicyGate::new(&providers);

        assert!(matches!(
            gate.authorize(&request),
            Err(LlmError::ProviderPolicyDenied(_))
        ));
    }

    #[tokio::test]
    async fn authorized_execution_runs_only_after_both_boundaries_admit() {
        let request = request();
        let mut providers = ProviderToolAuthorizationRegistry::default();
        providers
            .grant(
                request.provider_id.clone(),
                request.call.tool_id.clone(),
                request.call.tool_version,
            )
            .unwrap();

        let mut executors = ToolExecutorRegistry::default();
        executors.register(Box::new(EchoExecutor)).unwrap();

        let result = ExecutionPolicyGate::new(&providers)
            .authorize_and_execute(&request, &executors)
            .await
            .unwrap();

        assert!(result.authorization.decision.allowed);
        assert_eq!(result.execution.status, ToolExecutionStatus::Succeeded);
    }

    #[tokio::test]
    async fn denied_request_cannot_reach_executor() {
        let request = request();
        let providers = ProviderToolAuthorizationRegistry::default();
        let mut executors = ToolExecutorRegistry::default();
        executors.register(Box::new(EchoExecutor)).unwrap();

        assert!(matches!(
            ExecutionPolicyGate::new(&providers)
                .authorize_and_execute(&request, &executors)
                .await,
            Err(LlmError::ProviderPolicyDenied(_))
        ));
    }
}

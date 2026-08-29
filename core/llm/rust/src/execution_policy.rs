use serde::{Deserialize, Serialize};

use crate::{
    LlmError, PolicyDecision, PromptToolPolicy, ProviderId,
    ProviderToolAuthorizationRegistry, ToolCall,
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

        let policy_result = request.policy.authorize_tool(&request.call);
        let provider_result = self
            .provider_registry
            .authorize(&request.provider_id, &request.call);

        let (allowed, reason) = match (policy_result, provider_result) {
            (Ok(()), Ok(())) => (true, "policy and provider grant admitted execution".to_owned()),
            (Err(policy_error), Ok(())) => {
                return Err(policy_error);
            }
            (Ok(()), Err(provider_error)) => {
                return Err(provider_error);
            }
            (Err(policy_error), Err(_provider_error)) => {
                return Err(policy_error);
            }
        };

        Ok(ExecutionAuthorizationResult {
            decision: PolicyDecision {
                policy_id: request.policy.id.clone(),
                policy_version: request.policy.version,
                subject: request.subject.clone(),
                allowed,
                reason,
            },
            provider_id: request.provider_id.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::{PolicyId, PolicyVersion, SafetyClass, ToolId, ToolVersion};

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
                allowed_tools: BTreeSet::from([(
                    ToolId::new("cat.search"),
                    ToolVersion(1),
                )]),
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
}

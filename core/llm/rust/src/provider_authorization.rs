use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{LlmError, ProviderId, ToolCall, ToolId, ToolVersion};

/// Explicit provider/tool boundary policy. This is intentionally independent
/// from routing: a route selects a provider, while this contract decides
/// whether that provider may execute the requested capability.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderToolPolicy {
    pub provider_id: ProviderId,
    pub enabled: bool,
    pub allowed_tools: BTreeSet<(ToolId, ToolVersion)>,
}

impl ProviderToolPolicy {
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.provider_id.0.trim().is_empty() {
            return Err(LlmError::InvalidProviderPolicy(
                "provider id must not be empty".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn authorize(&self, call: &ToolCall) -> Result<(), LlmError> {
        self.validate()?;
        call.validate()?;
        if !self.enabled {
            return Err(LlmError::ProviderPolicyDenied(format!(
                "provider {} is disabled for tool execution",
                self.provider_id.0
            )));
        }
        if !self
            .allowed_tools
            .contains(&(call.tool_id.clone(), call.tool_version))
        {
            return Err(LlmError::ProviderPolicyDenied(format!(
                "provider {} is not authorized for tool {} version {}",
                self.provider_id.0, call.tool_id.0, call.tool_version.0
            )));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProviderToolAuthorizationRegistry {
    policies: BTreeSet<(ProviderId, ToolId, ToolVersion)>,
}

impl ProviderToolAuthorizationRegistry {
    pub fn grant(
        &mut self,
        provider: ProviderId,
        tool_id: ToolId,
        tool_version: ToolVersion,
    ) -> Result<(), LlmError> {
        if provider.0.trim().is_empty() || tool_id.0.trim().is_empty() {
            return Err(LlmError::InvalidProviderPolicy(
                "provider and tool identities must not be empty".to_owned(),
            ));
        }
        self.policies.insert((provider, tool_id, tool_version));
        Ok(())
    }

    pub fn authorize(&self, provider: &ProviderId, call: &ToolCall) -> Result<(), LlmError> {
        call.validate()?;
        let key = (provider.clone(), call.tool_id.clone(), call.tool_version);
        if !self.policies.contains(&key) {
            return Err(LlmError::ProviderPolicyDenied(format!(
                "provider {} has no explicit grant for tool {} version {}",
                provider.0, call.tool_id.0, call.tool_version.0
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_boundary_fails_closed_without_exact_version_grant() {
        let provider = ProviderId::new("local.deterministic");
        let call = ToolCall {
            call_id: "provider-contract-1".to_owned(),
            tool_id: ToolId::new("cat.lookup"),
            tool_version: ToolVersion(2),
            input: serde_json::json!({"query":"CAT"}),
        };

        let mut registry = ProviderToolAuthorizationRegistry::default();
        registry
            .grant(provider.clone(), ToolId::new("cat.lookup"), ToolVersion(1))
            .unwrap();

        assert!(matches!(
            registry.authorize(&provider, &call),
            Err(LlmError::ProviderPolicyDenied(_))
        ));
    }
}

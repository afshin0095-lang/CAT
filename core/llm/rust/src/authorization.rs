use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{LlmError, PromptId, PromptVersion, SafetyClass, ToolCall, ToolId, ToolVersion};

/// Stable policy identity for an authorization decision.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct PolicyId(pub String);

impl PolicyId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct PolicyVersion(pub u32);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolGrant {
    pub tool_id: ToolId,
    pub tool_version: ToolVersion,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptToolPolicy {
    pub id: PolicyId,
    pub version: PolicyVersion,
    pub enabled: bool,
    pub max_safety: SafetyClass,
    pub allowed_prompts: BTreeSet<(PromptId, PromptVersion)>,
    pub allowed_tools: BTreeSet<(ToolId, ToolVersion)>,
    pub max_prompt_bytes: usize,
    pub max_tool_input_bytes: usize,
}

impl PromptToolPolicy {
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.id.0.trim().is_empty() {
            return Err(LlmError::InvalidPolicy(
                "policy id must not be empty".to_owned(),
            ));
        }
        if self.max_prompt_bytes == 0 || self.max_tool_input_bytes == 0 {
            return Err(LlmError::InvalidPolicy(
                "policy size limits must be greater than zero".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn authorize_prompt(
        &self,
        prompt_id: &PromptId,
        prompt_version: PromptVersion,
        safety: SafetyClass,
        rendered_content: &str,
    ) -> Result<(), LlmError> {
        self.validate()?;
        if !self.enabled {
            return Err(LlmError::PolicyDenied(
                "prompt/tool policy is disabled".to_owned(),
            ));
        }
        if !safety_allowed(self.max_safety, safety) {
            return Err(LlmError::PolicyDenied(
                "prompt safety class exceeds policy allowance".to_owned(),
            ));
        }
        if !self
            .allowed_prompts
            .contains(&(prompt_id.clone(), prompt_version))
        {
            return Err(LlmError::PolicyDenied(format!(
                "prompt {} version {} is not granted by policy",
                prompt_id.0, prompt_version.0
            )));
        }
        if rendered_content.len() > self.max_prompt_bytes {
            return Err(LlmError::PolicyDenied(
                "rendered prompt exceeds policy size limit".to_owned(),
            ));
        }
        Ok(())
    }

    pub fn authorize_tool(&self, call: &ToolCall) -> Result<(), LlmError> {
        self.validate()?;
        if !self.enabled {
            return Err(LlmError::PolicyDenied(
                "prompt/tool policy is disabled".to_owned(),
            ));
        }
        call.validate()?;
        if !self
            .allowed_tools
            .contains(&(call.tool_id.clone(), call.tool_version))
        {
            return Err(LlmError::PolicyDenied(format!(
                "tool {} version {} is not granted by policy",
                call.tool_id.0, call.tool_version.0
            )));
        }
        let encoded = serde_json::to_vec(&call.input).map_err(|error| {
            LlmError::InvalidTool(format!("tool input cannot be encoded: {error}"))
        })?;
        if encoded.len() > self.max_tool_input_bytes {
            return Err(LlmError::PolicyDenied(
                "tool input exceeds policy size limit".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PolicyDecision {
    pub policy_id: PolicyId,
    pub policy_version: PolicyVersion,
    pub subject: String,
    pub allowed: bool,
    pub reason: String,
}

pub fn safety_allowed(maximum: SafetyClass, requested: SafetyClass) -> bool {
    let rank = |value| match value {
        SafetyClass::Unclassified => 0,
        SafetyClass::Standard => 1,
        SafetyClass::Sensitive => 2,
        SafetyClass::Restricted => 3,
    };
    rank(requested) <= rank(maximum)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> PromptToolPolicy {
        PromptToolPolicy {
            id: PolicyId::new("cat.llm.default"),
            version: PolicyVersion(1),
            enabled: true,
            max_safety: SafetyClass::Sensitive,
            allowed_prompts: BTreeSet::from([(PromptId::new("cat.summary"), PromptVersion(1))]),
            allowed_tools: BTreeSet::from([(ToolId::new("cat.search"), ToolVersion(1))]),
            max_prompt_bytes: 128,
            max_tool_input_bytes: 64,
        }
    }

    #[test]
    fn prompt_policy_is_version_specific_and_safety_bounded() {
        let policy = policy();
        assert!(
            policy
                .authorize_prompt(
                    &PromptId::new("cat.summary"),
                    PromptVersion(1),
                    SafetyClass::Standard,
                    "safe prompt",
                )
                .is_ok()
        );
        assert!(matches!(
            policy.authorize_prompt(
                &PromptId::new("cat.summary"),
                PromptVersion(2),
                SafetyClass::Standard,
                "safe prompt",
            ),
            Err(LlmError::PolicyDenied(_))
        ));
        assert!(matches!(
            policy.authorize_prompt(
                &PromptId::new("cat.summary"),
                PromptVersion(1),
                SafetyClass::Restricted,
                "safe prompt",
            ),
            Err(LlmError::PolicyDenied(_))
        ));
    }

    #[test]
    fn tool_policy_rejects_unganted_or_oversized_inputs() {
        let policy = policy();
        let denied = ToolCall {
            call_id: "call-1".to_owned(),
            tool_id: ToolId::new("cat.write"),
            tool_version: ToolVersion(1),
            input: serde_json::json!({"x": 1}),
        };
        assert!(matches!(
            policy.authorize_tool(&denied),
            Err(LlmError::PolicyDenied(_))
        ));

        let oversized = ToolCall {
            call_id: "call-2".to_owned(),
            tool_id: ToolId::new("cat.search"),
            tool_version: ToolVersion(1),
            input: serde_json::json!({"query": "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz"}),
        };
        assert!(matches!(
            policy.authorize_tool(&oversized),
            Err(LlmError::PolicyDenied(_))
        ));
    }
}

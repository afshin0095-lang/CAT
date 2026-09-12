use std::collections::{BTreeMap, BTreeSet};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::LlmError;

/// Stable identity for a tool contract. Tool definitions are immutable once
/// registered for a given (id, version) pair.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ToolId(pub String);

impl ToolId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ToolVersion(pub u32);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: ToolId,
    pub version: ToolVersion,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub capabilities: BTreeSet<String>,
}

impl ToolDefinition {
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.id.0.trim().is_empty() {
            return Err(LlmError::InvalidTool(
                "tool id must not be empty".to_owned(),
            ));
        }
        if self.description.trim().is_empty() {
            return Err(LlmError::InvalidTool(
                "tool description must not be empty".to_owned(),
            ));
        }
        if !self.input_schema.is_object() {
            return Err(LlmError::InvalidTool(
                "tool input schema must be a JSON object".to_owned(),
            ));
        }
        if self
            .capabilities
            .iter()
            .any(|value| value.trim().is_empty())
        {
            return Err(LlmError::InvalidTool(
                "tool capability must not be empty".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct ToolRegistry {
    definitions: BTreeMap<(ToolId, ToolVersion), ToolDefinition>,
}

impl ToolRegistry {
    pub fn register(&mut self, definition: ToolDefinition) -> Result<(), LlmError> {
        definition.validate()?;
        let key = (definition.id.clone(), definition.version);
        if self.definitions.contains_key(&key) {
            return Err(LlmError::InvalidTool(format!(
                "tool {} version {} is already registered",
                key.0.0, key.1.0
            )));
        }
        self.definitions.insert(key, definition);
        Ok(())
    }

    pub fn get(&self, id: &ToolId, version: ToolVersion) -> Option<&ToolDefinition> {
        self.definitions.get(&(id.clone(), version))
    }

    pub fn len(&self) -> usize {
        self.definitions.len()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolCall {
    pub call_id: String,
    pub tool_id: ToolId,
    pub tool_version: ToolVersion,
    pub input: serde_json::Value,
}

impl ToolCall {
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.call_id.trim().is_empty() {
            return Err(LlmError::InvalidTool(
                "tool call id must not be empty".to_owned(),
            ));
        }
        if !self.input.is_object() {
            return Err(LlmError::InvalidTool(
                "tool call input must be a JSON object".to_owned(),
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ToolExecutionStatus {
    Succeeded,
    Rejected,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolExecution {
    pub call_id: String,
    pub tool_id: ToolId,
    pub tool_version: ToolVersion,
    pub status: ToolExecutionStatus,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub provenance: BTreeMap<String, String>,
}

impl ToolExecution {
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.call_id.trim().is_empty() {
            return Err(LlmError::InvalidTool(
                "execution call id must not be empty".to_owned(),
            ));
        }
        match self.status {
            ToolExecutionStatus::Succeeded if self.error.is_some() => Err(LlmError::InvalidTool(
                "successful execution must not carry an error".to_owned(),
            )),
            ToolExecutionStatus::Succeeded if self.output.is_none() => Err(LlmError::InvalidTool(
                "successful execution must carry output".to_owned(),
            )),
            ToolExecutionStatus::Rejected | ToolExecutionStatus::Failed
                if self.error.as_deref().unwrap_or("").trim().is_empty() =>
            {
                Err(LlmError::InvalidTool(
                    "non-success execution must carry an error".to_owned(),
                ))
            }
            _ => Ok(()),
        }
    }
}

/// Execution is provider-neutral and side-effect ownership remains outside the
/// LLM core. Executors receive an already validated call and return derived
/// execution evidence; canonical business state mutation requires the owning
/// domain runtime and its own authorization path.
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    fn tool_id(&self) -> &ToolId;
    fn tool_version(&self) -> ToolVersion;

    async fn execute(&self, call: &ToolCall) -> Result<ToolExecution, LlmError>;
}

#[derive(Default)]
pub struct ToolExecutorRegistry {
    executors: BTreeMap<(ToolId, ToolVersion), Box<dyn ToolExecutor>>,
}

impl ToolExecutorRegistry {
    pub fn register(&mut self, executor: Box<dyn ToolExecutor>) -> Result<(), LlmError> {
        let key = (executor.tool_id().clone(), executor.tool_version());
        if self.executors.contains_key(&key) {
            return Err(LlmError::InvalidTool(format!(
                "executor {} version {} is already registered",
                key.0.0, key.1.0
            )));
        }
        self.executors.insert(key, executor);
        Ok(())
    }

    pub async fn execute(&self, call: &ToolCall) -> Result<ToolExecution, LlmError> {
        call.validate()?;
        let key = (call.tool_id.clone(), call.tool_version);
        let executor = self.executors.get(&key).ok_or_else(|| {
            LlmError::InvalidTool(format!(
                "no executor registered for {} version {}",
                key.0.0, key.1.0
            ))
        })?;

        let execution = executor.execute(call).await?;
        if execution.call_id != call.call_id
            || execution.tool_id != call.tool_id
            || execution.tool_version != call.tool_version
        {
            return Err(LlmError::InvalidTool(
                "executor returned execution evidence for a different tool call".to_owned(),
            ));
        }
        execution.validate()?;
        Ok(execution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_append_only_per_tool_version() {
        let definition = ToolDefinition {
            id: ToolId::new("cat.search"),
            version: ToolVersion(1),
            description: "Search derived knowledge".to_owned(),
            input_schema: serde_json::json!({"type":"object"}),
            capabilities: BTreeSet::from(["knowledge.read".to_owned()]),
        };
        let mut registry = ToolRegistry::default();
        registry.register(definition.clone()).unwrap();
        assert!(registry.register(definition).is_err());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn execution_evidence_requires_consistent_status_fields() {
        let execution = ToolExecution {
            call_id: "call-1".to_owned(),
            tool_id: ToolId::new("cat.search"),
            tool_version: ToolVersion(1),
            status: ToolExecutionStatus::Succeeded,
            output: None,
            error: None,
            provenance: BTreeMap::new(),
        };
        assert!(execution.validate().is_err());
    }
}

use crate::{
    IntegrationCommand, IntegrationContext, IntegrationTarget, PlatformError, PlatformResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterRequest {
    pub command_id: Uuid,
    pub target: IntegrationTarget,
    pub operation: String,
    pub context: IntegrationContext,
    pub payload: Value,
}

impl AdapterRequest {
    pub fn from_command(command: IntegrationCommand, payload: Value) -> Self {
        Self {
            command_id: command.command_id,
            target: command.target,
            operation: command.operation,
            context: command.context,
            payload,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdapterResponse {
    pub request_id: Uuid,
    pub target: IntegrationTarget,
    pub operation: String,
    pub accepted: bool,
    pub payload: Value,
}

pub trait PlatformAdapter: Send + Sync {
    fn target(&self) -> IntegrationTarget;
    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse>;
}

#[derive(Default)]
pub struct AdapterRegistry {
    adapters: BTreeMap<String, Box<dyn PlatformAdapter>>,
}

impl AdapterRegistry {
    pub fn register(
        &mut self,
        name: impl Into<String>,
        adapter: Box<dyn PlatformAdapter>,
    ) -> PlatformResult<()> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(PlatformError::InvalidCommand(
                "adapter name cannot be empty".into(),
            ));
        }
        if self.adapters.contains_key(&name) {
            return Err(PlatformError::InvalidCommand(format!(
                "adapter already registered: {name}"
            )));
        }
        self.adapters.insert(name, adapter);
        Ok(())
    }

    pub fn execute(&self, name: &str, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        let adapter = self
            .adapters
            .get(name)
            .ok_or_else(|| PlatformError::AdapterNotFound(name.to_owned()))?;
        if adapter.target() != request.target {
            return Err(PlatformError::TargetMismatch {
                expected: adapter.target(),
                actual: request.target,
            });
        }
        adapter.execute(request)
    }

    pub fn len(&self) -> usize {
        self.adapters.len()
    }
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }
}

#[derive(Default)]
pub struct PassthroughAdapter {
    target: Option<IntegrationTarget>,
}

impl PassthroughAdapter {
    pub fn new(target: IntegrationTarget) -> Self {
        Self {
            target: Some(target),
        }
    }
}

impl PlatformAdapter for PassthroughAdapter {
    fn target(&self) -> IntegrationTarget {
        self.target
            .expect("PassthroughAdapter target is always initialized")
    }

    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        Ok(AdapterResponse {
            request_id: request.context.request_id,
            target: request.target,
            operation: request.operation.clone(),
            accepted: true,
            payload: request.payload.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_routes_only_to_matching_target() {
        let mut registry = AdapterRegistry::default();
        registry
            .register(
                "planning",
                Box::new(PassthroughAdapter::new(IntegrationTarget::Planning)),
            )
            .unwrap();
        let command = IntegrationCommand::new(
            IntegrationTarget::Planning,
            "validate",
            IntegrationContext::new("test"),
        );
        let request = AdapterRequest::from_command(command, serde_json::json!({"plan":"p1"}));
        let response = registry.execute("planning", &request).unwrap();
        assert!(response.accepted);
        assert_eq!(response.target, IntegrationTarget::Planning);
    }

    #[test]
    fn registry_rejects_target_mismatch() {
        let mut registry = AdapterRegistry::default();
        registry
            .register(
                "planning",
                Box::new(PassthroughAdapter::new(IntegrationTarget::Planning)),
            )
            .unwrap();
        let command = IntegrationCommand::new(
            IntegrationTarget::Decision,
            "decide",
            IntegrationContext::new("test"),
        );
        let request = AdapterRequest::from_command(command, Value::Null);
        assert!(matches!(
            registry.execute("planning", &request),
            Err(PlatformError::TargetMismatch { .. })
        ));
    }
}

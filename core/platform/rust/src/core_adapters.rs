use crate::{AdapterRequest, AdapterResponse, AdapterRegistry, IntegrationTarget, PlatformAdapter, PlatformError, PlatformResult};
use serde_json::json;

/// A concrete adapter at the platform boundary.
///
/// The adapter validates the operation contract and emits a normalized response. It does not
/// reimplement domain logic; the target core remains the owner of execution semantics.
pub struct CoreAdapter {
    target: IntegrationTarget,
    name: &'static str,
    operations: &'static [&'static str],
}

impl CoreAdapter {
    pub const fn new(
        target: IntegrationTarget,
        name: &'static str,
        operations: &'static [&'static str],
    ) -> Self {
        Self { target, name, operations }
    }

    fn supports(&self, operation: &str) -> bool {
        self.operations.iter().any(|candidate| *candidate == operation)
    }
}

impl PlatformAdapter for CoreAdapter {
    fn target(&self) -> IntegrationTarget {
        self.target
    }

    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        if request.context.actor.trim().is_empty() {
            return Err(PlatformError::InvalidCommand("adapter actor cannot be empty".into()));
        }
        if request.operation.trim().is_empty() {
            return Err(PlatformError::InvalidCommand("adapter operation cannot be empty".into()));
        }
        if !self.supports(&request.operation) {
            return Err(PlatformError::InvalidCommand(format!(
                "operation '{}' is not supported by {} adapter",
                request.operation, self.name
            )));
        }

        Ok(AdapterResponse {
            request_id: request.context.request_id,
            target: request.target,
            operation: request.operation.clone(),
            accepted: true,
            payload: json!({
                "adapter": self.name,
                "mode": "boundary_dispatch",
                "domain_owner": self.name,
                "command_id": request.command_id,
                "correlation_id": request.context.correlation_id,
                "causation_id": request.context.causation_id,
                "workflow_id": request.context.workflow_id,
                "input": request.payload,
            }),
        })
    }
}

pub const EVENT_BUS_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::EventBus,
    "eventbus",
    &["publish", "subscribe", "replay"],
);

pub const KNOWLEDGE_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Knowledge,
    "knowledge",
    &["upsert_node", "upsert_edge", "traverse", "validate"],
);

pub const MEMORY_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Memory,
    "memory",
    &["store", "recall", "validate", "forget"],
);

pub const LLM_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Llm,
    "llm",
    &["generate", "health"],
);

pub const REASONING_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Reasoning,
    "reasoning",
    &["reason", "explain"],
);

pub const DECISION_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Decision,
    "decision",
    &["decide", "evaluate_policy"],
);

pub const PLANNING_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Planning,
    "planning",
    &["build_plan", "validate_plan"],
);

pub const ORCHESTRATOR_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Orchestrator,
    "orchestrator",
    &["schedule", "status"],
);

pub const RETRIEVAL_ADAPTER: CoreAdapter = CoreAdapter::new(
    IntegrationTarget::Retrieval,
    "retrieval",
    &["retrieve", "rank"],
);

/// Build the complete provider-neutral registry used by the platform composition root.
pub fn default_core_adapter_registry() -> PlatformResult<AdapterRegistry> {
    let mut registry = AdapterRegistry::default();
    registry.register("eventbus", Box::new(EVENT_BUS_ADAPTER))?;
    registry.register("knowledge", Box::new(KNOWLEDGE_ADAPTER))?;
    registry.register("memory", Box::new(MEMORY_ADAPTER))?;
    registry.register("llm", Box::new(LLM_ADAPTER))?;
    registry.register("reasoning", Box::new(REASONING_ADAPTER))?;
    registry.register("decision", Box::new(DECISION_ADAPTER))?;
    registry.register("planning", Box::new(PLANNING_ADAPTER))?;
    registry.register("orchestrator", Box::new(ORCHESTRATOR_ADAPTER))?;
    registry.register("retrieval", Box::new(RETRIEVAL_ADAPTER))?;
    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AdapterRequest, IntegrationCommand, IntegrationContext};
    use serde_json::json;

    #[test]
    fn default_registry_wires_every_integration_target() {
        let registry = default_core_adapter_registry().unwrap();
        assert_eq!(registry.len(), 9);
    }

    #[test]
    fn adapters_preserve_context_without_owning_domain_truth() {
        let registry = default_core_adapter_registry().unwrap();
        let command = IntegrationCommand::new(
            IntegrationTarget::Decision,
            "decide",
            IntegrationContext::new("platform-test"),
        );
        let request = AdapterRequest::from_command(command, json!({"objective":"test"}));
        let response = registry.execute("decision", &request).unwrap();
        assert!(response.accepted);
        assert_eq!(response.payload["mode"], "boundary_dispatch");
        assert_eq!(response.payload["domain_owner"], "decision");
    }

    #[test]
    fn unsupported_operation_is_rejected_at_boundary() {
        let registry = default_core_adapter_registry().unwrap();
        let command = IntegrationCommand::new(
            IntegrationTarget::Memory,
            "publish_money",
            IntegrationContext::new("platform-test"),
        );
        let request = AdapterRequest::from_command(command, json!({}));
        assert!(matches!(
            registry.execute("memory", &request),
            Err(PlatformError::InvalidCommand(_))
        ));
    }

    #[test]
    fn target_mismatch_remains_structural() {
        let registry = default_core_adapter_registry().unwrap();
        let command = IntegrationCommand::new(
            IntegrationTarget::Planning,
            "validate_plan",
            IntegrationContext::new("platform-test"),
        );
        let request = AdapterRequest::from_command(command, json!({}));
        assert!(matches!(
            registry.execute("decision", &request),
            Err(PlatformError::TargetMismatch { .. })
        ));
    }
}

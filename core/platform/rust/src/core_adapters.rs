use crate::{
    AdapterRegistry, AdapterRequest, AdapterResponse, IntegrationContext, IntegrationTarget,
    PlatformAdapter, PlatformError, PlatformResult,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

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
        Self {
            target,
            name,
            operations,
        }
    }

    fn supports(&self, operation: &str) -> bool {
        self.operations
            .iter()
            .any(|candidate| *candidate == operation)
    }
}

impl PlatformAdapter for CoreAdapter {
    fn target(&self) -> IntegrationTarget {
        self.target
    }

    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        if request.context.actor.trim().is_empty() {
            return Err(PlatformError::InvalidCommand(
                "adapter actor cannot be empty".into(),
            ));
        }
        if request.operation.trim().is_empty() {
            return Err(PlatformError::InvalidCommand(
                "adapter operation cannot be empty".into(),
            ));
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

/// Typed platform invocation contract.
///
/// These variants intentionally carry the shared integration context and a JSON payload rather
/// than recreating domain objects inside the platform crate. The platform owns routing and
/// cross-core context; each target core remains the owner of its domain request schema.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TypedCoreCommand {
    EventBus(CoreCommand),
    Knowledge(CoreCommand),
    Memory(CoreCommand),
    Llm(CoreCommand),
    Reasoning(CoreCommand),
    Decision(CoreCommand),
    Planning(CoreCommand),
    Orchestrator(CoreCommand),
    Retrieval(CoreCommand),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CoreCommand {
    pub command_id: Uuid,
    pub operation: String,
    pub context: IntegrationContext,
    pub payload: Value,
}

impl CoreCommand {
    pub fn new(
        command_id: Uuid,
        operation: impl Into<String>,
        context: IntegrationContext,
        payload: Value,
    ) -> Self {
        Self {
            command_id,
            operation: operation.into(),
            context,
            payload,
        }
    }
}

impl TypedCoreCommand {
    pub fn target(&self) -> IntegrationTarget {
        match self {
            Self::EventBus(_) => IntegrationTarget::EventBus,
            Self::Knowledge(_) => IntegrationTarget::Knowledge,
            Self::Memory(_) => IntegrationTarget::Memory,
            Self::Llm(_) => IntegrationTarget::Llm,
            Self::Reasoning(_) => IntegrationTarget::Reasoning,
            Self::Decision(_) => IntegrationTarget::Decision,
            Self::Planning(_) => IntegrationTarget::Planning,
            Self::Orchestrator(_) => IntegrationTarget::Orchestrator,
            Self::Retrieval(_) => IntegrationTarget::Retrieval,
        }
    }

    fn command(&self) -> &CoreCommand {
        match self {
            Self::EventBus(command)
            | Self::Knowledge(command)
            | Self::Memory(command)
            | Self::Llm(command)
            | Self::Reasoning(command)
            | Self::Decision(command)
            | Self::Planning(command)
            | Self::Orchestrator(command)
            | Self::Retrieval(command) => command,
        }
    }

    pub fn into_request(self) -> AdapterRequest {
        let target = self.target();
        let command = self.command().clone();
        AdapterRequest {
            command_id: command.command_id,
            target,
            operation: command.operation,
            context: command.context,
            payload: command.payload,
        }
    }
}

/// Typed result returned by the platform composition boundary.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypedCoreResponse {
    pub request_id: Uuid,
    pub target: IntegrationTarget,
    pub operation: String,
    pub accepted: bool,
    pub payload: Value,
}

impl From<AdapterResponse> for TypedCoreResponse {
    fn from(response: AdapterResponse) -> Self {
        Self {
            request_id: response.request_id,
            target: response.target,
            operation: response.operation,
            accepted: response.accepted,
            payload: response.payload,
        }
    }
}

/// Execute a typed cross-core command through the provider-neutral adapter registry.
///
/// This is the transition layer between the platform composition root and concrete core APIs.
/// It preserves target ownership and correlation/causation context while avoiding domain logic
/// duplication in the platform crate.
pub fn execute_typed(
    registry: &AdapterRegistry,
    command: TypedCoreCommand,
) -> PlatformResult<TypedCoreResponse> {
    let target = command.target();
    let request = command.into_request();
    let adapter_name = adapter_name(target);

    registry
        .execute(adapter_name, &request)
        .map(TypedCoreResponse::from)
}

fn adapter_name(target: IntegrationTarget) -> &'static str {
    match target {
        IntegrationTarget::EventBus => "eventbus",
        IntegrationTarget::Knowledge => "knowledge",
        IntegrationTarget::Memory => "memory",
        IntegrationTarget::Llm => "llm",
        IntegrationTarget::Reasoning => "reasoning",
        IntegrationTarget::Decision => "decision",
        IntegrationTarget::Planning => "planning",
        IntegrationTarget::Orchestrator => "orchestrator",
        IntegrationTarget::Retrieval => "retrieval",
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

pub const LLM_ADAPTER: CoreAdapter =
    CoreAdapter::new(IntegrationTarget::Llm, "llm", &["generate", "health"]);

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
    fn typed_command_selects_target_from_variant() {
        let command = TypedCoreCommand::Decision(CoreCommand::new(
            Uuid::now_v7(),
            "decide",
            IntegrationContext::new("platform-test"),
            json!({"objective":"test"}),
        ));
        assert_eq!(command.target(), IntegrationTarget::Decision);
        assert_eq!(command.command().operation, "decide");
    }

    #[test]
    fn typed_execution_preserves_correlation_and_target() {
        let registry = default_core_adapter_registry().unwrap();
        let context = IntegrationContext::new("platform-test");
        let correlation_id = context.correlation_id;
        let command = TypedCoreCommand::Planning(CoreCommand::new(
            Uuid::now_v7(),
            "validate_plan",
            context,
            json!({"plan_id":"p-1"}),
        ));

        let response = execute_typed(&registry, command).unwrap();
        assert!(response.accepted);
        assert_eq!(response.target, IntegrationTarget::Planning);
        assert_eq!(response.payload["correlation_id"], json!(correlation_id));
        assert_eq!(response.payload["domain_owner"], json!("planning"));
    }

    #[test]
    fn typed_execution_rejects_disallowed_operation() {
        let registry = default_core_adapter_registry().unwrap();
        let command = TypedCoreCommand::Memory(CoreCommand::new(
            Uuid::now_v7(),
            "publish_money",
            IntegrationContext::new("platform-test"),
            json!({}),
        ));

        assert!(matches!(
            execute_typed(&registry, command),
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

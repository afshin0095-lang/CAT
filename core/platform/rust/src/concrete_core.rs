use std::sync::{Arc, Mutex};

use cat_eventbus::{EventBus, EventEnvelope, PublishOutcome};
use cat_knowledge::{KnowledgeEdge, KnowledgeGraph, KnowledgeNode};
use cat_memory::{InMemoryMemoryStore, MemoryObject, MemoryStore};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::{
    IntegrationTarget, PlatformError, PlatformResult, TypedCoreCommand, TypedCoreResponse,
};

/// Concrete composition boundary for the first three stateful CAT cores.
///
/// The platform owns only wiring and context propagation. Event semantics remain in
/// `cat-eventbus`, graph semantics in `cat-knowledge`, and memory policy/lifecycle semantics
/// in `cat-memory`.
#[derive(Clone)]
pub struct ConcreteCoreRuntime {
    event_bus: Arc<EventBus>,
    knowledge: Arc<Mutex<KnowledgeGraph>>,
    memory: Arc<Mutex<InMemoryMemoryStore>>,
}

impl Default for ConcreteCoreRuntime {
    fn default() -> Self {
        let now_ms = current_time_ms();
        Self {
            event_bus: Arc::new(EventBus::new()),
            knowledge: Arc::new(Mutex::new(KnowledgeGraph::default())),
            memory: Arc::new(Mutex::new(InMemoryMemoryStore::new(
                cat_memory::MemoryPolicy::default(),
                now_ms,
            ))),
        }
    }
}

impl ConcreteCoreRuntime {
    pub fn new(
        event_bus: Arc<EventBus>,
        knowledge: KnowledgeGraph,
        memory: InMemoryMemoryStore,
    ) -> Self {
        Self {
            event_bus,
            knowledge: Arc::new(Mutex::new(knowledge)),
            memory: Arc::new(Mutex::new(memory)),
        }
    }

    pub fn execute(&self, command: TypedCoreCommand) -> PlatformResult<TypedCoreResponse> {
        match command {
            TypedCoreCommand::EventBus(command) => self.execute_event_bus(command),
            TypedCoreCommand::Knowledge(command) => self.execute_knowledge(command),
            TypedCoreCommand::Memory(command) => self.execute_memory(command),
            other => Err(PlatformError::AdapterNotFound(format!(
                "concrete execution is not wired yet for {:?}",
                other.target()
            ))),
        }
    }

    fn execute_event_bus(&self, command: crate::CoreCommand) -> PlatformResult<TypedCoreResponse> {
        match command.operation.as_str() {
            "publish" => {
                let envelope: EventEnvelope = serde_json::from_value(command.payload)
                    .map_err(|error| PlatformError::InvalidCommand(format!("invalid event envelope: {error}")))?;
                let event_id = envelope.event_id;
                let outcome = self.event_bus.publish(envelope).map_err(core_error)?;
                let handlers_called = match outcome {
                    PublishOutcome::Published { handlers_called } => handlers_called,
                    PublishOutcome::DuplicateSuppressed => 0,
                };
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::EventBus,
                    "publish",
                    json!({
                        "event_id": event_id,
                        "published": matches!(outcome, PublishOutcome::Published { .. }),
                        "duplicate_suppressed": matches!(outcome, PublishOutcome::DuplicateSuppressed),
                        "handlers_called": handlers_called,
                        "correlation_id": command.context.correlation_id,
                        "causation_id": command.context.causation_id,
                    }),
                ))
            }
            "subscribe" => Err(PlatformError::InvalidCommand(
                "event subscriptions require a typed handler and must be registered at composition time".into(),
            )),
            "replay" => Err(PlatformError::InvalidCommand(
                "event replay is owned by the durable event/replay boundary".into(),
            )),
            _ => Err(PlatformError::InvalidCommand(format!(
                "unsupported eventbus operation: {}",
                command.operation
            ))),
        }
    }

    fn execute_knowledge(&self, command: crate::CoreCommand) -> PlatformResult<TypedCoreResponse> {
        let mut graph = self
            .knowledge
            .lock()
            .map_err(|_| PlatformError::InvalidCommand("knowledge graph lock poisoned".into()))?;

        match command.operation.as_str() {
            "upsert_node" => {
                let node: KnowledgeNode = serde_json::from_value(command.payload)
                    .map_err(|error| PlatformError::InvalidCommand(format!("invalid knowledge node: {error}")))?;
                let id = graph.insert_node(node).map_err(core_error)?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Knowledge,
                    "upsert_node",
                    json!({
                        "node_id": id,
                        "node_count": graph.node_count(),
                        "edge_count": graph.edge_count(),
                    }),
                ))
            }
            "upsert_edge" => {
                let edge: KnowledgeEdge = serde_json::from_value(command.payload)
                    .map_err(|error| PlatformError::InvalidCommand(format!("invalid knowledge edge: {error}")))?;
                let id = graph.insert_edge(edge).map_err(core_error)?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Knowledge,
                    "upsert_edge",
                    json!({
                        "edge_id": id,
                        "node_count": graph.node_count(),
                        "edge_count": graph.edge_count(),
                    }),
                ))
            }
            "traverse" | "validate" => Err(PlatformError::InvalidCommand(
                "knowledge traversal/validation requires typed query contracts and is not a JSON command shortcut".into(),
            )),
            _ => Err(PlatformError::InvalidCommand(format!(
                "unsupported knowledge operation: {}",
                command.operation
            ))),
        }
    }

    fn execute_memory(&self, command: crate::CoreCommand) -> PlatformResult<TypedCoreResponse> {
        let mut store = self
            .memory
            .lock()
            .map_err(|_| PlatformError::InvalidCommand("memory store lock poisoned".into()))?;

        match command.operation.as_str() {
            "store" => {
                let object: MemoryObject =
                    serde_json::from_value(command.payload).map_err(|error| {
                        PlatformError::InvalidCommand(format!("invalid memory object: {error}"))
                    })?;
                let id = object.id;
                store.insert(object).map_err(core_error)?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Memory,
                    "store",
                    json!({"memory_id": id, "size": store.len()}),
                ))
            }
            "recall" => {
                let id: cat_memory::MemoryId =
                    serde_json::from_value(command.payload).map_err(|error| {
                        PlatformError::InvalidCommand(format!("invalid memory id: {error}"))
                    })?;
                let object = store.get(id).ok_or_else(|| {
                    PlatformError::InvalidCommand(format!("memory object not found: {id:?}"))
                })?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Memory,
                    "recall",
                    serde_json::to_value(object).map_err(|error| {
                        PlatformError::InvalidCommand(format!(
                            "memory serialization failed: {error}"
                        ))
                    })?,
                ))
            }
            "forget" => {
                let id: cat_memory::MemoryId =
                    serde_json::from_value(command.payload).map_err(|error| {
                        PlatformError::InvalidCommand(format!("invalid memory id: {error}"))
                    })?;
                store.remove(id).map_err(core_error)?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Memory,
                    "forget",
                    json!({"memory_id": id, "removed": true, "size": store.len()}),
                ))
            }
            "validate" => Err(PlatformError::InvalidCommand(
                "memory validation is performed by the memory core before persistence".into(),
            )),
            _ => Err(PlatformError::InvalidCommand(format!(
                "unsupported memory operation: {}",
                command.operation
            ))),
        }
    }
}

fn response(
    request_id: Uuid,
    target: IntegrationTarget,
    operation: &str,
    payload: Value,
) -> TypedCoreResponse {
    TypedCoreResponse {
        request_id,
        target,
        operation: operation.to_owned(),
        accepted: true,
        payload,
    }
}

fn core_error(error: impl std::fmt::Display) -> PlatformError {
    PlatformError::InvalidCommand(error.to_string())
}

fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CoreCommand, IntegrationContext};
    use cat_eventbus::{EventEnvelope, EventKind};
    use cat_knowledge::KnowledgeNode;
    use serde_json::json;

    #[test]
    fn eventbus_publish_is_executed_by_the_real_core() {
        let runtime = ConcreteCoreRuntime::default();
        let envelope = EventEnvelope {
            event_id: Uuid::now_v7(),
            event_type: "affiliate.conversion".into(),
            version: 1,
            kind: EventKind::Domain,
            occurred_at_ms: 1,
            producer: "platform-test".into(),
            correlation_id: None,
            causation_id: None,
            subject_id: None,
            payload: json!({"amount": 42}),
        };
        let response = runtime
            .execute(TypedCoreCommand::EventBus(CoreCommand::new(
                Uuid::now_v7(),
                "publish",
                IntegrationContext::new("platform-test"),
                serde_json::to_value(envelope).unwrap(),
            )))
            .unwrap();
        assert!(response.accepted);
        assert_eq!(response.target, IntegrationTarget::EventBus);
        assert_eq!(response.payload["published"], true);
    }

    #[test]
    fn knowledge_upsert_uses_the_real_graph_store() {
        let runtime = ConcreteCoreRuntime::default();
        let node = KnowledgeNode::new("merchant", "merchant:1");
        let response = runtime
            .execute(TypedCoreCommand::Knowledge(CoreCommand::new(
                Uuid::now_v7(),
                "upsert_node",
                IntegrationContext::new("platform-test"),
                serde_json::to_value(node).unwrap(),
            )))
            .unwrap();
        assert_eq!(response.payload["node_count"], 1);
    }

    #[test]
    fn memory_store_and_recall_use_the_real_memory_store() {
        let runtime = ConcreteCoreRuntime::default();
        let object = cat_memory::MemoryObject {
            id: cat_memory::MemoryId::new(),
            kind: cat_memory::MemoryKind::Observation,
            namespace: "platform-test".into(),
            subject: Some("integration".into()),
            classification: cat_memory::Classification::Internal,
            state: cat_memory::LifecycleState::Proposed,
            version: 1,
            content: json!({"fact":"hello"}),
            provenance: cat_memory::Provenance {
                source: "platform-test".into(),
                source_version: None,
                captured_at_ms: 1,
                captured_by: "platform-test".into(),
            },
            consent: cat_memory::Consent {
                required: false,
                granted: true,
                scope: "platform-test".into(),
                policy_version: "v1".into(),
            },
            retention: cat_memory::Retention {
                expires_at_ms: None,
                legal_hold: false,
                policy_version: "v1".into(),
            },
            created_at_ms: 1,
            updated_at_ms: 1,
        };
        let id = object.id;
        runtime
            .execute(TypedCoreCommand::Memory(CoreCommand::new(
                Uuid::now_v7(),
                "store",
                IntegrationContext::new("platform-test"),
                serde_json::to_value(&object).unwrap(),
            )))
            .unwrap();
        let response = runtime
            .execute(TypedCoreCommand::Memory(CoreCommand::new(
                Uuid::now_v7(),
                "recall",
                IntegrationContext::new("platform-test"),
                serde_json::to_value(id).unwrap(),
            )))
            .unwrap();
        assert_eq!(response.payload["id"], json!(id));
    }
}

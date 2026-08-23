use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationTarget {
    EventBus,
    Knowledge,
    Memory,
    Llm,
    Reasoning,
    Decision,
    Planning,
    Orchestrator,
    Retrieval,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IntegrationContext {
    pub request_id: Uuid,
    pub correlation_id: Uuid,
    pub causation_id: Option<Uuid>,
    pub actor: String,
    pub workflow_id: Option<Uuid>,
}

impl IntegrationContext {
    pub fn new(actor: impl Into<String>) -> Self {
        let correlation_id = Uuid::now_v7();
        Self {
            request_id: Uuid::now_v7(),
            correlation_id,
            causation_id: None,
            actor: actor.into(),
            workflow_id: None,
        }
    }

    pub fn with_causation(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_workflow(mut self, workflow_id: Uuid) -> Self {
        self.workflow_id = Some(workflow_id);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IntegrationCommand {
    pub command_id: Uuid,
    pub target: IntegrationTarget,
    pub operation: String,
    pub context: IntegrationContext,
}

impl IntegrationCommand {
    pub fn new(
        target: IntegrationTarget,
        operation: impl Into<String>,
        context: IntegrationContext,
    ) -> Self {
        Self {
            command_id: Uuid::now_v7(),
            target,
            operation: operation.into(),
            context,
        }
    }
}

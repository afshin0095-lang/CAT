use serde::Serialize;
use uuid::Uuid;

use crate::{ExecutionAuthorization, OrchestratorResult};

/// Pre-admission execution intent. It contains no authorization and cannot be sent to a worker.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExecutionIntent {
    workflow_id: Uuid,
    step_id: String,
    attempt: u32,
    requested_at_ms: u64,
}

impl ExecutionIntent {
    pub fn new(
        workflow_id: Uuid,
        step_id: impl Into<String>,
        attempt: u32,
        requested_at_ms: u64,
    ) -> Self {
        Self {
            workflow_id,
            step_id: step_id.into(),
            attempt,
            requested_at_ms,
        }
    }

    pub fn workflow_id(&self) -> Uuid {
        self.workflow_id
    }

    pub fn step_id(&self) -> &str {
        &self.step_id
    }

    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    pub fn requested_at_ms(&self) -> u64 {
        self.requested_at_ms
    }
}

/// Executable request. Construction is crate-private and requires a kernel-backed
/// ExecutionAuthorization bound to the exact workflow/step/attempt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExecutionRequest {
    intent: ExecutionIntent,
    authorization: ExecutionAuthorization,
}

impl ExecutionRequest {
    pub(crate) fn from_authorized(
        intent: ExecutionIntent,
        authorization: ExecutionAuthorization,
    ) -> OrchestratorResult<Self> {
        if !authorization.matches_execution(
            intent.workflow_id(),
            intent.step_id(),
            intent.attempt(),
        ) {
            return Err(crate::OrchestratorError::InvalidAuthorizationInput(
                "execution authorization does not match workflow step attempt".to_owned(),
            ));
        }

        Ok(Self {
            intent,
            authorization,
        })
    }

    pub fn workflow_id(&self) -> Uuid {
        self.intent.workflow_id()
    }

    pub fn step_id(&self) -> &str {
        self.intent.step_id()
    }

    pub fn attempt(&self) -> u32 {
        self.intent.attempt()
    }

    pub fn requested_at_ms(&self) -> u64 {
        self.intent.requested_at_ms()
    }

    pub fn authorization(&self) -> &ExecutionAuthorization {
        &self.authorization
    }

    pub fn intent(&self) -> &ExecutionIntent {
        &self.intent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intent_preserves_execution_identity() {
        let workflow_id = Uuid::now_v7();
        let intent = ExecutionIntent::new(workflow_id, "research", 3, 100);

        assert_eq!(intent.workflow_id(), workflow_id);
        assert_eq!(intent.step_id(), "research");
        assert_eq!(intent.attempt(), 3);
        assert_eq!(intent.requested_at_ms(), 100);
    }
}

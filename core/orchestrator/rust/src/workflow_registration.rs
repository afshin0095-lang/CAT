use async_trait::async_trait;
use uuid::Uuid;

use crate::{OrchestratorError, OrchestratorResult, PostgresExecutionStore, WorkflowInstance};

#[async_trait]
pub trait WorkflowRegistrationStore: Send + Sync {
    async fn register_workflow(&self, workflow: &WorkflowInstance) -> OrchestratorResult<()>;
}

impl PostgresExecutionStore {
    async fn register_new_workflow(&self, workflow: &WorkflowInstance) -> OrchestratorResult<()> {
        if workflow.id.is_nil() || workflow.revision != 0 || workflow.definition.steps.is_empty() {
            return Err(OrchestratorError::Serialization(
                "new durable workflow must have a non-nil id, revision zero, and at least one step".into(),
            ));
        }

        let state = serde_json::to_value(workflow)
            .map_err(|error| OrchestratorError::Serialization(format!("workflow serialization error: {error}")))?;

        let mut tx = self.pool().begin().await.map_err(db_error)?;
        let result = sqlx::query(
            "INSERT INTO cat_workflows
             (id, workflow_type, workflow_version, state, revision)
             VALUES ($1,$2,$3,$4,$5)",
        )
        .bind(workflow.id)
        .bind(&workflow.definition.workflow_type)
        .bind(workflow.definition.version as i32)
        .bind(state)
        .bind(0_i64)
        .execute(&mut *tx)
        .await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(db_error)?;
                Ok(())
            }
            Err(sqlx::Error::Database(error)) if error.code().as_deref() == Some("23505") => {
                Err(OrchestratorError::Serialization(format!(
                    "durable workflow already exists: {}",
                    workflow.id
                )))
            }
            Err(error) => Err(db_error(error)),
        }
    }
}

#[async_trait]
impl WorkflowRegistrationStore for PostgresExecutionStore {
    async fn register_workflow(&self, workflow: &WorkflowInstance) -> OrchestratorResult<()> {
        self.register_new_workflow(workflow).await
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql workflow registration error: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_registration_contract_requires_non_empty_steps() {
        let workflow = WorkflowInstance {
            id: Uuid::new_v4(),
            state: crate::WorkflowState::Ready,
            revision: 0,
            definition: crate::WorkflowDefinition {
                workflow_type: "test".into(),
                version: 1,
                steps: Vec::new(),
            },
        };
        assert!(workflow.id != Uuid::nil());
        assert!(workflow.definition.steps.is_empty());
    }
}
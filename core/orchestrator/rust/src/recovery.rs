use uuid::Uuid;

use crate::{
    AsyncPostgresExecutionStore, OrchestratorResult, StepState, WorkflowInstance, WorkflowState,
};

/// Deterministic recovery classification for a durable workflow after process restart.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryAction {
    ResumeReady,
    AwaitApproval,
    ReconcileRunning,
    Terminal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkflowRecoveryReport {
    pub workflow_id: Uuid,
    pub state: WorkflowState,
    pub runnable_steps: Vec<String>,
    pub waiting_steps: Vec<String>,
    pub reconciliation_steps: Vec<String>,
    pub action: RecoveryAction,
}

impl WorkflowRecoveryReport {
    pub fn from_workflow(workflow: &WorkflowInstance) -> Self {
        let mut runnable_steps = Vec::new();
        let mut waiting_steps = Vec::new();
        let mut reconciliation_steps = Vec::new();

        for step in &workflow.definition.steps {
            match step.state {
                StepState::Ready => runnable_steps.push(step.id.clone()),
                StepState::Waiting => waiting_steps.push(step.id.clone()),
                StepState::Running => reconciliation_steps.push(step.id.clone()),
                _ => {}
            }
        }

        let action = if !reconciliation_steps.is_empty() {
            RecoveryAction::ReconcileRunning
        } else if !runnable_steps.is_empty() {
            RecoveryAction::ResumeReady
        } else if !waiting_steps.is_empty() {
            RecoveryAction::AwaitApproval
        } else {
            RecoveryAction::Terminal
        };

        Self {
            workflow_id: workflow.id,
            state: workflow.state,
            runnable_steps,
            waiting_steps,
            reconciliation_steps,
            action,
        }
    }
}

/// Restart-safe recovery boundary. Implementations load authoritative durable state;
/// they never infer success from process memory or mutate worker side effects.
#[async_trait::async_trait]
pub trait WorkflowRecoveryStore: Send + Sync {
    async fn recover(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowRecoveryReport>;
}

/// Thin recovery facade around any asynchronous durable execution store.
pub struct AsyncWorkflowRecovery<'a, S> {
    pub store: &'a S,
}

impl<'a, S> AsyncWorkflowRecovery<'a, S> {
    pub fn new(store: &'a S) -> Self {
        Self { store }
    }
}

#[async_trait::async_trait]
impl<S> WorkflowRecoveryStore for AsyncWorkflowRecovery<'_, S>
where
    S: AsyncPostgresExecutionStore,
{
    async fn recover(&self, workflow_id: Uuid) -> OrchestratorResult<WorkflowRecoveryReport> {
        let workflow = self.store.load_workflow(workflow_id).await?;
        Ok(WorkflowRecoveryReport::from_workflow(&workflow))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WorkflowDefinition, WorkflowStep};

    fn workflow(states: &[StepState]) -> WorkflowInstance {
        WorkflowInstance {
            id: Uuid::now_v7(),
            state: WorkflowState::Running,
            revision: 7,
            definition: WorkflowDefinition {
                workflow_type: "recovery.test".into(),
                version: 1,
                steps: states
                    .iter()
                    .enumerate()
                    .map(|(index, state)| WorkflowStep {
                        id: format!("step-{index}"),
                        dependencies: Vec::new(),
                        state: *state,
                        attempt: 1,
                        max_attempts: 3,
                        compensation_step: None,
                    })
                    .collect(),
            },
        }
    }

    #[test]
    fn running_steps_require_reconciliation_before_replay() {
        let workflow = workflow(&[StepState::Running, StepState::Ready]);
        let report = WorkflowRecoveryReport::from_workflow(&workflow);
        assert_eq!(report.action, RecoveryAction::ReconcileRunning);
        assert_eq!(report.reconciliation_steps, vec!["step-0"]);
        assert_eq!(report.runnable_steps, vec!["step-1"]);
    }

    #[test]
    fn ready_workflow_resumes_when_no_unknown_attempt_exists() {
        let workflow = workflow(&[StepState::Ready]);
        let report = WorkflowRecoveryReport::from_workflow(&workflow);
        assert_eq!(report.action, RecoveryAction::ResumeReady);
    }

    #[test]
    fn completed_workflow_is_terminal() {
        let mut workflow = workflow(&[StepState::Succeeded]);
        workflow.state = WorkflowState::Succeeded;
        let report = WorkflowRecoveryReport::from_workflow(&workflow);
        assert_eq!(report.action, RecoveryAction::Terminal);
    }
}

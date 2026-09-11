use crate::{StepState, WorkflowInstance};

/// Immutable view of the next executable work known by the orchestrator state machine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionCursor {
    pub workflow_id: uuid::Uuid,
    pub revision: u64,
    pub ready_steps: Vec<String>,
}

pub fn cursor(workflow: &WorkflowInstance) -> ExecutionCursor {
    ExecutionCursor {
        workflow_id: workflow.id,
        revision: workflow.revision,
        ready_steps: workflow
            .definition
            .steps
            .iter()
            .filter(|step| step.state == StepState::Ready)
            .map(|step| step.id.clone())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WorkflowDefinition, WorkflowState, WorkflowStep};

    #[test]
    fn cursor_exposes_only_ready_steps() {
        let workflow = WorkflowInstance {
            id: uuid::Uuid::now_v7(),
            definition: WorkflowDefinition {
                workflow_type: "test".into(),
                version: 1,
                steps: vec![
                    WorkflowStep {
                        id: "ready".into(),
                        dependencies: vec![],
                        state: StepState::Ready,
                        attempt: 0,
                        max_attempts: 3,
                        compensation_step: None,
                    },
                    WorkflowStep {
                        id: "pending".into(),
                        dependencies: vec![],
                        state: StepState::Pending,
                        attempt: 0,
                        max_attempts: 3,
                        compensation_step: None,
                    },
                ],
            },
            state: WorkflowState::Running,
            revision: 7,
        };

        let view = cursor(&workflow);
        assert_eq!(view.workflow_id, workflow.id);
        assert_eq!(view.revision, 7);
        assert_eq!(view.ready_steps, vec!["ready"]);
    }
}

use std::collections::HashSet;

use crate::{OrchestratorError, OrchestratorResult, StepState, WorkflowInstance, WorkflowState};

#[derive(Default)]
pub struct ExecutionEngine;

impl ExecutionEngine {
    pub fn start(&self, workflow: &mut WorkflowInstance) -> OrchestratorResult<()> {
        if workflow.state != WorkflowState::Pending {
            return Err(OrchestratorError::InvalidStateTransition { from: format!("{:?}", workflow.state), to: "running".into() });
        }
        workflow.state = WorkflowState::Running;
        workflow.revision = workflow.revision.saturating_add(1);
        self.refresh_ready_steps(workflow)
    }

    pub fn refresh_ready_steps(&self, workflow: &mut WorkflowInstance) -> OrchestratorResult<()> {
        if workflow.state != WorkflowState::Running && workflow.state != WorkflowState::Waiting { return Ok(()); }
        let succeeded: HashSet<String> = workflow.definition.steps.iter().filter(|step| step.state == StepState::Succeeded).map(|step| step.id.clone()).collect();
        for step in &mut workflow.definition.steps {
            if step.state == StepState::Pending && step.dependencies.iter().all(|dependency| succeeded.contains(dependency)) { step.state = StepState::Ready; }
        }
        if workflow.definition.steps.iter().any(|step| matches!(step.state, StepState::Ready | StepState::Running)) {
            workflow.state = WorkflowState::Running;
        } else if workflow.definition.steps.iter().all(|step| matches!(step.state, StepState::Succeeded | StepState::Compensated | StepState::Skipped)) {
            workflow.state = WorkflowState::Succeeded;
        } else {
            workflow.state = WorkflowState::Waiting;
        }
        workflow.revision = workflow.revision.saturating_add(1);
        Ok(())
    }

    pub fn begin_step(&self, workflow: &mut WorkflowInstance, step_id: &str) -> OrchestratorResult<()> {
        let step = workflow.definition.steps.iter_mut().find(|step| step.id == step_id).ok_or_else(|| OrchestratorError::UnknownStep { step_id: step_id.to_owned() })?;
        if step.state != StepState::Ready {
            return Err(OrchestratorError::InvalidStepTransition { step_id: step_id.to_owned(), from: format!("{:?}", step.state), to: "running".into() });
        }
        step.state = StepState::Running;
        step.attempt = step.attempt.saturating_add(1);
        workflow.state = WorkflowState::Running;
        workflow.revision = workflow.revision.saturating_add(1);
        Ok(())
    }

    pub fn succeed_step(&self, workflow: &mut WorkflowInstance, step_id: &str) -> OrchestratorResult<()> {
        let step = workflow.definition.steps.iter_mut().find(|step| step.id == step_id).ok_or_else(|| OrchestratorError::UnknownStep { step_id: step_id.to_owned() })?;
        if step.state != StepState::Running {
            return Err(OrchestratorError::InvalidStepTransition { step_id: step_id.to_owned(), from: format!("{:?}", step.state), to: "succeeded".into() });
        }
        step.state = StepState::Succeeded;
        workflow.revision = workflow.revision.saturating_add(1);
        self.refresh_ready_steps(workflow)
    }

    pub fn wait_step(&self, workflow: &mut WorkflowInstance, step_id: &str) -> OrchestratorResult<()> {
        let step = workflow.definition.steps.iter_mut().find(|step| step.id == step_id).ok_or_else(|| OrchestratorError::UnknownStep { step_id: step_id.to_owned() })?;
        if step.state != StepState::Running {
            return Err(OrchestratorError::InvalidStepTransition { step_id: step_id.to_owned(), from: format!("{:?}", step.state), to: "waiting".into() });
        }
        step.state = StepState::Waiting;
        workflow.state = WorkflowState::Waiting;
        workflow.revision = workflow.revision.saturating_add(1);
        Ok(())
    }

    pub fn cancel_step(&self, workflow: &mut WorkflowInstance, step_id: &str) -> OrchestratorResult<()> {
        let step = workflow.definition.steps.iter_mut().find(|step| step.id == step_id).ok_or_else(|| OrchestratorError::UnknownStep { step_id: step_id.to_owned() })?;
        if step.state != StepState::Running {
            return Err(OrchestratorError::InvalidStepTransition { step_id: step_id.to_owned(), from: format!("{:?}", step.state), to: "skipped".into() });
        }
        step.state = StepState::Skipped;
        workflow.state = WorkflowState::Cancelled;
        workflow.revision = workflow.revision.saturating_add(1);
        Ok(())
    }

    pub fn fail_step(&self, workflow: &mut WorkflowInstance, step_id: &str) -> OrchestratorResult<()> {
        let step = workflow.definition.steps.iter_mut().find(|step| step.id == step_id).ok_or_else(|| OrchestratorError::UnknownStep { step_id: step_id.to_owned() })?;
        if step.state != StepState::Running {
            return Err(OrchestratorError::InvalidStepTransition { step_id: step_id.to_owned(), from: format!("{:?}", step.state), to: "failed".into() });
        }
        step.state = StepState::Failed;
        workflow.state = WorkflowState::Failed;
        workflow.revision = workflow.revision.saturating_add(1);
        Ok(())
    }

    pub fn cancel(&self, workflow: &mut WorkflowInstance) -> OrchestratorResult<()> {
        if workflow.state.terminal() {
            return Err(OrchestratorError::InvalidStateTransition { from: format!("{:?}", workflow.state), to: "cancelled".into() });
        }
        workflow.state = WorkflowState::Cancelled;
        workflow.revision = workflow.revision.saturating_add(1);
        Ok(())
    }
}

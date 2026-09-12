use cat_orchestrator::{ScheduleRequest, Scheduler};
use cat_planning::{Plan, PlanStatus, validate_plan};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{IntegrationContext, PlatformError, PlatformResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowExecutionReceipt {
    pub request_id: Uuid,
    pub workflow_id: Uuid,
    pub plan_id: cat_planning::PlanId,
    pub plan_version: u32,
    pub validated: bool,
}

/// Concrete wiring for planning validation and orchestration scheduling.
///
/// The platform owns only the composition boundary: planning remains authoritative for plan
/// semantics and orchestration remains authoritative for workflow scheduling. No domain state is
/// copied into this runtime beyond the transient execution receipt.
#[derive(Default)]
pub struct WorkflowRuntime {
    scheduler: Scheduler,
}

impl WorkflowRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate_plan(
        &self,
        context: &IntegrationContext,
        plan: &Plan,
    ) -> PlatformResult<WorkflowExecutionReceipt> {
        let report = validate_plan(plan);
        if !report.is_valid() {
            return Err(PlatformError::InvalidCommand(format!(
                "planning validation failed: {} issue(s)",
                report.errors.len()
            )));
        }

        if plan.status != PlanStatus::Draft && plan.status != PlanStatus::Validated {
            return Err(PlatformError::InvalidCommand(
                "only draft or validated plans may enter the execution boundary".into(),
            ));
        }

        Ok(WorkflowExecutionReceipt {
            request_id: context.request_id,
            workflow_id: context.workflow_id.unwrap_or_else(Uuid::now_v7),
            plan_id: plan.id,
            plan_version: plan.version,
            validated: true,
        })
    }

    pub fn schedule(
        &mut self,
        context: &IntegrationContext,
        workflow_id: Uuid,
        not_before_ms: u64,
        priority: i32,
    ) -> PlatformResult<()> {
        if context.workflow_id != Some(workflow_id) {
            return Err(PlatformError::InvalidCommand(
                "workflow context does not match scheduled workflow".into(),
            ));
        }
        self.scheduler.schedule(ScheduleRequest {
            workflow_id,
            not_before_ms,
            priority,
        });
        Ok(())
    }

    pub fn pop_ready(&mut self, now_ms: u64) -> Option<ScheduleRequest> {
        self.scheduler.pop_ready(now_ms)
    }

    pub fn queue_depth(&self) -> usize {
        self.scheduler.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_planning::{PlanBuilder, StepKind};

    #[test]
    fn validated_plan_produces_execution_receipt() {
        let mut builder = PlanBuilder::new("publish approved offer");
        builder = builder.metadata("domain", serde_json::json!("affiliate"));
        let _ = builder.step("approve", StepKind::Approval);
        let plan = builder.build();
        let context = IntegrationContext::new("platform-test").with_workflow(Uuid::now_v7());
        let runtime = WorkflowRuntime::new();

        let receipt = runtime.validate_plan(&context, &plan).unwrap();
        assert!(receipt.validated);
        assert_eq!(receipt.request_id, context.request_id);
        assert_eq!(receipt.plan_id, plan.id);
    }

    #[test]
    fn scheduling_preserves_workflow_context_boundary() {
        let workflow_id = Uuid::now_v7();
        let context = IntegrationContext::new("platform-test").with_workflow(workflow_id);
        let mut runtime = WorkflowRuntime::new();

        runtime.schedule(&context, workflow_id, 100, 5).unwrap();
        assert_eq!(runtime.queue_depth(), 1);
        assert!(runtime.pop_ready(99).is_none());
        assert_eq!(runtime.pop_ready(100).unwrap().workflow_id, workflow_id);
        assert_eq!(runtime.queue_depth(), 0);
    }

    #[test]
    fn mismatched_workflow_context_is_rejected() {
        let context = IntegrationContext::new("platform-test").with_workflow(Uuid::now_v7());
        let mut runtime = WorkflowRuntime::new();
        let error = runtime
            .schedule(&context, Uuid::now_v7(), 100, 1)
            .unwrap_err();
        assert!(matches!(error, PlatformError::InvalidCommand(_)));
    }
}

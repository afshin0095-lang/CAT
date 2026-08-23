use cat_decision::{DecisionRequest, DecisionResult, DeterministicDecisionEngine};
use cat_orchestrator::{ScheduleRequest, Scheduler};
use cat_planning::{Plan, PlanValidationReport, validate_plan};

use crate::{IntegrationCommand, PlatformError, PlatformResult};

#[derive(Debug)]
pub struct ReadyWork {
    pub workflow_id: uuid::Uuid,
    pub priority: i32,
}

#[derive(Default)]
pub struct PlatformRuntime {
    decision: DeterministicDecisionEngine,
    scheduler: Scheduler,
}

impl PlatformRuntime {
    pub fn new(decision: DeterministicDecisionEngine) -> Self {
        Self { decision, scheduler: Scheduler::default() }
    }

    pub fn validate_plan(&self, plan: &Plan) -> PlanValidationReport {
        validate_plan(plan)
    }

    pub fn decide(&self, request: &DecisionRequest) -> cat_decision::DecisionResult {
        self.decision.decide(request)
    }

    pub fn schedule(&mut self, request: ScheduleRequest) {
        self.scheduler.schedule(request);
    }

    pub fn ready_work(&mut self, now_ms: u64) -> Option<ReadyWork> {
        self.scheduler.pop_ready(now_ms).map(|request| ReadyWork {
            workflow_id: request.workflow_id,
            priority: request.priority,
        })
    }

    pub fn accept_command(&self, command: &IntegrationCommand) -> PlatformResult<()> {
        if command.operation.trim().is_empty() || command.context.actor.trim().is_empty() {
            return Err(PlatformError::InvalidCommand("operation and actor are required".into()));
        }
        Ok(())
    }

    pub fn queue_depth(&self) -> usize {
        self.scheduler.len()
    }
}

#[allow(dead_code)]
fn _decision_result_type_is_used(_: Option<DecisionResult>) {}

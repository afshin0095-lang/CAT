use crate::{validate_plan, Plan, PlanId, StepId};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum PlanScheduleError {
    #[error("plan {0:?} is invalid: {1:?}")]
    InvalidPlan(PlanId, crate::PlanValidationError),
    #[error("step {step:?} references a missing prerequisite {prerequisite:?}")]
    MissingPrerequisite { step: StepId, prerequisite: StepId },
    #[error("unable to produce a complete schedule because the dependency graph contains a cycle")]
    CyclicDependencies,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanSchedule {
    plan_id: PlanId,
    levels: Vec<Vec<StepId>>,
}

impl PlanSchedule {
    pub fn plan_id(&self) -> PlanId { self.plan_id }
    pub fn levels(&self) -> &[Vec<StepId>] { &self.levels }
    pub fn execution_order(&self) -> impl Iterator<Item = StepId> + '_ {
        self.levels.iter().flat_map(|level| level.iter().copied())
    }
    pub fn level_for(&self, step_id: StepId) -> Option<usize> {
        self.levels.iter().position(|level| level.contains(&step_id))
    }
}

/// Build a deterministic level-based schedule from a validated plan.
/// Steps in the same level may be considered for parallel execution by a future
/// orchestrator. Ordering inside a level follows the plan's explicit `order`.
pub fn schedule_plan(plan: &Plan) -> Result<PlanSchedule, PlanScheduleError> {
    let report = validate_plan(plan);
    if let Some(error) = report.errors.first() {
        return Err(PlanScheduleError::InvalidPlan(plan.id, error.clone()));
    }

    let mut by_id = BTreeMap::new();
    for step in &plan.steps {
        by_id.insert(step.id, step);
    }

    let mut indegree: BTreeMap<StepId, usize> = plan.steps.iter().map(|step| (step.id, 0)).collect();
    let mut dependents: BTreeMap<StepId, Vec<StepId>> = BTreeMap::new();

    for step in &plan.steps {
        for dependency in &step.dependencies {
            if !by_id.contains_key(&dependency.prerequisite) {
                return Err(PlanScheduleError::MissingPrerequisite {
                    step: step.id,
                    prerequisite: dependency.prerequisite,
                });
            }
            *indegree.get_mut(&step.id).expect("step ids initialized above") += 1;
            dependents.entry(dependency.prerequisite).or_default().push(step.id);
        }
    }

    let mut levels = Vec::new();
    let mut remaining: BTreeSet<StepId> = plan.steps.iter().map(|step| step.id).collect();

    while !remaining.is_empty() {
        let mut current: Vec<&crate::PlanStep> = plan
            .steps
            .iter()
            .filter(|step| remaining.contains(&step.id) && indegree[&step.id] == 0)
            .collect();

        if current.is_empty() {
            return Err(PlanScheduleError::CyclicDependencies);
        }

        current.sort_by_key(|step| (step.order, step.id));
        let ids: Vec<StepId> = current.iter().map(|step| step.id).collect();
        for step_id in &ids {
            remaining.remove(step_id);
            if let Some(children) = dependents.get(step_id) {
                for child in children {
                    *indegree.get_mut(child).expect("dependent initialized above") -= 1;
                }
            }
        }
        levels.push(ids);
    }

    Ok(PlanSchedule { plan_id: plan.id, levels })
}

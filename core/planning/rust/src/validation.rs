use crate::{Plan, PlanId, StepId};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum PlanValidationError {
    #[error("plan goal must not be empty")]
    EmptyGoal,
    #[error("plan must contain at least one step")]
    EmptyPlan,
    #[error("duplicate step id: {0:?}")]
    DuplicateStep(StepId),
    #[error("step order is not contiguous")]
    NonContiguousOrder,
    #[error("step {step:?} depends on unknown prerequisite {prerequisite:?}")]
    UnknownDependency { step: StepId, prerequisite: StepId },
    #[error("dependency cycle detected at {0:?}")]
    DependencyCycle(StepId),
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PlanValidationReport {
    pub plan_id: Option<PlanId>,
    pub errors: Vec<PlanValidationError>,
}

impl PlanValidationReport {
    pub fn is_valid(&self) -> bool { self.errors.is_empty() }
}

pub fn validate_plan(plan: &Plan) -> PlanValidationReport {
    let mut report = PlanValidationReport { plan_id: Some(plan.id), errors: Vec::new() };
    if plan.goal.trim().is_empty() { report.errors.push(PlanValidationError::EmptyGoal); }
    if plan.steps.is_empty() { report.errors.push(PlanValidationError::EmptyPlan); return report; }

    let mut ids = BTreeSet::new();
    let mut order = BTreeMap::new();
    for step in &plan.steps {
        if !ids.insert(step.id) { report.errors.push(PlanValidationError::DuplicateStep(step.id)); }
        order.insert(step.id, step.order);
    }
    let mut expected: Vec<u32> = (0..plan.steps.len() as u32).collect();
    let mut actual: Vec<u32> = plan.steps.iter().map(|s| s.order).collect();
    actual.sort_unstable();
    if actual != expected { report.errors.push(PlanValidationError::NonContiguousOrder); }

    for step in &plan.steps {
        for dep in &step.dependencies {
            if !ids.contains(&dep.prerequisite) { report.errors.push(PlanValidationError::UnknownDependency { step: step.id, prerequisite: dep.prerequisite }); }
        }
    }

    fn visit(id: StepId, plan: &Plan, visiting: &mut BTreeSet<StepId>, visited: &mut BTreeSet<StepId>, errors: &mut Vec<PlanValidationError>) {
        if visited.contains(&id) { return; }
        if !visiting.insert(id) { errors.push(PlanValidationError::DependencyCycle(id)); return; }
        if let Some(step) = plan.steps.iter().find(|s| s.id == id) {
            for dep in &step.dependencies { visit(dep.prerequisite, plan, visiting, visited, errors); }
        }
        visiting.remove(&id);
        visited.insert(id);
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for step in &plan.steps { visit(step.id, plan, &mut visiting, &mut visited, &mut report.errors); }
    report
}

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct PlanId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct StepId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus { Draft, Validated, Ready, Executing, Completed, Failed, Cancelled }

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepKind { Action, Decision, Observation, Approval, Compensation }

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct StepDependency {
    pub prerequisite: StepId,
    pub required: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanStep {
    pub id: StepId,
    pub name: String,
    pub kind: StepKind,
    pub order: u32,
    pub dependencies: Vec<StepDependency>,
    pub inputs: BTreeMap<String, serde_json::Value>,
    pub policy: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    pub version: u32,
    pub status: PlanStatus,
    pub goal: String,
    pub steps: Vec<PlanStep>,
    pub metadata: BTreeMap<String, serde_json::Value>,
}

pub struct PlanBuilder {
    plan: Plan,
}

impl PlanBuilder {
    pub fn new(goal: impl Into<String>) -> Self {
        Self { plan: Plan { id: PlanId(Uuid::now_v7()), version: 1, status: PlanStatus::Draft, goal: goal.into(), steps: Vec::new(), metadata: BTreeMap::new() } }
    }

    pub fn metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.plan.metadata.insert(key.into(), value); self
    }

    pub fn step(mut self, name: impl Into<String>, kind: StepKind) -> StepId {
        let id = StepId(Uuid::now_v7());
        let order = self.plan.steps.len() as u32;
        self.plan.steps.push(PlanStep { id, name: name.into(), kind, order, dependencies: Vec::new(), inputs: BTreeMap::new(), policy: BTreeMap::new() });
        id
    }

    pub fn depends_on(mut self, step: StepId, prerequisite: StepId, required: bool) -> Self {
        if let Some(target) = self.plan.steps.iter_mut().find(|s| s.id == step) {
            target.dependencies.push(StepDependency { prerequisite, required });
        }
        self
    }

    pub fn build(self) -> Plan { self.plan }
}

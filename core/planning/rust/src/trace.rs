use crate::{Plan, PlanId, PlanStatus, StepId};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanTraceKind { Created, Validated, Readied, Started, StepStarted, StepCompleted, Failed, Completed, Cancelled }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanTraceEntry {
    pub trace_id: Uuid,
    pub plan_id: PlanId,
    pub kind: PlanTraceKind,
    pub status: PlanStatus,
    pub step_id: Option<StepId>,
    pub timestamp_ms: u64,
    pub details: serde_json::Value,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlanTrace { entries: Vec<PlanTraceEntry> }

impl PlanTrace {
    pub fn entries(&self) -> &[PlanTraceEntry] { &self.entries }

    pub fn append(&mut self, plan: &Plan, kind: PlanTraceKind, step_id: Option<StepId>, details: serde_json::Value) {
        let timestamp_ms = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or_default();
        self.entries.push(PlanTraceEntry { trace_id: Uuid::now_v7(), plan_id: plan.id, kind, status: plan.status, step_id, timestamp_ms, details });
    }

    pub fn last(&self) -> Option<&PlanTraceEntry> { self.entries.last() }
}

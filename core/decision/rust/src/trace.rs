use crate::{DecisionError, DecisionOutcome, DecisionRequest, DecisionStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionTraceStep {
    pub step_id: Uuid,
    pub stage: String,
    pub description: String,
    pub evidence: Vec<Uuid>,
    pub metadata: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionTrace {
    pub trace_id: Uuid,
    pub decision_id: Uuid,
    pub steps: Vec<DecisionTraceStep>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DecisionReplay {
    pub decision_id: Uuid,
    pub trace_id: Uuid,
    pub selected_id: Option<String>,
    pub status: DecisionStatus,
    pub confidence: f64,
    pub advisory_only: bool,
    pub policy_reasons: Vec<String>,
    pub step_count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct DecisionTraceStore {
    traces: HashMap<Uuid, DecisionTrace>,
}

impl DecisionTrace {
    pub fn new(decision_id: Uuid) -> Self {
        Self { trace_id: Uuid::now_v7(), decision_id, steps: Vec::new() }
    }

    pub fn push(&mut self, stage: impl Into<String>, description: impl Into<String>) {
        self.steps.push(DecisionTraceStep {
            step_id: Uuid::now_v7(),
            stage: stage.into(),
            description: description.into(),
            evidence: Vec::new(),
            metadata: Value::Null,
        });
    }

    /// Traces are append-only after recording. This method communicates the
    /// contract to callers; mutation is intentionally kept behind ownership.
    pub fn is_immutable_view(&self) -> bool { true }

    /// Returns a deterministic, serializable replay summary without granting
    /// the replay path authority to execute the selected alternative.
    pub fn replay_summary(&self, outcome: &DecisionOutcome) -> Result<DecisionReplay, DecisionError> {
        self.replay(outcome)
    }

    pub fn replay(&self, outcome: &DecisionOutcome) -> Result<DecisionReplay, DecisionError> {
        if self.decision_id != outcome.decision_id || self.trace_id != outcome.trace_id {
            return Err(DecisionError::InvalidRequest(
                "trace and outcome identities must match for replay".into(),
            ));
        }

        Ok(DecisionReplay {
            decision_id: outcome.decision_id,
            trace_id: self.trace_id,
            selected_id: outcome.selected.as_ref().map(|alternative| alternative.id.clone()),
            status: outcome.status.clone(),
            confidence: outcome.confidence,
            advisory_only: outcome.advisory_only,
            policy_reasons: outcome.policy_reasons.clone(),
            step_count: self.steps.len(),
        })
    }
}

impl DecisionTraceStore {
    pub fn record(&mut self, trace: DecisionTrace) -> Result<(), DecisionError> {
        if self.traces.contains_key(&trace.trace_id) {
            return Err(DecisionError::InvalidRequest("trace_id already recorded".into()));
        }
        self.traces.insert(trace.trace_id, trace);
        Ok(())
    }

    pub fn get(&self, trace_id: Uuid) -> Option<&DecisionTrace> {
        self.traces.get(&trace_id)
    }

    pub fn contains(&self, trace_id: Uuid) -> bool {
        self.traces.contains_key(&trace_id)
    }

    pub fn len(&self) -> usize {
        self.traces.len()
    }

    pub fn is_empty(&self) -> bool {
        self.traces.is_empty()
    }

    pub fn replay(
        &self,
        trace_id: Uuid,
        outcome: &DecisionOutcome,
    ) -> Result<DecisionReplay, DecisionError> {
        self.get(trace_id)
            .ok_or_else(|| DecisionError::InvalidRequest("trace does not exist".into()))?
            .replay(outcome)
    }
}

pub fn trace_from_request(
    request: &DecisionRequest,
    outcome: &DecisionOutcome,
) -> Result<DecisionTrace, DecisionError> {
    if request.decision_id != outcome.decision_id {
        return Err(DecisionError::InvalidRequest(
            "request and outcome identities must match".into(),
        ));
    }

    let mut trace = DecisionTrace {
        trace_id: outcome.trace_id,
        decision_id: outcome.decision_id,
        steps: Vec::new(),
    };
    trace.push("request", "captured immutable decision request identity");
    trace.push("outcome", "captured advisory decision outcome for replay");
    Ok(trace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Alternative;
    use serde_json::json;

    fn outcome() -> DecisionOutcome {
        DecisionOutcome {
            decision_id: Uuid::now_v7(),
            selected: Some(Alternative {
                id: "safe".into(),
                label: "Safe".into(),
                rationale: "policy compliant".into(),
                expected_value: 1.0,
                confidence: 0.9,
                constraints_satisfied: true,
                metadata: json!({}),
            }),
            status: DecisionStatus::Proposed,
            confidence: 0.9,
            policy_reasons: vec!["advisory".into()],
            advisory_only: true,
            trace_id: Uuid::now_v7(),
        }
    }

    #[test]
    fn replay_is_deterministic_for_the_same_trace_and_outcome() {
        let outcome = outcome();
        let mut trace = DecisionTrace {
            trace_id: outcome.trace_id,
            decision_id: outcome.decision_id,
            steps: Vec::new(),
        };
        trace.push("rank", "ranked candidates");
        let first = trace.replay(&outcome).unwrap();
        let second = trace.replay(&outcome).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn store_rejects_duplicate_trace_identity() {
        let outcome = outcome();
        let trace = DecisionTrace {
            trace_id: outcome.trace_id,
            decision_id: outcome.decision_id,
            steps: Vec::new(),
        };
        let mut store = DecisionTraceStore::default();
        store.record(trace.clone()).unwrap();
        assert!(store.record(trace).is_err());
        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());
    }
}

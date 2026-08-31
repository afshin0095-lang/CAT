#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod approval;
mod engine;
mod error;
mod model;
mod policy;
mod persistence;
mod trace;

pub use approval::{ApprovalGate, ApprovalRecord, ApprovalState};
pub use engine::DeterministicDecisionEngine;
pub use error::{DecisionError, DecisionResult};
pub use model::{Alternative, DecisionOutcome, DecisionRequest, DecisionStatus};
pub use persistence::DecisionTraceEventStore;
pub use policy::{DecisionPolicy, PolicyError, PolicyEvaluation};
pub use trace::{
    trace_from_request, DecisionReplay, DecisionTrace, DecisionTraceStep, DecisionTraceStore,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    fn request() -> DecisionRequest {
        DecisionRequest {
            decision_id: Uuid::now_v7(),
            objective: "select a safe growth option".into(),
            alternatives: vec![
                Alternative { id: "a".into(), label: "A".into(), rationale: "higher expected value".into(), expected_value: 0.91, confidence: 0.94, constraints_satisfied: true, metadata: json!({}) },
                Alternative { id: "b".into(), label: "B".into(), rationale: "lower risk".into(), expected_value: 0.70, confidence: 0.80, constraints_satisfied: true, metadata: json!({}) },
            ],
            required_confidence: 0.70,
            require_human_approval: false,
            context: json!({"domain": "affiliate"}),
        }
    }

    #[test]
    fn selects_highest_expected_value_deterministically() {
        let result = DeterministicDecisionEngine::default().decide(&request()).unwrap();
        assert_eq!(result.selected.unwrap().id, "a");
        assert_eq!(result.status, DecisionStatus::Proposed);
        assert!(result.advisory_only);
    }

    #[test]
    fn blocked_candidate_is_not_executed() {
        let mut policy = DecisionPolicy::default();
        policy.require_constraint_satisfaction = true;
        let engine = DeterministicDecisionEngine::new(policy).unwrap();
        let mut req = request();
        req.alternatives[0].constraints_satisfied = false;
        let result = engine.decide(&req).unwrap();
        assert_eq!(result.status, DecisionStatus::Rejected);
        assert!(result.selected.is_none());
        assert!(!result.policy_reasons.is_empty());
    }

    #[test]
    fn empty_objective_is_rejected() {
        let mut req = request();
        req.objective.clear();
        assert!(matches!(DeterministicDecisionEngine::default().decide(&req), Err(DecisionError::InvalidRequest(_))));
    }

    #[test]
    fn tie_breaking_is_stable_by_id() {
        let mut req = request();
        req.alternatives[0].expected_value = 0.8;
        req.alternatives[1].expected_value = 0.8;
        let result = DeterministicDecisionEngine::default().decide(&req).unwrap();
        assert_eq!(result.selected.unwrap().id, "a");
    }
}

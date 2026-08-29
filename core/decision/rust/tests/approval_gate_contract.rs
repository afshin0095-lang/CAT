use cat_decision::{Alternative, ApprovalGate, ApprovalState, DecisionOutcome, DecisionStatus};
use serde_json::json;
use uuid::Uuid;

fn proposed() -> DecisionOutcome {
    DecisionOutcome {
        decision_id: Uuid::now_v7(),
        selected: Some(Alternative {
            id: "candidate-safe".into(),
            label: "Safe candidate".into(),
            rationale: "deterministic selection".into(),
            expected_value: 42.0,
            confidence: 0.95,
            constraints_satisfied: true,
            metadata: json!({}),
        }),
        status: DecisionStatus::Proposed,
        confidence: 0.95,
        policy_reasons: vec!["approval required before execution".into()],
        advisory_only: true,
        trace_id: Uuid::now_v7(),
    }
}

#[test]
fn approval_is_idempotent_and_terminal() {
    let outcome = proposed();
    let mut gate = ApprovalGate::default();

    let first = gate.request(&outcome).unwrap();
    let second = gate.request(&outcome).unwrap();
    assert_eq!(first.approval_id, second.approval_id);
    assert_eq!(first.state, ApprovalState::Pending);

    let approved = gate.approve(outcome.decision_id, "human-reviewer").unwrap();
    assert_eq!(approved.state, ApprovalState::Approved);
    assert!(gate.approve(outcome.decision_id, "another-reviewer").is_err());
}

#[test]
fn rejected_decision_cannot_enter_approval() {
    let mut outcome = proposed();
    outcome.status = DecisionStatus::Rejected;
    outcome.selected = None;

    assert!(ApprovalGate::default().request(&outcome).is_err());
}

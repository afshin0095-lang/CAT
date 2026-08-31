use cat_decision::{
    Alternative, DecisionOutcome, DecisionStatus, DecisionTrace, DecisionTraceStore,
};
use serde_json::json;
use uuid::Uuid;

fn outcome() -> DecisionOutcome {
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
        policy_reasons: vec!["policy evaluated".into()],
        advisory_only: true,
        trace_id: Uuid::now_v7(),
    }
}

#[test]
fn recorded_trace_replays_the_same_advisory_outcome() {
    let outcome = outcome();
    let mut trace = DecisionTrace {
        trace_id: outcome.trace_id,
        decision_id: outcome.decision_id,
        steps: Vec::new(),
    };
    trace.push("validate", "validated immutable request");
    trace.push("rank", "ranked alternatives deterministically");

    let mut store = DecisionTraceStore::default();
    store.record(trace).unwrap();

    let replay = store.replay(outcome.trace_id, &outcome).unwrap();
    assert_eq!(replay.selected_id.as_deref(), Some("candidate-safe"));
    assert_eq!(replay.status, DecisionStatus::Proposed);
    assert!(replay.advisory_only);
    assert_eq!(replay.step_count, 2);
}

#[test]
fn replay_fails_closed_when_outcome_identity_does_not_match_trace() {
    let outcome = outcome();
    let trace = DecisionTrace {
        trace_id: outcome.trace_id,
        decision_id: outcome.decision_id,
        steps: Vec::new(),
    };
    let mut store = DecisionTraceStore::default();
    store.record(trace).unwrap();

    let mut tampered = outcome.clone();
    tampered.trace_id = Uuid::now_v7();
    assert!(store.replay(outcome.trace_id, &tampered).is_err());
}

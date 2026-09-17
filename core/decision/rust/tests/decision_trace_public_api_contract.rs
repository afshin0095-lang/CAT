use cat_decision::{
    Alternative, DecisionOutcome, DecisionRequest, DecisionStatus, DecisionTraceStore,
    trace_from_request,
};
use serde_json::json;
use uuid::Uuid;

fn request_and_outcome() -> (DecisionRequest, DecisionOutcome) {
    let decision_id = Uuid::now_v7();
    let trace_id = Uuid::now_v7();

    let request = DecisionRequest {
        decision_id,
        objective: "select a policy-safe alternative".into(),
        alternatives: vec![Alternative {
            id: "safe".into(),
            label: "Safe".into(),
            rationale: "meets policy constraints".into(),
            expected_value: 1.0,
            confidence: 0.9,
            constraints_satisfied: true,
            metadata: json!({}),
        }],
        required_confidence: 0.8,
        require_human_approval: false,
        context: json!({"domain": "affiliate"}),
    };

    let outcome = DecisionOutcome {
        decision_id,
        selected: request.alternatives.first().cloned(),
        status: DecisionStatus::Proposed,
        confidence: 0.9,
        policy_reasons: vec!["policy evaluated".into()],
        advisory_only: true,
        trace_id,
    };

    (request, outcome)
}

#[test]
fn public_api_builds_records_and_replays_without_execution_authority() {
    let (request, outcome) = request_and_outcome();
    let trace = trace_from_request(&request, &outcome).unwrap();

    assert_eq!(trace.steps.len(), 2);
    assert!(trace.is_immutable_view());

    let mut store = DecisionTraceStore::default();
    let trace_id = trace.trace_id;
    store.record(trace).unwrap();

    assert!(store.contains(trace_id));
    assert_eq!(store.len(), 1);

    let replay = store.replay(trace_id, &outcome).unwrap();
    assert_eq!(replay.selected_id.as_deref(), Some("safe"));
    assert!(replay.advisory_only);
    assert_eq!(replay.status, DecisionStatus::Proposed);
}

#[test]
fn public_api_rejects_request_outcome_identity_mismatch() {
    let (mut request, outcome) = request_and_outcome();
    request.decision_id = Uuid::now_v7();

    assert!(trace_from_request(&request, &outcome).is_err());
}

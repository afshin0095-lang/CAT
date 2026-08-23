use cat_reasoning::{DeterministicReasoningEngine, Evidence, ReasoningError, ReasoningMode, ReasoningPolicy, ReasoningRequest};
use serde_json::json;
use uuid::Uuid;

fn request(evidence: Vec<Evidence>) -> ReasoningRequest {
    ReasoningRequest {
        request_id: Uuid::now_v7(),
        objective: "choose the best-supported hypothesis".into(),
        context: json!({"suite": "contracts"}),
        evidence,
        mode: ReasoningMode::Deterministic,
        max_steps: 8,
    }
}

fn evidence(statement: &str, confidence: f32, authoritative: bool) -> Evidence {
    Evidence {
        evidence_id: Uuid::now_v7(),
        source: "contract-test".into(),
        statement: statement.into(),
        confidence,
        authoritative,
        metadata: json!({}),
    }
}

#[test]
fn rejects_empty_objective_and_empty_evidence() {
    let mut empty_objective = request(vec![evidence("A", 0.9, true)]);
    empty_objective.objective.clear();
    assert!(matches!(
        DeterministicReasoningEngine::default().reason(&empty_objective),
        Err(ReasoningError::InvalidRequest(_))
    ));

    let empty_evidence = request(Vec::new());
    assert!(matches!(
        DeterministicReasoningEngine::default().reason(&empty_evidence),
        Err(ReasoningError::EmptyEvidence)
    ));
}

#[test]
fn rejects_invalid_confidence_and_step_budget() {
    let invalid_confidence = request(vec![evidence("A", 1.1, true)]);
    assert!(matches!(
        DeterministicReasoningEngine::default().reason(&invalid_confidence),
        Err(ReasoningError::InvalidConfidence)
    ));

    let mut invalid_budget = request(vec![evidence("A", 0.9, true)]);
    invalid_budget.max_steps = 0;
    assert!(matches!(
        DeterministicReasoningEngine::default().reason(&invalid_budget),
        Err(ReasoningError::PolicyRejected(_))
    ));
}

#[test]
fn authoritative_evidence_outweighs_non_authoritative_evidence() {
    let req = request(vec![
        evidence("candidate-a", 0.8, true),
        evidence("candidate-b", 0.95, false),
    ]);

    let result = DeterministicReasoningEngine::default().reason(&req).unwrap();
    assert_eq!(result.hypotheses[0].statement, "candidate-a");
    assert!(result.advisory_only);
    assert_eq!(result.evidence_used.len(), 2);
}

#[test]
fn equal_confidence_hypotheses_have_stable_lexical_tie_breaking() {
    let req = request(vec![
        evidence("Zulu", 0.75, true),
        evidence("alpha", 0.75, true),
    ]);

    let result = DeterministicReasoningEngine::default().reason(&req).unwrap();
    assert_eq!(result.hypotheses[0].statement, "alpha");
    assert_eq!(result.hypotheses[1].statement, "zulu");
}

#[test]
fn custom_policy_caps_step_budget() {
    let policy = ReasoningPolicy {
        max_steps: 1,
        min_evidence_confidence: 0.5,
        require_authoritative_evidence_for_execution: true,
    };
    let req = request(vec![
        evidence("A", 0.9, true),
        evidence("B", 0.9, true),
    ]);

    let error = DeterministicReasoningEngine::new(policy).reason(&req).unwrap_err();
    assert!(matches!(error, ReasoningError::PolicyRejected(_)));
}

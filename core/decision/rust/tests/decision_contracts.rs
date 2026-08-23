use cat_decision::{ApprovalRequirement, DecisionCandidate, DecisionClass, DecisionEngine, DecisionPolicy, DecisionState};
use serde_json::json;

fn candidate(class: DecisionClass, confidence_bps: u16, risk_score_bps: u16) -> DecisionCandidate {
    let mut c = DecisionCandidate::new("default", class, "test decision", json!({"action":"rebalance"}));
    c.confidence_bps = confidence_bps;
    c.risk_score_bps = risk_score_bps;
    c.rationale = "contract test evidence".into();
    c
}

#[test]
fn advisory_decision_is_approved_without_human_gate() {
    let engine = DecisionEngine::new(DecisionPolicy::standard("default"));
    let record = engine.decide(candidate(DecisionClass::Advisory, 9000, 1000), "system").unwrap();
    assert_eq!(record.state, DecisionState::Approved);
    assert_eq!(record.approval, ApprovalRequirement::None);
    assert!(DecisionEngine::execute(&record).is_ok());
}

#[test]
fn high_impact_decision_requires_human_approval() {
    let engine = DecisionEngine::new(DecisionPolicy::standard("default"));
    let mut record = engine.decide(candidate(DecisionClass::HighImpact, 9500, 1000), "agent").unwrap();
    assert_eq!(record.state, DecisionState::Proposed);
    assert_eq!(record.approval, ApprovalRequirement::Human);
    assert!(DecisionEngine::execute(&record).is_err());
    engine.approve_human(&mut record, "human-operator").unwrap();
    assert_eq!(record.state, DecisionState::Approved);
    assert!(DecisionEngine::execute(&record).is_ok());
}

#[test]
fn policy_rejects_low_confidence_and_excessive_risk() {
    let engine = DecisionEngine::new(DecisionPolicy::standard("default"));
    assert!(engine.decide(candidate(DecisionClass::Operational, 6999, 1000), "agent").is_err());
    assert!(engine.decide(candidate(DecisionClass::Operational, 9000, 4001), "agent").is_err());
}

#[test]
fn policy_identity_must_match() {
    let engine = DecisionEngine::new(DecisionPolicy::standard("default"));
    let mut c = candidate(DecisionClass::Operational, 9000, 1000);
    c.policy_id = "other-policy".into();
    assert!(engine.decide(c, "agent").is_err());
}

#[test]
fn empty_human_approver_is_rejected() {
    let engine = DecisionEngine::new(DecisionPolicy::standard("default"));
    let mut record = engine.decide(candidate(DecisionClass::HighImpact, 9500, 1000), "agent").unwrap();
    assert!(engine.approve_human(&mut record, "  ").is_err());
}

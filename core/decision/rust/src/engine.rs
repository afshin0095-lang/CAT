use crate::{DecisionError, DecisionOutcome, DecisionPolicy, DecisionRequest, DecisionStatus, DecisionTrace};

#[derive(Clone, Debug)]
pub struct DeterministicDecisionEngine {
    policy: DecisionPolicy,
}

impl Default for DeterministicDecisionEngine {
    fn default() -> Self { Self { policy: DecisionPolicy::default() } }
}

impl DeterministicDecisionEngine {
    pub fn new(policy: DecisionPolicy) -> Result<Self, DecisionError> {
        policy.validate().map_err(DecisionError::Policy)?;
        Ok(Self { policy })
    }

    pub fn policy(&self) -> &DecisionPolicy { &self.policy }

    pub fn decide(&self, request: &DecisionRequest) -> Result<DecisionOutcome, DecisionError> {
        if request.objective.trim().is_empty() {
            return Err(DecisionError::InvalidRequest("objective must not be empty".into()));
        }
        if request.alternatives.is_empty() {
            return Err(DecisionError::InvalidRequest("at least one alternative is required".into()));
        }
        if !(0.0..=1.0).contains(&request.required_confidence) {
            return Err(DecisionError::InvalidRequest("required_confidence must be between 0 and 1".into()));
        }
        for alternative in &request.alternatives {
            if !(0.0..=1.0).contains(&alternative.confidence) {
                return Err(DecisionError::InvalidConfidence);
            }
            if !alternative.expected_value.is_finite() {
                return Err(DecisionError::InvalidRequest("expected_value must be finite".into()));
            }
        }

        let mut trace = DecisionTrace::new(request.decision_id);
        trace.push("validate", "validated objective, alternatives, confidence and finite values");

        let mut eligible: Vec<_> = request.alternatives.iter()
            .filter(|candidate| candidate.confidence >= request.required_confidence)
            .collect();
        if eligible.is_empty() {
            eligible = request.alternatives.iter().collect();
        }
        eligible.sort_by(|a, b| b.expected_value.total_cmp(&a.expected_value).then_with(|| a.id.cmp(&b.id)));
        trace.push("rank", "ranked candidates by expected value with deterministic ID tie-break");

        let mut policy_reasons = Vec::new();
        let mut selected_allowed = None;

        for candidate in eligible {
            let evaluation = self.policy.evaluate(request, candidate);
            if evaluation.allowed {
                trace.push("policy", "selected first highest-ranked alternative that satisfies policy");
                selected_allowed = Some((candidate.clone(), evaluation));
                break;
            }

            policy_reasons.extend(
                evaluation.reasons.into_iter().map(|reason| {
                    format!("alternative {} blocked: {reason}", candidate.id)
                }),
            );
        }

        let Some((selected, evaluation)) = selected_allowed else {
            trace.push("policy", "all ranked alternatives were blocked by policy");
            return Ok(DecisionOutcome {
                decision_id: request.decision_id,
                selected: None,
                status: DecisionStatus::Rejected,
                confidence: 0.0,
                policy_reasons,
                advisory_only: true,
                trace_id: trace.trace_id,
            });
        };

        if evaluation.requires_human_approval {
            trace.push("approval", "policy requires human approval before execution");
            policy_reasons.push("human approval is required before execution".into());
        }

        let confidence = selected.confidence.min(1.0);
        Ok(DecisionOutcome {
            decision_id: request.decision_id,
            selected: Some(selected),
            status: DecisionStatus::Proposed,
            confidence,
            policy_reasons,
            advisory_only: true,
            trace_id: trace.trace_id,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Alternative, DecisionPolicy};
    use serde_json::json;
    use uuid::Uuid;

    fn request() -> DecisionRequest {
        DecisionRequest {
            decision_id: Uuid::now_v7(),
            objective: "choose a compliant alternative".into(),
            alternatives: vec![
                Alternative {
                    id: "blocked-high".into(),
                    label: "Blocked".into(),
                    rationale: "highest value but violates a constraint".into(),
                    expected_value: 100.0,
                    confidence: 0.95,
                    constraints_satisfied: false,
                    metadata: json!({}),
                },
                Alternative {
                    id: "allowed-lower".into(),
                    label: "Allowed".into(),
                    rationale: "lower value and policy compliant".into(),
                    expected_value: 80.0,
                    confidence: 0.90,
                    constraints_satisfied: true,
                    metadata: json!({}),
                },
            ],
            required_confidence: 0.70,
            require_human_approval: false,
            context: json!({}),
        }
    }

    #[test]
    fn falls_back_to_next_ranked_policy_compliant_alternative() {
        let outcome = DeterministicDecisionEngine::default().decide(&request()).unwrap();
        assert_eq!(outcome.status, DecisionStatus::Proposed);
        assert_eq!(outcome.selected.unwrap().id, "allowed-lower");
        assert!(outcome.policy_reasons.iter().any(|reason| reason.contains("blocked-high")));
    }

    #[test]
    fn rejects_only_when_all_ranked_alternatives_are_blocked() {
        let mut req = request();
        req.alternatives[1].constraints_satisfied = false;
        let outcome = DeterministicDecisionEngine::default().decide(&req).unwrap();
        assert_eq!(outcome.status, DecisionStatus::Rejected);
        assert!(outcome.selected.is_none());
        assert_eq!(outcome.confidence, 0.0);
    }

    #[test]
    fn records_human_approval_requirement_without_executing() {
        let policy = DecisionPolicy {
            require_human_approval_above_value: Some(50.0),
            ..DecisionPolicy::default()
        };
        let outcome = DeterministicDecisionEngine::new(policy)
            .unwrap()
            .decide(&request())
            .unwrap();
        assert_eq!(outcome.status, DecisionStatus::Proposed);
        assert!(outcome.policy_reasons.iter().any(|reason| reason.contains("human approval")));
        assert!(outcome.advisory_only);
    }
}

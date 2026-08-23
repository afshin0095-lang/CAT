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

        let selected = eligible.into_iter().next().cloned();
        let selected = match selected {
            Some(value) => value,
            None => return Err(DecisionError::NoCandidate),
        };
        let evaluation = self.policy.evaluate(request, &selected);
        trace.push("policy", if evaluation.allowed { "selected alternative satisfies policy" } else { "selected alternative blocked by policy" });

        let confidence = selected.confidence.min(1.0);
        let status = if !evaluation.allowed {
            DecisionStatus::Rejected
        } else {
            DecisionStatus::Proposed
        };

        Ok(DecisionOutcome {
            decision_id: request.decision_id,
            selected: evaluation.allowed.then_some(selected),
            status,
            confidence,
            policy_reasons: evaluation.reasons,
            advisory_only: true,
            trace_id: trace.trace_id,
        })
    }
}

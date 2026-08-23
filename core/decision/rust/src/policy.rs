use crate::model::{ApprovalRequirement, DecisionCandidate, DecisionClass};

#[derive(Clone, Debug)]
pub struct DecisionPolicy {
    pub id: String,
    pub minimum_confidence_bps: u16,
    pub maximum_risk_bps: u16,
    pub high_impact_requires_human: bool,
}

impl DecisionPolicy {
    pub fn standard(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            minimum_confidence_bps: 7_000,
            maximum_risk_bps: 4_000,
            high_impact_requires_human: true,
        }
    }

    pub fn evaluate(&self, candidate: &DecisionCandidate) -> PolicyDecision {
        if candidate.policy_id != self.id {
            return PolicyDecision::Rejected("policy_mismatch".into());
        }
        if candidate.confidence_bps < self.minimum_confidence_bps {
            return PolicyDecision::Rejected("insufficient_confidence".into());
        }
        if candidate.risk_score_bps > self.maximum_risk_bps {
            return PolicyDecision::Rejected("risk_limit_exceeded".into());
        }

        let approval = match candidate.class {
            DecisionClass::Advisory => ApprovalRequirement::None,
            DecisionClass::Operational => ApprovalRequirement::Policy,
            DecisionClass::HighImpact if self.high_impact_requires_human => ApprovalRequirement::Human,
            DecisionClass::HighImpact => ApprovalRequirement::Policy,
        };

        PolicyDecision::Allowed(approval)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolicyDecision {
    Allowed(ApprovalRequirement),
    Rejected(String),
}

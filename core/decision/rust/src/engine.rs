use crate::model::{ApprovalRequirement, DecisionRecord, DecisionState};
use crate::policy::{DecisionPolicy, PolicyDecision};
use crate::DecisionError;
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DecisionEngine {
    policy: DecisionPolicy,
}

impl DecisionEngine {
    pub fn new(policy: DecisionPolicy) -> Self {
        Self { policy }
    }

    pub fn decide(&self, candidate: crate::model::DecisionCandidate, actor: impl Into<String>) -> Result<DecisionRecord, DecisionError> {
        let actor = actor.into();
        match self.policy.evaluate(&candidate) {
            PolicyDecision::Rejected(reason) => Err(DecisionError::Rejected(reason)),
            PolicyDecision::Allowed(approval) => Ok(DecisionRecord {
                decision_id: Uuid::now_v7(),
                candidate_id: candidate.id,
                state: match approval {
                    ApprovalRequirement::Human => DecisionState::Proposed,
                    _ => DecisionState::Approved,
                },
                approval,
                policy_id: candidate.policy_id,
                actor,
                reason: candidate.rationale,
                action: candidate.proposed_action,
            }),
        }
    }

    pub fn approve_human(&self, record: &mut DecisionRecord, approver: impl Into<String>) -> Result<(), DecisionError> {
        if record.approval != ApprovalRequirement::Human {
            return Err(DecisionError::InvalidTransition("human approval not required".into()));
        }
        let approver = approver.into();
        if approver.trim().is_empty() {
            return Err(DecisionError::InvalidTransition("approver identity is required".into()));
        }
        record.actor = approver;
        record.state = DecisionState::Approved;
        Ok(())
    }

    pub fn execute(record: &DecisionRecord) -> Result<&Value, DecisionError> {
        if record.state != DecisionState::Approved {
            return Err(DecisionError::InvalidTransition("decision must be approved before execution".into()));
        }
        Ok(&record.action)
    }
}

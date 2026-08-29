use crate::{DecisionError, DecisionOutcome, DecisionStatus};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApprovalState {
    Pending,
    Approved,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovalRecord {
    pub approval_id: Uuid,
    pub decision_id: Uuid,
    pub state: ApprovalState,
    pub actor_id: Option<String>,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct ApprovalGate {
    records: HashMap<Uuid, ApprovalRecord>,
}

impl ApprovalGate {
    pub fn request(&mut self, outcome: &DecisionOutcome) -> Result<ApprovalRecord, DecisionError> {
        if outcome.status == DecisionStatus::Rejected {
            return Err(DecisionError::InvalidRequest(
                "rejected decisions cannot enter approval".into(),
            ));
        }

        if let Some(existing) = self.records.get(&outcome.decision_id) {
            return Ok(existing.clone());
        }

        let record = ApprovalRecord {
            approval_id: Uuid::now_v7(),
            decision_id: outcome.decision_id,
            state: ApprovalState::Pending,
            actor_id: None,
            reason: None,
        };
        self.records.insert(outcome.decision_id, record.clone());
        Ok(record)
    }

    pub fn approve(
        &mut self,
        decision_id: Uuid,
        actor_id: impl Into<String>,
    ) -> Result<ApprovalRecord, DecisionError> {
        self.transition(decision_id, ApprovalState::Approved, actor_id.into(), None)
    }

    pub fn reject(
        &mut self,
        decision_id: Uuid,
        actor_id: impl Into<String>,
        reason: impl Into<String>,
    ) -> Result<ApprovalRecord, DecisionError> {
        self.transition(
            decision_id,
            ApprovalState::Rejected,
            actor_id.into(),
            Some(reason.into()),
        )
    }

    pub fn get(&self, decision_id: Uuid) -> Option<&ApprovalRecord> {
        self.records.get(&decision_id)
    }

    fn transition(
        &mut self,
        decision_id: Uuid,
        target: ApprovalState,
        actor_id: String,
        reason: Option<String>,
    ) -> Result<ApprovalRecord, DecisionError> {
        let record = self.records.get_mut(&decision_id).ok_or_else(|| {
            DecisionError::InvalidRequest("approval record does not exist".into())
        })?;

        if record.state != ApprovalState::Pending {
            return Err(DecisionError::InvalidRequest(
                "approval is terminal and cannot be transitioned again".into(),
            ));
        }

        record.state = target;
        record.actor_id = Some(actor_id);
        record.reason = reason;
        Ok(record.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Alternative, DecisionOutcome};
    use serde_json::json;

    fn outcome() -> DecisionOutcome {
        DecisionOutcome {
            decision_id: Uuid::now_v7(),
            selected: Some(Alternative {
                id: "safe".into(),
                label: "Safe".into(),
                rationale: "policy compliant".into(),
                expected_value: 1.0,
                confidence: 0.9,
                constraints_satisfied: true,
                metadata: json!({}),
            }),
            status: DecisionStatus::Proposed,
            confidence: 0.9,
            policy_reasons: vec!["human approval is required before execution".into()],
            advisory_only: true,
            trace_id: Uuid::now_v7(),
        }
    }

    #[test]
    fn request_is_idempotent_per_decision() {
        let result = outcome();
        let mut gate = ApprovalGate::default();
        let first = gate.request(&result).unwrap();
        let second = gate.request(&result).unwrap();
        assert_eq!(first.approval_id, second.approval_id);
        assert_eq!(first.state, ApprovalState::Pending);
    }

    #[test]
    fn terminal_approval_cannot_be_replayed_as_new_transition() {
        let result = outcome();
        let mut gate = ApprovalGate::default();
        gate.request(&result).unwrap();
        let approved = gate.approve(result.decision_id, "operator-1").unwrap();
        assert_eq!(approved.state, ApprovalState::Approved);
        assert!(gate.reject(result.decision_id, "operator-2", "late").is_err());
    }
}

use crate::model::{Alternative, DecisionRequest};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct DecisionPolicy {
    pub minimum_confidence: f64,
    pub require_constraint_satisfaction: bool,
    pub require_human_approval_above_value: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyEvaluation {
    pub allowed: bool,
    pub requires_human_approval: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Error, PartialEq)]
pub enum PolicyError {
    #[error("minimum confidence must be between 0 and 1")]
    InvalidConfidence,
    #[error("human approval threshold must be finite")]
    InvalidThreshold,
}

impl Default for DecisionPolicy {
    fn default() -> Self {
        Self {
            minimum_confidence: 0.70,
            require_constraint_satisfaction: true,
            require_human_approval_above_value: None,
        }
    }
}

impl DecisionPolicy {
    pub fn validate(&self) -> Result<(), PolicyError> {
        if !(0.0..=1.0).contains(&self.minimum_confidence) {
            return Err(PolicyError::InvalidConfidence);
        }
        if self
            .require_human_approval_above_value
            .is_some_and(|v| !v.is_finite())
        {
            return Err(PolicyError::InvalidThreshold);
        }
        Ok(())
    }

    pub fn evaluate(
        &self,
        request: &DecisionRequest,
        alternative: &Alternative,
    ) -> PolicyEvaluation {
        let mut reasons = Vec::new();
        if alternative.confidence < self.minimum_confidence {
            reasons.push(format!(
                "confidence {} is below policy minimum {}",
                alternative.confidence, self.minimum_confidence
            ));
        }
        if self.require_constraint_satisfaction && !alternative.constraints_satisfied {
            reasons.push("alternative violates one or more policy constraints".into());
        }
        let threshold_gate = self
            .require_human_approval_above_value
            .is_some_and(|threshold| alternative.expected_value >= threshold);
        let requires_human_approval = request.require_human_approval || threshold_gate;
        PolicyEvaluation {
            allowed: reasons.is_empty(),
            requires_human_approval,
            reasons,
        }
    }
}

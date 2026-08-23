use crate::{ReasoningError, ReasoningRequest, ReasoningResult};

#[derive(Clone, Debug)]
pub struct ReasoningPolicy {
    pub max_steps: u16,
    pub min_evidence_confidence: f32,
    pub require_authoritative_evidence_for_execution: bool,
}

impl Default for ReasoningPolicy {
    fn default() -> Self {
        Self {
            max_steps: 64,
            min_evidence_confidence: 0.50,
            require_authoritative_evidence_for_execution: true,
        }
    }
}

impl ReasoningPolicy {
    pub fn validate(&self, request: &ReasoningRequest) -> ReasoningResult<()> {
        if request.objective.trim().is_empty() {
            return Err(ReasoningError::InvalidRequest("objective is empty".into()));
        }
        if request.evidence.is_empty() {
            return Err(ReasoningError::EmptyEvidence);
        }
        if request.max_steps == 0 || request.max_steps > self.max_steps {
            return Err(ReasoningError::PolicyRejected(format!(
                "max_steps must be between 1 and {}",
                self.max_steps
            )));
        }
        for evidence in &request.evidence {
            if !(0.0..=1.0).contains(&evidence.confidence) {
                return Err(ReasoningError::InvalidConfidence);
            }
        }
        Ok(())
    }
}

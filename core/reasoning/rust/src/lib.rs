#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod engine;
mod error;
mod model;
mod policy;
mod trace;

pub use engine::DeterministicReasoningEngine;
pub use error::{ReasoningError, ReasoningOutcome};
pub use model::{
    Evidence, Hypothesis, ReasoningMode, ReasoningRequest, ReasoningResult, ReasoningStep,
};
pub use policy::ReasoningPolicy;
pub use trace::ReasoningTrace;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;

    fn request() -> ReasoningRequest {
        ReasoningRequest {
            request_id: Uuid::now_v7(),
            objective: "select the best-supported hypothesis".into(),
            context: json!({"domain": "test"}),
            evidence: vec![
                Evidence {
                    evidence_id: Uuid::now_v7(),
                    source: "source-a".into(),
                    statement: "candidate-a is supported".into(),
                    confidence: 0.90,
                    authoritative: true,
                    metadata: json!({}),
                },
                Evidence {
                    evidence_id: Uuid::now_v7(),
                    source: "source-b".into(),
                    statement: "candidate-b is supported".into(),
                    confidence: 0.40,
                    authoritative: false,
                    metadata: json!({}),
                },
            ],
            mode: ReasoningMode::Deterministic,
            max_steps: 8,
        }
    }

    #[test]
    fn deterministic_engine_selects_best_supported_statement() {
        let result = DeterministicReasoningEngine::default()
            .reason(&request())
            .unwrap();
        assert!(result.confidence > 0.5);
        assert!(result.conclusion.contains("candidate-a"));
        assert!(result.advisory_only);
        assert_eq!(result.evidence_used.len(), 2);
    }

    #[test]
    fn policy_rejects_empty_objective() {
        let mut req = request();
        req.objective.clear();
        let error = DeterministicReasoningEngine::default()
            .reason(&req)
            .unwrap_err();
        assert!(matches!(error, ReasoningError::InvalidRequest(_)));
    }

    #[test]
    fn policy_rejects_invalid_confidence() {
        let mut req = request();
        req.evidence[0].confidence = 1.5;
        let error = DeterministicReasoningEngine::default()
            .reason(&req)
            .unwrap_err();
        assert!(matches!(error, ReasoningError::InvalidConfidence));
    }
}

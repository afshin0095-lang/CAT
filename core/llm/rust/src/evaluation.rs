use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{LlmError, PromptId, PromptVersion, RenderedPrompt};

/// An evaluation result is derived evidence about prompt/model behavior. It
/// must never be interpreted as canonical business truth.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PromptEvaluation {
    pub prompt_id: PromptId,
    pub prompt_version: PromptVersion,
    pub evaluator: String,
    pub passed: bool,
    pub score: Option<f64>,
    pub rationale: String,
}

impl PromptEvaluation {
    pub fn validate(&self) -> Result<(), LlmError> {
        if self.evaluator.trim().is_empty() {
            return Err(LlmError::InvalidEvaluation("evaluator must not be empty".to_owned()));
        }
        if self.rationale.trim().is_empty() {
            return Err(LlmError::InvalidEvaluation("rationale must not be empty".to_owned()));
        }
        if let Some(score) = self.score {
            if !score.is_finite() || !(0.0..=1.0).contains(&score) {
                return Err(LlmError::InvalidEvaluation(
                    "score must be finite and within the inclusive range [0, 1]".to_owned(),
                ));
            }
        }
        Ok(())
    }
}

/// Provider-neutral evaluation boundary. Implementations consume rendered
/// prompts and model output but cannot mutate prompt definitions or canonical
/// domain records through this contract.
#[async_trait]
pub trait PromptEvaluator: Send + Sync {
    fn id(&self) -> &str;

    async fn evaluate(
        &self,
        prompt: &RenderedPrompt,
        output: &str,
    ) -> Result<PromptEvaluation, LlmError>;
}

/// Deterministic baseline evaluator for local contracts and integration tests.
/// Production evaluators may apply richer policy or model-based assessment.
#[derive(Clone, Debug, Default)]
pub struct NonEmptyOutputEvaluator;

#[async_trait]
impl PromptEvaluator for NonEmptyOutputEvaluator {
    fn id(&self) -> &str { "cat.non_empty_output.v1" }

    async fn evaluate(
        &self,
        prompt: &RenderedPrompt,
        output: &str,
    ) -> Result<PromptEvaluation, LlmError> {
        let passed = !output.trim().is_empty();
        let evaluation = PromptEvaluation {
            prompt_id: prompt.id.clone(),
            prompt_version: prompt.version,
            evaluator: self.id().to_owned(),
            passed,
            score: Some(if passed { 1.0 } else { 0.0 }),
            rationale: if passed {
                "output contains non-whitespace content".to_owned()
            } else {
                "output is empty or whitespace only".to_owned()
            },
        };
        evaluation.validate()?;
        Ok(evaluation)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::{PromptId, PromptTemplate, SafetyClass};

    fn prompt() -> RenderedPrompt {
        PromptTemplate::new(
            PromptId::new("cat.test"),
            PromptVersion(1),
            "Hello {{name}}",
            ["name".to_owned()],
            SafetyClass::Standard,
        )
        .unwrap()
        .render(&BTreeMap::from([("name".to_owned(), "CAT".to_owned())]))
        .unwrap()
    }

    #[tokio::test]
    async fn baseline_evaluation_is_derived_and_bounded() {
        let evaluation = NonEmptyOutputEvaluator.evaluate(&prompt(), "derived answer").await.unwrap();
        assert!(evaluation.passed);
        assert_eq!(evaluation.score, Some(1.0));
        evaluation.validate().unwrap();
    }

    #[test]
    fn invalid_scores_are_rejected() {
        let evaluation = PromptEvaluation {
            prompt_id: PromptId::new("cat.test"),
            prompt_version: PromptVersion(1),
            evaluator: "test".to_owned(),
            passed: true,
            score: Some(1.5),
            rationale: "invalid".to_owned(),
        };
        assert!(matches!(evaluation.validate(), Err(LlmError::InvalidEvaluation(_))));
    }
}

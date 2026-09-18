use crate::{
    Hypothesis, ReasoningError, ReasoningOutcome, ReasoningPolicy, ReasoningRequest,
    ReasoningResult, ReasoningStep,
};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct DeterministicReasoningEngine {
    pub policy: ReasoningPolicy,
}

impl DeterministicReasoningEngine {
    pub fn new(policy: ReasoningPolicy) -> Self {
        Self { policy }
    }

    pub fn reason(&self, request: &ReasoningRequest) -> ReasoningOutcome<ReasoningResult> {
        self.policy.validate(request)?;
        let mut steps = Vec::new();
        let mut hypotheses = Vec::new();
        let mut support_by_statement: BTreeMap<String, (f32, f32, Vec<Uuid>, bool)> =
            BTreeMap::new();
        for evidence in &request.evidence {
            let key = evidence.statement.trim().to_lowercase();
            let entry = support_by_statement
                .entry(key)
                .or_insert((0.0, 0.0, Vec::new(), false));
            entry.0 += if evidence.authoritative {
                evidence.confidence
            } else {
                evidence.confidence * 0.75
            };
            entry.1 += 1.0 - evidence.confidence;
            entry.2.push(evidence.evidence_id);
            entry.3 |= evidence.authoritative;
        }
        for (statement, (support, contradiction, evidence_ids, _authoritative)) in support_by_statement {
            let confidence = if support + contradiction > 0.0 {
                support / (support + contradiction)
            } else {
                0.0
            };
            hypotheses.push(Hypothesis {
                hypothesis_id: Uuid::now_v7(),
                statement: statement.clone(),
                support,
                contradiction,
                evidence_ids: evidence_ids.clone(),
            });
            steps.push(ReasoningStep {
                step_id: Uuid::now_v7(),
                sequence: steps.len() as u16 + 1,
                operation: "evidence_aggregation".into(),
                input_ids: evidence_ids,
                output: format!("hypothesis confidence={confidence:.4}"),
                confidence,
            });
        }
        if steps.len() as u16 > request.max_steps {
            return Err(ReasoningError::PolicyRejected(
                "reasoning step budget exceeded".into(),
            ));
        }
        let authoritative_statements: BTreeMap<_, _> = request
            .evidence
            .iter()
            .fold(BTreeMap::new(), |mut statements, evidence| {
                let key = evidence.statement.trim().to_lowercase();
                statements
                    .entry(key)
                    .and_modify(|authoritative| *authoritative |= evidence.authoritative)
                    .or_insert(evidence.authoritative);
                statements
            });
        hypotheses.sort_by(|a, b| {
            let ca = a.support / (a.support + a.contradiction).max(f32::EPSILON);
            let cb = b.support / (b.support + b.contradiction).max(f32::EPSILON);
            authoritative_statements
                .get(&b.statement)
                .cmp(&authoritative_statements.get(&a.statement))
                .then_with(|| cb.total_cmp(&ca))
                .then_with(|| a.statement.cmp(&b.statement))
        });
        let top = hypotheses.first().ok_or(ReasoningError::EmptyEvidence)?;
        let confidence = top.support / (top.support + top.contradiction).max(f32::EPSILON);
        Ok(ReasoningResult {
            reasoning_id: Uuid::now_v7(),
            request_id: request.request_id,
            conclusion: format!("Best-supported hypothesis: {}", top.statement),
            confidence,
            hypotheses,
            steps,
            evidence_used: request.evidence.iter().map(|e| e.evidence_id).collect(),
            assumptions: vec!["Evidence weights are treated as independent signals.".into()],
            advisory_only: true,
        })
    }
}

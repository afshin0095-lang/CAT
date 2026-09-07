#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod approval;
mod engine;
mod error;
mod durable_persistence;
mod event_publication;
mod model;
mod policy;
mod persistence;
mod trace;
pub mod ab_testing;

pub use approval::{ApprovalGate, ApprovalRecord, ApprovalState};
pub use engine::DeterministicDecisionEngine;
pub use error::{DecisionError, DecisionResult};
pub use durable_persistence::{DurableDecisionTraceEventStore, DurableDecisionTracePersistenceError};
pub use event_publication::DecisionTraceEventPublisher;
pub use model::{Alternative, DecisionOutcome, DecisionRequest, DecisionStatus};
pub use persistence::DecisionTraceEventStore;
pub use policy::{DecisionPolicy, PolicyError, PolicyEvaluation};
pub use trace::{trace_from_request, DecisionReplay, DecisionTrace, DecisionTraceStep, DecisionTraceStore};
pub use ab_testing::{ABTest, TestRunner, Variant, VariantMetrics};

#[cfg(test)]
mod tests { use super::*; use serde_json::json; use uuid::Uuid; fn request() -> DecisionRequest { DecisionRequest { decision_id: Uuid::now_v7(), objective: "select a safe growth option".into(), alternatives: vec![Alternative { id: "a".into(), label: "A".into(), rationale: "higher expected value".into(), expected_value: 0.91, confidence: 0.94, constraints_satisfied: true, metadata: json!({}) }, Alternative { id: "b".into(), label: "B".into(), rationale: "lower risk".into(), expected_value: 0.70, confidence: 0.80, constraints_satisfied: true, metadata: json!({}) }], required_confidence: 0.70, require_human_approval: false, context: json!({"domain":"affiliate"}) } } #[test] fn selects_highest_expected_value_deterministically() { let result = DeterministicDecisionEngine::default().decide(&request()).unwrap(); assert_eq!(result.selected.unwrap().id, "a"); } }

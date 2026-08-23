mod engine;
mod model;
mod policy;

use thiserror::Error;

pub use engine::DecisionEngine;
pub use model::{ApprovalRequirement, DecisionCandidate, DecisionClass, DecisionRecord, DecisionState};
pub use policy::{DecisionPolicy, PolicyDecision};

#[derive(Debug, Error)]
pub enum DecisionError {
    #[error("decision rejected: {0}")]
    Rejected(String),
    #[error("invalid decision transition: {0}")]
    InvalidTransition(String),
}

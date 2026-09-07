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
pub mod volume_mode;

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
pub use volume_mode::{VolumeLedger, VolumeMode};

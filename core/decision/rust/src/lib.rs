#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod ab_testing;
mod approval;
mod durable_persistence;
mod engine;
mod error;
mod event_publication;
mod model;
mod persistence;
mod policy;
pub mod self_improver;
mod trace;
pub mod volume_mode;

pub use ab_testing::{ABTest, TestRunner, Variant, VariantMetrics};
pub use approval::{ApprovalGate, ApprovalRecord, ApprovalState};
pub use durable_persistence::{
    DurableDecisionTraceEventStore, DurableDecisionTracePersistenceError,
};
pub use engine::DeterministicDecisionEngine;
pub use error::{DecisionError, DecisionResult};
pub use event_publication::DecisionTraceEventPublisher;
pub use model::{Alternative, DecisionOutcome, DecisionRequest, DecisionStatus};
pub use persistence::DecisionTraceEventStore;
pub use policy::{DecisionPolicy, PolicyError, PolicyEvaluation};
pub use self_improver::{ExperimentOutcome, ImprovementProposal, SelfImprover};
pub use trace::{
    DecisionReplay, DecisionTrace, DecisionTraceStep, DecisionTraceStore, trace_from_request,
};
pub use volume_mode::{VolumeLedger, VolumeMode};

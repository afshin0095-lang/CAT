#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod cancellation;
mod execution;
mod health;
mod lifecycle;
mod runtime;
mod task;

pub mod execution_gate;
pub mod task_lease;

pub use cancellation::CancellationToken;
pub use execution::{ExecutionContext, ExecutionError, ExecutionState};
pub use execution_gate::{DispatchDecision, ExecutionGate};
pub use health::{HealthReport, HealthStatus, RuntimeHealth};
pub use lifecycle::{LifecycleError, LifecyclePhase, LifecycleState};
pub use runtime::{Runtime, RuntimeConfig, RuntimeError};
pub use task::{TaskId, TaskKind, TaskSpec};
pub use task_lease::{LeaseError, TaskLease};

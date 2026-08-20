#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod runtime;

pub use runtime::{lifecycle_event_type, Runtime, RuntimeDraining, RuntimeError, RuntimeResult, RuntimeStarted, RuntimeState, RuntimeStopped};

/// Stable application-runtime facade used by higher CAT layers.
///
/// The runtime owns lifecycle state and delegates event delivery to the EventBus;
/// it does not own domain truth, persistence, monetary calculations, or policy
/// decisions. Those concerns remain in their respective bounded contexts.
pub mod prelude {
    pub use crate::{Runtime, RuntimeError, RuntimeResult, RuntimeState};
    pub use cat_eventbus::{EventBus, EventBusResult, EventEnvelope, EventHandler, PublishOutcome, SubscriptionId};
}

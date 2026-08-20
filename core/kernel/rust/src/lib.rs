#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod clock;
pub mod context;
pub mod envelope;
pub mod ids;
pub mod result;
pub mod sequence;
pub mod time;
pub mod validation;

pub use clock::{Clock, FixedClock, SystemClock};
pub use context::ExecutionContext;
pub use envelope::EventEnvelope;
pub use ids::{CausationId, CorrelationId, EntityId, EventId, TenantId};
pub use result::{KernelError, KernelResult};
pub use sequence::SequenceNumber;
pub use time::TimestampMs;
pub use validation::{parse_uuid, require_non_nil};
pub use uuid::Uuid;

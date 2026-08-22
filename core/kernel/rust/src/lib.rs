#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod clock;
pub mod context;
pub mod envelope;
pub mod event_store;
pub mod idempotency;
pub mod ids;
pub mod projection;
pub mod replay;
pub mod result;
pub mod sequence;
pub mod time;
pub mod validation;

pub use clock::{Clock, FixedClock, SystemClock};
pub use context::ExecutionContext;
pub use envelope::EventEnvelope;
pub use event_store::{AppendReceipt, EventStore, ExpectedVersion, StoredEvent};
pub use idempotency::{IdempotencyKey, IdempotencyLedger, IdempotencyReceipt};
pub use ids::{CausationId, CorrelationId, EntityId, EventId, TenantId};
pub use projection::{ProjectionCheckpoint, ProjectionPhase};
pub use replay::replay;
pub use result::{KernelError, KernelResult};
pub use sequence::SequenceNumber;
pub use time::TimestampMs;
pub use validation::{parse_uuid, require_non_nil};
pub use uuid::Uuid;

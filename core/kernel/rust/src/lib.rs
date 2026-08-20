#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod clock;
pub mod context;
pub mod ids;
pub mod result;
pub mod time;
pub mod validation;

pub use clock::{Clock, FixedClock, SystemClock};
pub use context::ExecutionContext;
pub use ids::{CausationId, CorrelationId, EntityId, TenantId};
pub use result::{KernelError, KernelResult};
pub use time::TimestampMs;
pub use validation::{parse_uuid, require_non_nil};
pub use uuid::Uuid;

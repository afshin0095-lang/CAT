#![forbid(unsafe_code)]
#![deny(clippy::all)]

pub mod ids;
pub mod result;
pub mod time;

pub use ids::EntityId;
pub use result::{KernelError, KernelResult};

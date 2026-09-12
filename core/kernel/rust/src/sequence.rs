use serde::{Deserialize, Serialize};

use crate::{KernelError, KernelResult};

/// Monotonic position within a durable event stream.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub const ZERO: Self = Self(0);

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub fn next(self) -> KernelResult<Self> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or_else(|| KernelError::InvalidTimestamp("sequence number overflow".to_owned()))
    }

    pub fn checked_after(self, previous: Self) -> KernelResult<Self> {
        if self.0 <= previous.0 {
            return Err(KernelError::InvalidTimestamp(
                "sequence must be strictly after the previous sequence".to_owned(),
            ));
        }
        Ok(self)
    }
}

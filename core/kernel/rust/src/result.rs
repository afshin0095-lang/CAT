use thiserror::Error;

use crate::SequenceNumber;

pub type KernelResult<T> = Result<T, KernelError>;

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("concurrency conflict: expected sequence {expected:?}, actual sequence {actual:?}")]
    ConcurrencyConflict {
        expected: SequenceNumber,
        actual: SequenceNumber,
    },

    #[error("sequence conflict: expected sequence {expected:?}, actual sequence {actual:?}")]
    SequenceConflict {
        expected: SequenceNumber,
        actual: SequenceNumber,
    },
}

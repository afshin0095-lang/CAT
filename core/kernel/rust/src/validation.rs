use uuid::Uuid;

use crate::{KernelError, KernelResult};

/// Parse a UUID while preserving a kernel-owned error boundary.
pub fn parse_uuid(value: &str) -> KernelResult<Uuid> {
    Uuid::parse_str(value).map_err(|error| KernelError::InvalidIdentifier(error.to_string()))
}

/// Reject an all-zero UUID at the kernel boundary.
pub fn require_non_nil(value: Uuid) -> KernelResult<Uuid> {
    if value.is_nil() {
        return Err(KernelError::InvalidIdentifier(
            "identifier must not be nil".to_owned(),
        ));
    }
    Ok(value)
}

use crate::{Classification, LifecycleState, MemoryObject, MemoryValidationError};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MemoryOperation {
    Create = 1,
    Update = 2,
    Revoke = 3,
    Expire = 4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPolicy {
    pub minimum_classification: Classification,
    pub allow_restricted: bool,
    pub allow_secret: bool,
    pub require_consent: bool,
}

impl Default for MemoryPolicy {
    fn default() -> Self {
        Self {
            minimum_classification: Classification::Public,
            allow_restricted: true,
            allow_secret: false,
            require_consent: true,
        }
    }
}

impl MemoryPolicy {
    pub fn authorize(&self, object: &MemoryObject, operation: MemoryOperation) -> Result<(), MemoryValidationError> {
        if object.classification == Classification::Secret && !self.allow_secret {
            return Err(MemoryValidationError::ConsentRequired);
        }
        if object.classification == Classification::Restricted && !self.allow_restricted {
            return Err(MemoryValidationError::ConsentRequired);
        }
        if self.require_consent && object.consent.required && !object.consent.granted {
            return Err(MemoryValidationError::ConsentRequired);
        }
        if operation == MemoryOperation::Update && object.state == LifecycleState::Revoked {
            return Err(MemoryValidationError::LegalHoldExpired);
        }
        Ok(())
    }
}

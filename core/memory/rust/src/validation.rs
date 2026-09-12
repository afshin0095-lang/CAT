use crate::{LifecycleState, MemoryObject, MemoryStore, MemoryValidationError};

pub fn validate_transition(
    from: LifecycleState,
    to: LifecycleState,
) -> Result<(), MemoryValidationError> {
    let allowed = matches!(
        (from, to),
        (LifecycleState::Proposed, LifecycleState::Validated)
            | (LifecycleState::Validated, LifecycleState::Active)
            | (LifecycleState::Active, LifecycleState::Superseded)
            | (LifecycleState::Active, LifecycleState::Retained)
            | (LifecycleState::Active, LifecycleState::Expired)
            | (LifecycleState::Active, LifecycleState::Revoked)
            | (LifecycleState::Retained, LifecycleState::Expired)
            | (LifecycleState::Superseded, LifecycleState::Retained)
            | (LifecycleState::Validated, LifecycleState::Revoked)
            | (LifecycleState::Proposed, LifecycleState::Revoked)
    );
    if allowed {
        Ok(())
    } else {
        Err(MemoryValidationError::LegalHoldExpired)
    }
}

pub fn validate_store<S: MemoryStore>(
    store: &S,
    objects: &[MemoryObject],
    now_ms: u64,
) -> Result<(), MemoryValidationError> {
    for object in objects {
        object.validate(now_ms)?;
        if store.get(object.id).is_none() {
            return Err(MemoryValidationError::MissingProvenance);
        }
    }
    Ok(())
}

use crate::LifecycleState;

/// Validate a memory lifecycle transition before it is persisted.
pub fn validate_transition(
    from: LifecycleState,
    to: LifecycleState,
) -> Result<(), LifecycleTransitionError> {
    use LifecycleState::*;
    let allowed = matches!(
        (from, to),
        (Proposed, Validated)
            | (Validated, Active)
            | (Active, Superseded)
            | (Active, Retained)
            | (Active, Expired)
            | (Active, Revoked)
            | (Superseded, Retained)
            | (Superseded, Revoked)
            | (Retained, Revoked)
    );
    if allowed {
        Ok(())
    } else {
        Err(LifecycleTransitionError { from, to })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[error("invalid memory lifecycle transition: {from:?} -> {to:?}")]
pub struct LifecycleTransitionError {
    pub from: LifecycleState,
    pub to: LifecycleState,
}

pub fn is_terminal(state: LifecycleState) -> bool {
    matches!(state, LifecycleState::Expired | LifecycleState::Revoked)
}

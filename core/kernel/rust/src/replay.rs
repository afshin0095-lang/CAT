use crate::{EventEnvelope, KernelResult, SequenceNumber};

/// Deterministically folds an ordered event stream into domain state.
/// Replay is intentionally side-effect free: the caller supplies the state,
/// events are applied in stored order, and the resulting state is returned.
/// External effects must never be triggered from this primitive.
pub fn replay<TEvent, TState, F>(
    initial: TState,
    events: impl IntoIterator<Item = EventEnvelope<TEvent>>,
    mut apply: F,
) -> KernelResult<TState>
where
    F: FnMut(TState, &EventEnvelope<TEvent>) -> KernelResult<TState>,
{
    let mut state = initial;
    let mut previous = SequenceNumber::ZERO;
    for event in events {
        event.sequence.checked_after(previous)?;
        previous = event.sequence;
        state = apply(state, &event)?;
    }
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CorrelationId, EntityId, EventEnvelope, TenantId, TimestampMs};

    fn event(sequence: u64, value: i32) -> EventEnvelope<i32> {
        EventEnvelope::new(
            "cat.test.replay", 1, TenantId::new(), CorrelationId::new(), None,
            EntityId::new(), TimestampMs::new(1), SequenceNumber::new(sequence), value,
        ).unwrap()
    }

    #[test]
    fn replay_is_deterministic_and_ordered() {
        let events = vec![event(1, 10), event(2, 20), event(3, -5)];
        let result = replay(0, events, |state, event| Ok(state + event.payload)).unwrap();
        assert_eq!(result, 25);
    }

    #[test]
    fn replay_rejects_non_monotonic_history() {
        let events = vec![event(1, 10), event(1, 20)];
        assert!(replay(0, events, |state, event| Ok(state + event.payload)).is_err());
    }

    #[test]
    fn replay_does_not_execute_external_effects_by_itself() {
        let events = vec![event(1, 10)];
        let mut applied = 0;
        let result = replay(0, events, |state, event| {
            applied += 1;
            Ok(state + event.payload)
        }).unwrap();
        assert_eq!(result, 10);
        assert_eq!(applied, 1);
    }
}

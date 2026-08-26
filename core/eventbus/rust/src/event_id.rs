use uuid::Uuid;

/// Stable event identity used for idempotency, causation and audit correlation.
///
/// CAT generates UUIDv7 values so identifiers are globally unique while retaining
/// useful chronological ordering characteristics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct EventId(Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }

    pub const fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for EventId {
    fn from(value: Uuid) -> Self {
        Self::from_uuid(value)
    }
}

impl From<EventId> for Uuid {
    fn from(value: EventId) -> Self {
        value.into_uuid()
    }
}

#[cfg(test)]
mod tests {
    use super::EventId;

    #[test]
    fn generated_event_ids_round_trip_through_uuid() {
        let id = EventId::new();
        let uuid = id.into_uuid();
        assert_eq!(EventId::from(uuid), id);
    }
}

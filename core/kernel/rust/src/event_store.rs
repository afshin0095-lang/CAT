use std::collections::BTreeMap;

use crate::{EntityId, EventEnvelope, EventId, IdempotencyKey, IdempotencyLedger, IdempotencyReceipt, KernelError, KernelResult, SequenceNumber};

/// Expected stream version used for optimistic concurrency control.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExpectedVersion {
    Any,
    Exact(SequenceNumber),
    Empty,
}

/// An immutable event as stored in a stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredEvent<T> {
    pub stream_id: EntityId,
    pub envelope: EventEnvelope<T>,
}

/// Append result returned by the kernel event-store contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppendReceipt {
    pub event_id: EventId,
    pub sequence: SequenceNumber,
    pub idempotent_replay: bool,
}

/// Deterministic in-memory event store.
///
/// This is deliberately a kernel-level reference implementation: streams are
/// append-only, per-stream ordering is explicit, and optimistic concurrency
/// rejects stale writers. Durable storage adapters implement the same contract.
#[derive(Clone, Debug, Default)]
pub struct EventStore<T> {
    streams: BTreeMap<EntityId, Vec<StoredEvent<T>>>,
    idempotency: IdempotencyLedger,
}

impl<T: Clone> EventStore<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append(
        &mut self,
        stream_id: EntityId,
        expected: ExpectedVersion,
        key: IdempotencyKey,
        envelope: EventEnvelope<T>,
    ) -> KernelResult<AppendReceipt> {
        if let Some(receipt) = self.idempotency.lookup(&key) {
            return Ok(AppendReceipt {
                event_id: receipt.event_id,
                sequence: receipt.sequence,
                idempotent_replay: true,
            });
        }

        let stream = self.streams.entry(stream_id).or_default();
        let current = stream
            .last()
            .map(|event| event.envelope.sequence)
            .unwrap_or(SequenceNumber::ZERO);

        match expected {
            ExpectedVersion::Any => {}
            ExpectedVersion::Empty if current != SequenceNumber::ZERO => {
                return Err(KernelError::ConcurrencyConflict {
                    expected: SequenceNumber::ZERO,
                    actual: current,
                });
            }
            ExpectedVersion::Empty => {}
            ExpectedVersion::Exact(version) if current != version => {
                return Err(KernelError::ConcurrencyConflict {
                    expected: version,
                    actual: current,
                });
            }
            ExpectedVersion::Exact(_) => {}
        }

        let next = current.next()?;
        if envelope.sequence != next {
            return Err(KernelError::SequenceConflict {
                expected: next,
                actual: envelope.sequence,
            });
        }

        let event_id = envelope.event_id;
        stream.push(StoredEvent {
            stream_id,
            envelope,
        });

        self.idempotency.record(
            key,
            IdempotencyReceipt {
                event_id,
                sequence: next,
            },
        )?;

        Ok(AppendReceipt {
            event_id,
            sequence: next,
            idempotent_replay: false,
        })
    }

    pub fn read_stream(&self, stream_id: EntityId) -> &[StoredEvent<T>] {
        self.streams
            .get(&stream_id)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn current_version(&self, stream_id: EntityId) -> SequenceNumber {
        self.read_stream(stream_id)
            .last()
            .map(|event| event.envelope.sequence)
            .unwrap_or(SequenceNumber::ZERO)
    }

    pub fn stream_count(&self) -> usize {
        self.streams.len()
    }

    pub fn event_count(&self) -> usize {
        self.streams.values().map(Vec::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CorrelationId, EntityId, EventEnvelope, TenantId, TimestampMs};

    fn event(sequence: u64) -> EventEnvelope<&'static str> {
        EventEnvelope::new(
            "cat.test.event",
            1,
            TenantId::new(),
            CorrelationId::new(),
            None,
            EntityId::new(),
            TimestampMs::new(1).unwrap(),
            SequenceNumber::new(sequence).unwrap(),
            "payload",
        )
        .unwrap()
    }

    #[test]
    fn appends_in_explicit_stream_order() {
        let mut store = EventStore::new();
        let stream = EntityId::new();
        let first = event(1);
        let second = event(2);

        store
            .append(
                stream,
                ExpectedVersion::Empty,
                IdempotencyKey::new("a").unwrap(),
                first,
            )
            .unwrap();
        store
            .append(
                stream,
                ExpectedVersion::Exact(SequenceNumber::new(1).unwrap()),
                IdempotencyKey::new("b").unwrap(),
                second,
            )
            .unwrap();

        assert_eq!(store.current_version(stream), SequenceNumber::new(2).unwrap());
        assert_eq!(store.read_stream(stream).len(), 2);
    }

    #[test]
    fn rejects_stale_concurrent_writer() {
        let mut store = EventStore::new();
        let stream = EntityId::new();
        store
            .append(
                stream,
                ExpectedVersion::Empty,
                IdempotencyKey::new("a").unwrap(),
                event(1),
            )
            .unwrap();

        let error = store
            .append(
                stream,
                ExpectedVersion::Exact(SequenceNumber::ZERO),
                IdempotencyKey::new("b").unwrap(),
                event(2),
            )
            .unwrap_err();

        assert!(matches!(error, KernelError::ConcurrencyConflict { .. }));
    }

    #[test]
    fn duplicate_delivery_is_idempotent() {
        let mut store = EventStore::new();
        let stream = EntityId::new();
        let key = IdempotencyKey::new("delivery-1").unwrap();
        let receipt = store
            .append(stream, ExpectedVersion::Empty, key.clone(), event(1))
            .unwrap();
        let replay = store
            .append(
                stream,
                ExpectedVersion::Any,
                key,
                event(99),
            )
            .unwrap();

        assert_eq!(receipt.event_id, replay.event_id);
        assert!(replay.idempotent_replay);
        assert_eq!(store.event_count(), 1);
    }
}

use cat_kernel::{
    CorrelationId, EntityId, EventEnvelope, EventId, KernelResult, ProjectionDescriptor,
    ProjectionHandler, ProjectionRegistry, ProjectionRuntime, SequenceNumber, StoredEvent,
    TenantId, TimestampMs,
};

struct Counter;
impl ProjectionHandler<u64, u64> for Counter {
    fn projection_id(&self) -> &str {
        "counter-v1"
    }
    fn apply(&mut self, state: &mut u64, event: &StoredEvent<u64>) -> KernelResult<()> {
        *state += event.envelope.payload;
        Ok(())
    }
}

fn event(stream: EntityId, sequence: u64, value: u64) -> StoredEvent<u64> {
    StoredEvent {
        stream_id: stream,
        envelope: EventEnvelope::new(
            "cat.test.counter",
            1,
            TenantId::new(),
            CorrelationId::new(),
            None,
            EntityId::new(),
            TimestampMs::new(sequence),
            SequenceNumber::new(sequence),
            value,
        )
        .unwrap(),
    }
}

#[test]
fn projection_runtime_replays_safely_and_registry_bootstraps() {
    let mut registry = ProjectionRegistry::default();
    registry
        .register(ProjectionDescriptor::new(
            "counter-v1",
            1,
            "counter projection",
        ))
        .unwrap();
    assert!(registry.get("counter-v1").is_some());

    let stream = EntityId::new();
    let mut runtime =
        ProjectionRuntime::new(stream, SequenceNumber::ZERO, EventId::new(), 0, Counter).unwrap();
    let first = event(stream, 1, 2);
    let second = event(stream, 2, 3);
    runtime.catch_up(&[first.clone(), second.clone()]).unwrap();
    runtime.apply(&second).unwrap();

    assert_eq!(*runtime.state(), 5);
    assert_eq!(runtime.checkpoint().sequence, SequenceNumber::new(2));
}

# CAT EventBus Core

Typed, broker-neutral event infrastructure for CAT OMNISYSTEM.

## Current capabilities

- Typed event contracts via `CatEvent`
- UUIDv7 event identity
- Correlation, causation, producer and subject metadata
- Domain versus integration event classification
- In-memory synchronous event bus for deterministic local execution
- Durable inbox/outbox and idempotency boundaries
- Retry policy with bounded exponential backoff
- Dead-letter abstractions
- Broker-neutral transport registry and deterministic routing
- NATS JetStream transport and durable pull-consumer support
- Explicit broker acknowledgement ordering
- Event contract registry and compatibility policy
- Atomic event metrics for published, delivered, acknowledged, retried, dead-lettered and rejected events

## Reliability invariant

CAT records application-side success before advancing the broker acknowledgement floor. Consumers are therefore designed for at-least-once delivery with inbox-based replay suppression.

## Boundary

This crate owns event transport semantics. Domain engines own their canonical truth and decide which facts cross the event boundary.

## Validation

Run from this crate:

```bash
cargo test
cargo clippy -- -D warnings
```

NATS integration requires a reachable JetStream-capable NATS deployment.

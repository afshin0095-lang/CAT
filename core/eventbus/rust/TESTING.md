# CAT EventBus — Verification Surface

The EventBus is the local deterministic delivery boundary for CAT. The Rust crate intentionally keeps transport, persistence, routing, and processing behind explicit ports so the same contract can be exercised in-memory before NATS/JetStream deployment.

## Required invariants

1. Event identity is the idempotency boundary.
2. Event type and version are validated against the event registry before registered publication.
3. Handler registration order is deterministic for the local bus.
4. A failed handler removes the event from the in-flight set so a retry can execute again.
5. A successfully processed event is suppressed on replay.
6. Correlation and causation identifiers are transport-independent context.
7. Outbox, inbox, idempotency, and transport storage remain replaceable through ports.
8. Retry policy is explicit and bounded; scheduling belongs to the delivery adapter.

## Verification layers

### Unit layer

Core modules contain focused tests for registry lookup, duplicate contract rejection, deterministic dispatch, retry behavior, and state transitions.

### Integration layer

Downstream integration tests should exercise the public crate surface rather than internal module paths. At minimum, cover:

- typed event → envelope conversion;
- correlation/causation/subject propagation;
- exact event-contract version matching;
- duplicate delivery suppression;
- transient handler retry;
- bounded exponential retry delay;
- inbox replay rejection;
- idempotency claim/complete lifecycle;
- outbox enqueue/next/acknowledge lifecycle.

### Transport layer

NATS/JetStream tests must remain adapter tests. They must prove that a transport preserves the EventEnvelope identity and delivery semantics; they must not redefine domain semantics inside the transport implementation.

## Recommended command

```text
cargo test --manifest-path core/eventbus/rust/Cargo.toml
```

For the workspace-level verification, run the repository's standard Rust checks after the eventbus crate is wired into the root workspace.

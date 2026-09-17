# ADR-0002: Event-Driven Core

**Status:** Accepted
**Date:** 2026-09-17

## Context

CAT contains long-running workflows, multiple agents, external providers, projections, measurement, and recovery requirements. Direct synchronous coupling between every component would make failure isolation and evolution difficult.

## Decision

CAT uses an **event-driven architecture** for durable cross-boundary state propagation.

Commands express intent. Domain/application logic evaluates the command. Durable state changes produce versioned events. Consumers build projections, trigger workflows, update measurements, or initiate subsequent work.

```text
Command
  ↓
Decision / Domain Logic
  ↓
Durable State Change
  ↓
Domain Event
  ↓
Event Bus
  ├── Projection
  ├── Workflow Trigger
  ├── Measurement
  ├── Audit
  └── Other Consumers
```

The system assumes at-least-once delivery at event boundaries unless a stronger guarantee is explicitly implemented and verified.

## Consequences

- consumers MUST be idempotent;
- event schemas require compatibility management;
- inbox/outbox patterns are preferred for reliable publication and consumption;
- projections are derived state, not canonical truth;
- asynchronous behavior becomes observable and testable as first-class execution.

## Rejected Alternative

A fully synchronous request chain was rejected as the primary architecture because provider latency, retries, process crashes, and distributed execution would propagate failures across unrelated domains.

## Invariants

- Events MUST carry correlation and causation lineage when part of durable execution.
- Consumers MUST tolerate duplicate delivery.
- Event versioning MUST be explicit.
- Unknown external outcomes MUST remain distinguishable from confirmed failures.

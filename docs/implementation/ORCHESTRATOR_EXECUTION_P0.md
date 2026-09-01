# CAT Orchestrator Execution P0

## Purpose

This document defines the first executable control boundary between a validated plan and the runtime orchestration layer.

## Responsibilities

The Orchestrator owns runtime progress of a workflow instance:

- identify ready work;
- start and complete individual steps;
- track attempts;
- decide whether a failed attempt is retryable;
- expose deterministic execution state and events;
- support cancellation and later compensation/recovery flows.

The Orchestrator does **not** decide business authorization, invent plans, or silently mutate domain truth.

## P0 contracts

### Execution cursor
`ExecutionCursor` is a read-only view of the current workflow revision and ready step identifiers. It is intended to be consumed by workers or a future execution dispatcher without exposing mutable workflow internals.

### Retry decision
`RetryDecision` converts the existing retry policy into a pure decision:

- `Retry { delay_ms }`
- `Exhausted`

The decision layer does not sleep, enqueue, or execute a retry.

### Execution state and events
Execution state and event contracts provide a stable representation for future persistence, telemetry, EventBus publication, and worker integration.

## Execution sequence

```text
Validated Plan
    -> Orchestrator Workflow Instance
    -> Refresh Ready Steps
    -> Execution Cursor
    -> Worker/Dispatcher
    -> Step Result
       -> Success -> unlock dependents
       -> Failure -> RetryDecision
                    -> Retry -> reschedule
                    -> Exhausted -> failure/recovery path
```

## Safety boundary

The P0 implementation remains deterministic and infrastructure-light. Storage, leases, transport, worker execution, distributed locking, and durable retry scheduling remain explicit integration responsibilities rather than hidden side effects in the state machine.

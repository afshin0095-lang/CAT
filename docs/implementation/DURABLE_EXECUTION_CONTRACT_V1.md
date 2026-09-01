# CAT Durable Execution Contract V1

## Purpose

This contract defines the boundary between Orchestrator state transitions, worker execution, leases, durable persistence, and event delivery.

## Invariants

1. **State is authoritative.** A workflow revision is the source of truth for orchestration state.
2. **Optimistic concurrency is mandatory.** A durable commit includes the revision observed before the attempt. A stale writer must be rejected.
3. **State and events commit together.** Workflow state changes and their event envelopes are written to the durable event outbox in the same transaction.
4. **Event publication is asynchronous from the state transaction.** If EventBus delivery is unavailable, the durable outbox remains replayable.
5. **Lease precedes worker execution.** A worker attempt is dispatched only after the orchestrator acquires a lease for the workflow step resource.
6. **Workers do not mutate orchestration state.** Workers return machine-readable outcomes; Orchestrator decides retry, wait-for-approval, cancellation, or failure.
7. **Retry is deterministic.** Retry policy is evaluated from the attempt number and produces a bounded delay.
8. **The execution coordinator is transport-agnostic.** NATS, HTTP, local execution, queues, and future transports remain replaceable adapters.

## Execution sequence

```text
load workflow
    |
    v
validate READY step
    |
    v
acquire lease
    |
    v
claim step -> RUNNING
    |
    v
construct execution identity
    |
    v
WorkerExecutor::execute
    |
    v
map outcome -> DispatchAction
    |
    +--> success --------> SUCCEEDED
    +--> retry -----------> WAITING + scheduler delay
    +--> approval --------> WAITING
    +--> cancel ----------> CANCELLED / SKIPPED
    +--> exhausted -------> FAILED
    |
    v
commit workflow revision + outbox event
    |
    v
publish outbox event to EventBus
```

## Production adapters

The current in-memory implementations are test/local-development implementations only. A production deployment should provide:

- `DurableWorkflowStore`: transactional PostgreSQL or another strongly consistent durable store.
- `LeaseProvider`: distributed lease implementation with owner fencing or equivalent concurrency protection.
- `WorkerExecutor`: local worker runtime or a remote worker transport adapter.
- outbox dispatcher: durable polling/streaming process that publishes committed envelopes to `cat-eventbus` and safely retries failures.

The domain contract intentionally does not select one infrastructure technology. Technology selection belongs in the CAT technology decision records and must preserve portability and reversibility.

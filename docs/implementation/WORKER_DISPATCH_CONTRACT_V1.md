# CAT Worker Dispatch Contract v1

## Purpose

This document defines the boundary between the CAT Orchestrator and the component that performs actual work.

## Responsibility split

### Orchestrator
- decides which step is ready;
- validates workflow state and attempt number;
- creates an explicit execution request;
- decides what to do with the returned outcome;
- records execution state and events.

### Worker
- receives a fully specified execution input;
- performs the external or computational work;
- returns a machine-readable outcome;
- does not change orchestrator state directly.

## Outcome contract

Workers return one of four outcomes:

- `succeeded` — the requested work completed;
- `failed` — the work did not complete and may be retryable;
- `waiting_approval` — execution must stop until an approval boundary is satisfied;
- `cancelled` — the worker stopped without completing the requested work.

## Retry boundary

The worker reports the result of one attempt. Retry policy remains an Orchestrator concern. The Orchestrator converts a failed result into either a retry request with bounded delay or a terminal failure.

## Idempotency

A worker must treat `execution_id + step_id + attempt` as the logical execution identity. The underlying platform adapter may add stronger idempotency keys when required by the target system.

## Side-effect rule

No worker call is implied by constructing an execution request. The request is an intent object. Transport, queueing, leases, retries, and external side effects stay behind the worker/runtime boundary.

## Next integration stage

The next implementation stage can provide concrete adapters for EventBus, worker transport, durable persistence, and lease ownership without changing these domain contracts.

# CAT Worker Dispatch Contract v1

This document defines the boundary between the CAT Orchestrator and workers that perform actual work.

## Orchestrator responsibilities
- determine which step is ready;
- validate workflow state and attempt number;
- create an explicit execution request;
- interpret the worker outcome;
- record state and events.

## Worker responsibilities
- receive a fully specified execution input;
- perform the requested external or computational work;
- return a machine-readable outcome;
- never mutate orchestrator state directly.

## Outcomes
- `succeeded`: work completed;
- `failed`: work did not complete and may be retryable;
- `waiting_approval`: execution must pause at an approval boundary;
- `cancelled`: work stopped without completion.

## Retry
Workers report one attempt only. Retry policy remains an Orchestrator concern and converts a failure into either a bounded retry or a terminal failure.

## Idempotency
A worker should treat `execution_id + step_id + attempt` as the logical execution identity. Platform adapters may add stronger provider-specific idempotency keys.

## Side effects
Creating an execution request does not execute work. Transport, queues, leases, persistence, and external side effects remain behind the worker/runtime boundary.

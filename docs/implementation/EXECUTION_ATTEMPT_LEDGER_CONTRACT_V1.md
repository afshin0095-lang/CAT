# CAT Execution Attempt Ledger Contract V1

## Purpose

The execution-attempt ledger provides a durable identity and lifecycle record for every worker attempt. It closes the recovery gap between a workflow step being `RUNNING` and knowing whether the worker actually completed, failed, or disappeared.

## Invariants

1. Every execution has a stable `execution_id`.
2. `(workflow_id, step_id, attempt)` is unique and idempotent.
3. A worker must record the attempt before performing external work.
4. A running attempt emits heartbeats using the same owner and fencing token that acquired the execution lease.
5. Heartbeat and completion writes are fenced; a stale worker cannot update an attempt after ownership changes.
6. A stale heartbeat is evidence for reconciliation, not permission to replay automatically.
7. Terminal attempts are immutable from the worker's perspective.
8. Attempt results and errors are retained for deterministic recovery and audit.

## Lifecycle

`running -> succeeded | failed | cancelled`

An attempt with an old heartbeat is classified as `stale` while its status remains `running`. The orchestrator must reconcile the external execution before creating another attempt.

## Recovery semantics

At restart, CAT combines workflow state with the attempt ledger:

- terminal attempt: use the durable result/state and do not replay blindly;
- healthy running attempt: continue observation/heartbeat supervision;
- stale running attempt: enter reconciliation;
- missing ledger record for a running workflow step: treat the state as an incomplete execution record and require reconciliation before replay.

The ledger deliberately does not encode a universal timeout-to-failure rule. Timeout policy is workload-specific and must remain an explicit orchestration decision.

## PostgreSQL model

`cat_execution_attempts` stores execution identity, workflow/step/attempt, owner, fencing token, start time, heartbeat time, terminal time, result and error. A workflow foreign key prevents orphan attempt records.

## Next extensions

Future versions may add worker identity metadata, external provider execution IDs, cancellation timestamps, heartbeat sequence numbers, and a separate execution-result journal for exactly-once interpretation of provider callbacks.

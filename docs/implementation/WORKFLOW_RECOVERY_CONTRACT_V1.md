# CAT Workflow Recovery Contract V1

## Purpose

This contract defines how CAT reconstructs execution intent after an orchestrator process restart, node loss, or worker interruption.

The recovery layer is deliberately conservative: durable state is authoritative, but a `RUNNING` step does not prove whether its external side effect completed before the crash.

## Recovery classes

- `ResumeReady`: one or more steps are `READY` and there is no unresolved `RUNNING` step. The normal scheduler may resume them.
- `AwaitApproval`: execution is intentionally waiting for an approval or external condition.
- `ReconcileRunning`: at least one step is `RUNNING`. Its side-effect identity must be reconciled before replay or retry.
- `Terminal`: no runnable, waiting, or running step remains; the workflow is already complete, failed, compensated, or cancelled.

## Safety invariants

1. Recovery never fabricates success.
2. Recovery never converts `RUNNING` directly to `READY`.
3. Workers remain side-effect owners; recovery only classifies durable intent.
4. Any replay must retain the original execution identity and idempotency contract where the worker protocol supports it.
5. Optimistic concurrency remains mandatory for every state mutation.
6. Fencing tokens remain mandatory at external side-effect boundaries.
7. Recovery is deterministic for the same durable workflow snapshot.

## Reconciliation flow

```text
Postgres durable state
        |
        v
Recovery classifier
        |
   +----+----------------+
   |    |                |
 READY WAITING         RUNNING
   |    |                |
   v    v                v
resume approval      reconcile side effect
                        |
                  confirmed outcome
                        |
                  normal coordinator
```

## Current implementation

`core/orchestrator/rust/src/recovery.rs` provides:

- `RecoveryAction`
- `WorkflowRecoveryReport`
- `WorkflowRecoveryStore`
- `AsyncWorkflowRecovery`

The report is derived only from `WorkflowInstance` state and does not mutate it.

## Future extensions

A production recovery service should add durable worker heartbeats, execution-attempt timestamps, and an execution-result ledger keyed by `execution_id`. Those additions allow stronger automated reconciliation without weakening the conservative default above.

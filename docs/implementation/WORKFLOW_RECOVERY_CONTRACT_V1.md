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
5. Authorization evidence is durable and bound to the original execution attempt; recovery must not fabricate a new authorization context silently.
6. Optimistic concurrency remains mandatory for every state mutation.
7. Fencing tokens remain mandatory at external side-effect boundaries.
8. Recovery is deterministic for the same durable workflow snapshot.

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

## Current implementation

The Orchestrator now has a production async execution coordinator that records the attempt and authorization evidence before worker dispatch. The attempt ledger stores execution identity and fencing information, while `cat_execution_authorizations` stores the governance context used for admission.

## Future extensions

Future recovery versions may use the durable authorization record directly when producing reconciliation and audit projections, and may add stronger execution-result journaling keyed by `execution_id`.

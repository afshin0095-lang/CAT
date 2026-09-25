# Execution Reconciliation + Audit Evidence P0

Status: Implemented on `feat/capability-registry-p0`

## Purpose

This stage makes durable governance and provider execution history available to reconciliation and operator-facing audit tooling.

Canonical sources remain:

- `cat_execution_attempts` for execution identity and lifecycle;
- `cat_execution_authorizations` for exact capability admission evidence;
- `cat_provider_execution_results` for the current provider outcome;
- `cat_provider_execution_journal` for append-only provider execution history;
- `cat_execution_audit_events` for append-only audit evidence;
- `cat_execution_audit_read_model` for the latest operator-facing execution view.

Read models never replace canonical facts.

## Reconciliation contract

`ExecutionReconciliationStore` loads:

```text
attempt
   +
authorization evidence
   +
current provider result
        |
        v
deterministic reconciliation action
        |
        v
ExecutionAuditEvidence
```

Missing authorization evidence produces `ManualReview`. Provider success cannot substitute for missing governance evidence.

## Durable operator audit

`ExecutionAuditStore` exposes:

- idempotent append of a reconciliation audit event;
- latest audit record by execution;
- bounded operator queries filtered by agent, capability, or reconciliation action;
- explicit rebuild of the latest read model from the append-only audit log.

Audit writes are transactional:

```text
append-only audit event
        +
latest read-model upsert
        |
       COMMIT
```

The read model is keyed by `execution_id` and stores the source audit sequence. `rebuild_audit_read_model()` truncates and reconstructs it from the highest audit sequence for each execution.

A conflicting reuse of an audit event key fails closed.

## Provider execution journal

`ProviderExecutionJournalStore` maintains append-only provider execution history.

The lifecycle is:

```text
submitted
   |
   v
observed outcome
```

Submission and observation journal entries use deterministic event keys. Repeated identical observations are idempotently deduplicated.

The existing `cat_provider_execution_results` row remains the current-state projection used by reconciliation. Provider-result mutations now write that current row and the corresponding journal record in the same PostgreSQL transaction.

A conflicting provider execution identity or terminal outcome is rejected.

## PostgreSQL + EventBus verification

`core/orchestrator/rust/tests/durable_execution_postgres.rs` verifies with a real PostgreSQL service when `CAT_TEST_DATABASE_URL` or `DATABASE_URL` is configured:

1. governed execution loads its workflow from PostgreSQL;
2. capability admission occurs before worker dispatch;
3. the async worker receives authorization and the concrete fencing token;
4. execution attempt + authorization evidence are committed;
5. provider submission + observation are journaled;
6. duplicate provider journal writes remain deduplicated;
7. reconciliation reloads authorization and current provider outcome;
8. the audit projection round-trips through JSON;
9. the audit event is stored and exposed through the read model;
10. the audit read model can be rebuilt from the append-only log;
11. the audit query contract returns the execution;
12. missing authorization later forces `ManualReview` even when provider success exists;
13. the workflow event is claimed from the PostgreSQL outbox;
14. the claimed event is delivered through the EventBus boundary;
15. acknowledgement removes the outbox row.

## Safety invariants

1. Provider execution history is append-oriented and never silently rewritten.
2. Provider current state cannot override governance evidence.
3. Missing authorization remains reviewable.
4. Audit read data is reconstructible from the append-only audit log.
5. Workflow/outbox publication stays transactionally coupled.
6. Worker-side effects remain gated by the concrete fencing token.

## Next boundary

Stabilize CI on the latest PR head, then integrate operator authentication/authorization around audit queries and extend provider journaling to callback correlation and stronger execution-result reconciliation.

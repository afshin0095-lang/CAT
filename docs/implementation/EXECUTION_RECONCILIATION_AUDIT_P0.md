# Execution Reconciliation + Audit Evidence P0

Status: Implemented on `feat/capability-registry-p0`

## Purpose

This stage makes the durable authorization record a first-class input to reconciliation and audit projections.

Canonical sources remain:

- `cat_execution_attempts` for execution identity and lifecycle;
- `cat_execution_authorizations` for exact capability admission evidence;
- `cat_provider_execution_results` for provider-side observed outcomes;
- workflow/outbox state for durable orchestration and delivery.

The audit projection is derived data, not a replacement for those sources.

## Reconciliation contract

`ExecutionReconciliationStore` now combines the existing execution-attempt boundary with durable authorization loading.

The reconciliation sequence is:

```text
attempt
   +
authorization evidence
   +
provider result
        |
        v
deterministic reconciliation action
        |
        v
optional ExecutionAuditEvidence
```

Missing authorization evidence is not synthesized from workflow state. It produces `ManualReview`.

A persisted authorization record is decoded and validated before it becomes audit evidence.

## Audit projection

`ExecutionAuditEvidence` combines:

- execution ID;
- workflow ID;
- step ID;
- attempt;
- attempt status;
- authorization record;
- provider execution result, when available.

The projection is serializable and intended for audit consumers, reconciliation tooling, operators, and future read models.

Its construction fails closed for invalid execution identity or invalid authorization evidence.

## PostgreSQL + EventBus verification

`core/orchestrator/rust/tests/durable_execution_postgres.rs` is environment-gated on `CAT_TEST_DATABASE_URL` or `DATABASE_URL`.

With the PostgreSQL service available, the test verifies:

1. governed execution loads its workflow from PostgreSQL;
2. capability admission occurs before worker dispatch;
3. the async worker receives authorization and the concrete fencing token;
4. execution attempt + authorization evidence are committed;
5. reconciliation reloads authorization evidence from PostgreSQL;
6. the audit projection round-trips through JSON;
7. the workflow event is claimed from the PostgreSQL outbox;
8. the claimed event is delivered through the EventBus boundary;
9. successful acknowledgement removes the outbox row.

## Safety invariants

1. Provider success never substitutes for missing governance evidence.
2. Missing authorization evidence is reviewable, not silently authorized.
3. Audit evidence is reconstructible from canonical durable state.
4. Event publication occurs only after workflow + outbox commit.
5. Outbox acknowledgement still requires the claim owner.
6. Worker-side effects remain gated by the concrete fencing token.

## Next boundary

Stabilize the latest CI run on PR #61, then add durable operator-facing audit storage/read models and stronger provider result journaling keyed by `execution_id`.

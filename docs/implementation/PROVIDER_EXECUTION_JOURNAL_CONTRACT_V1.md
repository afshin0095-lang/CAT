# CAT Provider Execution Journal Contract V1

Status: Implemented on `feat/capability-registry-p0`

## Purpose

The provider execution journal preserves append-only evidence for external provider work while keeping `cat_provider_execution_results` as the current-state record used for fast reconciliation.

## Invariants

1. Every journal entry belongs to a durable `execution_id`.
2. Submission and observation identities are deterministic and namespaced by provider.
3. Duplicate identical journal writes are idempotently suppressed.
4. A journal identity conflict fails closed.
5. Provider current-state mutation and journal insertion commit in one PostgreSQL transaction.
6. Journal order is represented by a database sequence.
7. Journal records retain provider, provider execution identity, request hash, outcome, result/error, and record time.
8. Provider result terminal-state conflicts are rejected rather than overwritten.

## Event types

### Submitted

Captures the provider execution identity before the external result is known.

### Observed

Captures the externally observed provider outcome and optional payload/error.

## Storage

`cat_provider_execution_journal` is append-only and indexed by execution identity and provider execution identity.

`cat_provider_execution_results` remains the current-state projection for reconciliation.

## Recovery

Recovery can inspect journal history when the current provider record is insufficient to explain how the execution reached its observed state. Journal history is evidence, not permission to bypass capability authorization or fencing.

## Future extensions

Future versions may add provider callback correlation IDs, callback signatures, observation source, retry/correction semantics, and explicit remote request/response timestamps.

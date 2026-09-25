# CAT OMNISYSTEM — Provider Callback Reconciliation Contract V1

**Status:** Implemented P0
**Scope:** Durable replay of provider callbacks that arrive before CAT records the provider submission

## 1. Runtime problem

Provider callbacks can arrive out of order relative to CAT's submission persistence. Treating the callback as failure or success at ingestion time would make external timing part of system correctness.

P0 therefore persists the callback first and retries correlation after the provider execution identity becomes durable.

## 2. Execution flow

```text
Callback arrives
    ↓
Persist callback evidence
    ↓
No matching provider submission
    ↓
UNMATCHED
    ↓
Provider submission becomes durable
    ↓
ProviderCallbackReconciliationWorker
    ↓
Match provider + provider_execution_id
    ↓
Verify request_hash when supplied
    ↓
Correlated transaction
    ├── callback state → correlated
    ├── provider result → observed
    └── provider journal → observed event
```

## 3. Batch contract

`ProviderCallbackReconciliationWorker` operates on a bounded provider-specific batch.

Each run reports:

- `scanned` — callbacks selected for reconciliation;
- `correlated` — callbacks successfully resolved;
- `still_unmatched` — callbacks whose provider submission is not yet durable;
- `rejected` — callbacks whose identity or payload conflicts with the durable submission;
- `already_handled` — callbacks concurrently resolved by another worker.

The worker clamps the requested batch size to 1..500.

## 4. Concurrency and idempotency

Each callback reconciliation uses its own database transaction and locks the callback and matching provider-result row.

Concurrent workers therefore cannot both transition the same unmatched callback independently.

Journal insertion remains deterministic and idempotent.

## 5. Rejection semantics

A callback is moved to `rejected` when its request hash or terminal result conflicts with durable provider state.

Rejected callbacks remain durable evidence and carry a `correlation_error`. They are not retried as ordinary unmatched callbacks.

## 6. Security

Replay never creates authorization evidence, capability grants, or operator permissions.

External callback content remains data. The reconciler only updates provider execution state after deterministic correlation and consistency checks.

## 7. Next extension

Future revisions can add provider-specific callback signature verification, anti-replay timestamps/nonces, retry backoff metadata, and a multi-provider dispatcher while preserving the bounded per-provider worker contract.
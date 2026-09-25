# CAT OMNISYSTEM — Provider Callback Correlation Contract V1

**Status:** Implemented P0
**Scope:** Durable ingestion and correlation of asynchronous provider callbacks

## 1. Contract

A provider callback is external input, not trusted execution authority.

```text
Provider callback
    ↓
Validate callback identity
    ↓
Persist immutable callback evidence
    ↓
Correlate provider + provider_execution_id
    ↓
Verify request_hash when supplied
    ↓
Accept only a consistent terminal outcome
    ↓
Update current provider result + append journal observation
```

## 2. Durable callback evidence

`cat_provider_execution_callbacks` stores:

- callback_id;
- provider and provider_execution_id;
- optional request_hash;
- normalized outcome;
- provider result/error payload;
- received timestamp;
- correlated execution_id when resolved;
- unmatched/correlated/rejected state;
- correlation error for durable non-retryable conflicts;
- correlation timestamp.

Callback evidence keeps the referenced CAT execution UUID as historical evidence rather than depending on the lifecycle of a transient execution row.

## 3. Correlation rules

1. The provider name and provider execution ID must be present.
2. Callback IDs are unique and idempotent.
3. A duplicate callback is accepted only when its payload and identity match the stored callback exactly.
4. When a submitted provider execution exists, its stored request hash is authoritative.
5. A supplied callback request hash must equal the submitted request hash.
6. A callback that conflicts with an already-recorded terminal outcome is rejected.
7. Current provider result mutation and provider journal observation are committed in the same PostgreSQL transaction.
8. A callback received before provider submission is retained as `unmatched` rather than being discarded or treated as success.
9. `ProviderCallbackReconciliationWorker` retries unmatched callbacks after provider submission becomes durable.
10. Non-retryable request-hash or terminal-payload conflicts become `rejected` with durable error evidence.

## 4. Security boundary

Provider payloads remain untrusted data. No callback can grant agent capability, bypass policy, or create authorization evidence.

Provider callback correlation is an execution-result concern. Capability admission and operator authorization remain separate controls.

## 5. Recovery path

Unmatched callbacks are queryable in sequence order. `ProviderCallbackReconciliationWorker` processes a bounded provider-specific batch. A callback remains unmatched when its submission is not yet durable and is marked rejected when deterministic consistency checks fail.

Future revisions can add:

- signed callback verification;
- provider-specific signature metadata;
- callback replay windows and expiry;
- nonce/timestamp anti-replay controls;
- callback retry disposition;
- cryptographic evidence references.
# C12 — Idempotency Contract

Retryable mutations require an idempotency key scoped to the authenticated principal and operation.

```text
(principal, endpoint, idempotency_key) → one durable outcome
```

The server stores enough information to return the original outcome for a replay. A key cannot silently be reused with materially different request parameters; such reuse is a conflict. Keys have explicit retention/expiry rules.

Idempotency protects against client retries, network ambiguity, worker redelivery, and provider timeout uncertainty. It does not replace optimistic concurrency controls.

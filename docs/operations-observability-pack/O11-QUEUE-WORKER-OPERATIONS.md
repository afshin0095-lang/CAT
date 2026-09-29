# O11 — Queue & Worker Operations

CAT asynchronous work follows durable delivery principles.

```text
Command → Durable Queue → Claim → Execute → Ack
                         │
                         └→ Retry / Dead Letter
```

Workers must use bounded concurrency, explicit visibility/lease semantics, idempotent handlers, retry backoff, and dead-letter handling. Queue age and depth are first-class operational metrics.

A worker must never acknowledge work before the domain side effect is durably committed according to that operation's consistency contract.
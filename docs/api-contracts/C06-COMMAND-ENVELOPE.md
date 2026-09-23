# C06 — Command Envelope

Commands represent requested state-changing intent.

```json
{
  "command_id": "cmd_01...",
  "command_type": "opportunity.revalidate.request",
  "schema_version": 1,
  "occurred_at": "2026-01-01T00:00:00Z",
  "correlation_id": "cor_01...",
  "causation_id": "evt_01...",
  "principal_id": "usr_01...",
  "tenant_id": "ten_01...",
  "payload": {}
}
```

Command handlers validate authorization, schema, invariants, idempotency, and optimistic concurrency before changing durable state. A command is not a fact and must never be emitted as though it were a completed event.

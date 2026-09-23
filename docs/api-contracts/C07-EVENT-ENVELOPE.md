# C07 — Event Envelope

Events represent durable facts that already happened.

```json
{
  "event_id": "evt_01...",
  "event_type": "opportunity.revalidated",
  "schema_version": 1,
  "occurred_at": "2026-01-01T00:00:00Z",
  "aggregate_type": "opportunity",
  "aggregate_id": "opp_01...",
  "aggregate_revision": 8,
  "correlation_id": "cor_01...",
  "causation_id": "cmd_01...",
  "tenant_id": "ten_01...",
  "payload": {}
}
```

Consumers must be idempotent. Event order is guaranteed only within the explicitly documented ordering boundary. Consumers must tolerate redelivery and unknown future event fields according to schema policy.

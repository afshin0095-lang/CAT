# S12 — Webhook Security

Inbound provider callbacks are hostile until verified.

```text
HTTP Request
   ↓
Read raw body
   ↓
Verify signature
   ↓
Validate timestamp/replay window
   ↓
Validate schema
   ↓
Deduplicate provider event ID
   ↓
Authorize tenant/provider mapping
   ↓
Create command/fact
```

Signature verification must use the provider's canonicalization rules. Parsed JSON must never be re-serialized before verifying a signature scheme that signs the raw body. Duplicate events must be safely idempotent.

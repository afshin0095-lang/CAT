# C13 — Webhook Contract

Inbound webhooks are untrusted external messages until authenticated, schema-validated, deduplicated, and authorized.

```text
Provider → Signature Verification → Schema Validation → Deduplication → Command → Domain
```

Requirements:

- verify signature before trusting payload content;
- enforce timestamp/replay windows where supported;
- persist provider event identifiers for deduplication;
- acknowledge only after the accepted processing boundary is reached;
- do not perform long synchronous work before acknowledgement when provider semantics discourage it;
- retain raw payloads only when required and under explicit privacy/retention controls.

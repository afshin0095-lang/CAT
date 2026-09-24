# S14 — API Security

Every API endpoint follows the same security pipeline:

```text
Request → TLS/Edge → Authentication → Authorization → Validation → Rate Limit → Domain → Audit
```

Security-sensitive mutations use CSRF protections where browser sessions are involved, strict content types, bounded payload sizes, request IDs, replay protection where applicable, and safe error envelopes.

Authorization is performed for the requested resource and action, not merely at route-group level. Administrative endpoints require explicit elevated policy.

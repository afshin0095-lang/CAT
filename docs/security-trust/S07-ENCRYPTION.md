# S07 — Encryption Policy

CAT protects data in transit and at rest according to classification and operational requirements.

```text
Client ──TLS──> Edge ──TLS/mTLS where required──> Services
                                   │
                                   └── encrypted storage
```

TLS is mandatory for external service communication. Sensitive persisted data uses managed encryption mechanisms where supported. Encryption keys are separated from application data and are not embedded in source code.

Encryption does not replace authorization: decrypted data remains subject to tenant and resource policy.

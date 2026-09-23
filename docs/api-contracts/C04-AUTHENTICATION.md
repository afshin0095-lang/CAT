# C04 — Authentication Contract

Authentication establishes **who** is making a request; authorization decides **what** that principal may do.

## Requirements

- Credentials are never persisted in plaintext application records.
- Access tokens are short-lived where practical; refresh credentials are separately protected.
- Machine-to-machine identity uses scoped credentials.
- Every authenticated request receives a stable principal identity and authentication method in request context.
- Authentication failures use the same externally safe error shape and do not reveal credential validity details.
- Secret values are redacted from logs, traces, events, and audit payloads.

```text
Credential → Authentication Gateway → Principal → Authorization Context
```

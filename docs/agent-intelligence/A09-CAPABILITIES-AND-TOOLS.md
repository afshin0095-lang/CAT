# A09 — Capabilities & Tools

Tools are exposed as typed capabilities through one gateway.

```text
Agent → Capability Request → Policy → Budget → Tool Adapter → Result Validator
```

A capability declares input/output schemas, side-effect class, required permission, cost class, timeout, idempotency behavior, and audit requirements. Tool adapters hide vendor SDKs. Direct network, filesystem, database, shell, secret-store, and payment access is prohibited unless represented by an explicitly registered capability.

Side-effect classes: `read`, `compute`, `external-write`, `financial`, `administrative`.

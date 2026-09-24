# A03 — Agent Identity

Every agent has a stable logical identity independent of a model instance.

```text
agent_id
  ├─ class
  ├─ version
  ├─ owner/policy domain
  ├─ capabilities
  ├─ input/output contracts
  ├─ resource limits
  └─ lifecycle status
```

A model provider is an implementation detail. `agent.discovery.v1` can move between providers without changing its identity or domain contract. Invocation identity, correlation ID, parent run, and policy decision are retained for auditability. Agent identity is not user identity and must never be used as an authorization substitute for a human principal.

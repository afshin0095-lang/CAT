# O04 — Distributed Tracing

Tracing follows a request across synchronous calls, asynchronous jobs, agents, providers, and persistence.

```text
HTTP span
 ├─ auth span
 ├─ domain span
 ├─ agent span
 │   ├─ model span
 │   └─ tool span
 ├─ provider span
 └─ persistence span
```

Trace context must survive queue boundaries through explicit propagation. Sensitive model prompts and provider payloads are not automatically recorded as span attributes. Instead, traces carry bounded metadata and references to separately controlled evidence where required.

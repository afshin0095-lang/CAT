# O05 — Distributed Tracing

A trace represents one logical operation across synchronous and asynchronous boundaries.

```text
HTTP request
   │ trace_id
   ├── domain service span
   ├── database span
   ├── queue publish span
   │       └── worker continues trace context
   ├── provider span
   └── agent/tool spans
```

Trace propagation must be explicit across queues, jobs, provider calls, and agent execution. Span attributes are bounded and must respect privacy policy. Trace IDs are diagnostic identifiers, not authorization credentials.

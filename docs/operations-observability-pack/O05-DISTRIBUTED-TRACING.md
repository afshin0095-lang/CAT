# O05 — Distributed Tracing

Tracing follows work across synchronous calls, queues, agents, and provider adapters.

```text
HTTP Request
   │ trace/span
   ├── Domain command
   │     └── DB operation
   ├── Queue publish
   │     └── Worker span
   │           └── Provider call
   └── Agent tool call
```

Trace propagation is explicit at process and asynchronous boundaries. Span attributes use bounded cardinality and must not contain secrets or arbitrary prompt/provider payloads.

A trace explains execution flow; it is not itself an authorization mechanism or a replacement for audit records.
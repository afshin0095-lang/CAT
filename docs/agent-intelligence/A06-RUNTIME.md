# A06 — Agent Runtime

The runtime is the controlled execution shell around an agent implementation.

```text
Invocation
  → identity/policy check
  → context assembly
  → model inference
  → structured output validation
  → capability calls
  → result validation
  → persistence/events
  → telemetry
```

The runtime owns deadlines, cancellation, budgets, retry classification, tracing, and isolation. It does not own domain business rules. Model output is never directly persisted as authoritative state.

# A04 — Agent Lifecycle

```mermaid
stateDiagram-v2
  [*] --> Registered
  Registered --> Ready
  Ready --> Running
  Running --> Waiting
  Waiting --> Running
  Running --> Succeeded
  Running --> Failed
  Running --> Cancelled
  Failed --> Recovering
  Recovering --> Running
  Recovering --> Failed
  Succeeded --> Archived
  Cancelled --> Archived
  Failed --> Archived
```

Lifecycle state is execution metadata. Business truth remains in domain aggregates and events. Every transition has a reason and correlation context. Recovery is bounded by policy; an agent cannot retry indefinitely or change its own authority.

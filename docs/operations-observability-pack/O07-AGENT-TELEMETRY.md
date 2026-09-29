# O07 — Agent Telemetry

Every autonomous agent run is observable as a bounded execution.

```text
Agent Run
 ├─ goal/category
 ├─ policy version
 ├─ model/provider
 ├─ step count
 ├─ tool calls
 ├─ latency
 ├─ token/cost usage
 ├─ retries
 ├─ termination reason
 └─ outcome
```

Telemetry records metadata rather than unrestricted prompt contents. Tool-call telemetry identifies the capability invoked and result class. Agent runs exceeding step, time, cost, or concurrency budgets terminate through the runtime control plane.
# O11 — SLO / SLA / Error Budget Model

CAT distinguishes internal engineering targets from externally committed service guarantees.

An SLO is a target for a service-level indicator. An SLA is a contractual commitment. An error budget is the tolerated unreliability implied by the SLO.

```text
SLO target
   ↓
Allowed failure / latency budget
   ↓
Observed burn rate
   ├─ healthy → continue delivery
   ├─ elevated → investigate
   └─ exhausted → prioritize reliability work
```

Initial SLI families should cover availability, successful request rate, latency, queue freshness, provider success, and critical job completion. Exact numeric targets are deployment/business policy and must not be invented in implementation code.

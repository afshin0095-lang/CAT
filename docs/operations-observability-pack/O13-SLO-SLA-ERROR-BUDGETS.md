# O13 — SLO / SLA / Error Budgets

SLOs express internal reliability objectives; SLAs are external commitments when explicitly defined; error budgets quantify tolerated unreliability.

Typical CAT SLO dimensions:

| Dimension | Example measurement |
|---|---|
| availability | successful eligible requests |
| latency | p50/p95/p99 |
| freshness | age of valid opportunity data |
| queue health | maximum acceptable work age |
| provider reliability | successful normalized calls |

Targets must be selected from actual workload requirements and measured over explicit windows. Error-budget exhaustion should reduce risky change velocity and trigger reliability work rather than being ignored.
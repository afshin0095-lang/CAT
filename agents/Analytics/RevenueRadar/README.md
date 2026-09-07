# Revenue Radar

Daily CAT analytics agent for affiliate link health and revenue integrity.

## Runtime contract

- Runs once daily using the configured workspace timezone.
- Reads immutable click, conversion, attribution, commission, route-health, and revenue facts.
- Emits a report with observed facts, impact, confidence, and one recommended action.
- Treats missing telemetry as `not_measured`, never as zero revenue.
- Does not mutate routing, attribution, or provider state automatically.
- Escalates only evidence-backed critical conditions.

## Intended wiring

1. Scheduler starts the monitoring workflow.
2. `cat-orchestrator` claims the run with an idempotency key for the report date.
3. `cat-affiliate` supplies route and commission observations.
4. Event store supplies immutable raw events.
5. The report is persisted and published through the event bus.
6. A human-approved remediation can be turned into a separate workflow.

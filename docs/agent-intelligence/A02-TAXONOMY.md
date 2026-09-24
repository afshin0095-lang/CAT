# A02 — Agent Taxonomy

Agents are classified by responsibility, not by model vendor.

| Class | Responsibility |
|---|---|
| Supervisor | coordinates bounded multi-agent work |
| Planner | turns goals into executable plans |
| Researcher | gathers and normalizes evidence |
| Discovery | finds affiliate opportunities/sources |
| Enrichment | fills missing structured facts |
| Evaluator | scores/ranks according to explicit policy |
| Optimizer | proposes measurable improvements |
| Revenue | analyzes monetization and commission outcomes |
| Risk | detects fraud, policy, anomaly, and data-quality signals |
| Reconciliation | compares expected vs observed external state |
| Publisher | prepares approved content/assets for publication |
| Analyst | produces read-only analytical outputs |
| Validator | checks schema, invariants, and policy |
| Recovery | handles bounded retry/replay workflows |

One runtime may host multiple classes, but each invocation has one declared primary responsibility.

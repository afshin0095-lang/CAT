# A11 — Agent Budgets

Every invocation receives explicit limits.

| Budget | Example |
|---|---|
| wall-clock | 30s |
| model calls | 20 |
| tool calls | 50 |
| recursion depth | 4 |
| output tokens | bounded |
| external cost | monetary ceiling |
| concurrent children | bounded |

Counters are monotonic and checked before execution. Exhaustion causes a controlled stop or policy-defined degradation. Retries consume budget; they do not reset it. Child agents receive a subset of the parent remaining budget.

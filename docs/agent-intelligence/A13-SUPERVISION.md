# A13 — Supervisor & Orchestration

The supervisor owns coordination, not domain truth.

Responsibilities: accept goals, establish budgets, select approved agents, sequence dependencies, handle cancellation, collect results, invoke validators, and stop unsafe or non-progressing execution.

```text
Supervisor
 ├─ Planner
 ├─ Research/Discovery workers
 ├─ Validators
 ├─ Recovery workers
 └─ Finalizer
```

The supervisor cannot approve its own policy exceptions. Cycles, runaway delegation, repeated identical failures, and budget exhaustion terminate the run or escalate it according to policy.

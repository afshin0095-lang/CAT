# O16 — Disaster Recovery

CAT recovery is tiered by business criticality.

```text
Failure
  ├─ process → restart
  ├─ instance → reschedule
  ├─ service → fail over
  ├─ dependency → degrade / queue
  ├─ database → restore / failover
  └─ region → disaster recovery procedure
```

Recovery plans specify dependencies, order of operations, data consistency checks, credential recovery, DNS/traffic changes, verification, and rollback. Autonomous agents are disabled or constrained during an incident until their required dependencies and policies are restored.

# O15 — Disaster Recovery

Disaster recovery is defined by RPO and RTO.

```text
Failure
  ↓
Detect
  ↓
Contain / Freeze Risky Automation
  ↓
Recover Infrastructure/Data
  ↓
Validate Integrity
  ↓
Restore Traffic
  ↓
Reconcile Missed Work
```

Critical asynchronous work must be replayable or reconstructible from durable facts. Recovery must explicitly account for duplicate provider callbacks, queued commands, in-flight agent runs, and partially completed external operations.
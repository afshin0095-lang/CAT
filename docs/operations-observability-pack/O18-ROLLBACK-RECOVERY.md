# O18 — Rollback & Safe Recovery

Rollback is a controlled state transition, not simply a binary replacement.

```text
Incident
  ↓
Freeze risky automation
  ↓
Identify known-good artifact/config
  ↓
Rollback application/config
  ↓
Reconcile database/events/queues
  ↓
Verify health + correctness
  ↓
Resume automation gradually
```

If a database migration is irreversible, application rollback must use a compatibility release rather than pretending the old binary can safely run. External side effects require reconciliation and idempotency rather than blind replay.
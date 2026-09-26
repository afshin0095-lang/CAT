# O17 — Rollback Strategy

Rollback is a planned capability, not an emergency improvisation.

Rollback targets can include application version, feature flag, agent policy, provider routing, configuration, or deployment artifact. Database rollback is treated separately because destructive schema reversal can be unsafe.

```text
Detect regression
      ↓
Freeze expansion
      ↓
Identify safe target
      ↓
Rollback / disable capability
      ↓
Verify health + data integrity
      ↓
Resume controlled traffic
      ↓
Root-cause review
```

Forward fixes are preferred when schema/data changes cannot safely be reversed. Every production migration must document its compatibility and recovery strategy.

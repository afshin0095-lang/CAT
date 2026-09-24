# O18 — Rollback Strategy

Rollback must be an explicit engineering capability.

Rollback targets may include application artifact, configuration version, feature flag, provider route, agent policy, or traffic allocation.

```text
Detect regression
   ↓
Freeze expansion
   ↓
Select known-good version
   ↓
Rollback / disable capability
   ↓
Verify health + data integrity
   ↓
Resume gradually
```

Database rollback is not assumed to mean reversing SQL. Prefer additive, backward-compatible migrations and forward fixes when irreversible data transformations have occurred.

# O16 — Deployment Strategy

Deployments are immutable, observable, reversible operations.

```text
Build → Verify → Package → Deploy Candidate → Health Gate → Gradual Traffic → Full Rollout
                                      │
                                      └──────────────→ Rollback
```

Deployment artifacts must be versioned and traceable to source. Configuration is validated before startup. Database migrations are backward-compatible across the deployment transition whenever rolling upgrades require both old and new application versions.

Production rollout gates include automated tests, security checks, health checks, migration safety, and post-deployment telemetry.

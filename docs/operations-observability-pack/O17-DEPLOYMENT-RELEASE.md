# O17 — Deployment & Release Strategy

Deployments are immutable, observable, and reversible.

```text
Build → Test → Security Gates → Artifact
                    ↓
              Staging/Canary
                    ↓
               Verification
                    ↓
              Progressive Rollout
```

Artifacts include application version, configuration revision, dependency lock state, and relevant model/provider configuration references. Production rollout must have a defined abort condition and operator-visible health signals.

Database changes follow expand → migrate → contract when compatibility requires rolling deployments.
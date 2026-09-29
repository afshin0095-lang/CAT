# O01 — Operations Principles

CAT operations are designed around five properties: observable, bounded, idempotent, reversible, and auditable.

```text
Change → Observe → Validate → Promote
             │
             └── Detect → Contain → Rollback
```

Operational automation must fail safely. A missing metric, unavailable dependency, or ambiguous health signal must not be interpreted as proof of health.

Production changes are incremental and reversible. Configuration, model/provider selection, agent policies, and deployment artifacts are versioned so operators can identify exactly what was active at a given time.
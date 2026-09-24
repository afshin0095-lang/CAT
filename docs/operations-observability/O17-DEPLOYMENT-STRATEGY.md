# O17 — Deployment Strategy

Deployments use progressive exposure whenever operationally practical.

```mermaid
flowchart LR
A[Build] --> B[Static + Unit Gates]
B --> C[Integration Gates]
C --> D[Artifact]
D --> E[Small Exposure]
E --> F[Observe]
F --> G{Healthy?}
G -- Yes --> H[Expand]
G -- No --> I[Rollback]
```

Artifacts are immutable after release. Configuration and secrets are injected at runtime. Database migrations must be backward-compatible across the deployment boundary or explicitly coordinated.

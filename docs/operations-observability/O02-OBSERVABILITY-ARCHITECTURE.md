# O02 — Observability Architecture

CAT uses three primary telemetry signals: logs, metrics, and traces. Domain events provide business-level evidence and are distinct from infrastructure telemetry.

```mermaid
flowchart TB
S[Services / Agents / Workers] --> L[Logs]
S --> M[Metrics]
S --> T[Traces]
S --> E[Domain Events]
L --> C[Correlation / Context]
M --> C
T --> C
E --> C
C --> O[Observability Platform]
O --> A[Alerts]
O --> D[Dashboards]
O --> R[Incident Response]
```

Telemetry must preserve tenant/resource boundaries and redact sensitive values.

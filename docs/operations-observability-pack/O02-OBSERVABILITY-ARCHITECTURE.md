# O02 — Observability Architecture

CAT uses three primary telemetry signals plus structured audit evidence.

```text
                   ┌───────────────┐
                   │ CAT Workloads │
                   └───────┬───────┘
                           │
              ┌────────────┼────────────┐
              ↓            ↓            ↓
           Logs         Metrics       Traces
              └────────────┼────────────┘
                           ↓
                    Telemetry Bus
                           ↓
              ┌────────────┼────────────┐
              ↓            ↓            ↓
          Search        Metrics       Trace UI
```

Telemetry carries service, environment, version, tenant-safe context, request/correlation identifiers, and outcome. Sensitive data is excluded or redacted before emission.

Observability must not become an unrestricted data-exfiltration path: telemetry access follows the same security classification principles as application data.
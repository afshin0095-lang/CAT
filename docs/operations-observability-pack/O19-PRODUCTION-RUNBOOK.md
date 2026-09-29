# O19 — Production Runbook

## Start-of-day checks

1. Verify service health and readiness.
2. Review error-rate and latency anomalies.
3. Review queue depth/age and failed jobs.
4. Review provider availability and quota signals.
5. Review model/API spend against budget.
6. Check recent deployments and configuration changes.

## Incident checks

```text
Identify → Scope → Contain → Diagnose → Recover → Verify → Record
```

Operators should first inspect correlation IDs and traces, then service logs and dependency telemetry. Risky autonomous capabilities can be disabled independently when possible.

## End-of-change checks

Confirm health, queue convergence, error rates, provider success, and absence of unexpected cost spikes before declaring a change complete.
# O12 — Alerting

Alerts represent actionable conditions, not every error.

Alert design includes:

- condition;
- severity;
- affected service/capability;
- evidence window;
- owner/runbook;
- suppression/deduplication policy;
- recovery condition.

```text
Metric/Event → Rule → Deduplicate → Route → Operator/Automation → Recovery
```

Alerts must avoid high-cardinality explosions and must not expose sensitive payloads. Automated remediation is permitted only for predefined, reversible actions.
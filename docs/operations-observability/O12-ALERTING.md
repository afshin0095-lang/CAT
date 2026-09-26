# O12 — Alerting Strategy

Alerts represent actionable conditions, not every anomaly.

Alert classes:

1. **Critical:** immediate human/operator action required.
2. **High:** significant degradation or security risk.
3. **Warning:** sustained trend requiring investigation.
4. **Informational:** dashboard/event only.

Good alerts include symptom, impact, scope, first diagnostic action, runbook reference, and escalation policy. Alerts should be based on stable signals such as error-rate burn, queue age, provider failure, saturation, security events, or data-integrity violations.

Alert storms are controlled through grouping, deduplication, cooldowns, and dependency-aware suppression.

# O14 — Alerting Architecture

Alerts represent actionable conditions, not every error.

A useful alert contains:

1. what failed;
2. impact;
3. start time;
4. affected scope;
5. likely dependency;
6. runbook link;
7. current mitigation;
8. escalation owner.

Alert classes: availability, latency, saturation, data integrity, security, cost anomaly, provider degradation, and business-critical processing failure.

Repeated identical alerts should be grouped to prevent incident amplification.

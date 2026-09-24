# S17 — Fraud & Risk Security

Affiliate systems require a dedicated risk boundary because financial incentives create adversarial behavior.

Risk signals may include abnormal click/conversion patterns, impossible timing, duplicate identifiers, provider inconsistencies, sudden traffic changes, and policy violations.

```text
Signals → Feature Normalization → Risk Policy → Decision
                                      │
                         ┌────────────┼────────────┐
                         ↓            ↓            ↓
                       Allow       Review       Block
```

Risk scores are decision inputs, not proof of wrongdoing. High-impact decisions should retain explainable evidence and support human review according to policy.

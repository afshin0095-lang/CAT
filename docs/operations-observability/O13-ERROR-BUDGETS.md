# O13 — Error Budgets

An error budget converts reliability targets into an explicit change-management signal.

```text
SLO target → allowed unreliability → consumed budget
                                      │
                         ┌────────────┴────────────┐
                         ↓                         ↓
                  normal delivery          slow / freeze risky change
```

When budget consumption rises, CAT can reduce rollout scope, pause non-essential automation, increase review requirements, or prioritize reliability work. Budget policy must distinguish platform faults from provider outages and planned maintenance.

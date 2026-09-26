# O07 — Cost Telemetry

CAT treats AI and provider cost as an observable resource.

```text
Operation
  ↓
Provider / Model
  ↓
Usage measurement
  ├─ input units
  ├─ output units
  ├─ request count
  ├─ duration
  └─ estimated monetary cost
  ↓
Budget policy
  ↓
Allow / throttle / reject / review
```

Cost telemetry is advisory unless a budget policy explicitly enforces it. Monetary values must use deterministic decimal/integer representations rather than binary floating-point for accounting decisions. Provider prices are versioned configuration, not hard-coded assumptions in domain logic.

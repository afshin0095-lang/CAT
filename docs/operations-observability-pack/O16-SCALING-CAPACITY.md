# O16 — Scaling & Capacity

CAT scales by separating stateless request handling, durable work, stateful persistence, and provider/model concurrency.

```text
                Edge/API
                   │
          ┌────────┴────────┐
          ↓                 ↓
     Stateless API      Queue/Bus
                            ↓
                    Elastic Workers
                            ↓
                   Domain/Provider
                            ↓
                       Storage
```

Capacity planning tracks CPU, memory, database connections, queue throughput, provider quotas, model limits, and cost. Scaling one layer must not silently overload another.

Backpressure is preferred to unbounded concurrency. Provider quotas are treated as capacity constraints, not merely retry conditions.
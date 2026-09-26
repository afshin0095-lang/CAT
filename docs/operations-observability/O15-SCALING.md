# O15 — Scaling Strategy

CAT scales by separating stateless request processing from durable work and stateful persistence.

```text
                 ┌─ API replicas ─┐
Traffic → Edge ──┼─ Worker pool ──┼→ Services
                 └─ Agent pool ───┘
                         │
                    Durable Queue
                         │
                   Persistence
```

Scale dimensions include request concurrency, queue consumers, agent workers, provider concurrency, database connections, and cache capacity. Each has an explicit limit to prevent one scaling dimension from overwhelming another.

Horizontal scaling is preferred for stateless components. Stateful systems scale through controlled partitioning, replication, indexing, or managed capacity according to their technology.

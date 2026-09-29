# O10 — Health, Readiness & Liveness

Health signals have distinct semantics.

```text
Liveness  = process can continue running
Readiness = instance can safely receive work
Health    = broader dependency/service condition
```

A process should not report readiness merely because its HTTP server is alive. Critical dependencies are evaluated according to the workload's actual requirements. Temporary provider degradation should normally affect capability readiness rather than falsely declaring the entire process dead.
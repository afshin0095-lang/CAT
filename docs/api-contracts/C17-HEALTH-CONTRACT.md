# C17 — Health Contract

Health endpoints distinguish process liveness from dependency readiness.

```text
/liveness  → process can execute its basic loop
/readiness → required dependencies and initialization are usable
/health    → structured component/dependency state
```

Health responses must avoid leaking secrets or sensitive infrastructure details. A degraded optional dependency must not be reported as total service failure unless the dependency is required for the advertised capability.

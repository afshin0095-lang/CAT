# O11 — Health Checks

CAT distinguishes process health from dependency readiness.

| Check | Meaning |
|---|---|
| liveness | process is alive and able to make progress |
| readiness | instance can receive its intended workload |
| startup | initialization has completed |
| dependency health | required dependency is reachable/usable |

A transient optional dependency should not necessarily make the whole service fail liveness. Health endpoints must not expose secrets, internal topology, or unrestricted dependency diagnostics.

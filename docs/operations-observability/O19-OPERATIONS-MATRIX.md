# O19 — Operations Responsibility Matrix

| Area | Primary responsibility | Evidence |
|---|---|---|
| API | service runtime | request/error/latency telemetry |
| Agents | agent runtime | execution/tool/cost telemetry |
| Providers | adapter layer | normalized provider health |
| Queues | worker infrastructure | depth/age/retry metrics |
| Database | persistence layer | latency/error/saturation |
| Security | security boundary | audit/security events |
| Deployment | release system | artifact/version/rollout evidence |
| Recovery | operations | restore/recovery verification |

The matrix is a starting ownership contract. Concrete teams, on-call rotations, and escalation channels are deployment-specific configuration.

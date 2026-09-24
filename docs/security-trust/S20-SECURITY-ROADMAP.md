# S20 — Security Implementation Roadmap

Security implementation proceeds in dependency order.

```text
Identity
  ↓
Authorization + Tenant Context
  ↓
Secret Boundary
  ↓
Capability Gateway
  ↓
Input/Output Validation
  ↓
Audit + Observability
  ↓
Threat Detection / Risk
  ↓
Incident Automation
```

## Implementation gates

**Gate 1:** identity and authorization primitives exist before privileged endpoints.

**Gate 2:** capability enforcement exists before autonomous agents receive tools.

**Gate 3:** secret management exists before real provider credentials are connected.

**Gate 4:** audit and correlation exist before financial automation is enabled.

**Gate 5:** security regression tests pass before production deployment.

No later feature is allowed to bypass an earlier security gate merely to accelerate development.

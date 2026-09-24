# A12 — Agent Communication

Agents communicate through typed commands, results, events, and task messages. Shared mutable state is prohibited.

```mermaid
sequenceDiagram
 participant S as Supervisor
 participant P as Planner
 participant W as Worker
 participant V as Validator
 S->>P: Task
 P-->>S: Plan
 S->>W: Step command
 W-->>V: Candidate result
 V-->>S: Validated result
 S-->>W: Next bounded step
```

Messages carry correlation ID, causation ID, sender/receiver identities, schema version, deadline, and idempotency information where applicable. Agent-to-agent communication cannot bypass the capability gateway or policy layer.

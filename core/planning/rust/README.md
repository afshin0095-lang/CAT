# CAT Planning Core

Deterministic planning foundation for CAT OMNISYSTEM.

## Scope

The crate establishes the first executable planning contract:

- typed plan and step identity;
- ordered steps and explicit prerequisite edges;
- dependency-cycle detection;
- deterministic structural validation;
- immutable append-only planning trace records;
- a small builder API suitable for Decision Engine and future Orchestrator integration.

## Architectural boundary

Planning does not execute actions and does not mutate domain truth. A plan is a proposed execution graph. Validation determines whether the graph is structurally admissible; later policy/decision layers determine whether it is authorized to execute.

The dependency model intentionally permits required and optional prerequisites. Execution semantics, retries, leases, scheduling, compensation, and durable orchestration remain outside this foundation and will be added by the Orchestrator / Workflow Core.

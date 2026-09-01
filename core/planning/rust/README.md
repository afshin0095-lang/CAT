# CAT Planning Core

Deterministic planning foundation for CAT OMNISYSTEM.

## Scope

The crate establishes the first executable planning contract:

- typed plan and step identity;
- ordered steps and explicit prerequisite edges;
- dependency-cycle detection;
- deterministic structural validation;
- deterministic level-based execution scheduling;
- immutable append-only planning trace records;
- a small builder API suitable for Decision Engine and future Orchestrator integration.

## Scheduling contract

`plan::schedule_plan` compiles a validated plan graph into deterministic execution levels. Steps in the same level have no unresolved prerequisites between them and MAY be considered for parallel execution by a future Orchestrator. Ordering inside each level follows the plan's explicit step order.

The scheduler is intentionally execution-free: it does not call tools, acquire leases, mutate domain state, or retry work. Those responsibilities belong to the Workflow / Orchestrator layers.

## Architectural boundary

Planning does not execute actions and does not mutate domain truth. A plan is a proposed execution graph. Validation determines whether the graph is structurally admissible; later policy/decision layers determine whether it is authorized to execute.

The dependency model intentionally permits required and optional prerequisites. Execution semantics, retries, leases, compensation, durable orchestration, and runtime failure handling remain outside this foundation and will be added by the Orchestrator / Workflow Core.

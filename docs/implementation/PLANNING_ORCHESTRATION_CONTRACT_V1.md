# CAT Planning → Orchestrator Execution Contract v1

**Status:** Active foundation contract  
**Scope:** Planning Core and future Orchestrator integration

## Purpose

This document defines the boundary between the deterministic Planning Core and the runtime Orchestrator.

Planning produces a validated execution graph. Orchestration consumes that graph and is responsible for runtime execution.

## Planning owns

- plan identity and version;
- business goal representation;
- step identity and step kind;
- prerequisite relationships;
- structural validation;
- deterministic execution levels;
- append-only planning trace records.

## Orchestrator owns

- loading an approved plan for execution;
- runtime step state;
- scheduling and dispatch;
- leases and concurrency controls;
- retries and retry budgets;
- timeout and cancellation handling;
- tool invocation;
- compensation execution;
- runtime failure handling;
- durable execution state;
- runtime telemetry and operational metrics.

## Mandatory boundary rules

1. Orchestrator MUST NOT mutate a plan's definition while executing it.
2. Orchestrator MUST preserve the plan identity and version it received.
3. Orchestrator MUST NOT bypass Decision Core approval or policy checks.
4. A planning-level containing multiple steps means those steps are candidates for parallel execution; the Orchestrator MAY execute them concurrently only after applying its own safety, resource, dependency, and policy checks.
5. Runtime state belongs to execution state, not to the immutable plan definition.
6. Retries MUST NOT silently create a new plan version.
7. Every externally observable execution transition SHOULD be represented by an event suitable for the EventBus and audit trail.
8. Failure handling MUST be explicit: retry, compensate, pause for approval, cancel, or fail terminally.
9. Tool calls MUST remain behind the authorized execution boundary of the relevant core/service.
10. Durable execution state MUST be recoverable after process restart.

## Canonical flow

```text
Decision
   |
   v
Approved Plan
   |
   v
Planning Validation
   |
   v
Deterministic Schedule
   |
   v
Orchestrator Admission
   |
   +---- policy / resource / concurrency checks
   |
   v
Runtime Step Execution
   |
   +---- success ----> next runnable level
   |
   +---- retry ------> retry policy
   |
   +---- failure ----> compensate / pause / cancel / terminal failure
   |
   v
Execution Result + Events + Trace
```

## Example

A commerce campaign may contain:

```text
Level 0
├── Research product
└── Collect market signals

Level 1
└── Score opportunity

Level 2
└── Human approval

Level 3
├── Generate content
└── Prepare affiliate links

Level 4
└── Publish campaign
```

The Planning Core decides this dependency structure. The Orchestrator decides how to execute it safely in the current runtime.

## Future integration points

The next implementation layer should expose a small execution-facing API around:

- `ExecutionId`
- `ExecutionStatus`
- `StepExecutionStatus`
- admission/start;
- pause/resume;
- cancel;
- retry;
- step completion/failure;
- recovery after restart.

That API should depend on stable plan contracts rather than reaching into Planning internals.

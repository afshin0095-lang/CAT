# Orchestrator P0 Checklist

- deterministic execution cursor
- explicit retry decision
- exported execution state/event contracts
- tests at the core boundary
- documented separation between orchestration decisions and worker side effects

Next integration target: connect the orchestration state machine to durable EventBus-backed execution records and worker dispatch.

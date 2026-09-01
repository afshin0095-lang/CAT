# Orchestrator Execution P0 Flow

```text
Validated Plan
  -> Workflow Instance
  -> Ready Step Discovery
  -> Execution Cursor
  -> Execution Request
  -> Worker / Dispatcher
  -> Result
     -> Success -> unlock dependent steps
     -> Failure -> RetryDecision
          -> Retry -> reschedule
          -> Exhausted -> failure / recovery
```

The current P0 contracts only describe and control execution state. They do not perform external actions themselves.

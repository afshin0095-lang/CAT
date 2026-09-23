# C08 — Agent Contract

Agents are policy-governed workers, not unrestricted application services.

## Invocation

```json
{
  "run_id": "run_01...",
  "agent_id": "agent.discovery.v1",
  "capability": "opportunity.discover",
  "input": {},
  "budget": {"max_steps": 20, "deadline_ms": 30000},
  "correlation_id": "cor_01..."
}
```

## Guarantees

- Agent identity and capability are authenticated.
- Policy is evaluated before tool execution.
- Tool calls are mediated through the capability gateway.
- Side effects are auditable.
- Time, cost, retry, and recursion budgets are explicit.
- Agent output is treated as untrusted input until domain validation succeeds.
- An agent cannot directly obtain raw secrets or bypass domain repositories.

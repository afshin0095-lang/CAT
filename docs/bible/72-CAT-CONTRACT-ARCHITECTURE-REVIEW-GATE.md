# CAT Contract Architecture Review Gate

**Status:** L2 — Architecture specified

## Review questions

### Semantic integrity
- Does every capability represent a real CAT responsibility?
- Are Agent, Capability, Tool, Connector and Provider boundaries explicit?
- Are reasoning, decision, planning and execution still separate?

### Reliability
- Is every durable side effect idempotent or deduplicated?
- Are retryable and unknown outcomes distinguished?
- Is reconciliation defined for uncertain external state?

### Security
- Can the agent reach only authorized capabilities?
- Are secrets isolated?
- Are external instructions treated as untrusted data?
- Are network and economic boundaries explicit?

### Economics
- Can model/tool/provider costs be attributed?
- Can material spending be bounded before execution?
- Are expected and realized economics distinct?

### Operability
- Can every material action be correlated?
- Is there sufficient evidence to explain the decision and result?
- Are health, quota and provider degradation observable?

### Evolution
- Can providers change without rewriting business logic?
- Are contracts versioned?
- Is migration/rollback behavior defined?

## Gate outcome

The review gate produces one of:

```text
APPROVED
APPROVED_WITH_FOLLOW-UP
CHANGES_REQUIRED
BLOCKED
```

The result must be recorded with the relevant architectural change or implementation work.

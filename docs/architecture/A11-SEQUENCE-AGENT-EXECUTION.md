# A11 — Agent Execution and Recovery Sequence

**Status:** TARGET agent lifecycle sequence.

```mermaid
sequenceDiagram
    participant Sup as Supervisor
    participant Reg as Agent Registry
    participant Agent as Specialist Agent
    participant Policy as Policy Engine
    participant Tool as Capability Gateway
    participant Worker as Worker
    participant Store as Durable Store
    participant Eval as Evaluator
    participant Audit as Audit

    Sup->>Reg: resolve agent version/capabilities
    Reg-->>Sup: agent contract
    Sup->>Agent: start task
    Agent->>Policy: request capability
    Policy-->>Agent: allow / deny / approval-required
    alt allowed
        Agent->>Tool: typed capability call
        Tool->>Worker: durable execution
        Worker->>Store: claim/commit
        Worker-->>Tool: outcome
        Tool-->>Agent: result
    else approval required
        Policy-->>Sup: waiting for approval
    else denied
        Policy-->>Agent: rejected
    end
    Agent->>Eval: produce result + evidence
    Eval-->>Sup: quality/risk assessment
    Sup->>Audit: lifecycle evidence
    alt transient failure
        Sup->>Store: schedule retry
    else unknown external state
        Sup->>Worker: reconcile
    else repeated policy/quality failure
        Sup->>Reg: quarantine agent/version
    end
```

## Agent state machine

```text
REGISTERED
    ↓
VALIDATING → REJECTED
    ↓
READY → RUNNING → EVALUATING
                    ├→ READY
                    ├→ RETRY_WAIT → RUNNING
                    ├→ APPROVAL_WAIT
                    └→ QUARANTINED
```

## Recovery rules

- State transitions are durable.
- Worker restart must not erase ownership information.
- Retries are bounded and policy-aware.
- Side-effecting operations reconcile unknown remote state.
- Quarantine prevents a failing agent version from continuing autonomous side effects.
- Evaluation does not itself grant new authority.

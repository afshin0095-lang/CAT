# D05 — State Machines

## Opportunity
```mermaid
stateDiagram-v2
    [*] --> Active
    Active --> Stale: freshness threshold
    Stale --> Active: successful revalidation
    Stale --> Expired: expiry threshold
    Active --> Expired: policy expiry
    Expired --> Active: accepted fresh observation
```

## Revalidation request
```mermaid
stateDiagram-v2
    [*] --> Pending
    Pending --> Claimed
    Claimed --> Succeeded
    Claimed --> Failed
    Failed --> Pending: retry policy
    Failed --> DeadLetter: retry exhausted
```

## Agent run
`Created → Authorized → Running → Succeeded | Failed | Cancelled | Quarantined`.

Invalid transitions are rejected; state cannot be mutated by merely updating a projection.

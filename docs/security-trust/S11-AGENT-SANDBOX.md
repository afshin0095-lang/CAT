# S11 — Agent Sandbox

Agents execute inside explicit resource and capability boundaries.

```text
Agent
 ├─ identity
 ├─ tenant context
 ├─ capability allowlist
 ├─ time budget
 ├─ step budget
 ├─ cost budget
 ├─ concurrency budget
 └─ data-access scope
```

An agent cannot directly open arbitrary network connections, access arbitrary filesystem paths, query unrestricted tables, or invoke undeclared tools. High-risk tools may require an approval gate.

Sandbox enforcement must occur outside the model itself. A prompt saying "do not use X" is not a security control.

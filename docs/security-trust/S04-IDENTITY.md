# S04 — Identity Architecture

CAT distinguishes human, service, agent, provider, and workload identities.

```text
Principal
  ├── Human
  ├── Service
  ├── Agent
  ├── Provider
  └── Workload
```

Every authenticated principal receives a stable internal identity and context. Identity assertions are validated at the boundary where they are consumed. Agent identity does not imply human authority.

Authentication credentials must be short-lived where possible, scoped to the smallest useful audience, and revocable. Authorization decisions consume identity plus tenant/resource/action context.

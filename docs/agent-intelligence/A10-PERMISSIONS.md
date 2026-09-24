# A10 — Agent Permissions

Authorization is capability-based and contextual.

```text
principal + agent + capability + resource + operation + context → decision
```

Decisions are `allow`, `deny`, or `approval_required`. Default is deny. Permissions can constrain resource scope, tenant, provider, monetary value, rate, time window, and environment. An agent cannot grant itself permission, delegate a stronger permission, or convert a read capability into a write capability.

High-impact examples: publishing, financial mutations, credential rotation, destructive deletion, policy changes, and production administration.

# S02 — Trust Model

CAT models trust as explicit boundaries rather than assuming that components inside one deployment are trusted.

```text
Untrusted Internet
      │
      ▼
API/Webhook Boundary
      │ authenticated + validated
      ▼
Application Boundary
      │ policy checked
      ▼
Agent/Capability Boundary
      │ scoped capability
      ▼
Domain Boundary
      │ invariant enforcement
      ▼
Persistence Boundary
```

External providers, model outputs, user-generated content, scraped pages, uploaded files, and agent-generated text are data inputs, not trusted instructions.

A component may be authenticated while remaining unauthorized for a particular action. Trust therefore has at least three dimensions: identity, authority, and data trust.

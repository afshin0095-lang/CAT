# S18 — Incident Response

CAT incident handling follows containment before recovery.

```text
Detect → Triage → Contain → Preserve Evidence → Eradicate → Recover → Review
```

Examples of immediate containment include revoking compromised provider credentials, disabling a capability, isolating an agent class, blocking a webhook source, or pausing a risky automation path.

Incident records preserve timestamps, correlation IDs, affected components, policy decisions, and actions taken. Recovery must include validation that security boundaries were restored before automation resumes.

# S01 — Security Principles

Security is a system property, not a perimeter feature.

## Core principles

1. **Least privilege** — identities receive only the capabilities required for the current task.
2. **Deny by default** — absence of an explicit permission is denial.
3. **Verify at every trust boundary** — authentication is not authorization.
4. **Treat external data as hostile** — provider payloads, webhooks, scraped content, files, prompts, and model output require validation.
5. **Separate facts from instructions** — untrusted content must never silently become executable policy.
6. **Minimize secrets** — credentials are referenced, scoped, rotated, and never exposed to agents or clients unnecessarily.
7. **Make sensitive actions auditable** — security and financial decisions need durable evidence.
8. **Fail closed on ambiguity** — uncertainty in authorization, identity, validation, or policy must not grant access.
9. **Contain blast radius** — tenants, agents, providers, workers, and credentials are isolated where practical.
10. **Security controls must be testable** — every important control has automated negative-path tests.

CAT must prefer explicit policy and typed boundaries over implicit framework behavior.

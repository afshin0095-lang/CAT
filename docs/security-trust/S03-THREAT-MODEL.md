# S03 — Threat Model

CAT threat modeling covers application, agent, provider, data, and operational risks.

| Area | Representative threats | Primary controls |
|---|---|---|
| API | credential theft, abuse, injection | authn/authz, validation, rate limits |
| Agents | prompt injection, privilege escalation, runaway execution | capability gateway, budgets, sandboxing |
| Providers | malicious/compromised response, quota abuse | adapter isolation, normalization, secret scoping |
| Data | leakage, tampering, tenant crossover | encryption, access policy, integrity checks |
| Webhooks | spoofing, replay, duplicate delivery | signature verification, timestamp checks, idempotency |
| Supply chain | compromised dependency/image/action | pinning, scanning, provenance review |
| Operations | secret exposure, unsafe deployment | secret manager, CI policy, least privilege |

Threats are prioritized by impact, likelihood, exploitability, and blast radius. Controls are documented with prevention, detection, and recovery paths.

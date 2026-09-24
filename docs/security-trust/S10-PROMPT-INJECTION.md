# S10 — Prompt Injection Defense

Prompt injection is treated as an untrusted-data problem, not solved by wording alone.

```text
External Content
      ↓
Ingestion / Sanitization
      ↓
Content is DATA
      ↓
Agent Context Builder
      ↓
Policy + Capability Check
      ↓
Tool Invocation
```

Rules:

- Retrieved pages and provider responses cannot grant permissions.
- Instructions embedded in external content are data unless an authorized policy explicitly treats them as commands.
- Tool descriptions and system policy are separated from retrieved content.
- Sensitive operations require structured arguments and authorization rather than free-form model intent.
- Model output is validated before becoming a command.
- Security-critical decisions cannot rely solely on an LLM assertion.

Adversarial tests must include instruction smuggling, role confusion, data exfiltration attempts, tool-call manipulation, and indirect prompt injection through retrieved content.

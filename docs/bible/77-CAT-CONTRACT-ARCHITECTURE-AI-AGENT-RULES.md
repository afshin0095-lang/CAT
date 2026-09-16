# CAT Contract Architecture — AI Agent Rules

**Status:** L2 — Architecture specified

These rules are the compact operational memory for an AI agent working on CAT's contract layer.

## Before coding

```text
READ BIBLE
→ READ CONTEXT
→ FIND CONTRACT
→ FIND REGISTRY ENTRY
→ FIND EXISTING TYPE
→ FIND TESTS
→ INSPECT DEPENDENCIES
→ CODE
```

## Never

- invent a duplicate contract;
- bypass policy;
- embed provider-specific business semantics in domain code;
- expose secrets;
- treat model output as authority;
- retry uncertain side effects blindly;
- claim operational status without executable evidence;
- silently expand autonomy, data scope or economic authority.

## Always

- use canonical IDs;
- preserve versions;
- validate inputs and outputs;
- classify side effects;
- define idempotency;
- preserve provenance;
- emit appropriate telemetry;
- test failure and unknown states;
- update documentation with material contract changes.

## Core invariant

```text
WHAT CAT DOES  = CAPABILITY
HOW IT EXECUTES = TOOL / CONNECTOR / PROVIDER
WHO MAY DO IT   = POLICY
WHAT HAPPENED   = OUTCOME + EVIDENCE
```

This file is intentionally concise so it can be injected into AI-agent working context without replacing the detailed canonical chapters.

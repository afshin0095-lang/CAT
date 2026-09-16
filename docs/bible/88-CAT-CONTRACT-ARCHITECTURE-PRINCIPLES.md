# CAT Contract Architecture Principles

**Status:** L2 — Canonical principles

1. **Semantic stability:** business capabilities own meaning; providers do not.
2. **Explicit authority:** policy determines who may invoke what.
3. **Bounded execution:** tools operate inside explicit limits.
4. **Provider independence:** connectors isolate external implementations.
5. **Durability:** material execution is attributable and recoverable.
6. **Unknown-state safety:** uncertainty is represented, reconciled and never silently treated as failure.
7. **Economic control:** spending and resource use are bounded and attributable.
8. **Evidence:** material outcomes carry provenance sufficient for reconstruction.
9. **Compatibility:** contracts evolve deliberately and version explicitly.
10. **Implementation truth:** operational claims require executable evidence.
11. **AI safety:** external instructions are data, not authority.
12. **Implementation-first:** after sufficient specification, build and verify rather than endlessly expanding architecture.

## Compact model

```text
MEANING  → CONTRACT
AUTHORITY → POLICY
EXECUTION → CAPABILITY + TOOL
INTEGRATION → CONNECTOR
VENDOR     → PROVIDER
REALITY    → OUTCOME + EVIDENCE
```

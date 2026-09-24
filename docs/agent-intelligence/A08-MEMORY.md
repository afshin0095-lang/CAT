# A08 — Agent Memory

CAT separates memory by authority and retention.

| Memory | Purpose | Authority |
|---|---|---|
| Working context | current invocation | ephemeral |
| Run memory | execution trace/results | operational |
| Semantic memory | reusable normalized knowledge | governed |
| Episodic memory | prior successful/failed experiences | governed |
| Domain state | financial/business truth | domain repositories/events |

Memory retrieval is filtered by principal, tenant, policy, sensitivity, and relevance. Agents cannot treat retrieved text as truth without validation. Sensitive memory has explicit retention and deletion rules. Memory writes are observable and never used to bypass domain invariants.

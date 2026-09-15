# 38 — Knowledge & Memory Data Model

**Status:** Architecture specification / target-state.

## 1. Definitions

- **Knowledge:** validated information CAT is willing to use as a reusable fact, relationship, rule or procedure.
- **Memory:** contextual record of prior observations, executions, interactions or learned state.
- **Evidence:** source material supporting a claim.
- **Provenance:** lineage describing where a datum came from and how it changed.

Knowledge is not automatically true; trust and freshness are explicit properties.

## 2. Memory classes

| Class | Lifetime | Purpose |
|---|---|---|
| Working | task/session | immediate context |
| Episodic | long-term | execution history and experiences |
| Semantic | long-term | reusable concepts/facts |
| Procedural | long-term | validated ways of performing work |
| Operational | short/medium | provider and system state |
| Economic | long-term | costs, revenue, budgets and outcomes |

## 3. Canonical object

```text
KnowledgeItem
 ├─ id
 ├─ type
 ├─ subject
 ├─ predicate
 ├─ object/value
 ├─ source_refs[]
 ├─ provenance
 ├─ trust_level
 ├─ confidence
 ├─ observed_at
 ├─ valid_from / valid_to
 ├─ version
 ├─ status
 └─ access_scope
```

## 4. Knowledge pipeline

```mermaid
flowchart LR
    S[Source] --> I[Ingest]
    I --> N[Normalize]
    N --> D[Deduplicate]
    D --> E[Extract Claims]
    E --> V[Validate]
    V --> P[Provenance]
    P --> K[Knowledge Store]
    K --> G[Graph / Retrieval Index]
    G --> A[Agent Context]
```

## 5. Trust levels

Use explicit trust categories such as:

`UNKNOWN → OBSERVED → CORROBORATED → VALIDATED → GOVERNED`

Trust must not be inferred solely from language confidence. A polished generated statement without evidence remains unsupported.

## 6. Conflict resolution

When sources disagree:

1. retain both observations and provenance;
2. compare source authority, recency and scope;
3. determine whether the conflict is temporal, geographic, semantic or factual;
4. avoid destructive overwrite when uncertainty matters;
5. escalate unresolved high-impact conflicts.

## 7. Retrieval contract

Retrieval should consider:

`semantic relevance + lexical relevance + freshness + trust + scope + task importance`

The retrieval result must preserve source references so an agent can distinguish evidence from generated synthesis.

## 8. Memory writes

Agents should not write arbitrary long-term memory merely because something appeared in a conversation. Durable memory requires a declared memory type, scope, provenance and retention policy.

## 9. Privacy and retention

Sensitive or unnecessary information should not be retained. Retention, deletion, access and export policies belong to governance and must be enforceable independently of model behavior.

## 10. Learning boundary

A learned pattern is a proposal until evaluation and governance promote it. Online adaptation must not silently change critical policy or financial constraints.

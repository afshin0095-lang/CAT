# CAT OMNISYSTEM Bible — AI Reading Order V2

**Status:** Canonical navigation guide

## 1. Fast path for a new AI agent

```text
README.md
  -> docs/bible/README.md
  -> docs/CAT-ENGINEERING-CONSTITUTION.md
  -> context/00_PROJECT_CONTEXT.md
  -> context/02_PROJECT_RULES.md
  -> context/14_CODING_STANDARD.md
  -> context/15_DIRECTORY_STRUCTURE.md
  -> relevant Bible domain chapter
  -> relevant contract
  -> actual source + tests + migrations
```

## 2. Architecture path

Read in this order when implementing cross-domain architecture:

1. 00 Vision & Identity
2. 01 System Architecture
3. 02 Agent Operating System
4. 03 Capability Map
5. 08 Data Architecture
6. 09 Security, Trust & Governance
7. 10 Infrastructure, DevOps & Scaling
8. 19 Agent SDK & Capability Protocol
9. 20 Multi-Agent Collaboration
10. 21 Tool/Connector/Provider Ecosystem
11. 25 Decision, Planning & Autonomy
12. 26 Observability & Operations
13. 27 Database & Domain Model
14. 29 API/EventBus/Integration Contracts
15. 33–54 agent/capability/tool/provider specifications
16. 108 Master Domain Map
17. 109 Economic Flywheel
18. 110 Agent Execution Lifecycle
19. 111 Identity & Access
20. 112 NFR/SLO
21. 113 Data Lifecycle
22. 114 Failure/Recovery
23. 115 AI Safety
24. 116 Documentation Governance

## 3. Implementation path

For a feature inside an existing domain:

```text
Domain Bible chapter
 -> contract
 -> ADR
 -> package README
 -> source
 -> tests
 -> migration (if required)
 -> observability
 -> runbook
```

## 4. AI rule

Do not read historical status files as if they were current architecture. Prefer the latest canonical chapter, then verify claims against code and contracts.

## 5. Conflict resolution

If two documents disagree:

1. inspect executable behavior;
2. inspect contracts;
3. inspect ADRs and recent decisions;
4. identify whether one document is historical;
5. update the canonical navigation/index;
6. never silently choose a preferred interpretation.

## 6. Context minimization

Agents should load only the context necessary for the requested task, but must not omit a document that defines a security, data, economic or lifecycle invariant relevant to the change.

# CAT — Commerce AI Trinity — Project Overview (Part 1)

> **Primary orientation document for the CAT ecosystem**<br>
> This document explains what CAT is, why it exists, what it does, who it serves, how value is created, and where its boundaries are. Implementation detail is intentionally subordinate to product identity and operating intent.

---

## Document Metadata

| Field | Value |
|---|---|
| **Document ID** | CAT-OVR-01 |
| **Document name** | Project Overview |
| **Part** | Part 1 of the multi-part Project Overview |
| **Status** | Active — Part 1 complete; continuation parts remain |
| **Version** | 1.0.0 (Part 1) |
| **Project** | CAT (Commerce AI Trinity) |
| **Company** | Omni System |
| **Owner** | Lead Repository Architect, CAT Project |
| **Created** | 2026-08-02 |
| **Last updated** | 2026-08-02 |
| **Primary audience** | New engineers, AI coding agents, architects, product managers, operators, future contributors, and enterprise stakeholders |
| **Source of truth** | This document is authoritative for the project identity, executive orientation, product boundaries, operating model, capability map, and value model covered in Part 1. `context/00_PROJECT_CONTEXT.md` remains the root context authority for the overall project philosophy. |
| **Dependencies** | `context/00_PROJECT_CONTEXT.md`, `README.md`, `ROADMAP.md`, `CONTRIBUTING.md`, `CAT Front.md`, and the repository AI workspace |
| **Related context** | `context/02_PROJECT_RULES.md`, `context/03_TECH_STACK.md`, `context/04_ARCHITECTURE.md`, `context/05_AGENTS.md`, `context/06_KNOWLEDGE_ENGINE.md`, `context/07_TREASURY_CORE.md`, `context/08_AFFILIATE_ENGINE.md`, `context/09_CONTENT_ENGINE.md`, `context/10_UI_UX.md`, `context/11_DESIGN_LANGUAGE.md`, `context/12_DECISIONS.md`, `context/13_TERMINOLOGY.md`, `context/99_AI_BOOTSTRAP.md` |

### Authority and Conflict Resolution

CAT has a deliberately layered documentation system. The layers answer different questions and must not be collapsed into one document:

1. **`context/00_PROJECT_CONTEXT.md`** defines the highest-level project context, philosophy, mission, vision, and foundational direction. It is the root context document.
2. **This document** turns that direction into an executive and engineering orientation. It answers what the product is, what it does, where it begins and ends, and how a reader should reason about it.
3. **Numbered context documents `02–19`** provide the detailed rules, stack, architecture, agent, domain, design, operational, and development specifications.
4. **`README.md`, `ROADMAP.md`, `CONTRIBUTING.md`, and `CAT Front.md`** provide repository-facing orientation, roadmap intent, contribution expectations, and the visual-language source material.
5. **`adr/` and `decisions/`** are the decision-record surfaces. At the time of this part, their files are repository scaffolds and must not be treated as populated evidence for decisions that are not written elsewhere.

When two documents conflict, use the following order:

| Priority | Authority | Use |
|---:|---|---|
| 1 | Explicit current decision in the root context or an accepted decision record | Governs project direction |
| 2 | This Project Overview for the concepts covered here | Governs product identity and orientation |
| 3 | Dedicated numbered context document | Governs implementation detail in its domain |
| 4 | README, roadmap, contribution guide, visual notes | Governs repository orientation and declared intent |
| 5 | Draft, experimental, or historical material | May inform a proposal; does not override active direction |

If a contradiction is found, do not silently choose a convenient interpretation. Record the conflict, preserve the evidence, identify the higher-authority source, and propose the required documentation correction.

### Part 1 Reading Contract

Part 1 is intentionally the **what-before-how** layer. It establishes:

- The identity of CAT as an AI-native commerce operating system.
- The meaning of Commerce, AI, and Treasury as the three pillars.
- The problem CAT is intended to solve.
- The complete lifecycle CAT is intended to coordinate.
- The human/AI operating model and bounded autonomy.
- The capability and audience map.
- The explicit product boundaries and non-goals.
- The value model, success criteria, and current Phase A position.
- The official-versus-recommended-versus-future-versus-experimental classification used throughout this document.

Part 1 does **not** replace the detailed architecture, technology, security, prompt, deployment, agent, treasury, affiliate, content, or UI/UX specifications. Those documents are referenced at the point where a reader needs them.

---

## 1. How to Use This Overview

### Human Explanation

A new person should be able to read this document from top to bottom and answer five questions without opening the source code:

1. **What is CAT?**
2. **Why does CAT exist?**
3. **What work does CAT perform?**
4. **Who is responsible for decisions and approvals?**
5. **What is CAT explicitly not trying to become?**

This is not a feature catalog and it is not a sales brochure. It is the shared mental model for people who will make product, architecture, engineering, documentation, operations, or governance decisions. A feature belongs in CAT only when it supports the product identity and fits the boundaries defined here.

The preferred reading path is different for each audience:

| Reader | Start here | Then read | Primary question |
|---|---|---|---|
| New engineer | Sections 1–3 | Sections 5–7, then `context/02_PROJECT_RULES.md` and `context/03_TECH_STACK.md` | What am I building and under which constraints? |
| AI coding agent | Section 1, then the AI Context blocks | `context/00_PROJECT_CONTEXT.md`, `.ai/BOOTSTRAP.md`, `.ai/CONTEXT_ORDER.md`, then the relevant numbered context file | What context is authoritative before I change anything? |
| Architect | Sections 2–7 and 11 | `context/04_ARCHITECTURE.md`, `.ai/ARCHITECTURE_MAP.md`, decision records | Which boundaries and decisions must remain stable? |
| Product manager | Sections 2–5, 7, 9–11 | `ROADMAP.md`, domain context files, decision index | Which user and business outcomes justify the work? |
| Operator or supervisor | Sections 5–6, 8, and 10 | `context/07_TREASURY_CORE.md`, `context/08_AFFILIATE_ENGINE.md`, `context/09_CONTENT_ENGINE.md` | What can CAT do, and what must I approve? |
| Future contributor | Sections 1–3 and 9 | `CONTRIBUTING.md`, `context/14_CODING_STANDARD.md`, `context/19_DEVELOPMENT_GUIDE.md` | How do I extend CAT without violating its identity? |
| Enterprise stakeholder | Sections 2, 6, 8–11 | `context/17_SECURITY.md`, `context/16_DEPLOYMENT.md`, governance and decision records | How is autonomy controlled, audited, and integrated? |

The reader should not infer that a concept is implemented merely because it appears in the product vision. Current implementation status is stated in Section 11 and in `.ai/PROJECT_STATUS.md`.

### AI Context

An AI system must treat this document as orientation context, not as an invitation to invent implementation details. Before proposing code, the agent must:

- Load the root context and the AI workspace instructions.
- Identify which statements in this document are **Official Decisions**.
- Distinguish a current capability from a planned capability, a recommendation, a future idea, and an experimental concept.
- Follow links to the dedicated context document for implementation details.
- Check repository status before claiming that a subsystem exists.
- Preserve terminology exactly, especially `CAT`, `CATA`, `KATA`, Commerce, AI, Treasury, agent, approval, knowledge, and campaign.
- Ask for or record a decision when a requested change would alter product identity, authority boundaries, autonomy levels, or financial/reputational controls.

An AI agent must not convert a future vision statement into a completed feature, a planned technology into an installed dependency, or an empty scaffold into an implemented subsystem.

### Business Perspective

The overview is an alignment mechanism. It prevents product management, engineering, AI development, and operations from optimizing different products under the same name. A well-aligned CAT team spends time deciding how to create more value, not repeatedly redefining what the product is.

The business consequence of this document is practical:

- Product work can be evaluated against a stable mission.
- Engineering work can be prioritized by business capability rather than novelty.
- Enterprise conversations can describe controls and responsibilities precisely.
- Future partners can understand what CAT owns and what it expects integrations to provide.
- New team members can become productive without relying on undocumented oral history.

### Technical Perspective

The overview is a contract at the boundary between product intent and implementation. It defines the nouns, verbs, actors, lifecycle stages, authority boundaries, and maturity labels that downstream designs must use. It does not define database schemas or concrete service APIs; those belong to the relevant detailed documents.

A technical proposal is incomplete if it can be implemented but cannot be mapped to:

- A CAT capability or lifecycle stage.
- A responsible agent, human role, or service boundary.
- An approval and risk classification.
- A knowledge or audit outcome.
- A documented extension point.

### Visual Overview

```mermaid
flowchart TB
    Reader[Human or AI reader]
    Reader --> Root[00 Project Context<br/>Why and foundational direction]
    Root --> Overview[01 Project Overview<br/>What CAT is and how to orient]
    Overview --> Rules[02 Project Rules]
    Overview --> Stack[03 Tech Stack]
    Overview --> Architecture[04 Architecture]
    Overview --> Domains[05–19 Domain and operating context]
    Overview --> Bootstrap[99 AI Bootstrap]

    Style[Documentation Standard v2] --> Overview
    Status[.ai/PROJECT_STATUS.md] --> Reader

    style Overview fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Root fill:#7c3aed,color:#fff
    style Rules fill:#7c3aed,color:#fff
    style Stack fill:#7c3aed,color:#fff
    style Architecture fill:#7c3aed,color:#fff
    style Domains fill:#7c3aed,color:#fff
    style Bootstrap fill:#7c3aed,color:#fff
```

### Mermaid Diagram(s)

The following decision flow is the minimum reading protocol for an AI coding agent:

```mermaid
flowchart LR
    Start[Receive task] --> Workspace[Read .ai/README.md and BOOTSTRAP.md]
    Workspace --> Status[Read PROJECT_STATUS.md and TASKS.md]
    Status --> Root[Read context/00_PROJECT_CONTEXT.md]
    Root --> Overview[Read this overview]
    Overview --> Domain[Load task-specific context]
    Domain --> Decisions[Check accepted decisions and dependencies]
    Decisions --> Plan[Propose a bounded change]
    Plan --> Implement[Implement only after scope is clear]
    Implement --> Verify[Verify tests, docs, and traceability]
    Verify --> Update[Update status and relevant knowledge]
```

### Architecture Map

The orientation document sits between project philosophy and implementation context:

```text
Omni System
└── CAT product identity and mission
    ├── 00 Project Context: why, vision, mission, philosophy
    ├── 01 Project Overview: what CAT is, does, and excludes
    ├── 02–19 Detailed context: how each domain is governed and built
    ├── 99 AI Bootstrap: how AI systems load and apply context
    ├── Architecture: cross-cutting system maps
    ├── Knowledge: decisions, research, glossary, and institutional memory
    └── Code and runtime: future implementation of the documented intent
```

### Decision Tables

#### Document Classification Table

| Classification | Meaning in this document | How a reader may use it |
|---|---|---|
| **Official Decision** | Accepted project direction recorded in current authoritative context or repository policy | Implement or preserve unless an accepted decision changes |
| **Recommendation** | Preferred approach that supports an official decision but may be changed with documented reasoning | Use by default; record deviations |
| **Future Idea** | A deliberate direction beyond the current implementation phase | Do not implement merely because it is described |
| **Experimental Concept** | A hypothesis or prototype direction that requires evaluation | Isolate, label, measure, and keep out of production defaults |

#### Source-Use Table

| Source state | Can define current behavior? | Can justify a proposal? | Can override this document? |
|---|---:|---:|---:|
| Accepted root context | Yes | Yes | Only by a newer accepted authority |
| Active section of this overview | Yes for its scope | Yes | No, if root context conflicts |
| Dedicated active context document | Yes for its domain | Yes | No, outside its domain |
| Draft scaffold | No | Only as a file-location signal | No |
| Future or experimental note | No | Yes, as a labeled hypothesis | No |

### Cross References

- Root context and project philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md)
- AI reading and contribution rules: [`/.ai/README.md`](../.ai/README.md), [`/.ai/BOOTSTRAP.md`](../.ai/BOOTSTRAP.md), [`/.ai/CONTEXT_ORDER.md`](../.ai/CONTEXT_ORDER.md)
- Current progress: [`/.ai/PROJECT_STATUS.md`](../.ai/PROJECT_STATUS.md)
- Repository orientation: [`README.md`](../README.md)
- Phase roadmap: [`ROADMAP.md`](../ROADMAP.md)
- Contribution and Definition of Done: [`CONTRIBUTING.md`](../CONTRIBUTING.md)
- Visual identity source: [`CAT Front.md`](../CAT%20Front.md)
- Architecture navigation: [`/.ai/ARCHITECTURE_MAP.md`](../.ai/ARCHITECTURE_MAP.md)
- Decision navigation: [`/.ai/DECISION_INDEX.md`](../.ai/DECISION_INDEX.md)

### Dependencies

This section depends on the existence of the repository-level context system and does not assume that runtime code has been implemented. Its statements must remain compatible with the root context, the roadmap, and the AI workspace instructions.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Readers skip the root context | Local decisions contradict project philosophy | Require root-context loading in onboarding and AI bootstrap |
| Vision is mistaken for implementation | Stakeholders believe capabilities exist before they are built | Use explicit maturity labels and status checks |
| A draft scaffold is treated as authoritative | Agents infer unsupported technical or business rules | Record source status and authority in proposals |
| Overview becomes a second architecture document | Product intent is buried under implementation detail | Keep deep schemas and protocols in dedicated context files |
| Different audiences use different definitions | Product and engineering drift | Maintain shared terminology and cross-links |

### Best Practices

- Read the smallest sufficient set of documents, but always include root context and this overview for project-level changes.
- Name the capability and lifecycle stage a change supports.
- State whether a proposal is official, recommended, future, or experimental.
- Link to the source of a decision rather than restating it without attribution.
- Update this document when product identity, boundaries, or operating model changes.
- Preserve the distinction between a product promise and a current implementation.

### Anti-patterns

- Starting from a code directory and inferring CAT's purpose from filenames.
- Treating every sentence in a vision document as an existing feature.
- Adding a new feature because it is technically interesting but cannot be mapped to the CAT lifecycle.
- Replacing a precise definition with marketing language.
- Copying architecture detail into this document and allowing the two versions to drift.
- Editing an authority boundary without updating the decision and governance documents.

### Future Evolution

The overview will remain the stable entry point while deeper documents expand. Future parts may add more detailed product operating scenarios, domain contracts, ecosystem relationships, and decision indexes without changing the basic reading contract. If the product identity changes materially, this document must receive a versioned update and downstream documents must be reviewed.

### AI Construction Notes

When constructing or changing a section of this overview, an AI agent should:

1. Identify the user-visible product concept being documented.
2. Read the corresponding root-context section and dedicated context file.
3. Extract current facts separately from aspirations.
4. Write the human explanation before technical detail.
5. Add a decision table if multiple states, roles, or boundaries exist.
6. Add a Mermaid diagram when relationships or flows would otherwise be ambiguous.
7. Add risks and anti-patterns so future agents do not repeat predictable errors.
8. Add cross-references instead of duplicating deep specifications.
9. Update the project status after completing a documentation part.
10. Never claim that a component exists solely because a directory or placeholder file exists.

### Extension Points

This overview can be extended through:

- New capability domains, if they remain subordinate to the Trinity model.
- New lifecycle stages, if their inputs, outputs, owner, and approval behavior are documented.
- New audience or enterprise profiles, if their responsibilities and access boundaries are explicit.
- New integration categories, if the integration does not bypass CAT governance.
- New maturity labels, if their promotion and retirement rules are defined.

### Implementation Checklist

- [x] Document metadata identifies CAT, Omni System, the owner, status, and Part 1.
- [x] Authority hierarchy is explicit.
- [x] Human and AI reading paths are documented.
- [x] Product intent is separated from implementation status.
- [x] Classification vocabulary is defined.
- [x] Cross-references point to the root context, AI workspace, roadmap, and contribution rules.
- [x] Risks, anti-patterns, and AI construction guidance are included.
- [ ] Part 2 of this overview is written in a later task.

---

## 2. CAT Identity — What CAT Is

### Human Explanation

CAT stands for **Commerce AI Trinity**. CAT is an **AI-native autonomous commerce operating system** designed to coordinate the complete affiliate-commerce lifecycle: discovering opportunities, evaluating products and markets, preparing and distributing content, managing affiliate operations, observing performance, learning from outcomes, and maintaining treasury awareness.

The phrase “operating system” is intentional. CAT is not merely a screen, a chatbot, a content generator, or a collection of integrations. It is intended to provide the persistent coordination layer that:

- receives signals from markets, products, channels, agents, users, and financial activity;
- turns those signals into structured work;
- assigns work to specialized agents and services;
- pauses at critical human-approval boundaries;
- records decisions, evidence, outputs, and outcomes;
- uses accumulated knowledge to improve later work; and
- presents the whole operating state through a human command-center experience.

CAT therefore has two identities at once:

1. **A commerce operator:** it performs work that creates and manages affiliate-commerce activity.
2. **An intelligence platform:** it coordinates agents, memory, knowledge, reasoning, learning, and human oversight.

Neither identity is sufficient alone. Commerce without intelligence remains manual and fragmented. Intelligence without commerce is a generic AI system without a focused operating domain. CAT exists in the connection between them, with Treasury providing economic accountability.

The official orientation sentence is:

> **CAT is an AI-native autonomous commerce operating system that researches products, creates and publishes commerce content, manages affiliate operations, observes performance, learns continuously, and maintains treasury accountability under bounded human supervision.**

This sentence is a concise identity statement, not a claim that every capability is already implemented. Current maturity is defined in Section 11 and `.ai/PROJECT_STATUS.md`.

### AI Context

An AI system must use the following identity rules:

- Refer to CAT as a **system/platform/operating system**, not as a chatbot or a single model.
- Treat affiliate commerce as the initial operating domain.
- Treat AI agents as the primary execution mechanism and humans as supervisors and accountable approvers.
- Treat Commerce, AI, and Treasury as inseparable product pillars.
- Treat knowledge, memory, decisions, approvals, and outcomes as part of the product rather than incidental logs.
- Do not describe an unimplemented capability as available in production.
- Do not redefine CAT as a generic agent framework without an approved product decision.
- Do not remove Treasury from the identity; financial accountability is a load-bearing part of the Trinity.
- Use `KATA` for the human-facing persona/interface and `CATA` for the internal central orchestrator when those distinctions are relevant.

The following terms are not interchangeable:

| Term | Correct meaning |
|---|---|
| **CAT** | The complete Commerce AI Trinity product and operating system |
| **Commerce** | Market, product, affiliate, content, publishing, and performance operations |
| **AI** | Agent orchestration, reasoning, memory, knowledge, learning, and model capabilities |
| **Treasury** | Earnings, payouts, budgets, financial reporting, risk, and economic accountability |
| **CATA** | Internal central agent/orchestrator that coordinates specialized agents |
| **KATA** | Human-facing avatar/persona and interaction boundary for CAT |
| **Agent** | A specialized AI participant with an explicit role, tools, inputs, outputs, and autonomy boundary |
| **Human supervisor** | A person who monitors CAT and approves, rejects, modifies, or owns critical decisions |

### Business Perspective

CAT's identity defines the category in which the product competes. It is not competing only with writing tools, affiliate dashboards, or analytics products. Its intended value is to unify the work that those tools leave fragmented.

The business promise is a shift from **human-operated affiliate work** to **human-supervised commerce operations**:

| Traditional operating model | CAT operating model |
|---|---|
| Human searches for opportunities | Agents continuously discover and qualify opportunities |
| Human copies data between tools | CAT coordinates structured workflows and integrations |
| Human writes each content asset | Creative agents produce governed drafts and variants |
| Human manually manages every link | Affiliate capabilities prepare and monitor link operations |
| Human checks reports after the fact | Analytics and Treasury surface signals proactively |
| Human remembers what worked | Knowledge and memory preserve outcomes for future decisions |
| Human must be present for routine work | Agents operate continuously within policy |
| Human remains accountable for critical choices | Human authority is preserved at explicit approval gates |

This positioning matters because a narrow product can optimize one task while leaving the human bottleneck intact. CAT's product thesis is that value comes from closing the loop from opportunity to outcome, not from maximizing the number of isolated AI outputs.

### Technical Perspective

At the technical level, CAT is a coordination system with five essential properties:

1. **Persistent:** relevant memory, knowledge, decisions, and outcomes survive individual sessions and process restarts.
2. **Event-aware:** system and domain signals can initiate work without waiting for a human request.
3. **Modular:** specialized agents and domain modules have explicit responsibilities and contracts.
4. **Governed:** external, financial, reputational, legal, and irreversible actions are classified and routed through appropriate controls.
5. **Observable:** actions, evidence, approvals, errors, and outcomes are traceable.

These properties describe the required product shape, not a complete implementation recipe. The detailed architecture and technology choices belong in `context/03_TECH_STACK.md` and `context/04_ARCHITECTURE.md`.

### Visual Overview

```mermaid
flowchart TB
    CAT((CAT<br/>Commerce AI Trinity))
    CAT --> Commerce[Commerce<br/>Find, create, publish, optimize]
    CAT --> AI[AI<br/>Coordinate, reason, remember, learn]
    CAT --> Treasury[Treasury<br/>Measure, govern, account]

    Commerce <--> AI
    AI <--> Treasury
    Treasury <--> Commerce

    Human[Human Supervisor] --> KATA[KATA<br/>Human-facing boundary]
    KATA --> CATA[CATA<br/>Internal coordinator]
    CATA --> CAT

    style CAT fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:4px
    style Commerce fill:#10b981,color:#fff
    style AI fill:#7c3aed,color:#fff
    style Treasury fill:#f59e0b,color:#000
    style KATA fill:#00d4ff,color:#000
    style CATA fill:#7c3aed,color:#fff
```

### Mermaid Diagram(s)

#### Identity Boundary

```mermaid
graph LR
    HumanWorld[Human goals, constraints, approvals]
    KATA[KATA<br/>Translate, explain, present]
    CATA[CATA<br/>Coordinate and route]
    Agents[Specialized agents<br/>Research, Affiliate, Creative, Publisher]
    Domains[Commerce + Treasury + Analytics]
    Knowledge[Knowledge, memory, decisions, outcomes]
    HumanWorld --> KATA --> CATA --> Agents --> Domains
    Domains --> Knowledge
    Knowledge --> CATA
    CATA --> KATA --> HumanWorld

    External[External merchants, networks, channels]
    Domains <--> External
```

#### Identity Layers

```mermaid
flowchart BT
    Intent[Human intent and business goals]
    Identity[CAT product identity]
    Pillars[Commerce / AI / Treasury]
    OperatingModel[Agents with bounded human supervision]
    Capabilities[Research / affiliate / content / publishing / analytics / treasury]
    Evidence[Decisions / approvals / outcomes / knowledge]

    Intent --> Identity --> Pillars --> OperatingModel --> Capabilities --> Evidence
```

### Architecture Map

CAT is best understood as a **closed-loop operating model**, not a list of screens:

```text
Human intent and constraints
        │
        ▼
KATA — human-facing explanation, request, and approval boundary
        │
        ▼
CATA — internal coordination, task decomposition, routing, and policy checks
        │
        ├── Research and discovery agents
        ├── Affiliate operations agents
        ├── Creative and content agents
        ├── Publisher and channel agents
        ├── Analytics and learning agents
        ├── Treasury and financial agents
        └── Security, memory, and knowledge capabilities
        │
        ▼
Commerce and external integrations produce observable outcomes
        │
        ▼
Knowledge, memory, decisions, approvals, and treasury records
        │
        └────────────── feedback into the next decision
```

The architecture map intentionally omits concrete deployment topology. It identifies the product responsibilities that a later architecture must implement without changing their meaning.

### Decision Tables

#### Official Identity Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| OVR-DEC-001 | CAT is an AI-native autonomous commerce operating system. | A traditional dashboard with optional AI is insufficient as the product definition. |
| OVR-DEC-002 | CAT's initial business domain is affiliate commerce. | New capabilities must support the affiliate-commerce lifecycle or have an explicitly accepted platform rationale. |
| OVR-DEC-003 | Commerce, AI, and Treasury are the three identity pillars. | Product plans must account for operational value, intelligence, and financial accountability. |
| OVR-DEC-004 | Humans remain accountable supervisors for critical actions. | Autonomy must be bounded by policy and approval workflows. |
| OVR-DEC-005 | Specialized agents are preferred over one monolithic AI. | Agent roles, contracts, and orchestration are first-class design concerns. |
| OVR-DEC-006 | Knowledge and memory are persistent product capabilities. | A workflow that cannot preserve relevant context is incomplete for CAT's intended model. |
| OVR-DEC-007 | KATA is the human-facing boundary and CATA is the internal coordinator. | Human interaction and internal orchestration must remain conceptually separate. |

#### Classification of Related Ideas

| Item | Classification | Current interpretation |
|---|---|---|
| AI-native operation | **Official Decision** | CAT is designed around AI as operator, not AI as an optional assistant. |
| Human approval for high-impact actions | **Official Decision** | Critical boundaries are mandatory. |
| Multi-model orchestration | **Official Decision at direction level** | CAT should not depend on a single model; concrete routing is detailed later. |
| Cloud-native and scalable deployment | **Official Decision at direction level** | The system is intended to scale and be observable; exact infrastructure remains a downstream concern. |
| Cosmic cat command-center interface | **Official product/design direction** | The persona and visual identity are defined in root context and `CAT Front.md`; implementation belongs to UI/UX documents. |
| Public plugin marketplace | **Future Idea** | It is an ecosystem direction, not a current Phase A capability. |
| Cross-region CAT federation | **Future Idea** | It requires later architecture and operations decisions. |
| Fully self-modifying autonomous product strategy | **Experimental Concept** | It cannot be treated as a default operating mode without explicit governance and evaluation. |

### Cross References

- Root identity, vision, mission, and principles: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), especially Sections 4–12.
- Product-facing shorthand: [`README.md`](../README.md).
- Phase and capability sequence: [`ROADMAP.md`](../ROADMAP.md).
- Human/AI engineering principles: [`CONTRIBUTING.md`](../CONTRIBUTING.md).
- Visual persona and command-center intent: [`CAT Front.md`](../CAT%20Front.md).
- Future agent details: [`context/05_AGENTS.md`](./05_AGENTS.md).
- Future knowledge details: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md).
- Future Treasury details: [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md).

### Dependencies

CAT's identity depends on the three pillars remaining connected. Removing any one of them changes the product category:

- Without Commerce, CAT loses its domain and becomes generic AI infrastructure.
- Without AI, CAT loses its autonomous operating model and becomes manual commerce software.
- Without Treasury, CAT loses economic accountability and cannot close the business loop.
- Without human governance, CAT loses trust and violates the bounded-autonomy principle.
- Without knowledge, CAT cannot compound learning or preserve institutional context.

### Risks

| Risk | Why it threatens identity | Required response |
|---|---|---|
| Feature drift toward a generic chatbot | Reduces CAT to a conversational surface | Re-map proposals to lifecycle capabilities and operating outcomes |
| Feature drift toward a content-only tool | Leaves discovery, publishing, analytics, and treasury fragmented | Evaluate end-to-end workflow coverage |
| Treasury treated as back-office reporting | Breaks the Trinity feedback loop | Include financial impact and accountability in relevant workflows |
| AI treated as a co-pilot only | Preserves the human bottleneck | Design proactive, event-aware, bounded execution |
| “Autonomous” interpreted as unsupervised | Creates unacceptable risk | Use explicit autonomy levels and approval gates |
| KATA and CATA conflated | Leaks internal complexity into the human interface | Preserve the interface/coordinator boundary |

### Best Practices

- Use the official identity sentence when a short description is needed.
- When proposing a capability, identify which Trinity pillars it serves and which lifecycle stage it advances.
- Preserve human authority for actions with external visibility, financial exposure, legal consequence, reputational risk, or irreversible effect.
- Model knowledge and outcome capture as part of workflow completion.
- Prefer a clear role for each agent over a vague “AI layer.”
- State whether a component is a current implementation, a planned phase, or a future direction.

### Anti-patterns

- “CAT is basically ChatGPT for affiliate links.”
- “CAT is a dashboard where users can optionally ask AI for help.”
- “Treasury can be added later because it is not part of the user experience.”
- “One general model can replace all specialized agents.”
- “Autonomy means no approval is ever required.”
- “The avatar is the whole product.”
- “The existence of an empty repository directory proves a capability is implemented.”

### Future Evolution

The identity is intended to remain stable while the capability surface grows. Future expansion may include more commerce channels, stronger decision support, enterprise deployment models, additional Omni System products, governed plugins, and regional intelligence. Expansion must preserve the initial domain focus, the Trinity relationship, knowledge accumulation, and human accountability.

A future product-domain expansion should be evaluated with these questions:

1. Does it strengthen CAT's commerce operating-system role?
2. Does it have a clear relationship to Commerce, AI, or Treasury?
3. Can its risks be governed at the appropriate autonomy level?
4. Can its outcomes become durable knowledge?
5. Does it belong in CAT Core, a domain module, an integration, or a future Omni product?

### AI Construction Notes

When an AI agent describes CAT in generated documentation, code comments, issue triage, or pull requests:

- Start with the product identity, not the implementation technology.
- Use “intended,” “planned,” or “current” accurately.
- Do not invent customer, revenue, market-size, or deployment claims.
- Do not use `CATA` and `KATA` interchangeably.
- Do not propose a capability that bypasses the Trinity or approval model without explicitly labeling it as a conflict.
- If the requested task appears to turn CAT into a generic platform, ask whether a scope decision is intended before implementing it.

### Extension Points

The identity supports extension through:

- New affiliate networks and commerce channels.
- New specialized agents with bounded responsibilities.
- New analytical or learning capabilities.
- New Treasury reports and financial controls.
- New human-facing interaction modes that remain behind the KATA boundary.
- Future Omni System products that reuse stable, extracted contracts rather than importing CAT internals.

### Implementation Checklist

- [x] CAT is defined as an AI-native autonomous commerce operating system.
- [x] Commerce, AI, and Treasury are defined as the product pillars.
- [x] The human supervisor and AI operator relationship is explicit.
- [x] KATA and CATA are distinguished.
- [x] Current identity is separated from future and experimental ideas.
- [x] Product identity risks and anti-patterns are documented.
- [ ] Detailed agent contracts are defined in `context/05_AGENTS.md`.
- [ ] Detailed domain contracts are defined in the dedicated domain context documents.
- [ ] Concrete implementation status is maintained in `.ai/PROJECT_STATUS.md`.

---

## 3. Why CAT Exists

### Human Explanation

CAT exists because the affiliate-commerce operating model has a structural capacity problem. The number of products, markets, merchants, affiliate programs, channels, content formats, performance signals, and financial decisions grows faster than one human or a small team can continuously evaluate.

A typical affiliate operation requires a chain of dependent activities:

1. Discover a market, trend, product, or merchant opportunity.
2. Research whether the opportunity is real, relevant, and commercially viable.
3. Compare products, commissions, audience fit, compliance constraints, and competition.
4. Create content and assets appropriate to the audience and channel.
5. Create and validate affiliate links or partner relationships.
6. Publish through one or more channels.
7. Measure clicks, conversions, revenue, content quality, and operational cost.
8. Diagnose what happened and decide what to change.
9. Track earnings, payout timing, budgets, risk, and financial performance.
10. Preserve the lesson so that the next cycle is better.

In a fragmented toolchain, every handoff introduces delay, duplicated work, missing context, and inconsistent judgment. The person who researches an opportunity may not know the Treasury constraints. The writer may not know which product evidence is trustworthy. The analyst may see a conversion drop without knowing that a link or policy changed. The organization may repeat the same failed approach because the lesson was never captured.

CAT's response is not to remove humans from the system. It is to remove humans from repetitive coordination and execution where a governed agent can do the work, while keeping people responsible for judgment, accountability, and critical approval.

The central operating transformation is:

> **From human-in-the-loop execution to human-on-the-loop supervision, with explicit human gates wherever the impact requires authority.**

“Human-on-the-loop” does not mean that humans are passive. It means that humans supervise a system capable of continuous operation rather than manually initiating every step.

### AI Context

An AI system must understand the problem as a workflow and knowledge problem, not only as a generation problem. The goal is not “produce more text.” The goal is to coordinate a measurable, auditable path from opportunity to economic outcome.

When analyzing a feature request, the agent should ask:

- Which manual bottleneck does this remove?
- Which lifecycle handoff does it improve?
- Which evidence does it need?
- Which agent or domain owns the work?
- What is the risk if the result is wrong?
- Does the action need approval, notification, or neither?
- What outcome and lesson should be recorded?

An agent must not optimize for output volume in isolation. A large number of generated assets without quality, compliance, conversion, provenance, and treasury context is not progress toward CAT's mission.

### Business Perspective

The business problem has four connected dimensions:

| Dimension | Current industry friction | CAT opportunity |
|---|---|---|
| **Scale** | Human attention limits the number of markets and products that can be evaluated | Agents monitor and triage more opportunities continuously |
| **Speed** | Manual handoffs delay response to trends and campaign signals | Event-driven workflows reduce coordination latency |
| **Quality** | Repetition encourages low-quality, generic, or poorly evidenced content | Specialized agents, review gates, and learning improve consistency |
| **Economics** | Activity, conversions, and earnings are often managed in disconnected tools | Treasury closes the loop between work performed and value created |

CAT's economic thesis is not “more automation always produces more revenue.” The thesis is that better selection, better evidence, better execution, faster feedback, and disciplined financial management can create a more durable commerce operation.

CAT should therefore measure value across a balanced set of outcomes:

- **Operational value:** time saved, handoffs reduced, throughput increased, failure recovery improved.
- **Commerce value:** product-market fit, content usefulness, channel reach, conversion quality, partner coverage.
- **Intelligence value:** retrieval quality, recommendation quality, learning speed, knowledge reuse.
- **Financial value:** earnings quality, margin awareness, payout visibility, budget discipline, risk-adjusted return.
- **Trust value:** approval quality, auditability, compliance, explainability, and incident prevention.

### Technical Perspective

The problem requires a closed-loop architecture because isolated automation cannot solve cross-domain coordination. CAT must connect:

```text
Signals → context → decision → approval → execution → observation → learning
```

A system that only generates a recommendation is incomplete. A system that executes without observing is unsafe and unable to improve. A system that observes without preserving context becomes a reporting tool rather than an operating system.

The technical design must accommodate:

- asynchronous work and external API latency;
- long-running workflows and resumable tasks;
- structured evidence and provenance;
- approval state and responsibility transitions;
- retries, idempotency, and failure isolation;
- persistent knowledge and memory;
- financial consistency and immutable audit history;
- model and prompt versioning;
- human-readable explanations of agent decisions.

These are engineering consequences of the business problem. They are not a license to choose complexity without a documented need.

### Visual Overview

```mermaid
flowchart TB
    Problem[Fragmented affiliate-commerce lifecycle]
    Problem --> Bottleneck[Human coordination bottleneck]
    Bottleneck --> Delay[Slow research, creation, publishing, and optimization]
    Bottleneck --> Drift[Lost context and inconsistent decisions]
    Bottleneck --> Blind[Fragmented analytics and treasury visibility]

    CATResponse[CAT response]
    Delay --> CATResponse
    Drift --> CATResponse
    Blind --> CATResponse

    CATResponse --> Agents[Specialized agents]
    CATResponse --> Orchestration[Workflow and event coordination]
    CATResponse --> Oversight[Human approval for critical actions]
    CATResponse --> Memory[Persistent knowledge and outcomes]

    Agents --> Result[Closed-loop commerce operations]
    Orchestration --> Result
    Oversight --> Result
    Memory --> Result

    style Problem fill:#ef4444,color:#fff
    style CATResponse fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Result fill:#10b981,color:#fff
```

### Mermaid Diagram(s)

#### Traditional and CAT Operating Models

```mermaid
flowchart LR
    subgraph Traditional[Traditional affiliate operation]
        H1[Human researches] --> H2[Human selects]
        H2 --> H3[Human writes]
        H3 --> H4[Human publishes]
        H4 --> H5[Human checks reports]
        H5 --> H6[Human remembers lessons]
    end

    subgraph CATModel[CAT intended operating model]
        R[Agents sense and research] --> S[Agents recommend]
        S --> A1{Human approval if required}
        A1 --> C[Agents create and prepare]
        C --> A2{Human review if required}
        A2 --> P[Agents publish or execute]
        P --> M[Agents measure and Treasury reconciles]
        M --> L[Knowledge and learning update]
        L --> R
    end
```

#### Bottleneck Removal Map

```mermaid
graph TD
    ManualResearch[Manual research] --> ResearchAgent[Research capability]
    ManualHandoffs[Tool-to-tool handoffs] --> Orchestrator[Orchestration and events]
    ManualWriting[Manual content production] --> CreativeAgent[Creative capability]
    ManualPublishing[Manual distribution] --> PublisherAgent[Publishing capability]
    FragmentedMetrics[Fragmented metrics] --> Analytics[Analytics capability]
    FinancialBlindSpots[Financial blind spots] --> Treasury[Treasury capability]
    LostLessons[Lost lessons] --> Knowledge[Knowledge and memory]

    ResearchAgent --> ClosedLoop[Closed-loop operation]
    Orchestrator --> ClosedLoop
    CreativeAgent --> ClosedLoop
    PublisherAgent --> ClosedLoop
    Analytics --> ClosedLoop
    Treasury --> ClosedLoop
    Knowledge --> ClosedLoop
```

### Architecture Map

The problem-to-capability map is the high-level architecture requirement for the product:

| Problem boundary | CAT responsibility | Required evidence |
|---|---|---|
| Opportunity discovery | Detect, research, qualify, and explain opportunities | Sources, confidence, market context, recommendation |
| Product and affiliate selection | Compare fit, terms, risk, and expected value | Product record, program terms, decision trace |
| Content and asset production | Produce channel-appropriate, evidence-grounded drafts | Prompt/model version, source set, quality checks |
| Publishing and distribution | Prepare and execute approved publication workflows | Approval event, channel response, publication record |
| Measurement and optimization | Observe performance, detect anomalies, recommend changes | Metrics, attribution, analysis, confidence |
| Treasury and accountability | Reconcile earnings, payouts, budgets, and financial risk | Immutable financial records, approval, report |
| Learning and memory | Convert outcomes into reusable knowledge | Outcome record, lesson, confidence, review state |

### Decision Tables

#### Problem-to-Response Decisions

| Problem | CAT response | Must not be reduced to |
|---|---|---|
| Humans cannot monitor enough opportunities | Continuous agent research and prioritization | Blind bulk scraping or spam |
| Handoffs lose context | Shared workflow identity, evidence, and knowledge | Unstructured copy/paste between tools |
| Content is a bottleneck | Governed agent drafting and production | Unreviewed content flooding |
| Performance is reactive | Continuous analytics and anomaly detection | Vanity metrics only |
| Earnings are disconnected from activity | Treasury correlation and reporting | A separate accounting afterthought |
| Lessons disappear | Persistent memory and knowledge refinement | Raw logs with no retrieval or curation |

#### Outcome Quality Table

| Outcome type | Minimum question |
|---|---|
| Research | Was the opportunity supported by credible evidence and relevant context? |
| Recommendation | Why was this option selected, and what alternatives were rejected? |
| Content | Is the output accurate, useful, compliant, and appropriate for its channel? |
| Publication | Was the action authorized, successful, and traceable? |
| Analytics | Does the interpretation distinguish signal from noise? |
| Treasury | Are earnings, costs, risks, and payout state represented correctly? |
| Learning | What should change next time, with what confidence? |

### Cross References

- Root problem statement and CAT solution: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 4–5.
- Mission and lifecycle: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 7.
- Core architecture response: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.
- Roadmap phases: [`ROADMAP.md`](../ROADMAP.md).
- Treasury scope: [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md).
- Affiliate scope: [`context/08_AFFILIATE_ENGINE.md`](./08_AFFILIATE_ENGINE.md).
- Content scope: [`context/09_CONTENT_ENGINE.md`](./09_CONTENT_ENGINE.md).
- Knowledge scope: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md).

### Dependencies

Solving the problem depends on more than an AI model. CAT requires the coordinated existence of:

- trustworthy input sources and provenance;
- product, market, affiliate-program, content, channel, and campaign concepts;
- workflow and task coordination;
- approval and responsibility rules;
- analytics and attribution;
- Treasury records and reconciliation;
- persistent knowledge and memory;
- security controls for credentials and external actions;
- a human interface that makes system state understandable.

If one dependency is absent, the system may still provide a narrow utility, but it does not fully solve the stated problem.

### Risks

| Risk | Failure mode | Mitigation |
|---|---|---|
| Automation increases volume but not value | More content, links, or campaigns with no quality or earnings improvement | Tie evaluation to quality, outcomes, and risk-adjusted value |
| Bad source data drives decisions | Agents produce confident but wrong recommendations | Track provenance, confidence, freshness, and source reliability |
| Human approval becomes a bottleneck | Every trivial action waits for a person | Use risk-based autonomy and progressive trust |
| Agents optimize locally | One agent improves its metric while harming Treasury or reputation | Evaluate cross-domain outcomes and use CATA coordination |
| Learning captures false lessons | A noisy event changes future strategy | Require confidence, evidence, review, and sufficient sample size |
| External platforms change | Integrations fail or corrupt records | Isolate connectors, monitor contracts, and fail safely |

### Best Practices

- Define the user and business bottleneck before proposing automation.
- Prefer end-to-end lifecycle improvements over isolated output generation.
- Make the expected outcome measurable before implementation.
- Include Treasury and risk considerations early for commerce proposals.
- Preserve sources and assumptions with every material recommendation.
- Capture both successful and unsuccessful outcomes.
- Use human review to improve decision quality, not to manually repeat low-risk work.

### Anti-patterns

- Measuring success by generated tokens, number of agents, or number of integrations alone.
- Automating a bad process without clarifying ownership and desired outcomes.
- Treating analytics as a report that is disconnected from action.
- Treating Treasury as an after-the-fact spreadsheet.
- Creating content at scale without evidence, quality controls, or disclosure requirements.
- Using learning language to justify unreviewed policy changes.

### Future Evolution

As CAT matures, the closed loop can become more predictive: identifying emerging opportunities, estimating likely outcomes, recommending budget allocation, and scheduling work before a human asks. Predictive behavior must remain grounded in evidence and bounded by risk policy. The system should become more capable without becoming less accountable.

### AI Construction Notes

When an AI agent receives a task framed as “automate X,” it should not immediately write an automation. It should map X to the lifecycle, identify the upstream input and downstream outcome, ask what human authority is required, and identify the knowledge that must be preserved. If the task only increases output volume, the agent should flag the missing quality and value criteria.

### Extension Points

The problem response can be extended by:

- Adding new data sources with provenance and freshness contracts.
- Adding new commerce channels through connector boundaries.
- Adding new agent specializations where evaluation can be isolated.
- Adding new financial metrics that improve economic accountability.
- Adding new learning signals that have a documented confidence model.
- Adding new approval policies for regulated or enterprise deployments.

### Implementation Checklist

- [x] The structural problem CAT addresses is defined.
- [x] The human bottleneck and fragmented-tool problem are explicit.
- [x] The CAT response is described as a closed loop.
- [x] Operational, commerce, intelligence, financial, and trust outcomes are separated.
- [x] Technical dependencies and risks are identified.
- [x] Local optimization and output-volume anti-patterns are documented.
- [ ] Detailed problem-domain schemas are defined in downstream context files.
- [ ] Outcome metrics and evaluation thresholds are approved for implementation phases.

---

## 4. The Commerce AI Trinity

### Human Explanation

The name CAT contains the product's organizing model: **Commerce AI Trinity**. The Trinity is not three independent features placed next to one another. It is a continuous relationship among three responsibilities:

- **Commerce** determines where value can be created and performs market-facing operations.
- **AI** turns data and context into coordinated reasoning, agent work, memory, learning, and decisions.
- **Treasury** determines whether activity is economically visible, accountable, and sustainable.

Commerce asks, “What opportunity should we pursue, for whom, through which channel, and with what content?” AI asks, “What evidence do we have, what should happen next, which agent should do it, and what did we learn?” Treasury asks, “What did this activity cost or earn, what is the risk, what is the payout state, and should the next action be funded or approved?”

CAT exists because those questions are coupled. A product recommendation without economic context may be strategically attractive but financially weak. A Treasury report without Commerce context describes what happened but cannot improve the next action. AI is the connective intelligence that allows the two operational domains to reason together.

The Trinity should be treated as a balancing model:

```text
Commerce creates activity.
AI coordinates and improves activity.
Treasury measures and governs the economic result.
```

The result is not merely more automation. It is a system in which activity, intelligence, and economics inform each other continuously.

### AI Context

When an AI agent classifies a capability, it must identify its Trinity relationship:

- A capability may be primarily Commerce, primarily AI, primarily Treasury, or cross-pillar.
- A cross-pillar capability must document its data and authority boundaries.
- Treasury constraints must be visible to recommendations that spend resources, create financial exposure, or claim economic value.
- Commerce outputs must be grounded in sources and channel requirements.
- AI orchestration must not become an unbounded authority layer; it coordinates under policy.

A feature that cannot be connected to at least one pillar should be questioned. A feature that claims to optimize the whole system but ignores one of the pillars is incomplete.

### Business Perspective

The Trinity creates a differentiated business model and a product discipline. It prevents CAT from becoming:

- a generic AI platform with no operating domain;
- a content factory that ignores product economics;
- an affiliate dashboard that reports on activity without acting on it; or
- a financial system that cannot influence commerce decisions.

The pillars create several business loops:

| Loop | Commerce contribution | AI contribution | Treasury contribution |
|---|---|---|---|
| Opportunity loop | Finds products, markets, merchants, and channels | Scores and prioritizes opportunities | Estimates value, cost, exposure, and budget fit |
| Production loop | Defines content and distribution needs | Generates, validates, and coordinates assets | Tracks resource use and expected return |
| Performance loop | Supplies clicks, conversions, and channel results | Diagnoses patterns and proposes changes | Reconciles revenue, payout, and financial impact |
| Learning loop | Provides new market and campaign evidence | Converts outcomes into reusable knowledge | Confirms whether lessons improve economics |
| Governance loop | Identifies externally visible activity | Routes decisions and approvals | Enforces financial accountability |

### Technical Perspective

The Trinity implies shared identifiers and traceability across domains. A campaign, product, content asset, affiliate link, action, approval, and earnings record must be related enough that CAT can answer:

- Which product and source led to this recommendation?
- Which content and channel produced this conversion?
- Which agent and prompt created the asset?
- Which human approved the external action?
- Which Treasury record reconciles the result?
- What knowledge should be updated after the outcome?

The detailed implementation may use separate domain modules or services, but the product model requires these relationships to be coherent. Domain separation is not permission to create disconnected systems.

### Visual Overview

```mermaid
flowchart TB
    Commerce[COMMERCE<br/>Opportunities, products, affiliates,<br/>content, channels, performance]
    AI[AI<br/>Agents, orchestration, reasoning,<br/>memory, knowledge, learning]
    Treasury[TREASURY<br/>Earnings, payouts, budgets,<br/>reports, risk, accountability]

    Commerce -->|Signals, tasks, outcomes| AI
    AI -->|Recommendations, execution, learning| Commerce
    AI -->|Financial questions and forecasts| Treasury
    Treasury -->|Constraints, results, approvals| AI
    Treasury -->|Economic feedback| Commerce
    Commerce -->|Activity and attribution| Treasury

    Center((CAT))
    Commerce --> Center
    AI --> Center
    Treasury --> Center

    style Center fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:4px
    style Commerce fill:#10b981,color:#fff
    style AI fill:#7c3aed,color:#fff
    style Treasury fill:#f59e0b,color:#000
```

### Mermaid Diagram(s)

#### Closed-Loop Trinity Flow

```mermaid
sequenceDiagram
    participant C as Commerce
    participant A as AI / CATA
    participant K as Knowledge and Memory
    participant T as Treasury
    participant H as Human Supervisor

    C->>A: Opportunity or performance signal
    A->>K: Retrieve products, markets, decisions, and prior outcomes
    K-->>A: Context with provenance and confidence
    A->>T: Assess budget, earnings potential, risk, and payout constraints
    T-->>A: Financial context and recommendation inputs
    A->>H: Present recommendation and evidence when approval is required
    H-->>A: Approve, reject, modify, or request more evidence
    A->>C: Execute or schedule approved commerce work
    C->>T: Emit activity, conversion, and attribution data
    T->>K: Record financial outcome and lesson candidates
    C->>K: Record operational outcome and content/channel evidence
    K-->>A: Refined context for future work
```

#### Three-Pillar Balance

```mermaid
graph TD
    CAT((CAT))
    CAT --> C[Commerce value]
    CAT --> I[Intelligence and coordination]
    CAT --> E[Economic accountability]
    C --> O[Closed-loop operating outcome]
    I --> O
    E --> O
    O --> C
    O --> I
    O --> E
```

### Architecture Map

```text
                           CAT
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
 Commerce domain      AI/cognitive domain   Treasury domain
        │                   │                   │
 product/market       agents, CATA,          earnings, costs,
 affiliate/channel    memory, knowledge,    budgets, payouts,
 content/performance  reasoning, learning    reports, risk
        └─────────────── shared identity and traceability ───────────────┘
                            │
                            ▼
                 approved action and measured outcome
```

#### Pillar Responsibility Map

| Pillar | Owns | Consumes | Produces | Does not own alone |
|---|---|---|---|---|
| Commerce | Market and channel operations | Research, product data, policy, financial constraints | Campaign activity, content, links, performance signals | Final financial approval or system-wide governance |
| AI | Coordination, reasoning, memory, learning | Commerce signals, Treasury context, human goals | Recommendations, tasks, explanations, knowledge updates | Human accountability or independent policy override |
| Treasury | Financial records, reports, payout and budget context | Commerce attribution, approved actions, external financial data | Economic outcomes, constraints, risk signals | Content strategy or autonomous spending authority |

### Decision Tables

#### Official Trinity Decisions

| ID | Official Decision | Implication |
|---|---|---|
| TRI-DEC-001 | Commerce, AI, and Treasury form the three foundational CAT pillars. | Product plans must show their relationship to one or more pillars. |
| TRI-DEC-002 | The pillars are integrated through shared workflows and outcomes. | Independent departmental silos are not the intended product model. |
| TRI-DEC-003 | Treasury is part of the product identity, not a later reporting add-on. | Economic accountability is considered during opportunity, execution, and learning. |
| TRI-DEC-004 | AI coordinates the pillars but does not replace human accountability. | CATA and agents operate under policy and approval boundaries. |

#### Capability Classification Table

| Capability | Primary pillar | Secondary pillar(s) | Typical approval concern |
|---|---|---|---|
| Market research | Commerce | AI | Source quality and strategic fit |
| Product scoring | AI | Commerce, Treasury | Recommendation confidence and economic assumptions |
| Affiliate link creation | Commerce | AI, Treasury | Partner terms, external effect, attribution |
| Content generation | Commerce | AI | Accuracy, brand, legal, public reputation |
| Publishing | Commerce | AI | External publication and channel policy |
| Performance analysis | AI | Commerce, Treasury | Data quality and causal overclaiming |
| Earnings reconciliation | Treasury | Commerce, AI | Financial integrity and auditability |
| Learning update | AI | Commerce, Treasury | False lessons and policy impact |
| Budget allocation | Treasury | AI, Commerce | Financial approval and risk |

### Cross References

- Trinity definition in root context: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 5.3.
- Ecosystem treatment of the Trinity: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 15.2.
- Treasury implementation direction: [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md).
- Affiliate implementation direction: [`context/08_AFFILIATE_ENGINE.md`](./08_AFFILIATE_ENGINE.md).
- Knowledge and learning direction: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md).
- Agent and orchestration direction: [`context/05_AGENTS.md`](./05_AGENTS.md).

### Dependencies

The Trinity requires:

- a common identity for products, campaigns, content, links, actions, approvals, and outcomes;
- event or workflow coordination across domains;
- source and attribution provenance;
- a Treasury model capable of relating activity to economic outcomes;
- an AI knowledge model capable of retrieving cross-domain context;
- approval policies that apply to the combined risk of a workflow, not only to one component.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| One pillar dominates roadmap decisions | CAT becomes a narrow product | Require pillar impact in product proposals |
| Domains share data without ownership | Inconsistent or corrupted records | Define canonical owners and contract boundaries |
| AI recommendation ignores financial constraints | Activity consumes resources without rational economics | Include Treasury context before material recommendations |
| Treasury blocks all experimentation | Product becomes slow and over-controlled | Use risk-based thresholds and approved low-risk experiments |
| Shared identifiers are inconsistent | Attribution and learning fail | Establish terminology and identity contracts before implementation |
| The Trinity becomes branding only | Architecture remains fragmented | Require traceable cross-pillar workflows in acceptance criteria |

### Best Practices

- Treat every major campaign as a Trinity workflow, even if one pillar is temporarily dominant.
- Include expected economic impact and uncertainty in material recommendations.
- Keep domain ownership clear while preserving cross-domain traceability.
- Let AI synthesize and coordinate; keep authority with the correct human or policy owner.
- Evaluate outcomes across quality, operational, and financial dimensions.

### Anti-patterns

- “Commerce first; Treasury later.”
- “AI can decide because it has more data.”
- “Each domain maintains its own definition of campaign or outcome.”
- “Revenue is the only success metric.”
- “A dashboard joins disconnected data at presentation time and calls that integration.”
- “A cross-pillar workflow has no shared correlation or decision identifier.”

### Future Evolution

The Trinity can support richer forecasting, portfolio-level allocation, cross-channel optimization, and future Omni System products that consume shared contracts. These capabilities should emerge from validated workflows rather than from premature abstraction. Cross-product Treasury and knowledge relationships require explicit privacy, ownership, and governance decisions.

### AI Construction Notes

For any proposed feature, generate a small Trinity impact statement:

```text
Primary pillar:
Secondary pillars:
Input signals:
Decision or action:
Financial effect:
Human approval required:
Outcome captured:
Knowledge updated:
```

If the financial effect or outcome is unknown, mark the proposal incomplete rather than filling the gap with an invented assumption.

### Extension Points

- New commerce domains can be attached through shared campaign, product, channel, and outcome contracts.
- New AI capabilities can add reasoning, evaluation, memory, or agent skills without redefining Commerce or Treasury ownership.
- New Treasury capabilities can add currencies, reports, payout integrations, and risk controls through domain-specific context.
- Future Omni products may reuse proven Trinity patterns only after their boundaries are accepted.

### Implementation Checklist

- [x] Commerce, AI, and Treasury are defined separately and relationally.
- [x] The closed-loop relationship is visualized.
- [x] Pillar ownership and non-ownership are explicit.
- [x] Cross-pillar decision and risk concerns are documented.
- [x] Feature classification guidance is included.
- [ ] Detailed shared identifiers and schemas are defined in later context documents.
- [ ] Treasury and commerce outcome metrics are implemented in a later phase.

---

## 5. What CAT Does — The End-to-End Commerce Lifecycle

### Human Explanation

CAT is intended to operate across a connected lifecycle rather than a single task. The lifecycle begins with sensing and research and ends with measured outcomes becoming better context for the next cycle. Humans may set goals, constraints, budgets, markets, or approval policies; agents perform the repeatable investigation and execution inside those boundaries.

The lifecycle is described as ten stages. The stages are conceptual product responsibilities; their exact service and agent decomposition belongs in downstream documents.

#### Stage 1 — Sense and Discover

CAT observes market, product, merchant, channel, analytics, and Treasury signals. It may receive scheduled research work, an external event, a performance anomaly, or a human goal. The output is a candidate opportunity or a reason to investigate.

#### Stage 2 — Research and Qualify

Research capabilities gather evidence about the market, product, audience, merchant, affiliate program, channel, competition, compliance considerations, and likely effort. CAT should distinguish facts, assumptions, estimates, and unknowns.

#### Stage 3 — Select and Plan

AI capabilities compare candidate opportunities and prepare a recommendation or plan. The recommendation should include evidence, confidence, expected value, risk, alternatives, dependencies, and the approval level required.

#### Stage 4 — Affiliate and Commerce Preparation

CAT prepares affiliate-program relationships, product metadata, links, tracking identifiers, campaign structure, channel requirements, and required operational configuration. The system must validate terms and preserve provenance.

#### Stage 5 — Create Content and Assets

Creative capabilities produce drafts, comparisons, reviews, tutorials, images, video concepts, metadata, or other channel-ready assets. Content must be grounded in evidence, comply with policy, and carry enough traceability to explain its origin.

#### Stage 6 — Review and Approve

CAT checks quality, policy, technical readiness, financial exposure, and external impact. A human approves, rejects, modifies, or requests revision when the risk classification requires it. The approval event is part of the workflow record.

#### Stage 7 — Publish and Execute

Publisher or domain capabilities execute approved actions through the appropriate channel or integration. Actions must be idempotent where possible, report external responses, and fail safely when a channel is unavailable or has changed.

#### Stage 8 — Observe and Attribute

CAT records impressions, clicks, conversions, revenue, costs, content quality signals, channel results, errors, and operational latency. It connects observations to the originating product, content, campaign, link, agent, prompt, approval, and Treasury records.

#### Stage 9 — Reconcile and Manage Treasury

Treasury capabilities reconcile earnings, commissions, payouts, budgets, costs, currency, timing, and risk. Financial records must not be silently overwritten. Discrepancies become visible signals for investigation.

#### Stage 10 — Learn and Refine

Learning and knowledge capabilities evaluate the outcome against expectations, extract lessons, adjust confidence, mark stale information, and make validated knowledge available to later decisions. A workflow is not complete merely because it published; it is complete when its outcome is recorded and its learning path is explicit.

### AI Context

An AI coding agent should treat lifecycle stages as product vocabulary and map requested work to one or more stages. It should not invent a new stage when an existing stage is sufficient. If a feature crosses multiple stages, it must identify the handoff and shared correlation identity.

For every lifecycle implementation, the agent should identify:

- triggering event or request;
- owning agent or service;
- required context and sources;
- generated artifacts;
- approval or notification boundary;
- external side effects;
- idempotency and retry behavior;
- success and failure outcomes;
- Treasury impact;
- knowledge update.

### Business Perspective

The lifecycle is the product's value chain. Each stage should either create value directly, reduce risk, reduce coordination cost, improve decision quality, or preserve learning. A stage that adds work but does not improve one of those outcomes should be challenged.

| Lifecycle outcome | Business value |
|---|---|
| Better opportunity selection | Fewer resources wasted on weak products or markets |
| Faster preparation | Reduced time from signal to approved action |
| Higher-quality assets | More useful content and stronger trust |
| Reliable distribution | More consistent channel execution |
| Faster feedback | Earlier detection of successful or failing campaigns |
| Treasury visibility | Better financial discipline and planning |
| Compounding learning | Less repeated work and better future decisions |

### Technical Perspective

The lifecycle is a long-running, stateful workflow. It must support pauses, human decisions, asynchronous external responses, partial failure, retries, cancellation, resumption, and audit. It should not be implemented as one giant synchronous request.

At minimum, each stage should have a stable input/output contract. A stage may be implemented by one agent, multiple agents, a service, or a human-facing approval surface, but the stage boundary must remain observable.

### Visual Overview

```mermaid
flowchart LR
    S1[1. Sense and discover] --> S2[2. Research and qualify]
    S2 --> S3[3. Select and plan]
    S3 --> S4[4. Affiliate and commerce preparation]
    S4 --> S5[5. Create content and assets]
    S5 --> S6[6. Review and approve]
    S6 --> S7[7. Publish and execute]
    S7 --> S8[8. Observe and attribute]
    S8 --> S9[9. Reconcile and manage Treasury]
    S9 --> S10[10. Learn and refine]
    S10 --> S1

    Human[Human goal, constraint, or approval] -.-> S1
    Human -.-> S3
    Human -.-> S6
    Human -.-> S9
    Knowledge[(Persistent knowledge and memory)] --> S2
    Knowledge --> S3
    S10 --> Knowledge
```

### Mermaid Diagram(s)

#### Lifecycle Workflow with Gates

```mermaid
flowchart TD
    Trigger[Signal, schedule, or human goal]
    Trigger --> Discover[Discover opportunity]
    Discover --> Research[Research and qualify]
    Research --> Decision[Prepare recommendation and plan]
    Decision --> Risk{Risk and impact classification}

    Risk -->|Low risk| Prepare[Prepare commerce and affiliate work]
    Risk -->|Medium risk| Notify[Notify supervisor and continue under policy]
    Risk -->|High or external impact| Approval[Human approval gate]
    Risk -->|Critical financial/legal/irreversible| MultiApproval[Required multi-role review]

    Notify --> Prepare
    Approval -->|Approved| Prepare
    Approval -->|Rejected| Rejected[Record rejection and rationale]
    Approval -->|Revision requested| Decision
    MultiApproval -->|Approved| Prepare
    MultiApproval -->|Rejected| Rejected

    Prepare --> Create[Create and validate content/assets]
    Create --> Review[Quality, policy, and readiness checks]
    Review -->|Pass| Publish[Publish or execute]
    Review -->|Fail| Revise[Revise or terminate]
    Revise --> Create
    Publish --> Observe[Observe and attribute]
    Observe --> Treasury[Reconcile Treasury and risk]
    Treasury --> Learn[Learn, refine, and preserve knowledge]
    Learn --> Next[Next opportunity or optimization]
    Next --> Trigger
```

#### Stage-State Model

```mermaid
stateDiagram-v2
    [*] --> Discovered
    Discovered --> Researching
    Researching --> Planned
    Planned --> AwaitingApproval: impact requires gate
    Planned --> Preparing: low-risk policy permits
    AwaitingApproval --> Preparing: approved
    AwaitingApproval --> Rejected: rejected
    AwaitingApproval --> Planned: revision requested
    Preparing --> Creating
    Creating --> ReadyForReview
    ReadyForReview --> Publishing: checks pass
    ReadyForReview --> Creating: revision needed
    Publishing --> Observing
    Observing --> Reconciling
    Reconciling --> Learning
    Learning --> Completed
    Observing --> Investigating: anomaly
    Investigating --> Reconciling
    Completed --> [*]
    Rejected --> [*]
```

### Architecture Map

#### Stage Contract Map

| Stage | Primary owner | Inputs | Outputs | Side effects | Knowledge obligation |
|---|---|---|---|---|---|
| Sense and discover | Research/Analytics capabilities | Events, schedules, goals, signals | Candidate opportunity | May create a research task | Record source, timestamp, freshness |
| Research and qualify | Research Agent | Market/product/channel context | Evidence bundle and qualification | External reads | Preserve sources, assumptions, confidence |
| Select and plan | CATA + decision capabilities | Candidates, knowledge, Treasury context | Ranked recommendation and plan | May request approval | Record alternatives and rationale |
| Affiliate preparation | Affiliate capability | Approved product/campaign plan | Validated links, terms, tracking configuration | External partner calls | Record partner source and validity |
| Create assets | Creative capability | Product facts, evidence, brief, policy | Draft assets and metadata | Model calls, storage writes | Record model/prompt/version and source set |
| Review and approve | Human supervisor + policy | Draft, evidence, risk, Treasury context | Approval decision or revision | Audit event | Preserve approver, decision, scope, reason |
| Publish and execute | Publisher/domain capability | Approved action and credentials | External response and publication record | External side effect | Record channel, version, timestamp, response |
| Observe and attribute | Analytics capability | Channel and commerce signals | Metrics and anomaly signals | Data ingestion | Link metrics to origin and attribution assumptions |
| Treasury reconciliation | Treasury capability | Earnings, costs, payouts, attribution | Financial record and report | Ledger/projection writes | Preserve source, currency, reconciliation state |
| Learn and refine | Learning/Knowledge capability | Outcome, expectation, evidence | Lesson, confidence, updated knowledge | Knowledge updates | State what changed and why |

#### Human Touchpoint Map

```text
Autonomous sensing ──► agent research ──► recommendation
                                             │
                         ┌───────────────────┴───────────────────┐
                         │ approval may be required             │
                         ▼                                       ▼
                 human decision                           policy-approved path
                         │                                       │
                         └───────────────────┬───────────────────┘
                                             ▼
                                   agent execution
                                             ▼
                              observation and Treasury
                                             ▼
                                    learning update
```

### Decision Tables

#### Approval by Lifecycle Stage

| Stage | Default autonomy | Explicit human approval required when |
|---|---|---|
| Sense and discover | Autonomous | A discovery rule changes market scope or monitoring authority |
| Research and qualify | Autonomous with logging | Research accesses restricted data or creates material external commitment |
| Select and plan | Recommendation | Budget, strategic entry, legal exposure, or material risk is involved |
| Affiliate preparation | Prepare and validate | Joining a program, changing terms, or creating an externally consequential relationship requires it |
| Create assets | Draft autonomously | Content is sensitive, regulated, public, or reputationally material |
| Review and approve | Human/policy gate | The action has external visibility, financial effect, legal/reputational risk, or irreversibility |
| Publish and execute | Execute approved action | Any action that is not already covered by an accepted approval scope |
| Observe and attribute | Autonomous | Data policy, attribution model, or material interpretation changes |
| Treasury reconciliation | Autonomous recording, human oversight | Payouts, transfers, budget changes, or disputed financial entries require it |
| Learn and refine | Propose or low-risk update | The learning changes policy, autonomy, financial strategy, or public behavior |

#### Lifecycle Completion Table

| Completion condition | Required evidence |
|---|---|
| Technical completion | Stage output conforms to contract and is traceable |
| Operational completion | External response or terminal failure is recorded |
| Governance completion | Required approval and responsibility transitions are recorded |
| Financial completion | Relevant Treasury impact is reconciled or explicitly marked pending |
| Learning completion | Outcome and lesson state are recorded, even if no lesson is learned |

### Cross References

- Mission lifecycle in root context: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 7.2.1.
- Agent inventory and communication: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.8.
- Event-driven architecture: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13.9 and 13.11.
- Workflow and orchestration details: [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md), [`context/05_AGENTS.md`](./05_AGENTS.md).
- Content and publication details: [`context/09_CONTENT_ENGINE.md`](./09_CONTENT_ENGINE.md).

### Dependencies

The lifecycle depends on shared concepts for:

- opportunity, market, product, merchant, affiliate program, campaign, content, channel, link, action, approval, metric, earning, payout, outcome, and lesson;
- task identity, correlation identity, and idempotency;
- policies and risk classification;
- source provenance and evidence bundles;
- model/prompt/version traceability;
- human identity and role-based approval;
- storage, event, analytics, and Treasury reliability.

### Risks

| Risk | Effect | Control |
|---|---|---|
| Lifecycle stage silently skipped | Incomplete or unsafe operation | State machine and completion checklist |
| Approval recorded after execution | Governance becomes retrospective only | Enforce precondition at execution boundary |
| External side effect duplicated | Duplicate posts, links, or financial actions | Idempotency keys and reconciliation |
| Stage output lacks provenance | Decisions cannot be explained | Evidence and source contract |
| Learning runs before outcome stabilizes | False or premature lessons | Delayed evaluation and confidence thresholds |
| Long-running workflow loses state | Work is duplicated or abandoned | Durable workflow state and resumable tasks |
| Analytics cannot attribute outcome | Treasury and learning are unreliable | Shared campaign/link/content identities |

### Best Practices

- Define terminal states for success, rejection, cancellation, and failure.
- Make every stage resumable where external latency or human review is possible.
- Use explicit correlation identifiers across agent, domain, and Treasury events.
- Treat a human approval as a scoped, time-aware decision, not a generic “yes.”
- Record negative outcomes and rejected recommendations.
- Keep stage outputs useful to both the next stage and future retrieval.
- Prefer additive workflow changes and preserve backward compatibility.

### Anti-patterns

- One endpoint that performs the entire lifecycle synchronously.
- Publishing before the approval state is verified.
- Treating content generation as the end of the workflow.
- Measuring earnings without linking them to originating activity.
- Re-running failed work without checking idempotency.
- Updating global knowledge from one unverified outcome.

### Future Evolution

Later phases may add proactive campaign planning, multi-channel optimization, predictive Treasury, portfolio management, and continuous experimentation. Each evolution should add a visible stage, branch, or policy to the lifecycle rather than hiding behavior inside an opaque agent.

### AI Construction Notes

When implementing a lifecycle change, an AI agent should first write the stage contract and state transition table. It should then identify tests for:

- valid transition;
- invalid transition;
- approval required;
- rejection and revision;
- external failure and retry;
- duplicate event;
- partial completion;
- Treasury reconciliation;
- outcome and knowledge capture.

### Extension Points

- Add a new stage only when the existing stages cannot express the responsibility.
- Add new channel or affiliate connectors behind preparation and execution boundaries.
- Add new review gates through risk policy rather than embedding user-specific conditions in agents.
- Add new learning signals through outcome schemas and confidence rules.
- Add new workflow types through shared lifecycle primitives and versioned contracts.

### Implementation Checklist

- [x] Ten lifecycle responsibilities are defined in product language.
- [x] Inputs, outputs, owners, side effects, and knowledge obligations are mapped.
- [x] Approval behavior is classified by stage.
- [x] Lifecycle state and gate diagrams are included.
- [x] Risks for skipped stages, duplicate effects, and lost state are documented.
- [ ] Detailed state schemas and event contracts are defined downstream.
- [ ] Runtime workflow implementation is deferred until the documentation foundation is approved.

---

## 6. Operating Model — AI Operator, Human Supervisor

### Human Explanation

CAT's operating model assigns work according to the strengths of humans and AI agents. AI performs scalable research, synthesis, drafting, monitoring, coordination, and routine execution. Humans define goals, set constraints, judge ambiguity, accept material risk, protect reputation, and approve critical actions.

The model is not “AI makes decisions and humans watch.” It is:

> **AI prepares and performs bounded work; humans retain authority over intent, accountability, and consequential decisions.**

CAT uses a graduated autonomy model. Not every action deserves the same amount of human friction, and not every action is safe to perform automatically. The level depends on impact, reversibility, confidence, policy, and the scope of the existing approval.

| Level | Operating pattern | Human role | Example |
|---:|---|---|---|
| 0 | Human performs | Human does the work | Human writes a sensitive public statement |
| 1 | AI suggests | Human performs and decides | AI suggests a new market to investigate |
| 2 | AI drafts | Human edits and decides | AI drafts a compliance-sensitive article |
| 3 | AI prepares and executes after approval | Human approves before side effect | AI prepares and publishes an approved campaign |
| 4 | AI executes and notifies | Human reviews after the fact | AI adjusts a low-risk internal routing rule |
| 5 | AI operates autonomously under policy | Human owns policy and receives exception signals | Internal data routing, health checks, or safe cache maintenance |

**Level 3 is the default for actions with external visibility or material financial/reputational consequence.** Level 4 and Level 5 are not blanket permissions; they apply only to explicitly classified action types under accepted policy.

The supervisor should receive enough information to make a real decision:

- what CAT proposes;
- why it proposes it;
- which evidence supports it;
- what alternatives were considered;
- what could go wrong;
- what it will cost or expose;
- how reversible it is;
- what changes if the supervisor modifies the plan; and
- how the result will be measured.

### AI Context

AI agents are not accountable principals. They have roles and permissions, but the human or organization that owns the policy and approves a critical action remains accountable.

An AI agent must:

- operate only within its declared role and tool permissions;
- state uncertainty instead of fabricating confidence;
- escalate when required evidence is missing or impact exceeds policy;
- preserve the decision and approval context;
- avoid treating user urgency as permission to bypass controls;
- distinguish recommendation, draft, approved action, and completed action;
- honor rejection and modification reasons as learning input without automatically rewriting policy;
- stop or degrade safely when the governing service, source, or approval is unavailable.

### Business Perspective

Bounded autonomy is the mechanism that allows CAT to scale without asking the business to accept uncontrolled risk. It creates an operating leverage model:

- Routine work runs continuously.
- Specialists receive higher-quality evidence instead of raw queues.
- Human attention is spent where judgment has the highest value.
- Critical actions remain attributable and reviewable.
- Trust can increase progressively as performance is demonstrated.

The model also creates a governance promise to enterprises and partners: CAT can be autonomous without being unaccountable.

### Technical Perspective

The operating model requires an authorization and approval system that understands action type, scope, actor, subject, target, risk, reversibility, expiration, and outcome. A boolean `approved` flag is insufficient for high-impact work.

An approval record should be able to answer:

```text
Who approved?
What exact action and version were approved?
For which subject, target, channel, budget, and time window?
What evidence and policy were in force?
What modifications were requested?
Did execution match the approved scope?
What was the result?
```

Autonomy should be granted to **action classes**, not to a model in the abstract. A model may be capable of generating text but not authorized to publish it. An agent may be authorized to prepare a payout report but not execute a transfer.

### Visual Overview

```mermaid
flowchart TB
    Goal[Human goal and constraints] --> CATA[CATA decomposes and coordinates]
    CATA --> Agent[Specialized agent performs bounded work]
    Agent --> Risk[Classify impact, confidence, reversibility]
    Risk -->|Low| Auto[Execute under policy and log]
    Risk -->|Medium| Notify[Execute or continue with notification]
    Risk -->|High| Approval[KATA presents evidence for human approval]
    Risk -->|Critical| Review[Multi-role review and explicit approval]
    Approval -->|Approved| Execute[Execute exact approved scope]
    Review -->|Approved| Execute
    Approval -->|Rejected or revised| Replan[Record decision and re-plan]
    Execute --> Outcome[Observe outcome]
    Auto --> Outcome
    Notify --> Outcome
    Outcome --> Learn[Knowledge and learning]
    Learn --> CATA
```

### Mermaid Diagram(s)

#### Human–AI Responsibility Sequence

```mermaid
sequenceDiagram
    participant H as Human supervisor
    participant K as KATA
    participant C as CATA
    participant A as Specialized agent
    participant P as Policy/approval gateway
    participant X as External system
    participant M as Memory and knowledge

    H->>K: Goal, constraint, or question
    K->>C: Structured intent
    C->>M: Retrieve context and prior outcomes
    M-->>C: Evidence, rules, and confidence
    C->>A: Scoped task
    A-->>C: Recommendation, draft, or action plan
    C->>P: Classify impact and authorization
    alt Approval required
        P->>K: Evidence bundle and decision request
        K->>H: Explain recommendation and risk
        H-->>K: Approve, reject, or modify
        K-->>P: Decision with scope
    else Policy permits autonomy
        P-->>C: Authorized under policy
    end
    P-->>C: Execution authorization or rejection
    C->>X: Execute exact allowed action
    X-->>C: Result or failure
    C->>M: Record action, decision, outcome, and lesson candidate
    C-->>K: Status and explanation
    K-->>H: Result, exception, or next decision
```

#### Autonomy Promotion Path

```mermaid
flowchart LR
    Prototype[New action class] --> Observe[Human-performed or sandboxed]
    Observe --> Evaluate[Measure quality, error, risk, reversibility]
    Evaluate --> Gate{Evidence supports more autonomy?}
    Gate -->|No| Refine[Refine role, policy, or workflow]
    Refine --> Observe
    Gate -->|Yes| Pilot[Limited supervised pilot]
    Pilot --> Gate2{Stable under monitoring?}
    Gate2 -->|No| Refine
    Gate2 -->|Yes| Promote[Promote action class with explicit policy]
    Promote --> Monitor[Continuous monitoring and rollback]
```

### Architecture Map

```text
Human authority
    ├── Product and strategic intent
    ├── Risk acceptance
    ├── Critical approval
    ├── Policy ownership
    └── Accountability
            │
            ▼
KATA — translate, explain, present, protect boundary
            │
            ▼
CATA — decompose, route, coordinate, enforce workflow
            │
            ▼
Specialized agents — research, create, publish, analyze, reconcile, learn
            │
            ▼
Policy and execution boundaries — permissions, approvals, audit, external calls
            │
            ▼
Outcome and knowledge — measure, reconcile, refine, preserve
```

#### Responsibility Matrix

| Responsibility | Human | CATA | Specialized agent | Policy/controls |
|---|---:|---:|---:|---:|
| Set strategic intent | Owns | Interprets | Consumes | Constrains |
| Decompose work | Consulted | Owns | Receives | Validates allowed scope |
| Perform research | Reviews important findings | Coordinates | Owns execution | Controls data/tools |
| Draft content | Reviews sensitive output | Routes | Owns draft | Enforces policy |
| Approve external publication | Owns when required | Requests | Cannot self-approve | Blocks bypass |
| Execute approved action | Authorizes | Coordinates | Performs | Verifies scope |
| Reconcile financial outcome | Owns material decisions | Coordinates | Prepares analysis | Enforces integrity |
| Learn from outcome | Validates material changes | Routes | Proposes | Controls promotion |
| Change autonomy policy | Owns or delegates | Cannot self-grant | Cannot self-grant | Enforces change |

### Decision Tables

#### Risk-to-Autonomy Table

| Impact | Reversibility | Confidence | Default level | Human control |
|---|---|---|---:|---|
| Low | Easy | High | 4–5 | Notification, audit, sampling |
| Low | Easy | Low | 1–2 | Human review or more evidence |
| Medium | Reversible | High | 3–4 | Scoped approval or notification by policy |
| Medium | Partly reversible | Any | 2–3 | Explicit approval before side effect |
| High | Reversible | High | 3 | Explicit approval and evidence |
| High | Irreversible or public | Any | 1–3 | Explicit approval; possibly multi-role |
| Critical financial/legal/security | Any | Any | 1–3 | Named authorized human or multi-role review |

#### Approval Decision Table

| Action class | Default decision | Required evidence |
|---|---|---|
| Internal read-only retrieval | Autonomous | Identity, source, access scope |
| Internal low-risk transformation | Autonomous with logging | Input/output trace and validation |
| Draft generation | Autonomous draft | Source set, model/prompt version, policy checks |
| Public content publication | Approval by default | Final content, evidence, channel, disclosure, risk |
| Affiliate relationship or material link change | Approval by policy | Terms, target, attribution, expected impact |
| Budget or payout action | Human financial approval | Amount, currency, source, reconciliation, risk |
| Security or credential change | Authorized security approval | Scope, reason, rollback, audit trail |
| Knowledge correction | Review proportional to impact | Evidence, prior value, confidence, affected decisions |

### Cross References

- Bounded autonomy and AI ethics: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 12.
- KATA/CATA distinction: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.6.
- Human/AI engineering model: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 14.4–14.6.
- Agent details: [`context/05_AGENTS.md`](./05_AGENTS.md).
- Security and authorization details: [`context/17_SECURITY.md`](./17_SECURITY.md).
- Prompt and model behavior: [`context/18_PROMPTING.md`](./18_PROMPTING.md).

### Dependencies

Bounded autonomy depends on:

- identity and authentication;
- role and permission management;
- action and risk taxonomy;
- approval workflow and human interface;
- durable audit trail;
- agent role definitions;
- reliable policy evaluation;
- execution scope verification;
- rollback or compensation where possible;
- monitoring and incident response.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Approval fatigue | Humans approve without reading | Prioritize by risk and improve evidence presentation |
| Approval bypass | External action occurs without authority | Enforce at the execution boundary, not only in UI |
| Scope drift after approval | Executed action differs from approved action | Bind approval to immutable action version and scope |
| Model overconfidence | Uncertain output is treated as fact | Require confidence, evidence, and uncertainty language |
| Agent privilege creep | Agent can access more than its role requires | Least privilege and periodic permission review |
| Human accountability ambiguity | No one owns the result | Capture accountable owner separately from executing agent |
| Excessive conservatism | CAT becomes a manual queue | Use low-risk autonomous classes and measure queue latency |

### Best Practices

- Authorize action classes, not vague agent identities.
- Make approvals specific, scoped, time-bounded, and auditable.
- Show evidence and tradeoffs before asking for approval.
- Separate recommendation, approval, execution, and outcome states.
- Promote autonomy only after measured supervised performance.
- Keep a rollback, pause, or kill-switch path for externally consequential automation.
- Treat a policy change as a documented decision, not an agent configuration tweak.

### Anti-patterns

- Giving an agent broad access because it is “internal.”
- Treating a user message as permanent authorization.
- Recording only that someone clicked “approve” without what they approved.
- Allowing an agent to approve its own recommendation.
- Using a single global autonomy level for every workflow.
- Making humans review every low-risk internal action.
- Hiding risk and uncertainty behind a confidence score with no evidence.

### Future Evolution

CAT may support adaptive autonomy, where action classes move between levels based on measured performance, deployment context, and policy. Adaptive autonomy must be transparent, reversible, and human-governed. It must never silently expand an agent's authority because a model appears more capable.

### AI Construction Notes

An AI agent writing implementation code should treat every external side effect as denied until it identifies the relevant authorization path. It should include negative tests for unauthorized actions, stale approvals, modified payloads, expired scopes, missing approvers, and policy-service failure. It should never “temporarily” bypass an approval gate to make a test or demo work.

### Extension Points

- New action classes can be added with risk, scope, approval, and evidence definitions.
- New human roles can be added through ownership and approval policy.
- New agent capabilities can be added under least-privilege tool scopes.
- New enterprise policies can add stricter controls without weakening CAT defaults.
- New model providers can be introduced behind evaluation and routing controls.

### Implementation Checklist

- [x] Human and AI responsibilities are separated.
- [x] Autonomy Levels 0–5 are defined.
- [x] Level 3 is identified as the default for material external actions.
- [x] Approval records and execution scope requirements are explicit.
- [x] Promotion and risk controls are visualized.
- [x] Risks and anti-patterns are documented.
- [ ] Concrete authorization schemas are defined in security and architecture context.
- [ ] Runtime approval UI and policy engine are implemented in a later phase.

---

## 7. CAT Capability Map — What the Product Contains

### Human Explanation

A capability is a durable responsibility that CAT must perform or coordinate. A capability is not necessarily a screen, microservice, agent, or repository folder. One capability may be delivered by several components, and one component may support several capabilities. Keeping the distinction clear prevents the product roadmap from becoming a list of technical nouns.

Part 1 groups CAT's intended capabilities into eight domains:

1. **Intelligence and orchestration** — turn goals and signals into coordinated work.
2. **Market and product intelligence** — find, research, qualify, and prioritize opportunities.
3. **Affiliate operations** — manage programs, links, tracking, and partner context.
4. **Creative and content** — produce evidence-grounded, channel-appropriate assets.
5. **Publishing and distribution** — prepare, schedule, execute, and verify external distribution.
6. **Analytics and optimization** — observe performance, diagnose patterns, and recommend changes.
7. **Treasury and financial accountability** — track earnings, payouts, budgets, costs, and risk.
8. **Knowledge, supervision, and governance** — preserve context, expose decisions, enforce approval, and explain state.

These domains are product responsibilities, not a final package or deployment topology. The exact division between agents, services, core modules, and integrations is defined later.

### AI Context

An AI agent must classify requested work against the capability map before adding new abstractions. It should prefer an existing capability and extension point when the responsibility already exists. If a proposal introduces a new domain, it must explain why the eight-domain model cannot contain it and whether the new domain is CAT-specific or an Omni System platform concern.

Capability names are nouns with stable meaning. Avoid creating overlapping names such as “marketing intelligence,” “growth brain,” and “campaign intelligence” unless their ownership and boundaries are materially different.

### Business Perspective

The capability map is a portfolio tool. It allows product managers to see missing lifecycle coverage and prevents a single visible feature—such as content generation—from consuming the entire roadmap. It also makes packaging and enterprise conversations clearer: a customer can understand which capabilities are core, which are optional, and which require integrations.

### Technical Perspective

Each capability should eventually declare:

- purpose and non-goals;
- owning domain and agent(s);
- inputs and outputs;
- data classifications;
- events and APIs;
- human approval level;
- observability requirements;
- knowledge and Treasury obligations;
- dependencies and failure modes;
- extension points and versioning behavior.

A capability map is therefore the index for detailed architecture, not a substitute for it.

### Visual Overview

```mermaid
flowchart TB
    Core[CAT operating core]
    Core --> Intelligence[Intelligence and orchestration]
    Core --> Market[Market and product intelligence]
    Core --> Affiliate[Affiliate operations]
    Core --> Content[Creative and content]
    Core --> Publishing[Publishing and distribution]
    Core --> Analytics[Analytics and optimization]
    Core --> Treasury[Treasury and financial accountability]
    Core --> Governance[Knowledge, supervision, and governance]

    Market --> Content
    Affiliate --> Content
    Content --> Publishing
    Publishing --> Analytics
    Analytics --> Treasury
    Treasury --> Intelligence
    Analytics --> Governance
    Governance --> Intelligence
    Intelligence --> Market

    style Core fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Intelligence fill:#7c3aed,color:#fff
    style Market fill:#10b981,color:#fff
    style Affiliate fill:#10b981,color:#fff
    style Content fill:#7c3aed,color:#fff
    style Publishing fill:#7c3aed,color:#fff
    style Analytics fill:#06b6d4,color:#fff
    style Treasury fill:#f59e0b,color:#000
    style Governance fill:#00d4ff,color:#000
```

### Mermaid Diagram(s)

#### Capability-to-Lifecycle Coverage

```mermaid
graph LR
    Discover[Discover] --> Market[Market intelligence]
    Qualify[Qualify] --> Market
    Plan[Select and plan] --> Intelligence[Intelligence and orchestration]
    Prepare[Affiliate preparation] --> Affiliate[Affiliate operations]
    Create[Create assets] --> Content[Creative and content]
    Execute[Publish and execute] --> Publishing[Publishing and distribution]
    Observe[Observe and attribute] --> Analytics[Analytics and optimization]
    Reconcile[Reconcile] --> Treasury[Treasury]
    Learn[Learn] --> Knowledge[Knowledge and governance]

    Intelligence --> Discover
    Market --> Plan
    Affiliate --> Create
    Content --> Execute
    Publishing --> Observe
    Analytics --> Reconcile
    Treasury --> Learn
    Knowledge --> Intelligence
```

#### Capability Dependency Graph

```mermaid
flowchart TB
    Governance[Knowledge, supervision, and governance]
    Intelligence[Intelligence and orchestration]
    Market[Market and product intelligence]
    Affiliate[Affiliate operations]
    Content[Creative and content]
    Publishing[Publishing and distribution]
    Analytics[Analytics and optimization]
    Treasury[Treasury and financial accountability]
    Integrations[External integrations]

    Governance --> Intelligence
    Governance --> Treasury
    Intelligence --> Market
    Intelligence --> Affiliate
    Intelligence --> Content
    Intelligence --> Publishing
    Market --> Affiliate
    Market --> Content
    Affiliate --> Content
    Content --> Publishing
    Publishing --> Analytics
    Analytics --> Treasury
    Analytics --> Intelligence
    Treasury --> Intelligence
    Integrations <--> Market
    Integrations <--> Affiliate
    Integrations <--> Publishing
    Integrations <--> Analytics
    Integrations <--> Treasury
```

### Architecture Map

| Capability domain | Product responsibility | Likely internal participants | Primary external boundary |
|---|---|---|---|
| Intelligence and orchestration | Decompose, schedule, route, coordinate, and summarize work | CATA, workflow engine, scheduler, decision engine | Human intent, events, agent contracts |
| Market and product intelligence | Discover and qualify opportunities | Research Agent, data ingestion, knowledge retrieval | Search, feeds, merchant/product data |
| Affiliate operations | Manage affiliate programs, links, terms, and tracking | Affiliate Agent, connector adapters | Affiliate networks and merchants |
| Creative and content | Produce and validate content and media assets | Creative Agent, prompt/model services, asset storage | Model providers, brand/policy sources |
| Publishing and distribution | Send approved assets to channels and confirm status | Publisher Agent, channel adapters | CMS, social, video, email, storefront channels |
| Analytics and optimization | Measure, attribute, detect anomalies, recommend changes | Analytics Agent, event projections, reporting | Analytics and channel APIs |
| Treasury and financial accountability | Reconcile, report, budget, and monitor financial impact | Treasury Agent, financial services, audit records | Payout, accounting, banking, reporting systems |
| Knowledge, supervision, and governance | Preserve context, approval, decisions, memory, and explanation | KATA, Memory/Learning/Security agents, policy services | Human users, enterprise identity, governance systems |

### Decision Tables

#### Core vs Supporting Capabilities

| Capability type | Meaning | Promotion rule |
|---|---|---|
| Core | Required for CAT's identity and safe lifecycle | Must remain maintained and governed by CAT core owners |
| First-party domain capability | Important CAT responsibility with its own lifecycle | May evolve as a module or service under CAT contracts |
| Integration capability | Bridge to an external system | Must isolate external volatility and credentials |
| Optional extension | Useful to some deployments, not identity-defining | Prefer plugin, template, or policy extension |
| Experimental capability | Hypothesis under evaluation | Sandbox or non-default only until promoted |

#### Capability Readiness Table

| Readiness question | Acceptable evidence |
|---|---|
| Does it have a clear owner? | Named domain, module, or agent owner |
| Does it map to the lifecycle? | Stage(s), inputs, outputs, and downstream consumer |
| Is the risk understood? | Impact, reversibility, data, external side effects |
| Is the interface stable enough? | Contract, version, errors, idempotency |
| Is the outcome measurable? | Success, failure, quality, financial, and learning signals |
| Is it documented for AI and humans? | Context, examples, checklist, cross references |

### Cross References

- Root architecture layer model: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.3.
- Agent inventory: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.8.2.
- Detailed architecture: [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md).
- Domain documents: [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md), [`context/08_AFFILIATE_ENGINE.md`](./08_AFFILIATE_ENGINE.md), [`context/09_CONTENT_ENGINE.md`](./09_CONTENT_ENGINE.md).
- Directory intent: [`context/15_DIRECTORY_STRUCTURE.md`](./15_DIRECTORY_STRUCTURE.md).

### Dependencies

The capability map depends on shared lifecycle identities, agent contracts, event and API boundaries, knowledge policies, Treasury records, approval controls, and external integration adapters. Capability names should remain stable even when implementation technology changes.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Technical component mistaken for capability | Roadmap follows folders rather than value | Require purpose and outcome for each capability |
| Overlapping capability names | Duplicate ownership and inconsistent behavior | Maintain glossary and capability registry |
| Capability with no lifecycle stage | Scope expansion without product value | Reject or classify as platform/future work |
| Optional feature enters Core by convenience | Core becomes bloated and hard to govern | Use core/extension classification |
| Capability has no observable outcome | Impossible to evaluate or learn | Add outcome contract before implementation |

### Best Practices

- Describe capabilities in user and business language first.
- Give each capability one accountable owner even when several components implement it.
- Preserve domain boundaries while exposing cross-domain events and identifiers.
- Make optionality explicit to product managers and enterprise customers.
- Keep capability contracts stable while allowing internal implementations to evolve.

### Anti-patterns

- Naming a capability after a vendor or framework.
- Creating an agent for every noun without a real responsibility boundary.
- Adding a capability because a competitor has a feature, without mapping it to CAT's lifecycle.
- Calling a report a capability when it cannot influence an action or decision.
- Putting all capability logic into CATA because it is convenient.

### Future Evolution

The capability map may gain portfolio optimization, enterprise policy management, marketplace governance, cross-product knowledge, or new commerce channels. Those additions must be classified as core, domain, integration, extension, or experiment before they enter the roadmap.

### AI Construction Notes

Before editing code for a capability, an AI agent should generate a capability card:

```text
Capability name:
User/business outcome:
Lifecycle stages:
Primary Trinity pillar:
Owner:
Inputs:
Outputs:
External side effects:
Approval level:
Knowledge obligation:
Treasury obligation:
Failure modes:
Extension boundary:
```

A missing field is a design question, not permission to guess.

### Extension Points

- New domains can register a capability card and link to a dedicated context document.
- New integrations can implement an existing capability without changing its product meaning.
- Plugins can extend optional capability behavior through governed contracts.
- Enterprise policies can constrain a capability without forking its core implementation.

### Implementation Checklist

- [x] Eight high-level capability domains are defined.
- [x] Capability responsibilities are separated from technical components.
- [x] Lifecycle coverage and dependencies are visualized.
- [x] Core, domain, integration, optional, and experimental classifications are defined.
- [x] Capability readiness questions are documented.
- [ ] Detailed capability cards are authored in downstream context documents.
- [ ] Runtime ownership metadata is implemented with the product.

---

## 8. Who CAT Serves

### Human Explanation

CAT is designed for a hybrid audience. It must be understandable to a new engineer, usable by a human supervisor, inspectable by an architect, prioritizable by a product manager, and consumable by AI agents. The interface and documentation may differ by audience, but the underlying product model must remain consistent.

#### New Engineers

New engineers need to understand the product before touching implementation. They need the lifecycle, capabilities, identity boundaries, terminology, and decision hierarchy. Their primary risk is making a locally correct change that violates a global product rule.

#### AI Coding Agents

AI coding agents are first-class contributors to the repository. They need machine-readable context, explicit authority, current status, task boundaries, implementation checklists, and a clear distinction between official and speculative content. Their primary risk is confidently filling gaps that the repository has intentionally left undecided.

#### Architects

Architects need to preserve stable boundaries while allowing the system to evolve. They need the Trinity model, capability ownership, event/workflow responsibilities, approval model, and future extension paths. Their primary risk is designing infrastructure that is elegant but disconnected from product outcomes.

#### Product Managers

Product managers need to prioritize capabilities based on customer and business value. They need the problem statement, lifecycle coverage, value model, maturity phases, and explicit non-goals. Their primary risk is turning every request into Core and losing product focus.

#### Human Supervisors and Operators

Supervisors need to know what CAT can propose or execute, what requires their decision, how to inspect evidence, and how outcomes return to the knowledge system. Their primary risk is approval fatigue or insufficient context.

#### Future Contributors and Partners

Contributors and partners need extension rules, ownership boundaries, documentation expectations, and support posture. Their primary risk is coupling to internal implementation or bypassing governance.

#### Enterprise Stakeholders

Enterprise stakeholders need evidence that autonomy is controlled, data is traceable, financial operations are accountable, and local policy can be configured without forking CAT. Their primary risk is unclear shared responsibility.

### AI Context

Audience is a routing input. When producing an artifact, an AI agent must identify its primary reader and use the right level of detail:

| Artifact | Primary reader | Required emphasis |
|---|---|---|
| Project Overview | All audiences | Identity, boundaries, value, lifecycle |
| Context document | Humans and AI agents | Rules, source of truth, diagrams, implementation implications |
| ADR | Architects and maintainers | Decision, alternatives, consequences, review trigger |
| Agent specification | Agent owners and AI systems | Role, tools, inputs, outputs, autonomy, evaluation |
| API contract | Engineers, agents, partners | Schema, versioning, auth, errors, examples |
| Approval request | Human supervisor | Evidence, impact, alternatives, exact scope |
| Runbook | Operators and support | Procedures, failure modes, escalation |
| Roadmap item | Product and engineering | Outcome, scope, dependency, maturity, acceptance |

An AI agent must not optimize a document for one audience by making it unusable to the others. Use layered detail: human explanation first, structured tables and diagrams next, implementation references last.

### Business Perspective

A multi-audience product needs a shared truth with audience-specific views. The business benefit is lower coordination cost and a larger potential contributor ecosystem. A supervisor who understands approval boundaries and a developer who understands capability boundaries make fewer contradictory requests.

### Technical Perspective

Audience requirements influence access, presentation, metadata, and workflows:

- humans require summaries, evidence, and readable explanations;
- agents require structured contracts, stable terms, and deterministic rules;
- architects require dependency and evolution maps;
- enterprise users require roles, policy, audit, and integration boundaries;
- contributors require local development and review guidance.

These are presentation and governance concerns over the same canonical system state, not separate products with separate truths.

### Visual Overview

```mermaid
flowchart TB
    CAT[CAT shared product truth]
    CAT --> Engineer[New engineers]
    CAT --> Agent[AI coding agents]
    CAT --> Architect[Architects]
    CAT --> PM[Product managers]
    CAT --> Supervisor[Human supervisors]
    CAT --> Contributor[Future contributors and partners]
    CAT --> Enterprise[Enterprise stakeholders]

    Engineer --> Docs[Context, rules, architecture]
    Agent --> AIContext[Structured context and checklists]
    Architect --> Maps[Architecture and decisions]
    PM --> Roadmap[Capabilities, outcomes, phases]
    Supervisor --> Command[Evidence, approvals, status]
    Contributor --> Extensions[Contracts, SDKs, contribution rules]
    Enterprise --> Governance[Policy, audit, integration, support]
```

### Mermaid Diagram(s)

#### Human and AI Collaboration Surface

```mermaid
graph LR
    HumanAudiences[Humans<br/>engineers, architects, PMs, operators, enterprises]
    AIContributors[AI systems<br/>coding agents, domain agents, evaluators]
    Docs[Documentation and knowledge system]
    Product[CAT product and runtime]
    Governance[Policy, approval, audit]

    HumanAudiences <--> Docs
    AIContributors <--> Docs
    Docs --> Product
    Product --> Docs
    HumanAudiences <--> Governance
    AIContributors --> Governance
    Governance <--> Product
```

#### Reader Need to Artifact Map

```mermaid
flowchart TD
    Need{Reader need}
    Need -->|What is CAT?|Overview[Project Overview]
    Need -->|Why this decision?|ADR[Decision records]
    Need -->|How is it built?|Architecture[Architecture and stack]
    Need -->|How do I extend it?|Guide[Development and contribution guides]
    Need -->|What can I approve?|Approval[Approval and Treasury context]
    Need -->|How do agents behave?|Agents[Agent and prompting context]
    Need -->|What exists now?|Status[Project status and roadmap]
```

### Architecture Map

| Audience | Interface to CAT | Authority or responsibility |
|---|---|---|
| New engineer | Repository, context, code, tests, review workflow | Implements within documented rules |
| AI coding agent | `.ai` workspace, context files, task instructions, repository tools | Proposes and executes bounded changes under human review |
| Architect | Architecture maps, ADRs, contracts, dependency graphs | Preserves coherence and approves material boundaries |
| Product manager | Roadmap, capability map, outcomes, decision records | Owns prioritization and product tradeoffs |
| Supervisor | KATA/command center, approval queue, evidence | Approves or rejects consequential work |
| Contributor/partner | APIs, SDKs, plugin and contribution surfaces | Extends through governed contracts |
| Enterprise stakeholder | Policy, identity, audit, deployment, support | Configures organization-specific controls |

### Decision Tables

#### Information Need Table

| Question | Canonical source | Escalation if unclear |
|---|---|---|
| What is CAT? | This overview and root context | Lead product/architecture owner |
| What is allowed? | Project rules, security, decisions | Policy or security owner |
| What is planned? | Roadmap and status | Product owner |
| How is a component built? | Dedicated context and architecture | Domain architect |
| What requires approval? | Operating model, security, Treasury, policy | Authorized supervisor |
| What changed? | Git history, changelog, decision record | Maintainer or document owner |
| What is currently implemented? | Project status, code, tests, deployment evidence | Engineering owner |

#### Role Boundary Table

| Role | May do | May not assume |
|---|---|---|
| AI coding agent | Draft code/docs/tests, inspect repository, propose changes | Authority to redefine policy or merge its own work |
| Domain agent | Execute declared domain tasks | Authority outside its tools and action class |
| Human supervisor | Approve/reject within assigned scope | Authority to bypass higher-level policy |
| Product manager | Prioritize and define outcomes | Authority to ignore security or architecture constraints |
| Architect | Approve structural direction | Authority to accept financial or legal risk without the owner |
| External contributor | Extend approved surfaces | Access to internal secrets or unrestricted core behavior |

### Cross References

- AI workspace entry point: [`../.ai/README.md`](../.ai/README.md).
- AI workflow: [`../.ai/AI_WORKFLOW.md`](../.ai/AI_WORKFLOW.md).
- Contribution rules: [`CONTRIBUTING.md`](../CONTRIBUTING.md).
- Development guidance: [`context/19_DEVELOPMENT_GUIDE.md`](./19_DEVELOPMENT_GUIDE.md).
- Agent context: [`context/05_AGENTS.md`](./05_AGENTS.md).
- Human interface context: [`context/10_UI_UX.md`](./10_UI_UX.md).

### Dependencies

Audience support depends on documentation indexing, terminology, context ordering, role and permission models, accessible interfaces, and consistent status reporting. It also depends on maintaining one canonical product model rather than creating audience-specific contradictions.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| AI agents are treated as invisible tools | Repository cannot be safely automated | Give AI agents explicit context and review rules |
| Human supervisors receive raw technical data | Approval quality decreases | Present evidence, impact, and decisions in understandable form |
| Architects and PMs use different capability names | Roadmap and architecture diverge | Maintain capability map and glossary |
| Enterprise requirements are handled as forks | Core fragments | Use policies, extensions, and governed contracts |
| Documentation assumes expert knowledge | Onboarding slows and errors increase | Define terms and explain rationale |

### Best Practices

- Declare intended audience and reading level in every substantial document.
- Keep canonical facts shared; customize presentation, not truth.
- Give AI agents structured context before asking them to act.
- Make approval requests intelligible to a non-implementing decision owner.
- Provide examples and anti-patterns for extension surfaces.
- Update audience-facing guidance when roles or authority boundaries change.

### Anti-patterns

- Writing only for the person who already understands the system.
- Treating AI output as unreviewable because it is machine-generated.
- Giving an engineer a roadmap item with no acceptance outcome.
- Giving a product manager a technical component name instead of a capability.
- Making enterprise users edit source code to configure local policy.

### Future Evolution

CAT may provide audience-specific views of the same knowledge graph: an architectural view for engineers, a decision view for managers, a risk view for supervisors, and a governance view for enterprises. These views must remain traceable to canonical records and must not create separate undocumented truths.

### AI Construction Notes

Before generating a document or response, an AI agent should write a one-line audience declaration internally or in the artifact metadata. If several audiences are important, use a layered structure rather than choosing one and omitting the others.

### Extension Points

- New roles and personas can be added with responsibility and authority definitions.
- New document types can be added with source, owner, lifecycle, and review rules.
- Enterprise views can add organization-specific policy context without changing CAT's identity.
- AI agent interfaces can add structured tools and schemas while preserving human-readable explanations.

### Implementation Checklist

- [x] Major audiences are identified.
- [x] Reader questions and canonical sources are mapped.
- [x] Human and AI responsibilities are separated.
- [x] Audience risks and anti-patterns are included.
- [ ] Audience-specific runtime views and permissions are implemented later.
- [ ] Accessibility and localization details are defined in UI/UX context.

---

## 9. Product Boundaries — What CAT Is Not

### Human Explanation

A strong product definition includes exclusions. CAT's mission is broad, but it is not unlimited. The following boundaries protect the product from becoming a collection of unrelated AI features:

| CAT is not | Boundary meaning |
|---|---|
| **A chatbot** | Conversation is one interface, not the product's full operating model. CAT must perform and coordinate commerce work. |
| **A traditional CMS** | CAT may generate and publish content, but its value includes research, affiliate operations, analytics, learning, and Treasury. |
| **An ad network** | CAT may use affiliate networks and merchants; it is not itself an advertising exchange or network by default. |
| **A single-purpose AI tool** | CAT is not “just” a writer, researcher, link builder, or analytics dashboard. |
| **A no-code platform** | CAT is AI-native and engineering-led; no-code configuration is not its defining identity. |
| **A human replacement** | CAT shifts humans toward supervision and judgment; it does not erase accountability. |
| **A generic agent framework** | CAT may expose reusable patterns, but it has a focused commerce mission and Treasury accountability. |
| **A source of unverified truth** | CAT must preserve provenance, uncertainty, and review state. |
| **An excuse to bypass governance** | Automation does not authorize publication, spending, data access, or policy changes by itself. |

These exclusions are not statements that CAT can never integrate with a CMS, use a chatbot interface, or expose an agent SDK. They state what CAT itself is responsible for and what a surrounding tool or extension may provide.

### AI Context

The most common scope error is to treat a nearby capability as a new product identity. An AI agent must ask whether the request is:

- a CAT capability;
- a supporting implementation detail;
- an integration with another product;
- an optional extension;
- a future Omni System capability; or
- a scope conflict.

If a request would turn CAT into a generic platform, remove Treasury, remove human authority, or replace the lifecycle with one isolated task, the agent must flag the conflict before coding.

### Business Perspective

Boundaries make the business legible. Customers need to know what CAT owns, partners need to know where they integrate, and the roadmap needs to avoid competing with every adjacent category simultaneously. Saying “not a CMS” does not deny publishing; it communicates that publishing is one part of a larger closed loop.

### Technical Perspective

Boundary discipline determines module ownership and API surface. CAT may use external systems, but external systems remain external unless a formal decision makes a capability part of CAT Core. Direct database coupling, ungoverned plugin execution, and hidden fork behavior are boundary violations.

### Visual Overview

```mermaid
flowchart TB
    CAT[CAT Core identity]
    CAT --> Core[Commerce lifecycle + AI coordination + Treasury accountability]
    CAT -. integrates with .-> CMS[CMS and publishing platforms]
    CAT -. uses .-> Networks[Affiliate networks and merchants]
    CAT -. connects to .-> Analytics[External analytics systems]
    CAT -. may expose .-> Extensions[Governed plugins and APIs]
    CAT -. does not become .-> Chatbot[Chatbot only]
    CAT -. does not become .-> AdNetwork[Ad network]
    CAT -. does not become .-> Generic[Unbounded generic agent platform]
    CAT -. does not become .-> Humanless[Unaccountable human replacement]

    style CAT fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Core fill:#10b981,color:#fff
    style Chatbot fill:#ef4444,color:#fff
    style AdNetwork fill:#ef4444,color:#fff
    style Generic fill:#ef4444,color:#fff
    style Humanless fill:#ef4444,color:#fff
```

### Mermaid Diagram(s)

#### Core, Adjacent, and Excluded Zones

```mermaid
graph LR
    subgraph CoreZone[CAT Core zone]
        Lifecycle[End-to-end affiliate-commerce lifecycle]
        Trinity[Commerce + AI + Treasury]
        Governance[Knowledge and human governance]
    end
    subgraph AdjacentZone[Adjacent and integration zone]
        CMS[CMS]
        Networks[Affiliate networks]
        Channels[Publishing channels]
        Models[External AI models]
        Enterprise[Enterprise systems]
    end
    subgraph ExcludedZone[Excluded as product identity]
        ChatbotOnly[Chatbot-only product]
        AdNetworkOnly[Ad-network product]
        GenericAI[Unbounded generic AI platform]
        Unaccountable[Unaccountable automation]
    end

    CoreZone <--> AdjacentZone
    CoreZone -. remains distinct from .-> ExcludedZone
```

#### Scope Test

```mermaid
flowchart TD
    Proposal[New proposal] --> Lifecycle{Supports CAT lifecycle?}
    Lifecycle -->|No| Generic{Generic/platform-only?}
    Generic -->|Yes| Omni{Better as future Omni or external capability?}
    Generic -->|No| Reject[Reject or defer]
    Omni -->|Yes| Boundary[Define explicit contract and owner]
    Omni -->|No| Reject
    Lifecycle -->|Yes| Pillar{Supports Commerce, AI, or Treasury?}
    Pillar -->|No| Reject
    Pillar -->|Yes| Risk[Classify risk and ownership]
    Risk --> Placement{Core, domain, integration, extension, or experiment?}
    Placement --> Document[Document before implementation]
```

### Architecture Map

| Zone | CAT relationship | Ownership rule |
|---|---|---|
| Core | CAT must provide or coordinate it | CAT/Omni owners govern identity and contracts |
| Adjacent integration | CAT may connect to it | External owner remains responsible for external behavior |
| Extension | CAT may allow it through a governed seam | Extension owner is accountable within declared permissions |
| Excluded identity | CAT must not redefine itself as it | Requires separate product or explicit approved change |

### Decision Tables

#### Boundary Classification

| Proposal type | Default placement | Required decision |
|---|---|---|
| Required for every CAT deployment and identity | Core | Architecture and product approval |
| Important to one CAT domain | First-party domain module | Domain owner and architecture review |
| Connects to an external platform | Connector/integration | Security, reliability, and data review |
| Useful only to some deployments | Plugin/template/policy extension | Extension governance and compatibility review |
| General capability for many Omni products | Future Omni platform | Separate ownership and contract decision |
| Does not support lifecycle or pillars | Reject/defer | Explicit rationale recorded |

#### “Not” Does Not Mean “Never Integrates”

| Statement | Correct reading |
|---|---|
| Not a CMS | CAT can publish through or integrate with CMSs |
| Not a chatbot | CAT can use conversational interaction through KATA |
| Not an ad network | CAT can connect to affiliate and advertising-adjacent systems where governed |
| Not a generic agent framework | CAT can expose reusable contracts after its own boundaries are proven |
| Not a human replacement | CAT can automate more work while preserving human accountability |

### Cross References

- Root “What CAT Is Not”: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 5.5.
- Ecosystem and extension boundaries: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 15.
- Project rules and anti-scope-creep controls: [`context/02_PROJECT_RULES.md`](./02_PROJECT_RULES.md).
- API and plugin direction: [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md), [`context/15_DIRECTORY_STRUCTURE.md`](./15_DIRECTORY_STRUCTURE.md).

### Dependencies

Boundary enforcement depends on product ownership, architecture review, capability registry, API and plugin contracts, security policies, roadmap discipline, and a maintained glossary. A boundary that exists only in prose but is not reflected in review and implementation practices will erode.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Scope expands by analogy | CAT becomes an incoherent platform | Apply the scope test and classify placement |
| “Core” becomes a convenience label | Optional features increase maintenance and risk | Require core justification and ownership |
| Integrations leak into core | External API changes destabilize CAT | Use adapters and explicit external boundaries |
| Exclusions are read as technical bans | Useful integrations are rejected unnecessarily | Distinguish product identity from integration capability |
| Boundary policy is ignored under deadline | Shortcuts create long-term coupling | Require architecture and governance review |

### Best Practices

- Say what a feature is not when its adjacent category is likely to cause confusion.
- Prefer stable contracts over direct internal access.
- Make core-versus-extension placement part of the proposal template.
- Revisit a boundary only when evidence or strategy changes.
- Preserve excluded concepts as alternatives or future products rather than silently absorbing them.

### Anti-patterns

- Adding a generic AI assistant surface and calling it CAT.
- Building a CMS and adding affiliate links as an afterthought.
- Treating every integration request as a reason to modify Core.
- Creating a “temporary” fork for an enterprise customer.
- Calling a product boundary an implementation limitation when it is an intentional strategy.

### Future Evolution

Future Omni products may address adjacent capabilities that CAT intentionally does not own. CAT may also expose well-governed APIs, SDKs, and plugin surfaces. Any such evolution must preserve the distinction between CAT as a commerce operating system and the broader Omni platform ecosystem.

### AI Construction Notes

If a user asks for a feature that appears adjacent rather than core, the agent should produce a placement recommendation before implementation. The recommendation must state the user value, lifecycle relationship, owner, dependencies, risk, and why Core is or is not appropriate.

### Extension Points

- Connectors to CMSs, affiliate networks, analytics, models, and enterprise systems.
- Governed plugins that add specialized workflows or scoring.
- Future Omni products for capabilities outside CAT's identity.
- Public or partner APIs that expose stable, permission-scoped capabilities.

### Implementation Checklist

- [x] Explicit product exclusions are documented.
- [x] Core, adjacent, extension, and excluded zones are visualized.
- [x] Scope classification rules are provided.
- [x] Integration-versus-identity distinction is explicit.
- [x] Scope risks and anti-patterns are included.
- [ ] Detailed plugin and API contracts are defined later.
- [ ] Product review workflow enforces scope placement later.

---

## 10. Value Model and Success Criteria

### Human Explanation

CAT's value is created when it helps a commerce operation make better decisions and perform more reliable work with less manual coordination, while preserving trust and financial accountability. The product should not be judged by how futuristic it looks, how many models it calls, or how many assets it generates in isolation.

CAT has five value layers:

1. **Opportunity value:** more relevant and better-evidenced opportunities are found.
2. **Execution value:** approved work is produced and distributed faster and more consistently.
3. **Learning value:** outcomes become reusable knowledge that improves later work.
4. **Economic value:** activity, costs, earnings, payouts, and risk are visible and managed.
5. **Trust value:** humans can understand, supervise, audit, and correct the system.

A healthy CAT measurement system must balance leading indicators and lagging outcomes. A rise in generated content is a leading operational signal, not proof of business success. A rise in earnings may be a lagging signal, but it can be caused by temporary or risky behavior. Metrics must be interpreted together.

### AI Context

AI agents must not optimize an isolated metric without checking its relationship to the other value layers. For example:

- increasing publication volume may reduce content quality;
- reducing approval time may increase unauthorized actions;
- maximizing clicks may reduce conversion quality or violate trust;
- increasing model complexity may increase cost without improving decisions;
- changing a prompt may improve one category while degrading another.

Every optimization proposal should include the metric being improved, the expected side effects, the guardrail metrics, and the learning plan.

### Business Perspective

The value model connects product strategy to business outcomes without promising a specific revenue result before implementation evidence exists. It also gives product managers a way to compare foundation work with visible features:

- A knowledge or approval capability may create trust and learning value before direct revenue appears.
- A connector may create execution value by removing a costly manual handoff.
- A Treasury capability may not create revenue directly, but it can prevent loss and improve allocation.
- A research capability may create opportunity value that becomes financial value only after later stages execute.

### Technical Perspective

The value model should be represented as linked measures with consistent identities and timestamps. A measurement should state:

- definition and unit;
- source and freshness;
- population or scope;
- attribution assumptions;
- confidence and uncertainty;
- related action or decision;
- guardrails and known limitations.

The analytics and Treasury systems should not infer causal relationships merely because two events share a timestamp. Attribution and experiment design belong in detailed analytics specifications.

### Visual Overview

```mermaid
flowchart TB
    Inputs[Signals, goals, evidence, constraints]
    Inputs --> Decisions[Better decisions]
    Decisions --> Execution[More reliable approved execution]
    Execution --> Outcomes[Commerce and financial outcomes]
    Outcomes --> Learning[Knowledge and learning]
    Learning --> Decisions

    Trust[Human approval, auditability, safety, explainability]
    Trust --> Decisions
    Trust --> Execution
    Trust --> Outcomes

    Outcomes --> Value[Value for users and business]
    Learning --> Value
    Trust --> Value
```

### Mermaid Diagram(s)

#### Balanced Scorecard

```mermaid
graph TD
    CAT((CAT value))
    CAT --> Opportunity[Opportunity quality]
    CAT --> Operations[Operational efficiency]
    CAT --> Intelligence[Learning and reuse]
    CAT --> Economics[Economic accountability]
    CAT --> Trust[Trust and governance]

    Opportunity --> Leading[Leading and lagging measures]
    Operations --> Leading
    Intelligence --> Leading
    Economics --> Leading
    Trust --> Leading
```

#### Metric Guardrail Loop

```mermaid
flowchart LR
    Change[Proposed optimization] --> Primary[Primary metric]
    Change --> Guardrails[Guardrail metrics]
    Change --> Risk[Risk and approval checks]
    Primary --> Observe[Observe over defined period]
    Guardrails --> Observe
    Risk --> Observe
    Observe --> Decision{Improvement without unacceptable harm?}
    Decision -->|Yes| Promote[Promote and document]
    Decision -->|No| Rollback[Rollback or revise]
    Promote --> Knowledge[Record lesson]
    Rollback --> Knowledge
```

### Architecture Map

| Value layer | Example measures | Source categories | Primary consumer |
|---|---|---|---|
| Opportunity | Research precision, qualification confidence, product fit, source reliability | Research and knowledge | Product owner, Research Agent |
| Execution | Time to prepare, task completion, publication success, retry rate | Workflow and integration events | Operators, engineers |
| Learning | Retrieval reuse, lesson confidence, recommendation improvement, stale-knowledge rate | Knowledge and evaluation | AI and architecture owners |
| Economics | Earnings, payout state, cost, margin proxy, budget variance, reconciliation rate | Treasury and attribution | Treasury reviewer, product owner |
| Trust | Approval latency, rejection quality, policy violations, audit completeness, incident rate | Governance and security | Supervisors, enterprise stakeholders |

### Decision Tables

#### Success Metric Categories

| Category | What success means | What it must not become |
|---|---|---|
| Reliable automation | Work completes predictably within policy | Blind autonomy or volume at any cost |
| High-quality documentation | Humans and AI agents can act from current context | Documentation measured only by word count |
| Extensible architecture | New domains/integrations add value without core coupling | More abstractions without stable contracts |
| Secure operations | Risks are prevented, contained, and auditable | Security theater with no tests or controls |
| Continuous improvement | Outcomes improve decisions and workflows over time | Automatic changes without evidence or review |
| Commerce performance | Better product/channel/content outcomes | Click or revenue maximization without quality and trust |

#### Metric Interpretation Table

| Observation | Safe interpretation | Unsafe interpretation |
|---|---|---|
| More content generated | Production capacity increased | Business value definitely increased |
| Faster approval | Review workflow may be more efficient | Risk decreased or reviewers understood more |
| More clicks | Traffic changed | Conversion quality and earnings improved |
| Higher earnings | Financial outcome improved in measured scope | Strategy is universally better |
| Fewer errors | Observed errors decreased | Unobserved failures do not exist |
| Better model score | Evaluation set improved | Production behavior is safe in all contexts |

#### Current Success Direction

The repository-level roadmap names these high-level success metrics:

- reliable automation;
- high-quality documentation;
- extensible architecture;
- secure operations; and
- continuous improvement.

These are official success directions. Numeric targets, baselines, instrumentation, and acceptance thresholds are implementation and product-management work still to be specified.

### Cross References

- Roadmap success metrics: [`ROADMAP.md`](../ROADMAP.md).
- Root vision success criteria: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 6.4.
- Knowledge and learning: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md).
- Analytics and Treasury: [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md), [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md).
- AI evaluation and prompts: [`context/18_PROMPTING.md`](./18_PROMPTING.md).

### Dependencies

The value model depends on reliable event and data capture, attribution identity, Treasury reconciliation, evaluation datasets, source provenance, approval records, and a knowledge lifecycle. Without those dependencies, metrics can create false confidence.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Vanity metrics dominate | CAT optimizes activity rather than value | Require balanced scorecard and guardrails |
| Attribution is overstated | Teams credit CAT for unrelated outcomes | Document attribution assumptions and uncertainty |
| Metrics become targets | Agents game the measure | Use multiple measures and review behavior |
| Baselines are missing | Improvement cannot be demonstrated | Establish baseline before promotion |
| Financial data is delayed | Decisions use stale Treasury context | Mark freshness and pending reconciliation |
| Trust is unmeasured | Product appears successful while becoming unsafe | Include governance and incident measures |

### Best Practices

- Define a baseline and measurement window before calling an optimization successful.
- Pair every primary metric with at least one quality, risk, or cost guardrail.
- Measure knowledge reuse and decision quality, not only output count.
- Separate correlation, attribution, and causation in language and data models.
- Let humans review metric definitions for business meaning.
- Preserve metric versions when definitions change.

### Anti-patterns

- “More AI output equals more value.”
- “Revenue is the only metric that matters.”
- “A dashboard is complete when it has charts.”
- “A/B test results can update policy automatically without review.”
- “Lower latency is always better even if it removes evidence or approval.”
- “A model benchmark score is a production safety guarantee.”

### Future Evolution

CAT may develop portfolio-level value models, risk-adjusted campaign scoring, lifetime knowledge value measures, and adaptive resource allocation. Such models should expose assumptions and remain understandable to human decision owners. Future metrics must not hide value judgments inside opaque optimization systems.

### AI Construction Notes

When an agent proposes a performance improvement, require a measurement plan in the change description:

```text
Primary outcome:
Baseline:
Target or expected direction:
Guardrails:
Data sources:
Attribution assumptions:
Approval requirement:
Rollback signal:
Knowledge update:
```

### Extension Points

- New metrics can be registered with definitions, sources, owners, and guardrails.
- New evaluation datasets can measure agent or workflow quality.
- Enterprise deployments can add policy-specific compliance measures.
- New Treasury reports can add financial views without changing the product identity.
- New learning systems can consume outcome streams through governed contracts.

### Implementation Checklist

- [x] Five value layers are defined.
- [x] Leading and lagging metric distinction is explicit.
- [x] Roadmap success directions are recorded without inventing numeric targets.
- [x] Metric risks and guardrails are documented.
- [x] Optimization and learning loops are visualized.
- [ ] Baselines, numeric targets, and production instrumentation are defined in later phases.
- [ ] Metric ownership and governance are implemented with analytics and Treasury systems.

---

## 11. Current Maturity and Phase A Position

### Human Explanation

CAT is currently in **Phase A — Documentation and Foundation**. The repository is building the context, architecture, knowledge, decision, design, and engineering foundation before implementation begins in earnest. The current repository status explicitly reports implementation progress at zero and identifies documentation as the active workstream.

The roadmap describes the intended progression:

- **Phase 0 — Foundation:** repository structure, documentation framework, CAT Bible, knowledge base, ADRs, and design system.
- **Phase 1 — Core Platform:** core kernel, memory system, knowledge graph, workflow engine, agent orchestrator, and prompt engine.
- **Phase 2 — AI Agent Ecosystem:** Research, Affiliate, Creative, Publisher, Treasury, Analytics, Learning, and Security agents.
- **Phase 3 — Commerce Platform:** product discovery, store integrations, affiliate link management, content generation, and multi-channel publishing.
- **Phase 4 — Intelligence:** continuous learning, knowledge refinement, performance optimization, and decision support.
- **Phase 5 — Public Release:** beta, production, and enterprise features.

These phases are a sequence of intended capability maturity, not a guarantee that every item will be implemented exactly as listed. A phase item becomes implementation-ready when its context, decision, dependencies, acceptance criteria, and status are sufficiently defined.

The repository currently contains substantial scaffolding and documentation structure. Empty directories, placeholder documents, and planned technology names are **not evidence of runtime implementation**. This distinction is essential for accurate engineering and product communication.

### AI Context

An AI agent working in Phase A must prioritize clarity, consistency, and decision quality over premature code. It must:

- update the relevant context document before proposing implementation;
- use `.ai/PROJECT_STATUS.md` as the current progress signal;
- distinguish a completed document part from a completed runtime capability;
- avoid generating fake APIs, schemas, benchmarks, integrations, or deployment evidence;
- preserve future ideas as labeled future ideas rather than silently promoting them;
- update status after documentation work as required by repository rules.

The phrase “planned” means that the concept is in project direction or roadmap. The phrase “implemented” requires code, tests, documentation, and repository evidence. The phrase “production” requires deployment and operational evidence in addition to implementation.

### Business Perspective

A documentation-first Phase A reduces expensive rework. CAT is intended to operate across commerce, AI, and Treasury, so ambiguous foundations would multiply downstream cost. The business is intentionally investing in a shared mental model before investing in the runtime.

This phase also creates future leverage:

- AI coding agents can work from durable context.
- Architects can evaluate tradeoffs before code hardens them.
- Product managers can prioritize a coherent capability sequence.
- Enterprise stakeholders can inspect governance intent early.
- Future contributors can join without reconstructing the original vision.

### Technical Perspective

Phase A is an engineering deliverable, not administrative overhead. Its outputs include:

- authoritative context documents;
- repository and AI workflow rules;
- architecture maps and decision surfaces;
- domain boundaries and terminology;
- documentation and development standards;
- status and change-tracking mechanisms;
- a basis for tests, schemas, and implementation plans.

The implementation phase should begin from these artifacts and should treat discrepancies between code and context as defects.

### Visual Overview

```mermaid
timeline
    title CAT intended maturity path
    Phase A : Documentation and foundation
             : Repository structure
             : Context and knowledge
             : Architecture and decisions
             : Design and engineering rules
    Phase B : Core platform
             : Kernel, memory, knowledge graph
             : Workflow and orchestration
             : Prompt engine
    Phase C : Agent ecosystem
             : Research, Affiliate, Creative, Publisher
             : Treasury, Analytics, Learning, Security
    Phase D : Commerce platform
             : Product discovery and integrations
             : Content and multi-channel publishing
    Phase E : Intelligence
             : Continuous learning and optimization
             : Decision support
    Phase F : Public release
             : Beta, production, enterprise features
```

### Mermaid Diagram(s)

#### Documentation-to-Implementation Gate

```mermaid
flowchart TD
    Idea[Product or architecture idea] --> Context[Context and specification]
    Context --> Decision[Decision and tradeoff record]
    Decision --> Plan[Implementation plan and dependencies]
    Plan --> Code[Code, tests, and configuration]
    Code --> Review[Human review and automated checks]
    Review --> Docs[Documentation and status update]
    Docs --> Evidence[Runtime or repository evidence]
    Evidence --> Promotion{Ready for next maturity level?}
    Promotion -->|No| Iterate[Refine context, code, or evidence]
    Iterate --> Context
    Promotion -->|Yes| Next[Promote capability]
```

#### Status Truth Model

```mermaid
graph LR
    Direction[Vision and roadmap direction]
    Docs[Documented specification]
    Code[Implemented code]
    Tests[Verified behavior]
    Runtime[Deployment evidence]
    Status[Project status]

    Direction --> Docs --> Code --> Tests --> Runtime
    Docs --> Status
    Code --> Status
    Tests --> Status
    Runtime --> Status
```

### Architecture Map

| Maturity claim | Minimum evidence | Current Phase A interpretation |
|---|---|---|
| Concept | Product or architecture intent | Many CAT capabilities are at this level |
| Documented | Active context, scope, dependencies, decisions | The current task advances this level |
| Planned | Roadmap item with intended sequence | Roadmap contains future phases |
| Implemented | Code and tests exist and are maintained | Not generally claimed for the runtime |
| Verified | Tests, review, and observable evidence pass | Future implementation gate |
| Production | Deployed, monitored, supported, and governed | Future release gate |

### Decision Tables

#### Current State Classification

| Area | Current state | Authority |
|---|---|---|
| Project phase | Phase A / documentation and foundation | `.ai/PROJECT_STATUS.md`, `README.md` |
| Repository structure | Established and populated with scaffolds and documentation surfaces | Repository tree and previous foundation work |
| Root context | Part 4 complete for current documented scope | `context/00_PROJECT_CONTEXT.md` |
| Project Overview | Part 1 complete in this change; continuation remains | This document and project status |
| Runtime implementation | Not started / 0% in current status dashboard | `.ai/PROJECT_STATUS.md` |
| Detailed numbered context | Mostly scaffolded beyond root context and this part | Repository state; each file must be completed in sequence |
| ADR content | ADR file surfaces exist but are scaffolded | `adr/`, `decisions/`; do not infer decisions from empty files |

#### Phase Gate Table

| Gate | Entry condition | Exit evidence |
|---|---|---|
| Documentation foundation | Scope and authority are known | Context, rules, maps, terminology, and decisions are coherent |
| Core platform design | Required product concepts are documented | Architecture, interfaces, dependencies, and acceptance criteria are approved |
| Runtime implementation | Design is sufficiently stable | Code, tests, observability, and documentation are aligned |
| Agent ecosystem | Core runtime supports safe orchestration | Agent contracts, evaluations, permissions, and lifecycle are verified |
| Commerce platform | Agents and core can operate controlled workflows | Integrations, content, publishing, analytics, and Treasury are verified |
| Public release | Product and governance are operationally mature | Beta/production evidence, support, security, and enterprise controls |

### Cross References

- Current progress dashboard: [`../.ai/PROJECT_STATUS.md`](../.ai/PROJECT_STATUS.md).
- Roadmap phases: [`../ROADMAP.md`](../ROADMAP.md).
- Repository status statement: [`../README.md`](../README.md).
- Root context lifecycle and change management: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 2–4.
- AI development workflow: [`../.ai/AI_WORKFLOW.md`](../.ai/AI_WORKFLOW.md).
- Contribution Definition of Done: [`../CONTRIBUTING.md`](../CONTRIBUTING.md).

### Dependencies

Phase A depends on disciplined document ownership, cross-references, status updates, decision recording, repository structure, and consistent terminology. Implementation phases depend on Phase A not merely being large, but being internally coherent and actionable.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Documentation becomes disconnected from implementation | Future code follows outdated assumptions | Make documentation updates part of Definition of Done |
| Foundation phase becomes endless | No evidence or exit criteria for implementation | Define phase gates and implementation readiness |
| Roadmap is treated as a promise | Plans become misleading when evidence changes | Label direction, status, and decisions separately |
| Placeholder files create false confidence | Agents invent missing behavior | Check file content and runtime evidence |
| Documentation volume hides unresolved conflicts | More words do not create clarity | Maintain decision tables, owners, and contradiction reviews |
| Premature implementation hardens bad assumptions | Rework and architectural drift | Document first and require accepted boundaries |

### Best Practices

- Keep status accurate after every task.
- Use “documented,” “planned,” “implemented,” “verified,” and “production” precisely.
- Add phase exit criteria before adding large implementation scope.
- Treat documentation quality as a technical quality attribute.
- Use small, reviewable documentation and implementation increments.
- Record unresolved questions explicitly in the appropriate decision surface rather than hiding them.

### Anti-patterns

- Implementing a runtime subsystem because its directory exists.
- Claiming a phase complete because its roadmap bullets are written.
- Updating status percentages by intuition without explaining the basis.
- Writing architecture prose that cannot lead to tests or interfaces.
- Treating future ideas as requirements.
- Leaving the status dashboard stale after completing a task.

### Future Evolution

Phase A will eventually hand off to core-platform implementation, but the documentation system continues throughout the product lifecycle. Later phases should add implementation evidence, operational metrics, incident learning, and decision revisions rather than abandoning context work once code exists.

### AI Construction Notes

For a documentation task, an AI agent should update only the status claims supported by the task. For a runtime task, it should inspect code, tests, and deployment evidence before changing status. Percentages are progress indicators, not proof; the evidence and completed checklist carry the actual meaning.

### Extension Points

- Add new roadmap phases only through explicit prioritization and dependency review.
- Add maturity states when the existing evidence model cannot distinguish a meaningful transition.
- Add automated status checks that compare documented claims with repository evidence.
- Add generated architecture maps after the underlying contracts stabilize.

### Implementation Checklist

- [x] Phase A purpose and current position are stated.
- [x] Roadmap progression is connected to maturity evidence.
- [x] Documentation and runtime claims are separated.
- [x] Current repository limitations are stated without inventing implementation.
- [x] Phase gates, risks, and anti-patterns are documented.
- [x] Project status is updated alongside this Part 1.
- [ ] Remaining Project Overview parts are completed in later tasks.

---

## 12. Official Decisions, Recommendations, Future Ideas, and Experimental Concepts

### Human Explanation

CAT documentation must distinguish what the project has decided from what it prefers, imagines, or is testing. Without that distinction, an AI agent may treat a cinematic future interface as a current requirement, or a recommendation as a mandatory architectural constraint.

This section is the classification register for Part 1. It does not replace formal ADRs. It makes the current orientation explicit until the corresponding decision surfaces are expanded.

### AI Context

When a statement is not clearly classified, treat it as **not yet safe to implement as a binding requirement**. An AI agent should locate or create the appropriate decision record before promoting it. The agent should not silently change a Future Idea into an Official Decision through code.

### Business Perspective

Classification protects investment. Official decisions receive preservation and implementation effort. Recommendations can be optimized. Future ideas protect strategic imagination without consuming current capacity. Experimental concepts allow learning without putting unstable behavior into the default product.

### Technical Perspective

Classification affects architecture and change control:

- Official Decisions become constraints and compatibility obligations.
- Recommendations become defaults with documented deviation paths.
- Future Ideas remain outside current acceptance criteria unless promoted.
- Experimental Concepts require isolation, measurement, rollback, and explicit promotion criteria.

### Visual Overview

```mermaid
flowchart LR
    Idea[Idea or statement] --> Classify{Classify}
    Classify --> Official[Official Decision]
    Classify --> Recommend[Recommendation]
    Classify --> Future[Future Idea]
    Classify --> Experiment[Experimental Concept]

    Official --> Preserve[Implement or preserve]
    Recommend --> Default[Use by default; record deviations]
    Future --> Backlog[Keep in roadmap or strategic backlog]
    Experiment --> Sandbox[Isolate, measure, and govern]
```

### Mermaid Diagram(s)

#### Promotion and Demotion Paths

```mermaid
stateDiagram-v2
    [*] --> Idea
    Idea --> Recommendation: useful default
    Idea --> FutureIdea: beyond current phase
    Idea --> Experimental: requires evidence
    Recommendation --> Official: accepted by authority
    Experimental --> Recommendation: evidence supports default
    Experimental --> Official: formal decision and review
    FutureIdea --> Official: scope and dependencies accepted
    Official --> Superseded: newer decision
    Recommendation --> Rejected: no longer useful
    FutureIdea --> Rejected: strategy changed
    Experimental --> Rejected: evidence fails or risk unacceptable
    Superseded --> Archived
    Rejected --> Archived
    Archived --> [*]
```

### Architecture Map

| Classification | Repository behavior | Runtime behavior |
|---|---|---|
| Official | Active source, linked from indexes, required in reviews | Default constraint or capability |
| Recommendation | Documented preferred path, deviation requires rationale | Default configuration or approach |
| Future | Roadmap/context note with no current acceptance claim | Not enabled by default |
| Experimental | Isolated design, feature flag/sandbox, evaluation record | Limited exposure and rollback |

### Decision Tables

#### Part 1 Official Decisions

| ID | Decision | Evidence/source | Status |
|---|---|---|---|
| CAT-OVR-DEC-001 | CAT is an AI-native autonomous commerce operating system. | `README.md`, `context/00_PROJECT_CONTEXT.md` | Official |
| CAT-OVR-DEC-002 | CAT automates the complete affiliate-commerce lifecycle. | `README.md`, `ROADMAP.md`, root mission | Official |
| CAT-OVR-DEC-003 | Commerce, AI, and Treasury are the Trinity pillars. | Root context and product identity | Official |
| CAT-OVR-DEC-004 | Humans approve critical financial, reputational, legal, external, or irreversible actions. | Root philosophy, contribution principles | Official |
| CAT-OVR-DEC-005 | CAT uses a modular specialized-agent ecosystem coordinated by CATA. | Root architecture and agent philosophy | Official direction |
| CAT-OVR-DEC-006 | Knowledge, memory, and continuous improvement are product requirements. | Root philosophy and knowledge sections | Official direction |
| CAT-OVR-DEC-007 | Security is designed in, not added after implementation. | README, contribution principles, root philosophy | Official |
| CAT-OVR-DEC-008 | The product is documented before runtime implementation is expanded. | README, roadmap, contribution rules, current Phase A status | Official process decision |
| CAT-OVR-DEC-009 | KATA is the human-facing persona/interface and CATA is the internal coordinator. | Root context | Official product architecture direction |
| CAT-OVR-DEC-010 | CAT is initially focused on affiliate commerce under Omni System. | Project identity and roadmap | Official scope |

#### Recommendations

| Recommendation | Why it is preferred | How to deviate |
|---|---|---|
| Use event-driven, asynchronous workflows for long-running operations | Supports continuous operation and failure isolation | Record a decision when synchronous behavior is materially better |
| Use risk-based approval rather than approval for every action | Preserves safety without creating a human bottleneck | Add stricter enterprise policy when required |
| Use structured evidence bundles for recommendations | Improves human review and auditability | Document why a simpler output is safe |
| Use shared identifiers across Trinity domains | Enables attribution, Treasury, and learning | Add a translation layer only with a documented boundary |
| Prefer additive, reversible evolution | Reduces long-term blast radius | Justify destructive migration or breaking change |
| Keep extensions behind stable contracts | Enables ecosystem growth and isolates external volatility | Record a boundary exception |

#### Future Ideas

| Future idea | Why it is not a current requirement |
|---|---|
| Public plugin marketplace | Requires mature permissions, packaging, review, support, and revocation |
| Multi-node or federated CAT instances | Requires validated single-node architecture and distributed governance |
| Enterprise multi-tenant capabilities | Requires security, identity, isolation, billing, and support decisions |
| Cross-product Omni knowledge federation | Requires stable shared schemas and privacy/ownership decisions |
| Highly proactive autonomous strategy | Requires evidence, policy, and business trust beyond the documentation phase |

#### Experimental Concepts

| Experimental concept | Safe interpretation |
|---|---|
| Adaptive autonomy based on performance | Hypothesis to test under explicit policy, not automatic authority expansion |
| Self-improving prompt/model routing | Must use evaluation, versioning, rollback, and cost controls |
| Autonomous policy recommendation | May propose changes; cannot silently enact them |
| Rich cosmic-avatar state expression | Visual experiment until accessibility and performance requirements are verified |
| Cross-agent emergent collaboration | Must remain inside explicit contracts and observability |

### Cross References

- Formal decision directory: [`../decisions/README.md`](../decisions/README.md), [`../decisions/ADR_INDEX.md`](../decisions/ADR_INDEX.md), [`../adr/`](../adr/).
- Root decision rationale: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 8, 10, 12–15.
- Rules and development decisions: [`context/02_PROJECT_RULES.md`](./02_PROJECT_RULES.md), [`context/12_DECISIONS.md`](./12_DECISIONS.md).

### Dependencies

Decision classification depends on authority, ownership, versioning, change history, evidence, review, and cross-document consistency. A classification without an owner or promotion path will become ambiguous over time.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Future idea treated as requirement | Premature scope and implementation | Use visible maturity labels |
| Recommendation treated as law | Useful experimentation is blocked | Record default and deviation path |
| Experimental concept reaches production silently | Unmeasured risk and instability | Sandbox, flag, evaluation, and promotion gate |
| Official decision has no formal record | Future agents cannot find rationale | Promote accepted decisions into decision surfaces |
| Conflicting sources remain active | Contributors choose arbitrarily | Add authority and supersession links |

### Best Practices

- Put classification next to the statement, not only in a separate index.
- Record evidence and owner for every official decision.
- Define promotion and review triggers for future and experimental concepts.
- Preserve rejected alternatives and rationale.
- Use versioned changes when a classification changes.

### Anti-patterns

- Using “must” for a future idea.
- Calling a prototype “the CAT architecture.”
- Hiding experimental behavior behind a default configuration.
- Treating a roadmap bullet as an accepted design.
- Creating an ADR number without writing its decision content.

### Future Evolution

As `adr/` and `decisions/` are populated, this register should link each official decision to its formal record and reduce duplication. The overview should retain the decision's product consequence while the formal record retains detailed alternatives and implementation history.

### AI Construction Notes

An AI agent should use the classification vocabulary in issue descriptions, plans, code comments, and documentation. If uncertain, classify conservatively and ask for the missing authority rather than promoting the statement.

### Extension Points

- Add decision IDs and formal records.
- Add automated checks for unlabeled future or experimental language.
- Add review triggers and ownership metadata.
- Add architecture and product indexes that consume the classification register.

### Implementation Checklist

- [x] Four classifications are defined.
- [x] Official direction for Part 1 is recorded.
- [x] Recommendations, future ideas, and experiments are separated.
- [x] Promotion and demotion flow is visualized.
- [x] Current scaffold limitation for ADR files is stated accurately.
- [ ] Formal decision records are expanded in the decisions phase.

---

## 13. Cross-Document Orientation and Dependency Map

### Human Explanation

CAT is documented as a connected system. A reader should not treat this file as an isolated encyclopedia. The overview points to more specialized documents, and those documents should point back when they depend on product identity or lifecycle rules.

The recommended context path is:

```text
.ai/README.md
  → .ai/BOOTSTRAP.md and .ai/CONTEXT_ORDER.md
  → .ai/PROJECT_STATUS.md and .ai/TASKS.md
  → context/00_PROJECT_CONTEXT.md
  → context/01_PROJECT_OVERVIEW.md
  → context/02_PROJECT_RULES.md
  → context/03_TECH_STACK.md
  → context/04_ARCHITECTURE.md
  → task-specific context documents
  → context/99_AI_BOOTSTRAP.md
```

A human does not need to read every document for every task. An AI system working on a project-level change does. A domain change should load this overview, the root context, the relevant domain document, the rules, architecture, security, and decisions that govern the change.

### AI Context

Cross-references are executable context. An AI agent should follow links by relevance and authority, not merely count them. Before editing a file, it should search for:

- the owning context document;
- the term definition;
- the relevant decision;
- architecture and security implications;
- dependent documents and status claims.

Broken links, duplicate definitions, or stale references are documentation defects and should be reported or corrected as part of the task when in scope.

### Business Perspective

A dependency map reduces onboarding and coordination cost. It makes it possible for a product manager to know where a decision lives, for an engineer to find the implementation contract, and for an enterprise reviewer to trace a control from product identity to runtime policy.

### Technical Perspective

The dependency map is a documentation architecture. It defines a directed graph of context, decisions, code, tests, and operations. Changes to a high-authority node should trigger review of dependent nodes. A project overview change may affect rules, agents, domain documents, UI language, terminology, and roadmap status.

### Visual Overview

```mermaid
flowchart TB
    Root[00 Project Context]
    Overview[01 Project Overview]
    Rules[02 Project Rules]
    Stack[03 Tech Stack]
    Architecture[04 Architecture]
    Agents[05 Agents]
    Knowledge[06 Knowledge Engine]
    Treasury[07 Treasury Core]
    Affiliate[08 Affiliate Engine]
    Content[09 Content Engine]
    UI[10 UI/UX]
    Design[11 Design Language]
    Decisions[12 Decisions]
    Terms[13 Terminology]
    Coding[14 Coding Standard]
    Structure[15 Directory Structure]
    Deployment[16 Deployment]
    Security[17 Security]
    Prompting[18 Prompting]
    Development[19 Development Guide]
    Bootstrap[99 AI Bootstrap]

    Root --> Overview
    Overview --> Rules
    Rules --> Stack
    Stack --> Architecture
    Architecture --> Agents
    Architecture --> Knowledge
    Architecture --> Treasury
    Architecture --> Affiliate
    Architecture --> Content
    Overview --> UI
    UI --> Design
    Rules --> Decisions
    Overview --> Terms
    Coding --> Structure
    Stack --> Deployment
    Rules --> Security
    Agents --> Prompting
    All[All active context] --> Development
    Development --> Bootstrap

    style Overview fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Root fill:#7c3aed,color:#fff
    style Architecture fill:#7c3aed,color:#fff
    style All fill:#06b6d4,color:#fff
```

### Mermaid Diagram(s)

#### Change Impact Flow

```mermaid
flowchart LR
    Change[Change to product identity or lifecycle] --> Overview[Project Overview]
    Overview --> Terms[Terminology]
    Overview --> Rules[Project Rules]
    Overview --> Architecture[Architecture]
    Overview --> Agents[Agent roles]
    Overview --> Domains[Commerce and Treasury domains]
    Overview --> UI[UI and design language]
    Overview --> Roadmap[Roadmap and status]
    Domains --> Tests[Tests and acceptance criteria]
    Architecture --> Tests
    Rules --> Tests
```

#### Task Context Selection

```mermaid
flowchart TD
    Task[Task arrives] --> Scope{Scope}
    Scope -->|Project identity or roadmap| P[00 + 01 + status + roadmap]
    Scope -->|Architecture or stack| A[00 + 01 + 02 + 03 + 04 + decisions]
    Scope -->|Agent behavior| G[00 + 01 + 02 + 04 + 05 + 06 + 17 + 18]
    Scope -->|Treasury| T[00 + 01 + 02 + 04 + 06 + 07 + 17]
    Scope -->|Affiliate/content| C[00 + 01 + 02 + 04 + 05 + 08/09 + 17]
    Scope -->|UI/UX| U[00 + 01 + 02 + 10 + 11 + 17]
    Scope -->|Repository change| R[.ai workspace + rules + coding + directory + development]
```

### Architecture Map

| Layer | Documents | Responsibility |
|---|---|---|
| Root intent | `context/00_PROJECT_CONTEXT.md` | Vision, mission, philosophy, foundational context |
| Product orientation | `context/01_PROJECT_OVERVIEW.md` | Identity, lifecycle, boundaries, value, audiences |
| Rules and implementation context | `context/02–19` | Detailed engineering, agent, domain, design, and operating contracts |
| AI workspace | `.ai/*` | AI navigation, status, workflow, memory, and prompt guidance |
| Decisions | `adr/*`, `decisions/*`, `.ai/DECISION_INDEX.md` | Decision records, alternatives, and rationale |
| Architecture maps | `architecture/*`, `.ai/ARCHITECTURE_MAP.md` | Cross-cutting structural and flow views |
| Knowledge and research | `knowledge/*`, `research/*` | Curated domain evidence and institutional context |
| Runtime implementation | `core/*`, `backend/*`, `frontend/*`, `services/*`, etc. | Future code and deployment artifacts |

### Decision Tables

#### Dependency Type Table

| Dependency type | Example | Review trigger |
|---|---|---|
| Authority dependency | Overview depends on root context | Root identity or philosophy changes |
| Terminology dependency | “Campaign” used across Commerce and Treasury | Definition changes or ambiguity appears |
| Workflow dependency | Publishing depends on approval and content readiness | State or gate changes |
| Data dependency | Treasury depends on attribution identity | Schema or source changes |
| Governance dependency | Agent tool depends on security and approval policy | Permission or risk changes |
| Status dependency | Roadmap claims depend on implementation evidence | Phase or completion status changes |

#### Cross-Reference Quality Table

| Check | Good result |
|---|---|
| Target exists | Link resolves to a real repository path |
| Target is relevant | Reader lands on the source of the referenced rule |
| Authority is clear | Link text tells reader whether target is root, domain, decision, or guide |
| No hidden contradiction | Referenced document agrees or states a supersession relationship |
| Status is honest | Future/planned documents are not presented as implemented |

### Cross References

- AI context order: [`../.ai/CONTEXT_ORDER.md`](../.ai/CONTEXT_ORDER.md).
- Repository document index: [`../.ai/DOCUMENT_INDEX.md`](../.ai/DOCUMENT_INDEX.md).
- Architecture map: [`../.ai/ARCHITECTURE_MAP.md`](../.ai/ARCHITECTURE_MAP.md).
- Decision index: [`../.ai/DECISION_INDEX.md`](../.ai/DECISION_INDEX.md).
- Root hierarchy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Document Hierarchy section.

### Dependencies

This map depends on repository paths remaining stable or being updated when files move. It also depends on status and document indexes being maintained. A documentation graph cannot be trusted if files are renamed without repairing links and authority references.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Broken links | Readers and agents lose context | Link checks and review |
| Circular or contradictory authority | No reliable decision path | Explicit hierarchy and supersession |
| Over-linking | Readers cannot identify the important source | Link only relevant dependencies |
| Under-linking | Rationale and implementation context are missed | Cross-reference all material boundaries |
| Stale index | Agents load wrong or obsolete files | Update indexes with document changes |

### Best Practices

- Link to source documents at the point of use.
- Use repository-relative links for durable GitHub rendering.
- Keep links semantically descriptive.
- Add dependency and impact notes to material changes.
- Review downstream documents when identity or lifecycle definitions change.

### Anti-patterns

- Linking to a search result instead of the canonical document.
- Repeating a rule in many files without naming the source authority.
- Leaving a link because the path “probably” exists.
- Treating document indexes as optional navigation decoration.
- Using a cross-reference to avoid explaining the concept at all.

### Future Evolution

The repository may add generated indexes, broken-link checks, impact analysis, and machine-readable document metadata. Those tools should improve navigation without replacing human-readable context.

### AI Construction Notes

An AI agent should validate links and references as part of documentation work. If a referenced file is only a scaffold, state that it is a future or incomplete dependency. Never infer content from a filename.

### Extension Points

- Add domain-specific dependency maps.
- Add machine-readable document manifests.
- Add automated document graph and impact reports.
- Add decision-to-code and code-to-document traceability.

### Implementation Checklist

- [x] Context reading order is stated.
- [x] Document layers and dependencies are mapped.
- [x] Change impact and task-specific loading are visualized.
- [x] Cross-reference quality rules are included.
- [ ] Automated link and impact checks are implemented later.

---

## 14. Part 1 Completion Contract

### Human Explanation

Part 1 is complete when this document gives a reader a reliable orientation to CAT without requiring implementation details. It is deliberately broad but not shallow: each major product concept is tied to business purpose, technical consequence, risk, documentation, and future evolution.

The completion contract also defines what this part does **not** claim. It does not claim that the core kernel, knowledge graph, workflow engine, agent ecosystem, affiliate integrations, content pipeline, Treasury runtime, or production deployment already exist. It establishes the product model that those future implementations must honor.

### AI Context

An AI agent may use Part 1 to orient a task, classify a proposal, identify the responsible downstream context, and check whether a change violates product identity. It must load the more specific context before implementing domain behavior.

### Business Perspective

The value of Part 1 is alignment and reduced rework. It creates a shared product definition that can survive team, model, technology, and market changes. It also provides a basis for deciding which later work is core, optional, future, or experimental.

### Technical Perspective

The part is a traceability node between root philosophy and detailed specifications. Its acceptance criteria are documentation quality criteria: source authority, clear concepts, stable terminology, diagrams for complex relationships, explicit decisions, dependency awareness, and honest maturity claims.

### Visual Overview

```mermaid
flowchart TB
    Root[Root context: why and philosophy]
    Part1[Project Overview Part 1: what CAT is and how to orient]
    FutureParts[Project Overview Parts 2+]
    Detailed[Detailed context 02–19]
    Implementation[Future implementation and verification]

    Root --> Part1
    Part1 --> FutureParts
    Part1 --> Detailed
    FutureParts --> Detailed
    Detailed --> Implementation
    Implementation -->|evidence and lessons| Part1
    Implementation -->|evidence and lessons| Root
```

### Mermaid Diagram(s)

#### Part 1 Coverage Map

```mermaid
graph TD
    P1[Project Overview Part 1]
    P1 --> HowTo[Reading protocol]
    P1 --> Identity[CAT identity]
    P1 --> Problem[Why CAT exists]
    P1 --> Trinity[Commerce AI Trinity]
    P1 --> Lifecycle[End-to-end lifecycle]
    P1 --> Operating[AI operator / human supervisor]
    P1 --> Capabilities[Capability map]
    P1 --> Audiences[Audiences]
    P1 --> Boundaries[What CAT is not]
    P1 --> Value[Value and success]
    P1 --> Maturity[Phase A and maturity]
    P1 --> Decisions[Decision classifications]
    P1 --> Dependencies[Cross-document dependencies]
```

### Architecture Map

Part 1 hands off to later documentation through these boundaries:

| Part 1 establishes | Later document expands |
|---|---|
| Product identity and Trinity | Domain architecture and service boundaries |
| Lifecycle stages | Workflow state, events, APIs, and persistence |
| AI/human operating model | Agent roles, policy, security, and UI implementation |
| Capability map | Technical modules, schemas, integrations, and tests |
| Value model | Analytics, Treasury, evaluation, and operational metrics |
| Maturity model | Phase plans, implementation evidence, deployment, and release |
| Decision classifications | Formal ADRs, decision index, and review triggers |

### Decision Tables

#### Part 1 Acceptance Table

| Requirement | Evidence in this part | Status |
|---|---|---|
| Explain what CAT is before how it is built | Sections 2–5 lead with human/product meaning | Complete |
| Define the Commerce AI Trinity | Section 4 with maps and decisions | Complete |
| Explain why CAT exists | Section 3 with problem and response model | Complete |
| Describe the end-to-end lifecycle | Section 5 with ten stages and state flow | Complete |
| Define human and AI responsibilities | Section 6 with autonomy levels and gates | Complete |
| Map core capabilities | Section 7 with domains and dependencies | Complete |
| Identify primary audiences | Section 8 with reader needs and roles | Complete |
| Protect product boundaries | Section 9 with scope zones and tests | Complete |
| Define value and success direction | Section 10 with balanced metrics | Complete |
| State current maturity honestly | Section 11 with evidence model | Complete |
| Separate decision classes | Section 12 with official/recommendation/future/experimental tables | Complete |
| Explain documentation dependencies | Section 13 with maps and quality rules | Complete |
| Include CAT Documentation Standard v2 elements | Major sections include explanation, context, perspectives, visuals, maps, decisions, links, dependencies, risks, practices, anti-patterns, evolution, AI notes, extension points, and checklists | Complete |

#### Continuation Boundary Table

| Future part | Intended subject boundary |
|---|---|
| Part 2 | Deeper product model, operating scenarios, domain relationships, and capability contracts |
| Part 3 | Ecosystem, platform relationships, extensibility, and long-term product evolution |
| Part 4 | Decision integration, implementation-readiness guidance, and final overview closure |

These continuation boundaries are planning guidance, not permission to implement future capabilities before their documentation and decisions are complete.

### Cross References

- Root context closure and downstream context list: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md).
- Current status update: [`../.ai/PROJECT_STATUS.md`](../.ai/PROJECT_STATUS.md).
- Next numbered context document: [`context/02_PROJECT_RULES.md`](./02_PROJECT_RULES.md).
- Planned technology and architecture detail: [`context/03_TECH_STACK.md`](./03_TECH_STACK.md), [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md).
- Formal decisions: [`../decisions/`](../decisions/), [`../adr/`](../adr/).

### Dependencies

The completion contract depends on later documents preserving the identity and boundaries established here. If a later document requires a different product model, it must propose and accept a documented decision rather than silently diverging.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Part 1 is read without downstream context | Implementation details are guessed | Follow task-specific reading paths |
| Later parts duplicate or contradict Part 1 | Overview loses authority | Use section ownership and cross-reference review |
| Completion is mistaken for runtime delivery | Product claims become inaccurate | Keep status and maturity evidence explicit |
| Future parts become unbounded | Documentation itself becomes scope creep | Preserve continuation boundaries and task sequencing |

### Best Practices

- Use Part 1 as the orientation layer and defer details to the owning context document.
- Update this part only when product identity, lifecycle, boundaries, value, or maturity claims change.
- Keep future parts additive unless an accepted decision supersedes an earlier statement.
- Preserve all four decision classifications.
- Review the status dashboard whenever a part is completed.

### Anti-patterns

- Copying the entire architecture or tech stack into the overview.
- Treating a future part's subject list as implemented scope.
- Removing risks and anti-patterns to make the document look more certain.
- Updating a product definition without checking downstream context.
- Calling Part 1 complete while leaving the required status update undone.

### Future Evolution

The Project Overview will become the stable executive and engineering entry point for CAT. As implementation creates evidence, this document should gain precise links, validated examples, and measured outcomes while preserving its what-before-how role. It should become more accurate over time, not merely longer.

### AI Construction Notes

At the end of a future documentation part, an AI agent must verify:

- the document's declared part and status;
- consistency with the root context;
- all official decisions and classifications;
- cross-reference validity;
- implementation versus future claims;
- `.ai/PROJECT_STATUS.md` progress, active task, next task, and date;
- the required conventional commit and push workflow.

### Extension Points

- Later parts may add scenario catalogs, domain contracts, enterprise views, or validated implementation examples.
- Formal decisions can be linked as they are authored.
- Automated documentation quality tooling can consume the checklists and classification vocabulary.
- Runtime evidence can be attached to maturity claims after implementation begins.

### Implementation Checklist

- [x] Part 1 is written as GitHub-ready Markdown.
- [x] The document begins with product identity and orientation rather than implementation detail.
- [x] Mermaid is used for document hierarchy, identity, problem response, Trinity, lifecycle, autonomy, capabilities, audiences, scope, value, maturity, decisions, dependencies, and completion.
- [x] Official decisions are separated from recommendations, future ideas, and experimental concepts.
- [x] Risks, best practices, anti-patterns, future evolution, AI construction notes, extension points, and implementation checklists are present throughout.
- [x] Project status is updated in the same task.
- [x] The required commit message and branch push are part of the completion procedure.
- [ ] Project Overview Part 2 remains for a subsequent task.

---

*End of Part 1 of `context/01_PROJECT_OVERVIEW.md`. Part 2 continues the CAT executive and engineering overview from this product identity, lifecycle, capability, governance, and maturity foundation.*

---

# Part 2 — Internal Organization of the CAT Platform

> **Part 2 purpose:** explain how the CAT ecosystem is organized internally so that a reader can form a complete conceptual platform model before studying the detailed architecture or technology stack.
>
> Part 1 established what CAT is, why it exists, what it does, who it serves, and where it ends. Part 2 explains the internal arrangement that makes those promises coherent: conceptual layers, domain ownership, component relationships, user authority, platform boundaries, AI-first design, and extension contracts.

Part 2 uses the word **platform** in a precise sense. CAT is a platform because it supplies stable internal capabilities and contracts on which multiple commerce workflows, agents, integrations, user types, and future modules can operate. It is not merely a collection of screens, and it is not a single workflow hidden behind one API.

The diagrams in this part are conceptual architecture maps. They define responsibilities and relationships that later architecture documents must implement. They do not silently select a programming language, cloud vendor, database, model provider, or deployment topology.

---

## 15. Part 2 Reading Contract — How to Reason About Internal Organization

### Human Explanation

The most useful mental model for CAT is a set of **cooperating planes** rather than a single vertical stack:

- The **experience plane** lets people express intent, inspect state, and approve consequential actions.
- The **identity and governance plane** decides who or what may act, under which policy, and with which audit obligations.
- The **orchestration plane** turns goals and signals into durable, resumable work.
- The **domain plane** performs Commerce, Affiliate, Content, Marketing, Analytics, Treasury, and administrative work.
- The **intelligence plane** supplies knowledge, memory, reasoning, learning, and model capabilities.
- The **foundation plane** carries events, data, files, secrets, observability, and runtime reliability.

A plane is a conceptual responsibility boundary. It may be implemented by modules, services, agents, libraries, or managed infrastructure. A plane may contain several components, but it should not become an excuse for components to bypass ownership and contracts.

A reader should be able to trace any important CAT operation through these questions:

1. Who or what initiated the work?
2. Which identity and policy authorize it?
3. Which orchestrator or workflow owns its state?
4. Which domain owns the business responsibility?
5. Which intelligence capabilities provide context and reasoning?
6. Which foundation components persist, communicate, and observe it?
7. Which human or system receives the result?
8. Which knowledge, Treasury, and audit records are produced?

### AI Context

For an AI system, internal organization is a routing map. The agent must determine whether a requested change belongs to:

- experience and presentation;
- identity, policy, or administration;
- orchestration and automation;
- a business domain;
- knowledge and intelligence;
- a cross-cutting foundation capability; or
- an external integration or extension.

The agent must not put domain logic into a UI component, put policy into an arbitrary agent prompt, put Treasury rules into a reporting view, or use CATA as a place to hide all behavior. It should route work to the narrowest responsible boundary and load that boundary's context before editing.

### Business Perspective

Internal organization is a business asset. Clear boundaries allow CAT to add new affiliate networks, content channels, enterprise policies, user types, and AI capabilities without rewriting the whole platform. They also make ownership visible: an organization can know which team or role owns a product decision, a financial record, an integration, an agent, or an approval policy.

A platform that cannot explain who owns a capability is difficult to operate, sell, support, or audit. Internal clarity is therefore part of product quality, not only an engineering preference.

### Technical Perspective

The internal model combines four boundary types:

| Boundary type | Question it answers | Typical contract |
|---|---|---|
| Responsibility boundary | Who owns the meaning and outcome? | Domain or capability contract |
| Control boundary | Who may perform or approve it? | Identity, permission, policy, and approval contract |
| Data boundary | Which information is read or written? | Schema, provenance, classification, and retention contract |
| Execution boundary | How does work start, pause, resume, fail, and complete? | Workflow, event, task, and idempotency contract |

A component is not well-defined until these boundaries are explicit enough for another component or AI agent to interact with it safely.

### Official Decisions

| ID | Decision | Consequence |
|---|---|---|
| P2-ORG-DEC-001 | CAT is organized by responsibilities and contracts, not by a flat list of screens or repositories. | Component placement begins with product ownership and lifecycle responsibility. |
| P2-ORG-DEC-002 | Higher-level intent and governance may constrain lower-level execution, but lower-level components must not silently redefine higher-level policy. | Policy and ownership flow downward through explicit interfaces. |
| P2-ORG-DEC-003 | Cross-domain operations require shared identity, correlation, provenance, and outcome records. | Commerce, AI, and Treasury cannot become isolated data silos. |
| P2-ORG-DEC-004 | Conceptual organization is defined before concrete technology selection. | The tech stack must implement the model rather than define product meaning. |

### Recommendations

- Prefer a single accountable owner per capability, even when many components implement it.
- Use asynchronous, durable workflows for work that can wait on humans, external systems, or long-running AI tasks.
- Keep read/query views separate from side-effecting commands when this improves safety and clarity.
- Make data classification and audit requirements part of component design, not deployment cleanup.
- Treat every cross-plane call as a contract with explicit failure behavior.

### Experimental Ideas

- A machine-readable platform manifest that lets an AI agent discover planes, domains, owners, permissions, and extension points.
- An automated dependency reviewer that flags a component when it reaches across a boundary without a declared contract.
- A visual “system health organism” that maps live events and ownership to the CAT command-center experience.

These concepts are useful design explorations. They are not current runtime requirements.

### Future Ideas

- Region-specific plane deployment for latency, data residency, or market specialization.
- Federated domain instances coordinated by a global CAT control plane.
- A shared Omni System platform plane extracted from stable CAT contracts.
- Tenant-specific policy planes that extend governance while preserving CAT Core behavior.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Conceptual planes are mistaken for deployment services | Premature microservice fragmentation | Keep responsibility maps separate from runtime topology |
| Components cross boundaries for convenience | Hidden coupling and policy bypass | Require contract and dependency review |
| CATA becomes a “god component” | Every change becomes hard to test or own | Keep orchestration separate from domain execution |
| Domain ownership is unclear | Conflicting writes and unresolved incidents | Canonical owner and RACI metadata |
| AI agents infer structure from folder names | Incorrect implementation placement | Load context and contracts, not only directory listings |

### Anti-patterns

- Designing the platform from the navigation menu outward.
- Treating every plane as a deployable microservice.
- Allowing a dashboard to write domain state directly.
- Letting agents call one another through undocumented private functions.
- Storing policy only inside prompts or client-side code.
- Defining a subsystem without an owner, failure mode, or recovery path.

### Best Practices

- Begin every subsystem proposal with purpose, owner, responsibility, inputs, outputs, dependencies, side effects, and recovery.
- Use diagrams to show control flow, data flow, and knowledge flow separately when one diagram becomes ambiguous.
- Keep canonical writes close to the domain that owns the meaning of the data.
- Use events for facts that other components may consume and commands for requested actions.
- Make human approval and machine authorization visible in the same workflow model.

### Extension Points

- New planes may be introduced only when a responsibility is genuinely cross-cutting and cannot be owned by an existing plane.
- New domains register ownership, contracts, dependencies, and lifecycle coverage.
- New infrastructure providers implement foundation interfaces without changing domain meaning.
- New user types use identity and policy contracts rather than special-case domain code.

### Cross References

- Root architecture philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13 and 14.
- Part 1 lifecycle and capabilities: [Section 5](#5-what-cat-does--the-end-to-end-commerce-lifecycle), [Section 7](#7-cat-capability-map--what-the-product-contains).
- Detailed architecture target: [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md).
- Technology boundary target: [`context/03_TECH_STACK.md`](./03_TECH_STACK.md).
- AI and repository navigation: [`../.ai/ARCHITECTURE_MAP.md`](../.ai/ARCHITECTURE_MAP.md).

### Dependencies

This reading contract depends on stable terminology, domain ownership, identity and policy definitions, workflow state, knowledge provenance, and a maintained decision surface. It is also dependent on the repository continuing to distinguish product context from implementation detail.

### AI Construction Notes

When an AI agent receives a change request, it should first write an internal placement statement:

```text
Requested capability:
Primary plane:
Primary domain:
Owning component:
Control boundary:
Data boundary:
Execution boundary:
Downstream consumers:
```

If two planes or domains appear to own the same meaning, stop and request a boundary decision before implementing.

### AI Memory Anchor

> **CAT is organized by responsibility, control, data, and execution contracts. Find the owner before finding the file.**

### Implementation Checklist

- [x] Internal organization is defined as conceptual planes.
- [x] Responsibility, control, data, and execution boundaries are distinguished.
- [x] Official decisions and extension rules are recorded.
- [x] The placement method is explicit for humans and AI agents.
- [x] Failure, recovery, and ownership concerns are included.
- [ ] Concrete plane-to-service mappings are defined in the later Architecture document.

---

## 16. CAT Platform Structure — Internal Planes and Major Subsystems

### Human Explanation

CAT is internally organized into seven cooperating planes. The seven-plane model is intentionally detailed enough for onboarding and intentionally abstract enough to survive changes in technology:

1. **Experience Plane** — human and machine-facing ways to express intent, inspect work, and receive explanations.
2. **Identity, Governance, and Administration Plane** — identities, roles, permissions, policy, approval, settings, tenants, and administrative control.
3. **Orchestration and Automation Plane** — task decomposition, scheduling, workflow state, agent coordination, retries, and execution policy.
4. **Commerce Operations Plane** — market, product, affiliate, content, marketing, publishing, and channel responsibilities.
5. **Intelligence and Knowledge Plane** — knowledge graph, memory, retrieval, reasoning, model routing, evaluation, and learning.
6. **Treasury and Measurement Plane** — earnings, budgets, payouts, financial records, analytics, attribution, reporting, and optimization signals.
7. **Foundation and Integration Plane** — events, persistence, object storage, secrets, external adapters, observability, runtime, and reliability primitives.

Treasury and Measurement is shown as one plane here because economic measurement and operational analytics form the feedback surface of the product. Treasury remains a first-class Trinity pillar and retains ownership of financial meaning; Analytics owns measurement and interpretation within its declared boundaries.

A workflow normally crosses several planes. For example, a campaign launch may begin in the Experience Plane, be authorized in Governance, coordinated in Orchestration, executed by Commerce domains, grounded by Intelligence, measured by Analytics and Treasury, and persisted through Foundation. The planes cooperate without becoming a single undifferentiated application.

### AI Context

A coding agent should use the planes as a first-pass architecture map:

- UI changes usually begin in Experience but must consume governed APIs.
- Permission changes begin in Identity/Governance and propagate to execution boundaries.
- Workflow state, retries, and scheduling belong in Orchestration, not in individual domain agents.
- Product, campaign, content, affiliate, and channel meaning belongs in Commerce domains.
- Facts, memories, embeddings, prompts, reasoning, and learning belong in Intelligence and Knowledge.
- Financial facts belong in Treasury; operational measurements belong in Analytics; shared outcomes must be correlated.
- External service volatility belongs in Integration adapters and must not leak into core domain contracts.

The agent must not use a plane label as a reason to create a new package automatically. First identify the responsibility and its existing contract.

### Business Perspective

The plane model allows CAT to scale in more than one dimension:

- More users and organizations through Identity and Administration.
- More workflows through Orchestration and Automation.
- More commerce markets and channels through Commerce domains.
- Better decisions through Intelligence and Knowledge.
- More accountable value through Treasury and Analytics.
- More integrations and reliability through Foundation.

A platform becomes durable when each dimension can evolve without forcing every other dimension to change at the same time.

### Technical Perspective

The planes have different change rates, risk profiles, scaling patterns, and authorities. Experience may change rapidly for usability; Identity and Treasury change conservatively; integrations change when external providers change; Knowledge evolves continuously but must preserve history; Orchestration must remain reliable across all domains.

The model therefore supports:

- explicit interfaces between planes;
- event-driven facts and command-driven requests;
- policy enforcement at multiple boundaries;
- durable workflow state;
- domain-owned canonical records;
- read models for different audiences;
- observability that follows a correlation ID across planes.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-STRUCT-DEC-001 | CAT uses a seven-plane conceptual organization for platform orientation. | Future components must declare their plane and cross-plane dependencies. |
| P2-STRUCT-DEC-002 | Experience does not own business truth. | Screens, portals, and avatars request or display governed domain state. |
| P2-STRUCT-DEC-003 | Orchestration coordinates domain work but does not own every domain rule. | CATA routes and supervises; domain owners define domain meaning. |
| P2-STRUCT-DEC-004 | Treasury and Analytics are connected feedback responsibilities but retain distinct ownership of financial truth and measurements. | A metric cannot silently become a ledger entry, and a ledger entry cannot be inferred from a chart. |
| P2-STRUCT-DEC-005 | External integrations are isolated behind adapters. | Provider changes, credentials, retries, and rate limits remain outside core domain meaning. |

### Recommendations

- Use one canonical domain owner per record type.
- Use a shared correlation ID for any operation crossing three or more planes.
- Make plane boundaries visible in architecture diagrams and module manifests.
- Treat the Foundation Plane as a reliability contract, not a dumping ground for arbitrary utilities.
- Keep user-facing read models optimized for explanation and oversight rather than direct database shape.

### Experimental Ideas

- A “plane inspector” for developers that shows the path of an event, command, approval, and outcome across the platform.
- A generated architecture map derived from component manifests and event subscriptions.
- A policy simulation mode that runs a proposed workflow without external side effects.

### Future Ideas

- Independent regional instances of selected planes.
- A shared enterprise control plane for identity, policy, billing, and audit.
- Dedicated model-evaluation and agent-simulation planes as AI complexity grows.
- A federated knowledge plane shared by multiple Omni System products.

### Visual Overview

**Diagram ID:** P2-PLAT-001<br>
**Diagram Title:** CAT Seven-Plane Platform Structure<br>
**Purpose:** Show the major conceptual planes and the direction of intent, governance, execution, measurement, and learning.

```mermaid
flowchart TB
    Experience[Experience Plane<br/>KATA, command center, portals, APIs for intent]
    Governance[Identity, Governance & Administration Plane<br/>identity, roles, permissions, policy, approvals, settings]
    Orchestration[Orchestration & Automation Plane<br/>CATA, workflows, scheduler, task state, retries]
    Commerce[Commerce Operations Plane<br/>market, affiliate, content, marketing, publishing]
    Intelligence[Intelligence & Knowledge Plane<br/>knowledge, memory, retrieval, reasoning, learning, models]
    Measure[Treasury & Measurement Plane<br/>Treasury, Analytics, attribution, reporting, optimization]
    Foundation[Foundation & Integration Plane<br/>events, storage, secrets, adapters, observability, runtime]

    Experience --> Governance
    Experience --> Orchestration
    Governance --> Orchestration
    Orchestration --> Commerce
    Orchestration --> Intelligence
    Orchestration --> Measure
    Commerce <--> Intelligence
    Commerce --> Measure
    Intelligence --> Measure
    Measure --> Intelligence
    Governance --> Commerce
    Governance --> Measure
    Foundation <--> Experience
    Foundation <--> Governance
    Foundation <--> Orchestration
    Foundation <--> Commerce
    Foundation <--> Intelligence
    Foundation <--> Measure

    style Experience fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Governance fill:#f59e0b,color:#000
    style Orchestration fill:#7c3aed,color:#fff
    style Commerce fill:#10b981,color:#fff
    style Intelligence fill:#06b6d4,color:#fff
    style Measure fill:#f59e0b,color:#000
    style Foundation fill:#334155,color:#fff
```

### Mermaid Diagram(s)

**Diagram ID:** P2-PLAT-002<br>
**Diagram Title:** CAT Intent-to-Outcome Flow<br>
**Purpose:** Show how a human or external signal becomes governed work, domain activity, measurement, and persistent learning.

```mermaid
sequenceDiagram
    participant E as Experience or external signal
    participant G as Governance
    participant O as Orchestration
    participant D as Commerce domain
    participant I as Intelligence
    participant M as Analytics/Treasury
    participant F as Foundation

    E->>G: Intent, event, or request
    G->>O: Authorized command or policy decision
    O->>I: Retrieve context and form plan
    I-->>O: Evidence, recommendation, confidence
    O->>D: Scoped domain task
    D->>F: Read/write or external adapter request
    F-->>D: Result, event, or failure
    D->>M: Activity and attribution facts
    M->>I: Outcome and learning signal
    I->>F: Persist knowledge, memory, and trace
    O-->>E: Status, explanation, approval request, or result
```

**Diagram ID:** P2-PLAT-003<br>
**Diagram Title:** Plane Dependency Direction<br>
**Purpose:** Distinguish normal dependency direction from prohibited shortcuts.

```mermaid
graph BT
    Foundation[Foundation and integrations]
    Intelligence[Intelligence and knowledge]
    Commerce[Commerce domains]
    Orchestration[Orchestration and automation]
    Governance[Identity and governance]
    Experience[Experience]

    Experience --> Governance
    Experience --> Orchestration
    Governance --> Orchestration
    Orchestration --> Commerce
    Orchestration --> Intelligence
    Commerce --> Foundation
    Intelligence --> Foundation
    Governance --> Foundation

    Experience -. prohibited direct business write .-> Foundation
    Experience -. prohibited direct ledger write .-> Commerce
```

### Architecture Map

#### Plane Responsibility Matrix

| Plane | Primary owner | Responsibilities | Canonical outputs | Typical failure response |
|---|---|---|---|---|
| Experience | Product/UI owner | Translate intent, display state, explain decisions, collect approvals | Commands, queries, approval decisions, views | Degrade to text/API, preserve pending work |
| Identity/Governance/Admin | Security and platform governance owner | Authenticate, authorize, classify risk, administer tenants and policy | Identity claims, permissions, policies, approval records | Deny safely, alert, preserve audit |
| Orchestration/Automation | Workflow and agent platform owner | Decompose, schedule, route, pause, retry, resume, cancel | Tasks, workflow state, commands, lifecycle events | Retry, compensate, quarantine, escalate |
| Commerce Operations | Domain owners | Research, affiliate, content, marketing, publishing, channel work | Products, campaigns, content, links, publications | Isolate failed domain action, retain partial state |
| Intelligence/Knowledge | AI and knowledge owner | Retrieve, reason, remember, evaluate, learn | Context, recommendations, embeddings, lessons, evaluations | Fall back to known context, mark uncertainty, stop risky action |
| Treasury/Measurement | Treasury and analytics owners | Measure, attribute, reconcile, budget, report, optimize | Metrics, ledger records, payouts, reports, anomalies | Reconcile, hold financial action, escalate discrepancy |
| Foundation/Integration | Platform reliability owner | Persist, communicate, store files, manage secrets, observe, adapt external APIs | Events, durable records, traces, connector results | Retry, dead-letter, fail over, protect data integrity |

#### Plane Ownership Rule

The owner of a plane is accountable for its contract and reliability, not for every business outcome that passes through it. For example, the Foundation owner is accountable for event delivery guarantees, while the Commerce owner is accountable for the meaning of a `content_published` event.

### Engineering Subsystem Contract

Every major plane or subsystem must be documented with the following contract fields:

| Field | Required question |
|---|---|
| Purpose | Why does this subsystem exist? |
| Owner | Which human/team owns its meaning and operation? |
| Responsibilities | Which work is inside its boundary? |
| Inputs | Which commands, events, data, identities, and policies does it consume? |
| Outputs | Which records, events, views, decisions, or side effects does it produce? |
| Dependencies | Which planes, domains, providers, and contracts must be available? |
| Extension points | How can capability grow without bypassing ownership? |
| Failure modes | What can fail, and how is partial work represented? |
| Recovery strategy | How is work retried, resumed, compensated, quarantined, or escalated? |
| Future evolution | What changes are plausible without changing the subsystem's identity? |

This contract is the minimum information required before a subsystem becomes implementation-ready.

### Major Subsystem Inventory

The following inventory makes the plane model concrete without pretending to choose the final technology stack. These are the major conceptual subsystems that a future implementation must account for.

#### KATA — Human Interaction Boundary

- **Purpose:** Translate human intent into governed CAT requests and translate system state into understandable explanations, status, and approval experiences.
- **Owner:** Human interaction/product experience owner, with Security and domain owners governing exposed actions.
- **Responsibilities:** Natural-language and visual interaction; language and accessibility; approval presentation; status and notification; persona/state expression; protection of internal complexity.
- **Inputs:** Human text/voice/gesture; workflow status; evidence; approval requests; policy explanations; outcomes.
- **Outputs:** Structured intent; approval decision; correction; query; human-readable response; visual/avatar state.
- **Dependencies:** Identity; Governance; CATA; read models; notification; UI rendering; Knowledge explanations.
- **Extension Points:** Languages; modalities; accessibility adapters; supervised command surfaces; enterprise views.
- **Failure Modes:** Misinterpreted intent; inaccessible state; stale approval view; information leakage; visual overload; unavailable rendering.
- **Recovery Strategy:** Ask clarifying question; show structured confirmation; fall back to text/API; preserve workflow; deny sensitive details; provide accessible low-motion mode.
- **Future Evolution:** Multimodal, spatial, voice, and organization-specific supervisory experiences.

#### CATA — Central Coordination Agent

- **Purpose:** Coordinate specialized agents and domains, decompose intent, maintain execution context, and return coherent results.
- **Owner:** AI orchestration owner.
- **Responsibilities:** Task planning; agent selection; workflow coordination; context assembly; policy handoff; result aggregation; escalation; correlation and trace propagation.
- **Inputs:** KATA commands; events; schedules; agent capabilities; Knowledge context; policies; Treasury/Commerce constraints.
- **Outputs:** Scoped tasks; plans; approval requests; aggregated results; status; coordination events; learning signals.
- **Dependencies:** Automation; Identity; Governance; Knowledge; agent registry; domain contracts; Foundation.
- **Extension Points:** New agents; routing strategies; workflow templates; model routes; planning evaluators.
- **Failure Modes:** Incorrect decomposition; wrong agent; context loss; infinite plan; duplicate task; unauthorized escalation; result aggregation error.
- **Recovery Strategy:** Validate plan; bound recursion; retry/idempotency; route to fallback; quarantine task; request human clarification; replay from checkpoint.
- **Future Evolution:** Portfolio orchestration, multi-node coordination, and formally evaluated multi-agent planning.

#### Agent Runtime and Registry

- **Purpose:** Register, discover, initialize, evaluate, execute, pause, degrade, and retire specialized AI agents.
- **Owner:** Agent platform owner with domain owners for agent roles.
- **Responsibilities:** Agent manifests; capability discovery; tool binding; lifecycle; health; resource limits; evaluation; version compatibility; permission enforcement.
- **Inputs:** Agent packages/manifests; tasks; policies; model routes; tools; context; health signals.
- **Outputs:** Agent state; task results; events; traces; evaluations; capability registry entries; failures.
- **Dependencies:** Identity; Security; Automation; Knowledge; model providers; Foundation; observability.
- **Extension Points:** New role agents; skills; evaluators; model adapters; sandbox profiles.
- **Failure Modes:** Agent crash; tool misuse; incompatible manifest; output contract failure; privilege escalation; resource exhaustion.
- **Recovery Strategy:** Restart or fail over; revoke tools; quarantine version; retry bounded tasks; route to another approved agent; preserve trace.
- **Future Evolution:** Agent workforce scheduling, capability negotiation, and federated agent registries.

#### Workflow Engine and Scheduler

- **Purpose:** Persist workflow state and schedule, pause, resume, cancel, retry, compensate, and complete long-running operations.
- **Owner:** Automation/workflow platform owner.
- **Responsibilities:** State transitions; deadlines; dependencies; timers; task queues; retries; checkpoints; cancellation; compensation; human wait states.
- **Inputs:** Commands; events; workflow templates; policy; tasks; schedules; deadlines.
- **Outputs:** Task assignments; state transitions; timers; approval requests; retries; completion/failure events.
- **Dependencies:** Foundation persistence/events; Identity; Security; CATA; domain contracts; notification.
- **Extension Points:** New workflow definitions; scheduling policies; compensation handlers; human gates; simulation mode.
- **Failure Modes:** Lost checkpoint; deadlock; duplicate execution; timer failure; infinite retry; stale state.
- **Recovery Strategy:** Durable state; idempotency; dead-letter; replay; manual resume; timeout escalation; compensation.
- **Future Evolution:** Adaptive scheduling, simulation, priority optimization, and cross-region workflow continuity.

#### Event Bus and Contract Registry

- **Purpose:** Carry versioned commands, facts, lifecycle events, approvals, failures, and knowledge signals between components.
- **Owner:** Foundation/platform communication owner.
- **Responsibilities:** Publish/subscribe; ordering guarantees; delivery; retries; backpressure; schema validation; replay; dead-letter; contract discovery.
- **Inputs:** Domain facts; commands; task events; agent results; approval decisions; telemetry.
- **Outputs:** Delivered messages; delivery status; consumer offsets; replay streams; contract errors; dead-letter records.
- **Dependencies:** Foundation persistence; schema registry; Identity/Security; observability; every producer/consumer.
- **Extension Points:** Topics; event schemas; consumer groups; replay tools; partner/event gateways.
- **Failure Modes:** Delivery loss; duplicate; out-of-order event; schema mismatch; backlog; consumer poison message; unauthorized subscription.
- **Recovery Strategy:** At-least-once delivery with idempotent consumers; retry; quarantine; replay; backpressure; permission revocation.
- **Future Evolution:** Federated event bus, cross-region replication, event marketplace, and governed partner events.

#### Persistence and Domain Data Stores

- **Purpose:** Preserve canonical domain records, workflow state, immutable facts, projections, relationships, and audit-linked data.
- **Owner:** Data/platform owner together with domain owners for canonical meaning.
- **Responsibilities:** Durability; consistency; migrations; indexing; access control; retention; backups; recovery; separation of canonical and derived state.
- **Inputs:** Domain commands; events; projections; configuration; audit facts; reconciliation data.
- **Outputs:** Records; query results; change events; snapshots; migration status; recovery points.
- **Dependencies:** Foundation runtime/storage; Identity; Security; domain schemas; backup/restore; observability.
- **Extension Points:** New domain schemas; read models; partitioning; archival; enterprise data boundaries.
- **Failure Modes:** Corruption; unavailable store; migration error; constraint violation; stale projection; unauthorized read/write.
- **Recovery Strategy:** Transactions where required; backup restore; migration rollback; replay events; quarantine corrupt records; fail read/write paths safely.
- **Future Evolution:** Domain partitioning, replicas, federation, and policy-aware data residency.

#### Object and Asset Storage

- **Purpose:** Store content, media, documents, evidence packages, exports, model artifacts, and other large or versioned objects.
- **Owner:** Content/platform storage owner with domain ownership of object meaning.
- **Responsibilities:** Versioning; metadata; access control; integrity; lifecycle; retention; scanning; transformation; link stability.
- **Inputs:** Generated assets; human uploads; evidence; exports; model artifacts; content versions.
- **Outputs:** Object IDs; signed/scoped retrieval; metadata; integrity status; lifecycle events.
- **Dependencies:** Identity; Security; Content; Knowledge; Foundation; external storage provider.
- **Extension Points:** Media types; transformations; CDN/edge delivery; archive tiers; enterprise retention policies.
- **Failure Modes:** Corrupt upload; missing object; wrong access scope; malware; version mismatch; lifecycle deletion error.
- **Recovery Strategy:** Checksums; quarantine; restore version; revoke link; reprocess from source; preserve metadata and audit.
- **Future Evolution:** Rich multimodal assets, provenance-aware media, regional storage, and content-addressed knowledge.

#### Secret and Credential Management

- **Purpose:** Protect provider credentials, signing keys, tokens, encryption keys, and other sensitive operational material.
- **Owner:** Security owner with platform operations ownership of integration.
- **Responsibilities:** Store; rotate; scope; issue short-lived access; audit use; revoke; detect leakage; separate configuration from secrets.
- **Inputs:** Credential registration; identity; integration requests; rotation policy; incident signals.
- **Outputs:** Scoped secret material to authorized runtime; rotation events; access audit; revocation status.
- **Dependencies:** Identity; Security; Administration; runtime; external providers; observability.
- **Extension Points:** Secret providers; workload identity; enterprise key management; rotation automation.
- **Failure Modes:** Secret leak; expired credential; unauthorized access; provider mismatch; rotation outage.
- **Recovery Strategy:** Revoke and rotate; quarantine integration; deny access; preserve forensic events; restore known-good secret binding.
- **Future Evolution:** Hardware-backed keys, confidential workloads, and policy-driven ephemeral credentials.

#### Observability and Audit

- **Purpose:** Make system behavior, decisions, performance, security events, financial actions, and failures measurable and reconstructable.
- **Owner:** Platform reliability and governance owners, with each domain accountable for semantic events.
- **Responsibilities:** Metrics; logs; traces; audit records; alerting; retention; sampling; correlation; dashboards/read models; incident evidence.
- **Inputs:** Component telemetry; workflow events; approvals; security decisions; domain facts; Treasury records.
- **Outputs:** Metrics; traces; logs; audit trail; alerts; incident packages; operational views.
- **Dependencies:** All planes and domains; storage; Identity; Security; time/correlation; notification.
- **Extension Points:** New signals; SLOs; compliance reports; AI quality metrics; enterprise dashboards; trace visualizations.
- **Failure Modes:** Missing telemetry; high-cardinality overload; clock skew; retention gap; sensitive data leak; alert fatigue.
- **Recovery Strategy:** Buffer; sample safely; preserve critical audit; repair collector; replay source events; rotate/limit sensitive fields; escalate blind spots.
- **Future Evolution:** Predictive operations, AI behavior observability, and cross-product trace federation.

#### Runtime, Network, and Execution Security

- **Purpose:** Provide the protected runtime, network paths, isolation, resource control, and health mechanisms in which CAT components execute.
- **Owner:** Infrastructure/platform operations owner with Security oversight.
- **Responsibilities:** Runtime isolation; networking; scaling; health; resource limits; deployment; patching; traffic control; execution policy; disaster recovery.
- **Inputs:** Deployments; service identities; policies; workload demands; health signals; secrets; configuration.
- **Outputs:** Running components; network access; health state; scaling events; deployment state; runtime audit.
- **Dependencies:** Foundation storage/events; Identity; Security; Deployment; observability; external infrastructure.
- **Extension Points:** New runtimes; regions; schedulers; network policies; sandbox profiles; deployment targets.
- **Failure Modes:** Resource exhaustion; network partition; compromised workload; bad deployment; secret exposure; region outage.
- **Recovery Strategy:** Roll back; isolate; reschedule; fail over; restore; rotate credentials; invoke incident and disaster recovery procedures.
- **Future Evolution:** Multi-region runtime, confidential computing, agent sandboxes, and workload-aware placement.

### Major Subsystem Relationship Table

| Subsystem | Primary plane | Canonical responsibility | Main dependents |
|---|---|---|---|
| KATA | Experience | Human intent, explanation, approval surface | Humans, CATA, read models |
| CATA | Orchestration/Intelligence | Coordination and plan routing | Agents, workflows, domains |
| Agent Runtime/Registry | Orchestration/Intelligence | Agent lifecycle and capabilities | CATA, domain agents, Security |
| Workflow Engine/Scheduler | Automation | Durable task and state lifecycle | All long-running workflows |
| Event Bus/Contract Registry | Foundation | Message semantics and delivery | All cross-component communication |
| Persistence/Data Stores | Foundation | Durable canonical and derived state | All domains and workflows |
| Object Storage | Foundation/Commerce | Versioned large assets and evidence | Content, Knowledge, exports |
| Secret Management | Foundation/Security | Credential and key protection | Integrations, runtime, agents |
| Observability/Audit | Foundation/Governance | Evidence of behavior and health | All planes and operators |
| Runtime/Network | Foundation/Security | Execution and isolation | All deployed components |

### Cross References

- Six-layer runtime components: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13.3–13.4.
- KATA/CATA and cognitive core details: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13.6–13.8.
- Event-driven and infrastructure details: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13.9–13.12.
- Future architecture specification: [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md).
- Deployment and operations: [`context/16_DEPLOYMENT.md`](./16_DEPLOYMENT.md).

### Dependencies

The seven planes depend on a common vocabulary and on stable contracts for identity, tasks, workflows, domain records, events, knowledge, financial facts, audit, and external integrations. The Foundation Plane is a dependency for reliability, but it must not become the owner of domain semantics.

### Failure Modes and Recovery Strategy

| Failure mode | Detection | Recovery |
|---|---|---|
| Experience unavailable | Health check, client error, API still active | Preserve workflow state; notify through alternate channel |
| Governance unavailable | Policy timeout or identity failure | Fail closed for high-impact actions; queue low-risk reads |
| Orchestration unavailable | Queue depth, heartbeat failure | Durable task state, failover, resume from last checkpoint |
| Domain subsystem unavailable | Agent/service health, connector error | Mark domain degraded; retry or route to fallback |
| Knowledge retrieval unavailable | Retrieval timeout or stale index | Use bounded fallback context; do not fabricate certainty |
| Treasury/measurement delay | Reconciliation lag or data discrepancy | Mark pending; hold irreversible financial actions |
| Foundation failure | Event loss, storage errors, secret access failure | Retry with idempotency, dead-letter, incident escalation |

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Plane diagrams become rigid architecture | Design cannot adapt to technology | Label them conceptual and maintain detailed architecture separately |
| Plane ownership conflicts with domain ownership | Incidents and changes have no clear accountable party | Use capability and record ownership matrices |
| Foundation becomes a hidden monolith | All features depend on an unreviewable layer | Keep interfaces narrow and observability explicit |
| Governance is treated as optional middleware | Security and approval can be bypassed | Enforce authorization at command and side-effect boundaries |
| Measurement is detached from execution | Learning and Treasury lose causal context | Use correlation and provenance from workflow start |

### Anti-patterns

- “Everything depends on the database, so the database is the architecture.”
- “CATA owns Commerce, Treasury, policy, content, and every business rule.”
- “The UI can call whichever service is convenient.”
- “A failed external request is just a log message.”
- “A plane is complete because it has a folder.”

### Best Practices

- Review each component against the subsystem contract table.
- Use a separate diagram for dependency direction, event flow, and user interaction when needed.
- Keep source-of-truth ownership explicit for every mutable record.
- Model partial completion and degraded operation as normal states.
- Make recovery behavior part of acceptance criteria.

### Extension Points

- New planes only through architectural decision.
- New plane capabilities through registered contracts.
- New providers through adapters and compatibility tests.
- New read models through governed query surfaces.
- New execution paths through orchestration, authorization, and audit.

### AI Construction Notes

The agent should reject a design note that describes a subsystem only by its technology. “A FastAPI service” is not a subsystem definition. The agent must ask what the service owns, what it guarantees, what records it changes, how it fails, and who is accountable.

### AI Memory Anchor

> **Experience asks, Governance authorizes, Orchestration coordinates, Domains act, Intelligence informs, Treasury measures, and Foundation preserves.**

### Implementation Checklist

- [x] Seven conceptual planes are defined.
- [x] Plane relationships are visualized with diagrams carrying IDs, titles, and purposes.
- [x] Plane owners, responsibilities, outputs, and failure responses are mapped.
- [x] Major cross-cutting subsystems are individually contracted.
- [x] A reusable subsystem contract is defined.
- [x] Six-layer root architecture compatibility is explicitly addressed.
- [ ] Plane-to-runtime component mapping is completed in the later Architecture document.

### Dependencies

The seven planes depend on a common vocabulary and on stable contracts for identity, tasks, workflows, domain records, events, knowledge, financial facts, audit, and external integrations. The Foundation Plane is a dependency for reliability, but it must not become the owner of domain semantics.

### Failure Modes and Recovery Strategy

| Failure mode | Detection | Recovery |
|---|---|---|
| Experience unavailable | Health check, client error, API still active | Preserve workflow state; notify through alternate channel |
| Governance unavailable | Policy timeout or identity failure | Fail closed for high-impact actions; queue low-risk reads |
| Orchestration unavailable | Queue depth, heartbeat failure | Durable task state, failover, resume from last checkpoint |
| Domain subsystem unavailable | Agent/service health, connector error | Mark domain degraded; retry or route to fallback |
| Knowledge retrieval unavailable | Retrieval timeout or stale index | Use bounded fallback context; do not fabricate certainty |
| Treasury/measurement delay | Reconciliation lag or data discrepancy | Mark pending; hold irreversible financial actions |
| Foundation failure | Event loss, storage errors, secret access failure | Retry with idempotency, dead-letter, incident escalation |

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Plane diagrams become rigid architecture | Design cannot adapt to technology | Label them conceptual and maintain detailed architecture separately |
| Plane ownership conflicts with domain ownership | Incidents and changes have no clear accountable party | Use capability and record ownership matrices |
| Foundation becomes a hidden monolith | All features depend on an unreviewable layer | Keep interfaces narrow and observability explicit |
| Governance is treated as optional middleware | Security and approval can be bypassed | Enforce authorization at command and side-effect boundaries |
| Measurement is detached from execution | Learning and Treasury lose causal context | Use correlation and provenance from workflow start |

### Anti-patterns

- “Everything depends on the database, so the database is the architecture.”
- “CATA owns Commerce, Treasury, policy, content, and every business rule.”
- “The UI can call whichever service is convenient.”
- “A failed external request is just a log message.”
- “A plane is complete because it has a folder.”

### Best Practices

- Review each component against the subsystem contract table.
- Use a separate diagram for dependency direction, event flow, and user interaction when needed.
- Keep source-of-truth ownership explicit for every mutable record.
- Model partial completion and degraded operation as normal states.
- Make recovery behavior part of acceptance criteria.

### Extension Points

- New planes only through architectural decision.
- New plane capabilities through registered contracts.
- New providers through adapters and compatibility tests.
- New read models through governed query surfaces.
- New execution paths through orchestration, authorization, and audit.

### AI Construction Notes

The agent should reject a design note that describes a subsystem only by its technology. “A FastAPI service” is not a subsystem definition. The agent must ask what the service owns, what it guarantees, what records it changes, how it fails, and who is accountable.

### AI Memory Anchor

> **Experience asks, Governance authorizes, Orchestration coordinates, Domains act, Intelligence informs, Treasury measures, and Foundation preserves.**

### Implementation Checklist

- [x] Seven conceptual planes are defined.
- [x] Plane relationships are visualized with diagrams carrying IDs, titles, and purposes.
- [x] Plane owners, responsibilities, outputs, and failure responses are mapped.
- [x] A reusable subsystem contract is defined.
- [x] Six-layer root architecture compatibility is explicitly addressed.
- [ ] Plane-to-runtime component mapping is completed in the Architecture document.

---

## 17. The Commerce AI Trinity Inside the Platform

### Human Explanation

Part 1 defined Commerce, AI, and Treasury as the three pillars of CAT. Internally, each pillar is a coordinated stack of responsibilities rather than one subsystem. The Trinity is expanded below as three **value stacks** that share identity, workflow, knowledge, and governance.

#### Commerce Stack

The Commerce stack turns market opportunity into observable market activity. It includes market intelligence, product and merchant context, affiliate relationships, marketing strategy, content and creative production, channel publishing, and commerce performance.

Commerce owns the meaning of what is promoted, where it is promoted, how it is presented, and which market activity occurred. It does not own the truth of financial reconciliation or global policy.

#### AI Stack

The AI stack turns goals and signals into context-aware, coordinated work. It includes agent roles, CATA orchestration, KATA translation, memory, retrieval, reasoning, prompt/model routing, evaluation, learning, and decision support.

AI owns the intelligence process: how context is gathered, how work is decomposed, how recommendations are formed, and how outcomes become knowledge. It does not own human accountability or silently grant itself authority.

#### Treasury Stack

The Treasury stack turns commerce activity into economic records, constraints, and decisions. It includes earnings, commissions, payouts, budgets, costs, attribution, reconciliation, financial reporting, risk, and economic optimization.

Treasury owns financial meaning and financial integrity. It does not own content quality, market strategy, or unrestricted spending authority. A Treasury recommendation still passes through the governance and approval model when it changes financial state or exposure.

### AI Context

The Trinity is an internal routing rule:

```text
If the question is about market action, start in Commerce.
If the question is about reasoning, coordination, memory, or model behavior, start in AI.
If the question is about money, financial exposure, reconciliation, or payout, start in Treasury.
If the question crosses two or more pillars, use a correlated workflow and identify the strictest applicable approval rule.
```

AI agents must never resolve cross-pillar ambiguity by assigning ownership to whichever component is easiest to call. The owning domain is determined by the meaning of the record or decision.

### Business Perspective

The Trinity prevents local optimization. Commerce can grow activity, AI can grow intelligence, and Treasury can protect economics, but CAT creates durable value only when these goals remain connected.

| Trinity tension | Healthy resolution |
|---|---|
| Commerce wants speed; Governance wants evidence | Use risk-based approval and progressive trust |
| AI wants more context; Security limits data access | Use least privilege, scoped retrieval, and provenance |
| Treasury wants lower spend; Commerce wants more reach | Compare risk-adjusted expected value and owner-approved budgets |
| Analytics sees correlation; Product wants a decision | Separate evidence from inference and record uncertainty |
| Learning wants to update quickly; Governance wants stability | Use confidence, review states, and controlled promotion |

### Technical Perspective

The three stacks share four cross-pillar primitives:

1. **Business identity:** product, market, campaign, content, channel, link, earning, and payout IDs.
2. **Workflow identity:** task ID, workflow ID, correlation ID, actor, scope, and state.
3. **Knowledge identity:** source, evidence bundle, memory, decision, lesson, confidence, and version.
4. **Governance identity:** policy, approval request, approver, action scope, audit event, and outcome.

The stacks may use different storage models or processing patterns, but these primitives must remain related enough to support traceability.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-TRI-DEC-001 | Commerce owns market-facing commercial meaning and activity. | Product, campaign, content, channel, and affiliate workflows have Commerce ownership. |
| P2-TRI-DEC-002 | AI owns coordination, reasoning, memory, retrieval, learning, and model-related behavior. | AI components propose and coordinate but do not own human accountability. |
| P2-TRI-DEC-003 | Treasury owns financial truth, reconciliation, payout, budget, and economic-risk meaning. | Financial records cannot be inferred solely from activity or analytics views. |
| P2-TRI-DEC-004 | Cross-pillar workflows use shared IDs and preserve provenance. | The platform can trace opportunity → action → outcome → earnings → lesson. |
| P2-TRI-DEC-005 | The strictest applicable approval boundary governs a cross-pillar action. | A low-risk AI step cannot downgrade a high-risk financial or public side effect. |
| P2-TRI-DEC-006 | Pillar ownership and execution ownership may differ, but accountability must be explicit. | A Publisher Agent may execute a Commerce action while a human Commerce owner approves it. |

### Recommendations

- Represent pillar interactions as explicit workflow steps or events rather than hidden shared state.
- Use a domain-owned record plus cross-pillar projections when a record is needed in several views.
- Keep financial calculations in Treasury-owned components and expose read models to AI and Commerce.
- Keep AI explanations linked to the evidence and versions used to produce them.
- Let Commerce define business intent and Treasury define financial constraints; let AI coordinate between them.

### Experimental Ideas

- A Trinity policy engine that scores a proposed action across commercial value, intelligence confidence, and economic exposure.
- A portfolio “balance view” that visualizes how much current automation is spending on discovery, production, publishing, and learning.
- A simulation environment where a campaign can be evaluated against historical Commerce, AI, and Treasury data without live execution.

### Future Ideas

- Treasury-aware proactive opportunity selection.
- Cross-market transfer learning with explicit financial and regulatory boundaries.
- Multi-campaign portfolio optimization.
- Shared Trinity contracts for future Omni System products.
- Organization-specific pillar policies for enterprise deployments.

### Visual Overview

**Diagram ID:** P2-TRI-001<br>
**Diagram Title:** Internal Commerce AI Treasury Value Stacks<br>
**Purpose:** Expand each Trinity pillar into internal responsibilities while showing shared governance and knowledge boundaries.

```mermaid
flowchart LR
    subgraph C[Commerce Stack]
        C1[Market intelligence]
        C2[Product and merchant context]
        C3[Affiliate operations]
        C4[Marketing strategy]
        C5[Content and creative]
        C6[Publishing and channels]
        C7[Commerce performance]
        C1 --> C2 --> C3 --> C4 --> C5 --> C6 --> C7
    end

    subgraph A[AI Stack]
        A1[KATA and intent]
        A2[CATA orchestration]
        A3[Agents and tools]
        A4[Memory and retrieval]
        A5[Reasoning and decisions]
        A6[Model and prompt routing]
        A7[Evaluation and learning]
        A1 --> A2 --> A3 --> A4 --> A5 --> A6 --> A7
    end

    subgraph T[Treasury Stack]
        T1[Attribution]
        T2[Earnings and commissions]
        T3[Costs and budgets]
        T4[Reconciliation]
        T5[Payouts]
        T6[Financial reports]
        T7[Economic optimization]
        T1 --> T2 --> T3 --> T4 --> T5 --> T6 --> T7
    end

    C7 -->|activity and outcomes| T1
    T7 -->|constraints and value signals| A5
    A7 -->|learning and confidence| C1
    A5 -->|plans and decisions| C3
    A5 -->|financial questions| T3
    Governance[Shared identity, policy, approval, audit] --- C
    Governance --- A
    Governance --- T
    Knowledge[Shared provenance, memory, decisions, lessons] --- C
    Knowledge --- A
    Knowledge --- T
```

### Mermaid Diagram(s)

**Diagram ID:** P2-TRI-002<br>
**Diagram Title:** Trinity Lifecycle Ownership<br>
**Purpose:** Show how ownership moves through a cross-pillar campaign lifecycle without collapsing domain boundaries.

```mermaid
sequenceDiagram
    participant C as Commerce owner/domain
    participant A as AI owner/domain
    participant T as Treasury owner/domain
    participant G as Governance/approver
    participant K as Knowledge layer

    C->>A: Opportunity and business goal
    A->>K: Retrieve evidence and prior outcomes
    K-->>A: Context and confidence
    A->>T: Request economic assessment
    T-->>A: Budget, risk, earnings, payout context
    A->>C: Ranked plan and execution tasks
    C->>G: Request approval for material action
    G-->>C: Approved scope or revision
    C->>C: Execute approved commerce work
    C->>T: Activity, attribution, and earnings signals
    T->>K: Reconciled financial outcome
    C->>K: Operational outcome and lesson candidate
    K->>A: Refined context for next decision
```

**Diagram ID:** P2-TRI-003<br>
**Diagram Title:** Trinity Decision Boundary<br>
**Purpose:** Identify the correct owner for a request and the point at which cross-pillar coordination becomes mandatory.

```mermaid
flowchart TD
    Request[New request or signal] --> Meaning{What meaning is being changed?}
    Meaning -->|Market, product, campaign, content, channel, affiliate| Commerce[Commerce owner]
    Meaning -->|Reasoning, agent, memory, prompt, model, learning| AI[AI owner]
    Meaning -->|Earning, cost, budget, payout, reconciliation, financial risk| Treasury[Treasury owner]
    Commerce --> Cross{Does it affect another pillar?}
    AI --> Cross
    Treasury --> Cross
    Cross -->|No| Local[Local contract and owner]
    Cross -->|Yes| Shared[Correlated cross-pillar workflow]
    Shared --> Strict[Apply strictest approval and audit rule]
```

### Architecture Map

| Lifecycle responsibility | Commerce owner | AI owner | Treasury owner | Shared contract |
|---|---|---|---|---|
| Opportunity | Market relevance and commercial fit | Research, ranking, confidence | Expected value and budget context | Opportunity ID and evidence |
| Plan | Campaign scope and channel strategy | Decomposition and recommendation | Exposure and financial guardrails | Plan/version ID |
| Production | Content requirements and quality | Generation, evaluation, retrieval | Cost and expected-value context | Asset and prompt/model IDs |
| Publication | Channel and brand responsibility | Execution coordination | Financial attribution expectation | Approval and action scope |
| Measurement | Commerce outcomes | Interpretation and anomaly reasoning | Financial reconciliation | Correlation and attribution IDs |
| Learning | Market and campaign lesson | Knowledge promotion and evaluation | Economic validation | Lesson, confidence, review state |

### Dependencies

The internal Trinity model depends on:

- shared domain terminology;
- a stable campaign and workflow identity model;
- agent and human roles;
- a policy and approval gateway;
- trustworthy source and attribution data;
- knowledge and memory persistence;
- financial reconciliation and reporting;
- event delivery and observability.

### Failure Modes and Recovery Strategy

| Failure mode | Affected pillar | Recovery strategy |
|---|---|---|
| Commerce source data is stale | Commerce | Mark freshness, reduce confidence, request refresh, avoid high-impact action |
| AI retrieval fails | AI | Use bounded fallback context, surface uncertainty, pause risky recommendations |
| Model/provider fails | AI | Route to approved fallback, preserve prompt and model version, retry within budget |
| Treasury data is delayed | Treasury | Mark financial state pending, do not infer settled earnings, hold irreversible payout decisions |
| Attribution breaks | Commerce/Treasury | Preserve raw events, quarantine affected projections, reconcile before promotion |
| Cross-pillar event is duplicated | All | Use idempotency and correlation checks; do not double-count activity or money |
| Human approval expires | Governance | Re-request with current evidence; never reuse stale approval silently |

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Pillar boundaries are only labels | Cross-domain behavior becomes hidden and untestable | Require contracts and ownership metadata |
| AI becomes the de facto owner of all decisions | Human and domain accountability disappear | Separate recommendation, authorization, and execution |
| Treasury is read-only | Financial context cannot influence planning | Make Treasury constraints available before material actions |
| Commerce writes financial truth | Attribution and ledger integrity are compromised | Treasury-owned financial write boundary |
| Analytics is treated as causality | Wrong optimization lessons | Record attribution assumptions and uncertainty |

### Anti-patterns

- A single “Trinity service” that owns all three meanings without domain contracts.
- An AI recommendation that includes a budget but never calls Treasury context.
- A Treasury chart that silently modifies campaign state.
- A Commerce adapter writing directly to a financial ledger.
- A learning agent promoting a commercial or financial policy without review.

### Best Practices

- Use the Trinity as an architectural test for every major feature.
- Preserve distinct ownership while making relationships easy to traverse.
- Correlate every cross-pillar action and outcome.
- Show the source and confidence of a recommendation and the financial assumptions behind it.
- Treat reconciliation and learning as lifecycle stages, not optional reports.

### Extension Points

- New Commerce domains can connect to AI and Treasury through shared campaign and outcome contracts.
- New AI tools can consume and produce governed context.
- New Treasury instruments or payout sources can implement financial adapters.
- Future Omni products can adopt Trinity contract patterns after independent ownership is established.

### Cross References

- Part 1 Trinity model: [Section 4](#4-the-commerce-ai-trinity).
- Root Commerce AI Trinity concept: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 15.2.
- Domain context targets: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md), [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md), [`context/08_AFFILIATE_ENGINE.md`](./08_AFFILIATE_ENGINE.md), [`context/09_CONTENT_ENGINE.md`](./09_CONTENT_ENGINE.md).
- Decision documentation target: [`context/12_DECISIONS.md`](./12_DECISIONS.md).

### AI Construction Notes

When implementing cross-pillar behavior, an AI agent should produce a responsibility table before code. It should identify who owns each field, which event carries the relationship, which approval is required, and which record is canonical. If ownership is “shared,” the agent must explain how conflicting writes are resolved.

### AI Memory Anchor

> **Commerce acts, AI coordinates and learns, Treasury accounts; shared IDs and governance connect them without erasing ownership.**

### Implementation Checklist

- [x] Commerce, AI, and Treasury are expanded into internal value stacks.
- [x] Responsibilities, ownership, dependencies, communication, and recovery are defined.
- [x] Cross-pillar diagrams include IDs, titles, and purposes.
- [x] Strictest-approval rule is stated.
- [x] Failure modes and financial integrity boundaries are documented.
- [ ] Detailed pillar schemas and service contracts are defined in downstream documents.

---

## 18. Core Domain Map and Domain Contracts

### Human Explanation

A **domain** is an area of meaning and responsibility. Domains are not merely database tables or folders. A domain owns the rules, records, outcomes, and vocabulary for a business or platform concern. Domains can collaborate, but each important fact should have one canonical owner.

CAT's domains fall into three groups:

- **Commerce domains:** Commerce, Affiliate, Content, and Marketing.
- **Intelligence and operations domains:** Knowledge, Automation, and Analytics.
- **Platform control domains:** Treasury, Identity, Settings, Security, and Administration.

Commerce is the umbrella business domain. Affiliate, Content, and Marketing are specialized Commerce domains because they have distinct lifecycles and external dependencies. Automation coordinates work across domains but does not own their business meaning. Analytics measures activity, while Treasury owns financial truth. Identity, Settings, Security, and Administration make multi-user and enterprise operation possible.

The domain model is intentionally more detailed than the seven-plane model. Planes describe where a responsibility sits in the platform; domains describe what that responsibility means. A domain may use several planes.

### AI Context

An AI agent must distinguish:

- **domain ownership:** who owns the meaning of a record;
- **execution ownership:** which agent or service performs a task;
- **presentation ownership:** which interface displays a result; and
- **policy ownership:** which role controls whether the action is allowed.

These can be different. For example, the Affiliate domain owns link meaning, the Affiliate Agent executes link preparation, KATA presents a review request, Security controls credential access, and a human Commerce owner approves an externally visible change.

When two domains appear to own the same field, the agent must define a canonical source and derive other views through events or projections. It must not duplicate mutable truth simply because two screens need it.

### Business Perspective

Domain separation enables specialization without fragmentation. CAT can improve content generation without rewriting Treasury, add a new affiliate network without changing Identity, or add a new enterprise policy without changing the meaning of a campaign. It also gives future product teams a vocabulary for packaging, support, ownership, and extension.

### Technical Perspective

Each domain should eventually expose:

- commands for requested state changes;
- events for facts that occurred;
- queries or read models for inspection;
- policies for permitted behavior;
- versioned schemas;
- provenance and audit links;
- idempotency and consistency guarantees;
- an explicit failure and recovery model.

A domain boundary should be stricter around money, identity, security, and external side effects than around read-only analysis.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-DOM-DEC-001 | Knowledge, Commerce, Treasury, Affiliate, Content, Marketing, Automation, Analytics, Identity, Settings, Security, and Administration are recognized CAT domains. | New capabilities must declare their domain relationship. |
| P2-DOM-DEC-002 | Affiliate, Content, and Marketing are specialized Commerce domains, not unrelated products by default. | They share Commerce lifecycle and identity while retaining specialized contracts. |
| P2-DOM-DEC-003 | Treasury is the canonical owner of financial truth. | Analytics and AI may consume Treasury views but may not redefine ledger meaning. |
| P2-DOM-DEC-004 | Identity, Security, and Administration are distinct responsibilities. | Authentication, threat protection, and organizational operation cannot be collapsed into one ambiguous admin layer. |
| P2-DOM-DEC-005 | Automation coordinates work but does not own all domain semantics. | Workflow state and business records remain separately owned. |
| P2-DOM-DEC-006 | Knowledge preserves provenance, decisions, memory, and lessons across domains. | Domain activity must produce retrievable context and outcome links. |

### Recommendations

- Keep the domain vocabulary stable and add aliases only through terminology decisions.
- Use domain-owned commands and events instead of direct cross-domain writes.
- Let read models combine domain data without moving canonical ownership.
- Make data classification, retention, and approval requirements part of each domain contract.
- Prefer a small number of meaningful domains over a new domain for every feature.

### Experimental Ideas

- A domain registry that exposes each domain's owner, schemas, events, policies, health, and extension contracts to humans and AI agents.
- Automated domain-boundary linting based on import graphs, event subscriptions, and ownership metadata.
- A knowledge graph view that lets an operator follow one campaign across all twelve domains.

### Future Ideas

- A dedicated Experimentation domain for controlled hypotheses and measurement.
- A Partner and Marketplace domain for governed extension distribution.
- A Compliance domain if policy complexity grows beyond Security and Administration.
- Shared Omni System domains extracted from proven CAT contracts.

### Visual Overview

**Diagram ID:** P2-DOM-001<br>
**Diagram Title:** CAT Domain Constellation<br>
**Purpose:** Show the Commerce domains, platform-control domains, and intelligence/operations domains around the shared CAT lifecycle.

```mermaid
flowchart TB
    CAT((CAT Domain Model))

    subgraph CommerceDomains[Commerce domains]
        Commerce[Commerce]
        Affiliate[Affiliate]
        Content[Content]
        Marketing[Marketing]
        Commerce --> Affiliate
        Commerce --> Content
        Commerce --> Marketing
    end

    subgraph IntelligenceOperations[Intelligence and operations]
        Knowledge[Knowledge]
        Automation[Automation]
        Analytics[Analytics]
    end

    subgraph ControlDomains[Platform control domains]
        Treasury[Treasury]
        Identity[Identity]
        Settings[Settings]
        Security[Security]
        Administration[Administration]
    end

    CAT --> CommerceDomains
    CAT --> IntelligenceOperations
    CAT --> ControlDomains
    Knowledge <--> Commerce
    Automation <--> Commerce
    Analytics <--> Commerce
    Treasury <--> Commerce
    Identity --> Security
    Administration --> Identity
    Settings --> Administration
    Security --> Automation
    Knowledge --> Analytics
```

**Diagram ID:** P2-DOM-002<br>
**Diagram Title:** Canonical Ownership and Derived Views<br>
**Purpose:** Demonstrate how one domain owns a fact while other domains consume projections or explanations.

```mermaid
flowchart LR
    Source[Canonical domain owner]
    Command[Domain command]
    Event[Domain fact event]
    Projection[Derived read model]
    Explanation[Human/AI explanation]
    Learning[Knowledge and learning record]

    Source --> Command --> Source
    Source --> Event
    Event --> Projection
    Event --> Explanation
    Event --> Learning

    Projection -. read only .-> OtherDomain[Other domain view]
    Explanation -. context .-> Supervisor[Human supervisor]
    Learning -. future context .-> Agent[AI agent]
```

### Visual Asset Placeholder — Domain and Command Center Atlas

| Field | Value |
|---|---|
| **Visual Asset Name** | CAT Domain and Command Center Atlas |
| **Purpose** | Provide a future human-facing visual map of the twelve domains, their ownership, active workflows, and cross-domain signal paths. |
| **Recommended Resolution** | 3840 × 2160 px for command-center wall view; responsive 1920 × 1080 px crop |
| **Suggested Location inside `/design`** | `/design/Architecture/CAT_Domain_Command_Center_Atlas.md` and `/design/Architecture/assets/cat-domain-atlas.png` |
| **Mood** | Calm cosmic intelligence with clear operational hierarchy |
| **Style** | Layered glass panels, restrained neon edges, readable labels, data-flow light trails, accessible contrast |
| **Reference Category** | System architecture visualization / command-center information design |

### Mermaid Diagram(s)

**Diagram ID:** P2-DOM-003<br>
**Diagram Title:** Domain Interaction Context Map<br>
**Purpose:** Show the primary context relationships without implying that every domain calls every other domain directly.

```mermaid
graph TD
    User[User or enterprise intent]
    Identity[Identity]
    Administration[Administration]
    Settings[Settings]
    Security[Security]
    Automation[Automation]
    Knowledge[Knowledge]
    Commerce[Commerce]
    Affiliate[Affiliate]
    Content[Content]
    Marketing[Marketing]
    Analytics[Analytics]
    Treasury[Treasury]

    User --> Identity
    Administration --> Identity
    Administration --> Settings
    Identity --> Security
    Settings --> Automation
    Security --> Automation
    Automation --> Knowledge
    Automation --> Commerce
    Commerce --> Affiliate
    Commerce --> Content
    Commerce --> Marketing
    Affiliate --> Analytics
    Content --> Analytics
    Marketing --> Analytics
    Commerce --> Analytics
    Analytics --> Treasury
    Treasury --> Knowledge
    Analytics --> Knowledge
    Commerce --> Knowledge
    Knowledge --> Automation
```

**Diagram ID:** P2-DOM-004<br>
**Diagram Title:** Domain Lifecycle State Transition<br>
**Purpose:** Show the shared lifecycle pattern that individual domains specialize without duplicating governance.

```mermaid
stateDiagram-v2
    [*] --> Proposed
    Proposed --> Validating
    Validating --> Ready
    Validating --> Rejected
    Ready --> Authorized
    Authorized --> Executing
    Executing --> Succeeded
    Executing --> Failed
    Failed --> Retrying
    Retrying --> Executing
    Failed --> Quarantined
    Succeeded --> Observed
    Observed --> Learned
    Learned --> [*]
    Authorized --> Cancelled
    Executing --> Compensating
    Compensating --> Cancelled
    Rejected --> [*]
    Quarantined --> [*]
    Cancelled --> [*]
```

### Domain Contract: Knowledge

- **Purpose:** Preserve the facts, sources, decisions, memories, outcomes, lessons, and relationships that allow CAT to reason over time instead of treating every operation as isolated.
- **Owner:** Knowledge and AI platform owner, with domain owners accountable for the quality of knowledge they contribute.
- **Responsibilities:** Ingest and validate knowledge; maintain provenance; support retrieval; represent decisions and relationships; manage memory tiers; track confidence, freshness, and review state; promote lessons; mark stale knowledge without erasing required history.
- **Inputs:** Domain events; research evidence; human decisions; agent traces; model and prompt versions; operational outcomes; Treasury results; external source metadata.
- **Outputs:** Retrieval context; knowledge graph updates; memory records; decision links; lessons; confidence and freshness signals; explanation evidence.
- **Dependencies:** Foundation persistence and events; Identity and Security; domain schemas; embeddings/retrieval; evaluation; documentation and decision records.
- **Extension Points:** New entity types; relationship types; source adapters; retrieval strategies; memory tiers; evaluation indexes; domain-specific knowledge packs.
- **Failure Modes:** Stale or conflicting facts; duplicate entities; missing provenance; retrieval timeout; corrupted relationship; overconfident lesson; index lag.
- **Recovery Strategy:** Keep immutable source evidence; quarantine conflicting updates; rebuild derived indexes; lower confidence; fall back to validated prior context; require human review for canonical corrections; preserve audit history.
- **Future Evolution:** Cross-product knowledge federation, richer causal relationships, enterprise data boundaries, and learning systems that distinguish correlation from causation.

### Domain Contract: Commerce

- **Purpose:** Own the meaning of market opportunity, product selection, campaign intent, commercial activity, channel objectives, and the end-to-end business lifecycle.
- **Owner:** Commerce product/domain owner.
- **Responsibilities:** Define markets, products, merchants, campaigns, channel goals, commercial constraints, and operational outcomes; coordinate Affiliate, Content, Marketing, and publishing responsibilities; maintain business acceptance criteria.
- **Inputs:** Research evidence; merchant/product feeds; human goals; affiliate terms; content assets; channel specifications; Analytics signals; Treasury constraints; policies.
- **Outputs:** Opportunities; product and campaign records; commerce plans; approved actions; activity events; business outcomes; requests to specialized Commerce domains.
- **Dependencies:** Knowledge; Automation; Affiliate; Content; Marketing; Analytics; Treasury; Identity; external integrations.
- **Extension Points:** New markets; channel types; campaign archetypes; commerce workflows; regional policies; product scoring strategies.
- **Failure Modes:** Invalid product data; campaign scope ambiguity; duplicate campaign; unsupported market; content-channel mismatch; activity without attribution.
- **Recovery Strategy:** Preserve draft state; validate against current evidence; isolate invalid records; re-plan through Automation; reprocess idempotently; notify owner of unresolved commercial risk.
- **Future Evolution:** Portfolio planning, multi-market operations, enterprise commerce workspaces, and cross-product commerce contracts.

### Domain Contract: Treasury

- **Purpose:** Own the economic record and financial accountability of CAT activity, including earnings, commissions, costs, budgets, payouts, reconciliation, exposure, and financial reporting.
- **Owner:** Treasury/finance domain owner and authorized human financial approvers.
- **Responsibilities:** Record financial facts; reconcile external reports; distinguish pending, estimated, settled, disputed, and paid states; manage budgets and payout workflows; expose constraints; produce auditable reports; detect anomalies and financial risk.
- **Inputs:** Commerce activity; affiliate attribution; external network reports; costs; budgets; currency and timing data; Analytics projections; human approvals; policy.
- **Outputs:** Ledger or financial records; earnings state; payout recommendations and actions; budget status; reconciliation exceptions; financial reports; risk signals.
- **Dependencies:** Identity and Security; Commerce and Affiliate IDs; Analytics attribution; Foundation persistence; external payout/accounting adapters; approval policy.
- **Extension Points:** New currencies; networks; payout providers; accounting adapters; budget policies; risk models; enterprise reporting.
- **Failure Modes:** Duplicate earnings; missing attribution; currency mismatch; stale payout report; unauthorized transfer; reconciliation discrepancy; ledger write failure.
- **Recovery Strategy:** Use immutable or append-only financial facts where appropriate; hold disputed values; replay source reports; reconcile from source evidence; require explicit approval for corrective entries; never silently overwrite a financial record.
- **Future Evolution:** Risk-adjusted allocation, cash-flow planning, portfolio economics, enterprise finance integration, and multi-entity Treasury.

### Domain Contract: Affiliate

- **Purpose:** Manage affiliate-program context, merchant relationships, links, tracking identifiers, commission terms, compliance conditions, and partner-specific operational state.
- **Owner:** Affiliate operations/domain owner.
- **Responsibilities:** Discover and validate affiliate programs; store terms and effective dates; create and maintain links; map products to programs; monitor link health; preserve network provenance; expose commission and cookie context to Commerce and Treasury.
- **Inputs:** Products and merchants; network APIs; program terms; campaign plans; tracking requirements; credentials through Security; human approvals.
- **Outputs:** Affiliate program records; validated links; tracking metadata; link-health events; partner status; commission context; attribution inputs.
- **Dependencies:** Commerce; Identity; Security/secrets; external networks; Analytics; Treasury; Automation.
- **Extension Points:** New affiliate networks; regional partner adapters; link transformation rules; link-health checks; compliance metadata; marketplace connectors.
- **Failure Modes:** Expired program; malformed link; network rate limit; credential failure; changed terms; duplicate tracking; broken redirect; unapproved partner action.
- **Recovery Strategy:** Retry within provider policy; quarantine invalid links; preserve prior valid version; revalidate terms; route credential failures to Security; hold publication when attribution is uncertain; notify Affiliate owner.
- **Future Evolution:** Automated partner discovery, negotiated terms, portfolio link optimization, and governed third-party network plugins.

### Domain Contract: Content

- **Purpose:** Own the meaning, lifecycle, quality, provenance, versioning, and channel readiness of generated or human-edited content and creative assets.
- **Owner:** Content/creative domain owner.
- **Responsibilities:** Define content types; manage briefs and drafts; apply brand, evidence, accessibility, disclosure, and policy checks; track revisions; connect assets to products, campaigns, channels, agents, prompts, and approvals; support text, image, audio, and video workflows where accepted.
- **Inputs:** Commerce brief; product facts; affiliate links; sources; brand rules; channel requirements; model outputs; human edits; policy and approval decisions.
- **Outputs:** Drafts; content versions; quality findings; approved assets; metadata; publication packages; content performance links.
- **Dependencies:** Knowledge; Commerce; Affiliate; Marketing; model providers; asset storage; Identity; Security; Publisher capabilities.
- **Extension Points:** New content types; evaluators; template packs; model routes; localization; accessibility checks; enterprise brand profiles.
- **Failure Modes:** Hallucinated fact; missing disclosure; low quality; unsafe or prohibited content; lost revision; asset corruption; channel-format failure; duplicate publication package.
- **Recovery Strategy:** Preserve source and prior version; return to draft; require human revision; block publication; run evaluator again; restore last approved asset; quarantine unsafe output.
- **Future Evolution:** Multimodal production, personalized variants, localization, content knowledge graphs, and provenance-aware synthetic media workflows.

### Domain Contract: Marketing

- **Purpose:** Own audience, positioning, messaging, campaign strategy, channel objectives, segmentation, experimentation intent, and commercial communication plans.
- **Owner:** Marketing/product strategy owner.
- **Responsibilities:** Define audience and message; set campaign objectives; choose channel mix; maintain positioning and brand constraints; propose experiments; connect market evidence to Content and Commerce; evaluate strategic fit.
- **Inputs:** Market research; product and affiliate context; audience profiles; brand settings; historical campaigns; Treasury budget; Analytics outcomes; human strategy.
- **Outputs:** Marketing briefs; audience and channel plans; campaign hypotheses; messaging strategy; experiment proposals; performance interpretation requests.
- **Dependencies:** Commerce; Content; Analytics; Knowledge; Treasury; Settings/brand configuration; Identity and approval.
- **Extension Points:** New channels; audience models; experimentation methods; regional messaging; enterprise brand policies; partner co-marketing workflows.
- **Failure Modes:** Wrong audience; unapproved claim; inconsistent brand; budget mismatch; experiment without baseline; channel policy violation; strategy disconnected from product evidence.
- **Recovery Strategy:** Pause campaign; return to brief; revalidate audience and evidence; update settings; request human strategy review; preserve failed hypothesis and result.
- **Future Evolution:** Predictive audience planning, portfolio campaign optimization, lifecycle marketing, and cross-market learning with privacy controls.

### Domain Contract: Automation

- **Purpose:** Coordinate tasks and workflows across domains, agents, humans, schedules, events, approvals, retries, and external responses.
- **Owner:** Workflow/orchestration platform owner.
- **Responsibilities:** Decompose goals; create tasks; manage state; schedule work; route to agents; enforce prerequisites; pause for approvals; retry idempotently; compensate or quarantine failures; emit lifecycle events; expose status.
- **Inputs:** Human commands; domain events; schedules; policies; agent capabilities; dependencies; task priorities; deadlines.
- **Outputs:** Workflow instances; task assignments; commands; approval requests; retries; status events; completion and failure records.
- **Dependencies:** Identity; Security; Foundation event bus and persistence; CATA; domain contracts; Knowledge; Analytics; notification surfaces.
- **Extension Points:** Workflow templates; new triggers; scheduling policies; human approval steps; compensation handlers; domain-specific task types.
- **Failure Modes:** Lost state; duplicate task; deadlock; infinite retry; dependency timeout; wrong agent routing; stale approval; partial completion.
- **Recovery Strategy:** Durable checkpoints; idempotency keys; exponential retry limits; dead-letter/quarantine; manual resume; compensation; escalation; replay from event history.
- **Future Evolution:** Adaptive scheduling, simulation, portfolio orchestration, and policy-aware autonomous workflow composition.

### Domain Contract: Analytics

- **Purpose:** Measure operational, commerce, content, channel, agent, and workflow behavior; attribute signals with explicit assumptions; detect anomalies; support decision-making.
- **Owner:** Analytics and measurement domain owner.
- **Responsibilities:** Ingest facts; calculate metrics; maintain definitions and versions; build projections and reports; detect anomalies; distinguish observation from interpretation; expose signals to Commerce, AI, and Treasury.
- **Inputs:** Events; channel metrics; clicks; conversions; content outcomes; workflow timings; agent traces; Treasury facts; experiments; source metadata.
- **Outputs:** Metrics; dashboards/read models; anomaly events; reports; performance analyses; attribution views; optimization recommendations.
- **Dependencies:** Foundation events and storage; Commerce identities; Affiliate tracking; Content versions; Treasury records; Knowledge; Settings and access policy.
- **Extension Points:** New metrics; channels; attribution models; dashboards; evaluation datasets; enterprise reporting; real-time alerts.
- **Failure Modes:** Missing events; duplicate event; schema drift; attribution error; stale projection; metric definition change; false anomaly; privacy violation.
- **Recovery Strategy:** Replay source events; version metric definitions; mark gaps; quarantine invalid data; recalculate projections; review attribution; notify affected consumers.
- **Future Evolution:** Causal analysis, predictive optimization, self-serve governed analytics, and cross-product performance intelligence.

### Domain Contract: Identity

- **Purpose:** Establish who or what is acting, which organization and tenant it belongs to, what roles and credentials it holds, and which permissions can be evaluated.
- **Owner:** Identity and platform security owner.
- **Responsibilities:** Authenticate users and services; issue and validate identity claims; manage sessions and service principals; map roles; support organization/tenant boundaries; record identity events; integrate enterprise identity providers.
- **Inputs:** Login or service credentials; invitations; organization membership; role assignments; enterprise identity assertions; Security policy.
- **Outputs:** Authenticated identity; claims; roles; tenant context; tokens/session state; access decisions; identity audit events.
- **Dependencies:** Security; Administration; Settings; Foundation secrets and persistence; enterprise identity providers; approval system.
- **Extension Points:** OAuth/OIDC providers; service identities; workload identity; delegated access; enterprise federation; recovery and lifecycle workflows.
- **Failure Modes:** Credential compromise; token expiry; incorrect role; tenant leak; provider outage; session fixation; orphaned identity.
- **Recovery Strategy:** Fail closed for sensitive actions; revoke sessions; rotate credentials; quarantine identity; restore from audit-backed membership state; require admin/security review.
- **Future Evolution:** Fine-grained authorization, risk-adaptive authentication, cross-product Omni identity, and organization-controlled identity domains.

### Domain Contract: Settings

- **Purpose:** Store governed configuration that shapes user, organization, visual, workflow, model, notification, performance, and integration behavior without changing core code.
- **Owner:** Product/platform configuration owner, with Security and Administration owning sensitive settings.
- **Responsibilities:** Define setting schemas; validate values; scope settings to user, organization, environment, domain, or workflow; version changes; expose effective configuration; distinguish preferences from policy.
- **Inputs:** User preferences; organization policy; admin changes; feature configuration; theme/performance choices; environment defaults; integration settings.
- **Outputs:** Validated configuration; effective settings; change events; configuration snapshots; policy inputs; UI and workflow behavior changes.
- **Dependencies:** Identity; Administration; Security; Foundation persistence; UI/UX; Automation; model and integration contracts.
- **Extension Points:** New setting namespaces; tenant overrides; theme packs; performance profiles; policy adapters; feature flags.
- **Failure Modes:** Invalid value; unsafe default; precedence conflict; secret stored as preference; unversioned change; setting drift across environments.
- **Recovery Strategy:** Schema validation; safe defaults; immutable change history; rollback to last valid version; separate secret storage; admin escalation for policy conflict.
- **Future Evolution:** Policy-as-configuration, organization templates, adaptive performance settings, and cross-product configuration profiles.

### Domain Contract: Security

- **Purpose:** Protect identities, data, credentials, workflows, agents, integrations, and users from unauthorized access, misuse, tampering, and unsafe execution.
- **Owner:** Security owner/CISO-equivalent governance role, working with every domain owner.
- **Responsibilities:** Threat model; enforce least privilege; protect secrets; authorize tool use; detect threats; validate inputs; monitor incidents; manage vulnerability response; preserve security audit; define secure defaults.
- **Inputs:** Identity claims; policy; requests; events; network signals; dependency reports; vulnerability findings; agent actions; admin changes.
- **Outputs:** Allow/deny decisions; security alerts; audit events; quarantines; credentials/secret access; incident records; remediation requirements.
- **Dependencies:** Identity; Administration; Foundation; every domain's permission model; external providers; deployment/runtime controls.
- **Extension Points:** New threat detectors; policy engines; secret providers; compliance controls; agent sandboxing; enterprise security adapters.
- **Failure Modes:** Unauthorized access; secret leak; prompt injection; supply-chain compromise; privilege escalation; data exfiltration; monitoring blind spot.
- **Recovery Strategy:** Deny or isolate; revoke access; rotate secrets; quarantine component; preserve forensic evidence; restore known-good state; incident response and postmortem.
- **Future Evolution:** Continuous authorization, agent sandboxing, formal policy verification, privacy-preserving retrieval, and enterprise compliance automation.

### Domain Contract: Administration

- **Purpose:** Operate the organizational and platform control surface: tenants, memberships, roles, billing or plan configuration when introduced, policy assignment, support operations, lifecycle management, and administrative audit.
- **Owner:** Platform administration owner and authorized organization administrators.
- **Responsibilities:** Create and manage organizations; invite and deactivate members; assign roles; configure organization policies; manage approved integrations and extensions; support audit/export; coordinate lifecycle and support operations.
- **Inputs:** Organization requests; Identity events; enterprise policies; contracts; support actions; Security findings; Settings; billing/plan data when accepted.
- **Outputs:** Organization state; membership and role changes; policy assignments; administrative records; notifications; support/audit exports.
- **Dependencies:** Identity; Security; Settings; Foundation; Treasury for financial administration; enterprise integration; approval policy.
- **Extension Points:** Enterprise admin modules; private catalogs; delegated administration; support tooling; compliance exports; organization templates.
- **Failure Modes:** Accidental privilege grant; member not deactivated; tenant configuration leak; destructive admin action; inconsistent policy; incomplete audit.
- **Recovery Strategy:** Require confirmation and scoped approvals; use soft-delete/deactivation; restore prior configuration; break-glass audit; notify Security; preserve administrative history.
- **Future Evolution:** Multi-tenant enterprise operations, delegated governance, private extension catalogs, and Omni-wide organization management.

### Domain Relationship and Ownership Table

| Domain | Meaning owner | Main consumers | Strictest control |
|---|---|---|---|
| Knowledge | Knowledge/AI owner plus contributing domain owners | All domains and AI agents | Provenance and promotion review |
| Commerce | Commerce owner | Affiliate, Content, Marketing, Analytics, Treasury | External commercial action approval |
| Treasury | Treasury/finance owner | Commerce, AI, Analytics, Administration | Financial integrity and authorized approval |
| Affiliate | Affiliate owner | Commerce, Content, Analytics, Treasury | Credential, term, and external-link policy |
| Content | Content owner | Commerce, Marketing, Publisher, Analytics | Quality, disclosure, public reputation |
| Marketing | Marketing/strategy owner | Commerce, Content, Analytics, Treasury | Strategic and brand approval |
| Automation | Workflow owner | All operational domains | Execution scope and state integrity |
| Analytics | Measurement owner | Commerce, AI, Treasury, operators | Data quality and attribution assumptions |
| Identity | Identity owner | All protected actions | Authentication and tenant isolation |
| Settings | Configuration owner | All configurable behavior | Safe defaults and policy separation |
| Security | Security owner | All domains | Least privilege and incident response |
| Administration | Platform/org owner | Identity, Settings, Security, enterprise | Administrative authorization and audit |

### Cross References

- Root domain foundations: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13, 15, 16, and 17.
- Knowledge detail: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md).
- Treasury detail: [`context/07_TREASURY_CORE.md`](./07_TREASURY_CORE.md).
- Affiliate detail: [`context/08_AFFILIATE_ENGINE.md`](./08_AFFILIATE_ENGINE.md).
- Content detail: [`context/09_CONTENT_ENGINE.md`](./09_CONTENT_ENGINE.md).
- Security detail: [`context/17_SECURITY.md`](./17_SECURITY.md).
- Settings, terminology, and directory targets: [`context/13_TERMINOLOGY.md`](./13_TERMINOLOGY.md), [`context/15_DIRECTORY_STRUCTURE.md`](./15_DIRECTORY_STRUCTURE.md).

### Dependencies

The domain map depends on shared identifiers, event contracts, identity and permission claims, policy evaluation, persistent storage, provenance, and a reliable workflow engine. Domains also depend on each other asymmetrically: a consumer may read a projection without owning the source of truth.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Domain boundaries follow team names | Reorganization breaks the model | Define meaning and records, not org charts only |
| Commerce duplicates Affiliate/Content truth | Links and assets drift | Specialized domains own their records |
| Analytics writes conclusions as facts | Decisions become untraceable | Separate measurements, interpretations, and decisions |
| Settings become an ungoverned policy store | Security and behavior change invisibly | Classify preference, configuration, and policy separately |
| Administration bypasses Security | Privilege abuse becomes possible | Independent authorization and audit |
| Knowledge promotion is automatic everywhere | False or stale information spreads | Confidence, provenance, and review states |
| Automation owns domain state | Workflow refactor changes business truth | Keep orchestration and domain ownership separate |

### Anti-patterns

- One shared “users” table treated as the entire Identity model.
- One “content” table used for drafts, approved assets, publications, and analytics without state/version boundaries.
- A Treasury dashboard that edits earnings directly.
- A settings flag that silently grants permission.
- A Security agent that is the only security control.
- A domain that emits events with no owner or schema version.
- A domain that cannot explain how a failed operation is recovered.

### Best Practices

- Give each domain a canonical source of truth and a named owner.
- Distinguish mutable current state, immutable facts, derived projections, and human decisions.
- Keep domain contracts explicit and versioned.
- Make failure states queryable and recoverable.
- Use cross-domain events for facts and commands for requested work.
- Document both normal and degraded lifecycle behavior.

### Extension Points

- Domain-specific plugins may add behavior through the domain contract.
- New external providers should attach through adapters owned by the relevant domain.
- New user types should map to Identity and Administration rather than adding domain-specific auth paths.
- New settings should declare scope, precedence, sensitivity, and rollback behavior.
- New knowledge entity types should document provenance and lifecycle.

### AI Construction Notes

For any new subsystem, the agent should copy the following contract skeleton into its plan and fill every field before implementation:

```text
Subsystem/domain:
Purpose:
Owner:
Responsibilities:
Inputs:
Outputs:
Dependencies:
Extension points:
Failure modes:
Recovery strategy:
Future evolution:
Canonical records:
Commands:
Events:
Queries/read models:
Approval boundary:
Knowledge obligation:
Treasury obligation:
```

### AI Memory Anchor

> **A domain owns meaning; an agent performs work; a view explains state; a policy authorizes action. Do not confuse them.**

### Implementation Checklist

- [x] All twelve requested domains are defined.
- [x] Each domain includes purpose, owner, responsibilities, inputs, outputs, dependencies, extension points, failure modes, recovery, and future evolution.
- [x] Domain relationships, ownership, lifecycle, and communication are visualized.
- [x] Commerce subdomains and cross-cutting domains are distinguished.
- [x] Visual asset metadata is provided for the future domain atlas.
- [ ] Detailed domain schemas, events, and APIs are authored in downstream context documents.
- [ ] Domain ownership metadata is implemented in the runtime.

---

## 19. High-Level Component Relationships and Communication

### Human Explanation

CAT components cooperate through a small number of understandable relationship patterns. The goal is not to make every component know every other component. The goal is to make the right relationships explicit, observable, and recoverable.

The principal relationships are:

- **Intent relationship:** a human or external signal expresses a goal or fact.
- **Authorization relationship:** Identity and Governance determine whether the actor and action are allowed.
- **Orchestration relationship:** Automation creates and tracks work, and CATA coordinates agents and domains.
- **Execution relationship:** a domain component performs a task within its declared responsibility.
- **Knowledge relationship:** a component retrieves context before acting and records evidence and outcomes after acting.
- **Measurement relationship:** Analytics and Treasury observe activity and economic state.
- **Integration relationship:** adapters translate between CAT contracts and external systems.
- **Explanation relationship:** KATA and read models translate system state into human-understandable views.

The central rule is:

> **Components communicate through purpose-specific contracts, not through accidental knowledge of one another's internals.**

CATA is the central coordinator, but it is not the owner of every record or business rule. KATA is the human-facing boundary, but it is not the source of domain truth. The event bus is the communication nervous system, but an event is not permission to perform an action. Knowledge is shared context, but a retrieval result is not automatically canonical truth.

### AI Context

An AI agent must select the correct communication pattern:

| Need | Pattern |
|---|---|
| Request another component to do something | Command/task with scope, actor, deadline, and idempotency |
| Announce that something happened | Versioned event with source, timestamp, correlation, and provenance |
| Read current or historical information | Governed query/read model |
| Ask for reasoning context | Knowledge/memory retrieval request |
| Ask a human to decide | Approval request with evidence, risk, exact scope, and expiry |
| Report failure | Structured failure event with retryability and recovery state |
| Invoke an external provider | Adapter contract with credentials, rate limits, mapping, and failure behavior |

The agent must not replace a command with an event, use a query to perform a side effect, or treat a free-form message as a durable contract.

### Business Perspective

Clear component relationships make CAT safer to grow. A new channel can subscribe to approved publication events. A new analytics consumer can read projections without changing the Publisher. A new agent can use the orchestrator's task contract. A new human interface can use the same governed APIs and approval records.

This is how a platform creates leverage: one well-defined capability can serve many consumers without every consumer becoming coupled to implementation details.

### Technical Perspective

CAT uses four primary interaction channels:

1. **Synchronous request/response** for short, bounded reads or validations where latency and failure are understood.
2. **Asynchronous commands/tasks** for work that can be queued, retried, paused, or delegated.
3. **Events** for facts, observations, lifecycle transitions, and fan-out.
4. **Knowledge retrieval** for contextual information, evidence, memories, and decisions.

A component may use more than one channel, but each operation must declare its semantic intent. Side effects require authorization and audit. Events require idempotent consumers. Queries must be safe to repeat. Knowledge retrieval must expose source and confidence.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-COMP-DEC-001 | Agents do not communicate through undocumented direct calls as the default coordination mechanism. | CATA/orchestration and explicit contracts mediate collaboration. |
| P2-COMP-DEC-002 | Commands represent requested work; events represent facts that occurred. | Consumers can reason about intent versus history. |
| P2-COMP-DEC-003 | Queries and read models must not silently produce external side effects. | Inspection remains safe and repeatable. |
| P2-COMP-DEC-004 | External provider behavior is isolated through adapters. | Provider-specific volatility does not define CAT domain contracts. |
| P2-COMP-DEC-005 | Correlation, causation, approval, and provenance must remain traceable across component relationships. | Operators can reconstruct why an outcome occurred. |
| P2-COMP-DEC-006 | High-impact execution verifies authorization at the execution boundary. | A UI or upstream agent cannot bypass governance by calling a lower-level component. |

### Recommendations

- Prefer contract-first events and commands with versioned schemas.
- Include `actor_id`, `tenant_id` where applicable, `workflow_id`, `correlation_id`, `causation_id`, `source`, `occurred_at`, and `schema_version` in traceable messages.
- Use a dead-letter or quarantine path for messages that cannot be processed safely.
- Make consumers idempotent and record the deduplication key used.
- Keep event payloads meaningful but avoid copying every internal database field into public contracts.
- Expose human-readable explanations alongside machine-readable status.

### Experimental Ideas

- A contract simulation tool that replays commands and events against a sandboxed domain.
- A graph query that shows the live component path for a workflow and identifies the next owner.
- Automatic generation of sequence diagrams from trace records.
- A semantic event inspector that distinguishes facts, recommendations, approvals, and side effects for AI agents.

### Future Ideas

- Federated event backbones across geographic CAT nodes.
- Contract negotiation for third-party plugins and future Omni products.
- Policy-aware service meshes or execution brokers.
- Event replay as a first-class operator feature for safe reconstruction and learning.

### Visual Overview

**Diagram ID:** P2-COMP-001<br>
**Diagram Title:** CAT Component Relationship Map<br>
**Purpose:** Show the primary components and the direction of intent, coordination, domain execution, knowledge, measurement, and external effects.

```mermaid
flowchart TB
    Human[Human supervisor]
    External[External signals and providers]
    KATA[KATA<br/>Human-facing translator and explainer]
    CATA[CATA<br/>Central coordinator]
    Policy[Identity, policy, and approval gateway]
    Workflow[Automation<br/>workflow state and scheduler]
    Agents[Specialized agents]
    Domains[Commerce and platform domains]
    Knowledge[Knowledge and memory]
    Analytics[Analytics and measurement]
    Treasury[Treasury]
    Events[Event and command contracts]
    Foundation[Foundation<br/>persistence, secrets, observability]

    Human <--> KATA
    External <--> Foundation
    KATA <--> CATA
    CATA <--> Policy
    CATA <--> Workflow
    Workflow <--> Agents
    Agents <--> Domains
    Domains <--> Events
    Events <--> Analytics
    Analytics <--> Treasury
    Agents <--> Knowledge
    Domains <--> Knowledge
    Treasury <--> Knowledge
    Policy <--> Foundation
    Workflow <--> Foundation
    Knowledge <--> Foundation
    Domains <--> Foundation

    style KATA fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style CATA fill:#7c3aed,color:#fff,stroke:#7c3aed,stroke-width:3px
    style Policy fill:#f59e0b,color:#000
    style Domains fill:#10b981,color:#fff
    style Knowledge fill:#06b6d4,color:#fff
    style Treasury fill:#f59e0b,color:#000
```

### Mermaid Diagram(s)

**Diagram ID:** P2-COMP-002<br>
**Diagram Title:** Control Flow, Data Flow, and Knowledge Flow<br>
**Purpose:** Separate three flows that are often incorrectly collapsed into one implementation path.

```mermaid
graph LR
    subgraph ControlFlow[Control flow]
        Intent[Intent] --> Auth[Authorize]
        Auth --> Task[Task]
        Task --> Execute[Execute]
        Execute --> Approval[Approve when required]
    end

    subgraph DataFlow[Data and fact flow]
        Source[External or internal source] --> Record[Domain record]
        Record --> Event[Fact event]
        Event --> Projection[Read model]
        Projection --> Report[Report or view]
    end

    subgraph KnowledgeFlow[Knowledge flow]
        Evidence[Evidence] --> Context[Retrieved context]
        Context --> Decision[Decision or recommendation]
        Decision --> Outcome[Outcome]
        Outcome --> Lesson[Lesson and memory]
        Lesson --> Context
    end
```

**Diagram ID:** P2-COMP-003<br>
**Diagram Title:** Campaign Component Sequence<br>
**Purpose:** Show a representative campaign path across KATA, CATA, agents, domains, approval, Analytics, Treasury, and Knowledge.

```mermaid
sequenceDiagram
    participant H as Human
    participant K as KATA
    participant C as CATA
    participant R as Research Agent
    participant Q as Commerce/Marketing
    participant W as Content/Publisher
    participant G as Governance
    participant A as Analytics
    participant T as Treasury
    participant N as Knowledge

    H->>K: Request campaign for a market
    K->>C: Structured intent and constraints
    C->>N: Retrieve evidence and prior campaigns
    N-->>C: Context and confidence
    C->>R: Research and qualify opportunity
    R-->>C: Evidence and recommendation
    C->>Q: Build campaign plan
    Q-->>C: Plan with assets and channel scope
    C->>G: Request approval with risk and Treasury context
    G-->>C: Approved scope or revision
    C->>W: Create, validate, and publish approved assets
    W-->>C: Publication result
    W->>A: Emit channel and content events
    A->>T: Emit attributed economic signals
    A->>N: Emit operational outcome
    T->>N: Emit reconciled financial outcome
    N-->>C: Learning context for next optimization
    C-->>K: Status, result, and explanation
    K-->>H: Human-readable outcome
```

**Diagram ID:** P2-COMP-004<br>
**Diagram Title:** Component Failure Isolation<br>
**Purpose:** Show how failures should stop unsafe side effects while preserving recoverable work and evidence.

```mermaid
flowchart TD
    Task[Durable task] --> Dependency{Dependency available?}
    Dependency -->|Yes| Execute[Execute bounded step]
    Dependency -->|No| Classify{Retryable or unsafe?}
    Classify -->|Retryable| Backoff[Backoff and retry]
    Classify -->|Unsafe/unknown| Hold[Hold and request review]
    Execute --> Result{Result}
    Result -->|Success| Event[Emit fact and continue]
    Result -->|Transient failure| Backoff
    Result -->|Permanent failure| Quarantine[Quarantine with evidence]
    Backoff --> Limit{Retry limit reached?}
    Limit -->|No| Dependency
    Limit -->|Yes| Quarantine
    Hold --> Audit[Record policy and dependency failure]
    Quarantine --> Audit
    Event --> Next[Next workflow stage]
```

**Diagram ID:** P2-COMP-005<br>
**Diagram Title:** Component Dependency Graph<br>
**Purpose:** Identify primary dependency direction and prevent direct shortcuts from experience surfaces to domain or financial state.

```mermaid
graph BT
    Foundation[Foundation and integration contracts]
    Knowledge[Knowledge and memory]
    Domains[Domain services]
    Agents[Specialized agents]
    Automation[Automation and CATA]
    Governance[Identity and governance]
    Experience[KATA, UI, API, portals]
    External[External providers]

    External --> Foundation
    Foundation --> Knowledge
    Foundation --> Domains
    Knowledge --> Agents
    Domains --> Agents
    Governance --> Agents
    Agents --> Automation
    Domains --> Automation
    Governance --> Automation
    Automation --> Experience
    Governance --> Experience

    Experience -. no direct ledger or credential access .-> Foundation
    Experience -. no direct domain mutation .-> Domains
```

### Visual Asset Placeholder — Live Workflow Trace Board

| Field | Value |
|---|---|
| **Visual Asset Name** | CAT Live Workflow Trace Board |
| **Purpose** | Give supervisors, operators, and developers a visual trace of a workflow across components, approvals, domain stages, external calls, retries, and outcomes. |
| **Recommended Resolution** | 3840 × 2160 px desktop command-center view; 1440 × 900 px operational detail view |
| **Suggested Location inside `/design`** | `/design/UI/WorkflowTraceBoard/` |
| **Mood** | Focused, calm, high-signal operational intelligence |
| **Style** | Dark cosmic base, glass cards, accessible status colors, directional light trails, explicit text labels, reduced-motion mode |
| **Reference Category** | Observability dashboard / workflow operations / developer tooling |

### Architecture Map

#### Relationship Matrix

| Producer | Contract | Consumer | Meaning |
|---|---|---|---|
| Human/KATA | Intent command | CATA/Automation | A goal or requested operation |
| CATA | Scoped task | Agent/domain | Work assigned within a role and deadline |
| Domain | Fact event | Analytics/Treasury/Knowledge | Something occurred in the domain |
| Policy gateway | Approval decision | Automation/executor | A scoped authorization or rejection |
| Knowledge | Retrieval response | Agent/CATA | Context, evidence, memory, confidence |
| Analytics | Anomaly/performance event | CATA/Commerce/Treasury | A measured signal requiring attention |
| Treasury | Financial constraint/outcome | Commerce/AI/Administration | Economic context or reconciled result |
| Adapter | External result | Domain/Foundation | Provider response translated into CAT semantics |

#### Contract Semantics

| Contract | Should be | Must not be |
|---|---|---|
| Command | Intentional, scoped, authorized, retry-aware | A broadcast fact or unbounded instruction |
| Event | Immutable fact with provenance and version | An instruction that assumes a consumer will act |
| Query | Side-effect free and repeatable | A hidden mutation or implicit approval |
| Approval | Human or policy decision tied to exact scope | A permanent blanket permission |
| Retrieval | Evidence/context with confidence and source | An assertion of guaranteed truth |
| Failure | Structured, classified, recoverable state | A log line that loses the task |

### Communication Lifecycle

1. **Declare:** a human, schedule, external provider, or domain event creates intent or fact.
2. **Authenticate:** Identity determines the actor, tenant, and service principal.
3. **Authorize:** Security and Governance classify the action and permissions.
4. **Correlate:** Automation assigns workflow, task, causation, and idempotency identifiers.
5. **Contextualize:** Knowledge and memory provide relevant evidence and constraints.
6. **Execute:** an agent or domain performs the bounded responsibility.
7. **Observe:** Foundation, Analytics, and Treasury capture results and side effects.
8. **Explain:** KATA and read models present state, evidence, and exceptions.
9. **Learn:** Knowledge records outcomes and validated lessons.
10. **Recover or close:** Automation completes, retries, compensates, quarantines, or escalates.

### Cross References

- Root event bus and communication philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 13.9 and 13.11.
- Agent communication and lifecycle: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.8.
- KATA and CATA boundary: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.6.
- UI and design context targets: [`context/10_UI_UX.md`](./10_UI_UX.md), [`context/11_DESIGN_LANGUAGE.md`](./11_DESIGN_LANGUAGE.md).
- Security and data controls: [`context/17_SECURITY.md`](./17_SECURITY.md).

### Dependencies

Component relationships depend on schema versioning, event delivery, task persistence, identity, policy evaluation, knowledge retrieval, domain ownership, external adapters, observability, and human notification. The absence of one dependency must result in an explicit degraded state rather than an invented success.

### Failure Modes and Recovery Strategy

| Failure | Detection | Recovery |
|---|---|---|
| Invalid command | Schema validation | Reject with correction details; no side effect |
| Unauthorized command | Policy decision | Deny, audit, notify, preserve request if safe |
| Duplicate command | Idempotency key | Return existing task/result or safely deduplicate |
| Lost event | Delivery/reconciliation gap | Replay from event store or source; mark downstream uncertainty |
| Out-of-order event | Sequence/version check | Buffer, reorder within policy, or quarantine |
| Query timeout | Latency/health signal | Retry read, serve stale labeled view, or show unavailable |
| Agent output invalid | Contract/evaluator failure | Reject output, re-run with bounded fallback, escalate |
| External provider mismatch | Adapter validation | Quarantine response, preserve raw payload, update connector |
| Approval scope mismatch | Execution boundary check | Block action and request re-approval |

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Too many synchronous calls | Cascading latency and failure | Use durable tasks and events for long-running work |
| Event storm | Cost, backlog, and noisy automation | Partition, prioritize, deduplicate, and apply backpressure |
| Contract drift | Consumers misinterpret messages | Version schemas and run compatibility tests |
| Trace IDs lost at boundaries | Incidents cannot be reconstructed | Require trace propagation in contracts |
| Read model mistaken for source | Edits bypass domain ownership | Mark projections read-only and link canonical source |
| External adapter leaks raw provider semantics | Core becomes vendor-specific | Translate at adapter boundary |

### Anti-patterns

- Direct agent-to-agent calls with hidden authorization assumptions.
- Events that contain commands such as “please publish this now.”
- Commands with no actor, scope, deadline, or idempotency key.
- A workflow status that says “success” before external confirmation.
- Retrying financial or publishing actions without deduplication.
- A UI showing a cached projection as settled truth without freshness status.

### Best Practices

- Propagate correlation and causation identifiers end to end.
- Make contracts self-describing enough for humans and AI agents.
- Log decisions and state transitions, not only errors.
- Test duplicate, delayed, missing, and out-of-order messages.
- Make external side effects explicit and separately observable.
- Prefer read models that explain provenance and last update time.

### Extension Points

- New components register command, event, query, and retrieval contracts.
- New external adapters implement provider translation and health contracts.
- New workflow stages subscribe to facts and expose task contracts.
- New observability views consume trace and event projections without changing domain state.
- New interfaces use the same governed relationship patterns as KATA.

### AI Construction Notes

An AI agent generating a component interface should produce both a happy-path sequence and at least one failure-isolation diagram. It should name the authoritative record, the side effect, the idempotency strategy, and the recovery owner. A component that has no recovery owner is not implementation-ready.

### AI Memory Anchor

> **Commands request, events report, queries observe, retrieval contextualizes, approvals authorize, and traces connect the whole path.**

### Implementation Checklist

- [x] Component relationship patterns are defined.
- [x] Control, data, knowledge, failure, and dependency diagrams include metadata.
- [x] Command/event/query/retrieval semantics are separated.
- [x] Component failure isolation and recovery are documented.
- [x] A visual asset placeholder is included for the workflow trace surface.
- [ ] Concrete event schemas and API protocols are defined in Architecture and Tech Stack documents.
- [ ] Runtime tracing and contract validation are implemented later.

---

## 20. User Types, Permissions, and Responsibilities

### Human Explanation

CAT has more than one kind of user. A user type describes a responsibility and access posture, not a marketing persona alone. A person may hold more than one role in a small deployment, while an enterprise deployment may separate those roles across many people. An AI Agent and a Developer are also participants in the ecosystem, but they are not granted human authority simply because they can call an API.

The primary user types are:

- **Guest:** an unauthenticated or minimally identified visitor with public/read-only access.
- **Creator:** a person who prepares content, creative assets, or editorial material.
- **Affiliate:** a person or operator who manages affiliate programs, products, links, partner terms, and attribution context.
- **Business:** a business owner, campaign supervisor, strategist, or operator accountable for commercial outcomes.
- **Administrator:** a trusted platform or organization administrator who manages membership, settings, policies, and operational control.
- **AI Agent:** a non-human service principal that performs a declared role under tool, data, and autonomy restrictions.
- **Developer:** a person or service contributor who builds, tests, documents, integrates, or extends CAT.
- **Enterprise:** an organization-level participant with multi-user governance, policy, audit, integration, and support requirements.

These types are intentionally not a flat hierarchy. A Creator may also be a Developer. An Affiliate operator may be a Business owner. An Enterprise contains users and service principals but is itself a policy and ownership boundary. Administrator is a responsibility role, not a universal superuser entitlement.

### AI Context

An AI agent must treat identity as a structured object with at least:

```text
principal_id
principal_type
organization_id / tenant_id when applicable
roles
scopes
resource constraints
data classifications allowed
autonomy level
approval authority (usually none for the agent itself)
credential and tool bindings
expiry and revocation state
```

The agent must not infer permissions from a display name, email domain, UI route, prompt, or repository directory. It must not treat “administrator,” “business owner,” or “AI agent” as unlimited access. Authorization is evaluated for an action, resource, scope, context, and time.

### Business Perspective

User types allow CAT to serve both individual operators and organizations without redefining the product. They support:

- a creator who wants help producing approved assets;
- an affiliate operator who manages network relationships and link health;
- a business owner who approves campaigns and budgets;
- an administrator who configures a workspace;
- a developer who adds an integration or agent skill;
- an enterprise that requires separation of duties and audit.

The business value of role clarity is trust. People can understand what CAT may do on their behalf, which decisions remain theirs, and how responsibility is divided when an AI agent performs work.

### Technical Perspective

Authorization is multi-dimensional. CAT should evaluate:

| Dimension | Example |
|---|---|
| Principal | Human, service, agent, integration, organization |
| Action | Read, create, edit, approve, publish, export, pay, administer |
| Resource | Campaign, content asset, affiliate link, ledger record, model, policy |
| Scope | Own records, team records, organization, selected campaign, global platform |
| Context | Environment, risk level, data classification, time, approval state |
| Effect | Read-only, internal mutation, external publication, financial transfer, irreversible action |
| Decision | Allow, deny, require approval, allow with notification, quarantine |

Permissions should be least-privilege, explicit, auditable, revocable, and testable. Role-based permissions are useful for defaults; resource and policy conditions are needed for real enterprise boundaries.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-USER-DEC-001 | CAT recognizes Guest, Creator, Affiliate, Business, Administrator, AI Agent, Developer, and Enterprise participants. | New principal types require an explicit identity and permission decision. |
| P2-USER-DEC-002 | Human roles and AI service principals are distinct. | AI agents cannot inherit human accountability or approval authority by default. |
| P2-USER-DEC-003 | Administrator is scoped authority, not unrestricted power. | Platform, organization, security, Treasury, and support duties can be separated. |
| P2-USER-DEC-004 | Enterprise is an organizational boundary containing users, policies, resources, and audit requirements. | Enterprise behavior is configured through governed policies rather than core forks. |
| P2-USER-DEC-005 | High-impact actions require explicit scope and the appropriate human or policy approval. | Role membership alone does not authorize publishing, spending, credential changes, or irreversible deletion. |
| P2-USER-DEC-006 | Developer access and runtime agent access are separate permission surfaces. | Code contributors do not automatically receive production data or execution authority. |

### Recommendations

- Use role templates as defaults and resource-level scopes for real operations.
- Separate “can prepare” from “can approve” and “can execute.”
- Separate “can view financial data” from “can initiate or approve financial actions.”
- Give AI Agents narrow tool capabilities and explicit expiry/revocation.
- Require two-person or multi-role review for the highest-risk enterprise actions where appropriate.
- Make permission explanations available to humans and machine-readable to AI agents.

### Experimental Ideas

- A permission simulator that shows “why allowed” and “why denied” for a proposed action without executing it.
- Risk-adaptive access that increases verification requirements for unusual behavior while preserving human review.
- A role recommendation assistant that proposes least-privilege role bundles from observed work, subject to administrator approval.

### Future Ideas

- Cross-organization federation for partner and marketplace workflows.
- Delegated administration for regional or business-unit operators.
- Enterprise policy packs that can be tested and versioned before activation.
- Temporary just-in-time permissions for incident response and sensitive integrations.
- Organization-level AI workforce management with agent registration, evaluation, and retirement.

### Visual Overview

**Diagram ID:** P2-USER-001<br>
**Diagram Title:** CAT Principal and Authority Model<br>
**Purpose:** Show how human, organizational, and AI principals reach platform capabilities through identity and policy rather than direct privilege.

```mermaid
flowchart TB
    Guest[Guest]
    Creator[Creator]
    Affiliate[Affiliate operator]
    Business[Business owner/operator]
    Admin[Administrator]
    Developer[Developer]
    Enterprise[Enterprise organization]
    Agent[AI Agent service principal]

    Identity[Identity and tenant boundary]
    Policy[Permission, risk, and approval policy]
    Surface[Governed API, KATA, portal, or developer surface]
    Capability[CAT capabilities and domains]
    Audit[Audit, knowledge, and outcome records]

    Guest --> Identity
    Creator --> Identity
    Affiliate --> Identity
    Business --> Identity
    Admin --> Identity
    Developer --> Identity
    Enterprise --> Identity
    Agent --> Identity
    Identity --> Policy
    Policy --> Surface
    Surface --> Capability
    Capability --> Audit
    Policy --> Audit

    style Identity fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style Policy fill:#f59e0b,color:#000
    style Capability fill:#10b981,color:#fff
    style Agent fill:#7c3aed,color:#fff
```

### Mermaid Diagram(s)

**Diagram ID:** P2-USER-002<br>
**Diagram Title:** Action Authorization Decision Tree<br>
**Purpose:** Show the decisions applied before a principal can perform or initiate a CAT action.

```mermaid
flowchart TD
    Request[Action request] --> Authenticated{Principal authenticated?}
    Authenticated -->|No| GuestPath[Public/read-only path or deny]
    Authenticated -->|Yes| Tenant{Correct organization/tenant?}
    Tenant -->|No| Deny[Deny and audit]
    Tenant -->|Yes| Role{Role and scope permit resource?}
    Role -->|No| Deny
    Role -->|Yes| Data{Data classification allowed?}
    Data -->|No| Deny
    Data -->|Yes| Risk{Impact and reversibility}
    Risk -->|Low| Allow[Allow and log]
    Risk -->|Medium| Notify[Allow with notification or scoped policy]
    Risk -->|High| Approval[Require human approval]
    Risk -->|Critical| Multi[Require authorized multi-role approval]
    Approval --> Decision{Approved?}
    Multi --> Decision
    Decision -->|Yes| Verify[Verify exact scope at execution]
    Decision -->|No| Reject[Reject, record rationale, and stop]
    Verify --> Execute[Execute and observe]
```

**Diagram ID:** P2-USER-003<br>
**Diagram Title:** Role Lifecycle and Separation of Duties<br>
**Purpose:** Show how a user or agent is invited, assigned, constrained, reviewed, and revoked.

```mermaid
stateDiagram-v2
    [*] --> Invited
    Invited --> Verified
    Verified --> Provisioned
    Provisioned --> Active
    Active --> ScopedChange: role or permission change
    ScopedChange --> Active
    Active --> Suspended: risk, leave, incident, or policy
    Suspended --> Active: reviewed and restored
    Active --> Revoked: departure, compromise, or decommission
    Revoked --> Archived
    Archived --> [*]
```

**Diagram ID:** P2-USER-004<br>
**Diagram Title:** User Type Capability Surface<br>
**Purpose:** Compare the primary work surfaces available to different participant types.

```mermaid
graph LR
    Guest[Guest] --> Public[Public information]
    Creator[Creator] --> Creative[Draft and revise assets]
    Affiliate[Affiliate] --> Partner[Programs, links, attribution]
    Business[Business] --> Campaigns[Campaigns, decisions, approvals]
    Admin[Administrator] --> Org[Membership, settings, policy]
    Agent[AI Agent] --> Tools[Declared tools and workflows]
    Developer[Developer] --> Dev[Code, APIs, SDKs, tests]
    Enterprise[Enterprise] --> Governance[Organization governance, audit, integrations]
```

### Visual Asset Placeholder — Role and Permission Command Center

| Field | Value |
|---|---|
| **Visual Asset Name** | CAT Role and Permission Command Center |
| **Purpose** | Show a human-readable view of principals, roles, scopes, active sessions, pending approvals, policy explanations, and recent permission changes. |
| **Recommended Resolution** | 3840 × 2160 px desktop; 1440 × 900 px administrative workflow; mobile-safe 390 × 844 px review crop |
| **Suggested Location inside `/design`** | `/design/UI/IdentityAndGovernance/RolePermissionCommandCenter.md` |
| **Mood** | Trustworthy, precise, calm, and security-conscious |
| **Style** | Glass command panels, explicit scope chips, readable tables, status color plus text labels, no decorative ambiguity |
| **Reference Category** | Identity administration / enterprise governance / security operations |

### User Type Contracts

#### Guest

- **Purpose:** Discover public CAT information or access explicitly public resources without an authenticated workspace identity.
- **Owner:** Product and security owners define public exposure.
- **Responsibilities:** Respect public-use terms; authenticate before private or mutating actions; do not infer that a public page exposes private platform state.
- **Inputs:** Public requests, public documentation, invitation or sign-in action.
- **Outputs:** Public views, sign-in initiation, support/contact request where available.
- **Dependencies:** Experience; Identity; Security; public content policy.
- **Extension Points:** Public documentation, demos, status views, partner onboarding entry points.
- **Failure Modes:** Unauthorized private request; abusive traffic; bot or rate-limit violation.
- **Recovery Strategy:** Deny safely; rate limit; explain required authentication; preserve no sensitive detail in errors.
- **Future Evolution:** Public marketplace discovery, documentation portals, and controlled product demonstrations.

#### Creator

- **Purpose:** Develop, edit, review, and contribute content or creative assets within assigned campaigns and brand/policy boundaries.
- **Owner:** Content/Creative domain owner and the relevant Business owner.
- **Responsibilities:** Use approved sources; maintain accuracy and disclosure; revise drafts; respond to quality findings; respect channel and brand rules.
- **Inputs:** Briefs, product facts, source evidence, affiliate context, brand settings, agent drafts, reviewer feedback.
- **Outputs:** Draft content, revisions, asset metadata, quality feedback, approval requests.
- **Dependencies:** Content; Commerce; Knowledge; Affiliate; Marketing; Identity; Settings; Security.
- **Extension Points:** Creator templates, editorial workflows, localization, review tools, approved model assistance.
- **Failure Modes:** Unsupported claim; missing attribution/disclosure; asset version conflict; policy violation.
- **Recovery Strategy:** Return to draft; preserve prior version; require correction; block public publication until resolved.
- **Future Evolution:** Collaborative human/AI studios, multimodal asset production, and organization-specific editorial workspaces.

#### Affiliate

- **Purpose:** Operate affiliate program, merchant, link, term, and attribution workflows.
- **Owner:** Affiliate operations owner.
- **Responsibilities:** Validate partner terms; manage links; monitor health; maintain network credentials through approved paths; investigate attribution discrepancies.
- **Inputs:** Product/campaign scope, network data, terms, tracking requirements, Analytics and Treasury signals.
- **Outputs:** Links, program records, term changes, health events, attribution corrections, partner recommendations.
- **Dependencies:** Affiliate domain; Commerce; Identity; Security; Analytics; Treasury.
- **Extension Points:** New networks, regional partners, connector plugins, link validation tools.
- **Failure Modes:** Expired relationship; malformed link; wrong tracking; unauthorized term change; credential error.
- **Recovery Strategy:** Quarantine link; restore last valid version; revalidate provider terms; rotate credential; hold publication or financial interpretation.
- **Future Evolution:** Partner portfolio optimization and governed affiliate-network marketplace.

#### Business

- **Purpose:** Own commercial intent, campaign priorities, product/market strategy, risk acceptance, and required approvals.
- **Owner:** Business/product owner for the organization or campaign.
- **Responsibilities:** Set goals and constraints; review recommendations; approve material actions; interpret outcomes; own business consequences.
- **Inputs:** Research, plans, content packages, Treasury context, Analytics, policy, human strategy.
- **Outputs:** Goals, constraints, approvals, rejections, budget decisions, strategic feedback.
- **Dependencies:** Commerce; Marketing; Analytics; Treasury; Knowledge; Identity; Administration.
- **Extension Points:** Business-unit policies, campaign templates, approval thresholds, strategic dashboards.
- **Failure Modes:** Ambiguous objective; stale approval; conflict between budget and scope; approval without evidence.
- **Recovery Strategy:** Request clarification; re-open plan; expire or revoke approval; record rationale; route financial or legal questions to the correct specialist.
- **Future Evolution:** Portfolio-level command, predictive planning, and multi-business-unit governance.

#### Administrator

- **Purpose:** Manage organization membership, scoped roles, settings, approved integrations, policies, and operational controls.
- **Owner:** Platform Administration owner or organization owner, depending on scope.
- **Responsibilities:** Provision and revoke access; configure policy; maintain settings; manage approved extensions; support audit and recovery; avoid bypassing Security.
- **Inputs:** Organization requests, identity events, policy changes, security alerts, enterprise requirements.
- **Outputs:** Role changes, configuration versions, policy assignments, audit records, support actions.
- **Dependencies:** Identity; Security; Settings; Administration; Foundation; enterprise integration.
- **Extension Points:** Delegated admins, private catalogs, policy packs, support workflows.
- **Failure Modes:** Excess privilege; accidental deletion; incorrect tenant scope; unreviewed policy change.
- **Recovery Strategy:** Confirmation, scoped approval, rollback, audit review, break-glass protocol, Security escalation.
- **Future Evolution:** Delegated multi-region and enterprise control planes.

#### AI Agent

- **Purpose:** Perform a declared, evaluated, and bounded role in CAT's workflows as a non-human service principal.
- **Owner:** Agent owner, with Security and domain owners accountable for permission and domain behavior.
- **Responsibilities:** Accept scoped tasks; retrieve allowed context; perform role work; state uncertainty; emit outputs and traces; escalate; learn only through governed paths.
- **Inputs:** Structured tasks, allowed tools, context, policies, prompts, model routes, memory, deadlines.
- **Outputs:** Recommendations, drafts, task results, events, explanations, failure states, lesson candidates.
- **Dependencies:** Identity/service principal; CATA/Automation; Knowledge; domain APIs; Security; evaluation; observability.
- **Extension Points:** New tools, skills, models, prompts, domain agents, evaluators, sandboxed capabilities.
- **Failure Modes:** Hallucination; tool misuse; prompt injection; privilege escalation; non-idempotent retry; hidden uncertainty; output contract failure.
- **Recovery Strategy:** Stop or quarantine; revoke tool; retry with fallback; require human review; preserve trace; roll back policy or model route.
- **Future Evolution:** Agent teams, role-specific memory, evaluation-driven routing, and governed autonomous workforces.

#### Developer

- **Purpose:** Build, test, document, integrate, secure, and maintain CAT components or extensions.
- **Owner:** Engineering/architecture owner for the relevant repository or contract.
- **Responsibilities:** Follow context and coding rules; write tests; update documentation; preserve backward compatibility; review security and operational impact.
- **Inputs:** Tasks, context, ADRs, code, tests, APIs, design specs, status, feedback.
- **Outputs:** Code, migrations, tests, documentation, integrations, decision proposals, operational improvements.
- **Dependencies:** AI workspace; project rules; architecture; coding standards; repository access; CI/review; security.
- **Extension Points:** SDKs, plugins, connectors, agent skills, tooling, documentation automation.
- **Failure Modes:** Breaking contract; secret leak; undocumented behavior; test gap; unsafe migration; scope drift.
- **Recovery Strategy:** Review, revert, migration rollback, incident response, documentation correction, access reduction.
- **Future Evolution:** AI-assisted development, public SDKs, partner developer ecosystem, and generated contract tooling.

#### Enterprise

- **Purpose:** Represent an organization with its users, business units, policies, data boundaries, integrations, audit requirements, and support relationship.
- **Owner:** Enterprise customer owner in partnership with Omni/CAT platform owners.
- **Responsibilities:** Define local policy; manage users and approvals; protect credentials; review extensions; meet organizational and regulatory obligations; provide accountable decision owners.
- **Inputs:** Organization identity, policies, users, integrations, data, budgets, compliance requirements, support requests.
- **Outputs:** Enterprise configuration, approvals, audit exports, governed workflows, usage and financial records.
- **Dependencies:** Administration; Identity; Security; Settings; Treasury; APIs; deployment; support and documentation.
- **Extension Points:** Private plugins, delegated admin, private catalogs, enterprise identity, compliance exports, policy packs.
- **Failure Modes:** Misconfiguration; tenant isolation failure; approval ambiguity; unsupported integration; local policy conflict.
- **Recovery Strategy:** Suspend affected scope; restore configuration; rotate access; engage support/security; preserve audit and incident evidence.
- **Future Evolution:** Multi-tenant platform, regional controls, private AI models, and federated enterprise ecosystems.

### Permission and Capability Matrix

Legend: **R** = read; **D** = draft/prepare; **E** = execute within policy; **A** = approve; **C** = configure/administer; **—** = no default access. Exact permissions remain resource- and scope-dependent.

| Capability | Guest | Creator | Affiliate | Business | Administrator | AI Agent | Developer | Enterprise |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Public information | R | R | R | R | R | — | R | R |
| Private workspace state | — | R | R | R | R/C | Scoped R | Scoped R | R/C |
| Content draft | — | D/E | D | D/A | Scoped | D/E by role | D/E in dev/test | Policy-scoped |
| Content approval | — | —/A if delegated | —/A if delegated | A | A if assigned | — | —/A only in test | A by policy |
| Affiliate link preparation | — | D | D/E | D/A | Scoped | E within tool scope | Test/dev only by default | Policy-scoped |
| Public publication | — | —/E after approval | —/E after approval | A/E by policy | Scoped | E only after valid approval | Test/dev only | Policy-scoped |
| Analytics read | Public only | Scoped R | Scoped R | R | R | Scoped R | Synthetic/test by default | R by policy |
| Treasury read | — | —/limited | Scoped | R | R/C by role | Scoped, masked by policy | Synthetic/test by default | R by policy |
| Treasury action/payout | — | — | —/A if assigned | A | Scoped | — | — | Authorized human only |
| User/role administration | — | — | — | —/limited | C | — | Dev/test fixtures only | C by delegated policy |
| Security configuration | — | — | — | — | Scoped C | — | Review/propose | Enterprise security owner |
| Core code/extension development | — | — | — | — | — | — | E in authorized repo | Through approved developer access |
| Policy change | — | — | — | Propose/A by scope | C/A by scope | Propose only | Propose | Enterprise owner under CAT guardrails |

### User and Workflow Interaction Model

A user type does not directly own a screen. It receives a surface appropriate to its responsibilities and the current task. The same underlying workflow may appear as a conversational request to Business, a content workspace to Creator, a link-health queue to Affiliate, an approval card to Administrator, and structured task state to an AI Agent.

### Visual Asset Placeholder — Multi-Role CAT Workspace

| Field | Value |
|---|---|
| **Visual Asset Name** | CAT Multi-Role Workspace and Approval Surface |
| **Purpose** | Represent the same workflow through role-specific views: creator draft, affiliate link context, business decision, administrator policy, enterprise audit, and AI task trace. |
| **Recommended Resolution** | 3840 × 2160 px master board; 1920 × 1080 px role-view exports; 1440 × 900 px application view |
| **Suggested Location inside `/design`** | `/design/UI/MultiRoleWorkspace/` |
| **Mood** | Cohesive intelligence with role-appropriate focus |
| **Style** | Shared visual language with differentiated information density, accessible contrast, explicit approval states, calm motion |
| **Reference Category** | Role-based workspace / approval workflow / enterprise product UX |

### Mermaid Diagram(s)

**Diagram ID:** P2-USER-005<br>
**Diagram Title:** One Workflow, Multiple Responsibility Views<br>
**Purpose:** Show that role-specific interfaces are projections of one governed workflow rather than separate truths.

```mermaid
flowchart LR
    Workflow[Canonical campaign workflow]
    Workflow --> CreatorView[Creator view<br/>brief, draft, feedback]
    Workflow --> AffiliateView[Affiliate view<br/>terms, links, health]
    Workflow --> BusinessView[Business view<br/>plan, evidence, approval]
    Workflow --> AdminView[Admin view<br/>policy, roles, audit]
    Workflow --> EnterpriseView[Enterprise view<br/>governance, reports, integrations]
    Workflow --> AgentView[Agent view<br/>task, tools, context, output]

    CreatorView -. governed projection .-> Workflow
    AffiliateView -. governed projection .-> Workflow
    BusinessView -. approval decision .-> Workflow
    AdminView -. policy/configuration .-> Workflow
    EnterpriseView -. organizational policy .-> Workflow
    AgentView -. task/result .-> Workflow
```

**Diagram ID:** P2-USER-006<br>
**Diagram Title:** Separation of Preparation, Approval, and Execution<br>
**Purpose:** Prevent role capability from being confused with authority to create external effects.

```mermaid
flowchart TD
    Prepare[Creator, Affiliate, AI Agent, or Developer prepares]
    Prepare --> Review[Evidence, quality, policy, and scope review]
    Review --> Approver{Authorized human or policy approver}
    Approver -->|Reject| Record[Record rejection and reason]
    Approver -->|Modify| Prepare
    Approver -->|Approve| Scope[Bind exact action scope]
    Scope --> Executor[Agent/domain executor]
    Executor --> Outcome[Observe, reconcile, and learn]
```

**Diagram ID:** P2-USER-007<br>
**Diagram Title:** Enterprise Organization Boundary<br>
**Purpose:** Show how enterprise users and service principals remain inside organization, policy, data, and audit boundaries.

```mermaid
graph TB
    Omni[CAT/Omni platform governance]
    Enterprise[Enterprise organization]
    OrgPolicy[Enterprise policy]
    Users[Enterprise users]
    Services[Enterprise service principals and AI agents]
    Data[Enterprise-scoped data and knowledge]
    Audit[Enterprise audit and reports]
    Integrations[Enterprise integrations]

    Omni --> Enterprise
    Enterprise --> OrgPolicy
    Enterprise --> Users
    Enterprise --> Services
    OrgPolicy --> Users
    OrgPolicy --> Services
    Users --> Data
    Services --> Data
    Data --> Audit
    Enterprise --> Integrations
    Integrations --> Data
```

### Cross References

- Organizational philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 17.
- KATA human interface: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 13.6.
- Security and permission target: [`context/17_SECURITY.md`](./17_SECURITY.md).
- Administration and identity future context: [`context/02_PROJECT_RULES.md`](./02_PROJECT_RULES.md), [`context/16_DEPLOYMENT.md`](./16_DEPLOYMENT.md).
- Human interaction and design targets: [`context/10_UI_UX.md`](./10_UI_UX.md), [`context/11_DESIGN_LANGUAGE.md`](./11_DESIGN_LANGUAGE.md).

### Dependencies

User types depend on Identity, Administration, Security, Settings, policy, audit, organization boundaries, user interface surfaces, and domain ownership. Permissions also depend on accurate resource identity and workflow state. A role cannot be evaluated correctly if the target resource, tenant, or action scope is ambiguous.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Role explosion | Permissions become unmaintainable | Use role templates plus resource scopes |
| Administrator as universal superuser | Separation of duties fails | Split platform, organization, security, and Treasury authority |
| AI Agent inherits human role | Unaccountable or over-privileged automation | Separate service principals and explicit tool scopes |
| Enterprise policy conflicts with CAT safety | Local customization weakens core controls | Allow stricter policy, not weaker mandatory safeguards |
| Capability matrix mistaken for final authorization | Static table misses context and resource scope | Evaluate action, resource, tenant, risk, and state at runtime |
| Public and private views leak through projections | Data exposure | Classify data and enforce read-model access |

### Anti-patterns

- “If the user can see the button, the user can perform the action.”
- “Business owner means can spend any amount.”
- “Developer access is equivalent to production access.”
- “AI Agent is an administrator because it automates administration.”
- “Enterprise means bypassing CAT defaults.”
- “A role name is enough; scope and resource do not matter.”

### Best Practices

- Use separation of duties for high-impact actions.
- Explain permissions in human language and expose machine-readable denial reasons.
- Review service-principal permissions as rigorously as human permissions.
- Revoke and expire access explicitly.
- Keep organization, environment, and resource boundaries visible in audit records.
- Test both allowed and denied behavior for every sensitive capability.

### Extension Points

- New roles through identity, policy, and ownership records.
- New organization models through tenant and delegation contracts.
- New user surfaces through governed read/command APIs.
- New agent types through service-principal registration and evaluation.
- New enterprise controls through stricter policy packs and private extension catalogs.

### AI Construction Notes

When an agent receives “make this available to admins,” it must ask which administrators, in which organization, for which resources, with what side effects, and whether approval remains required. It should never translate a role label directly into a wildcard permission.

### AI Memory Anchor

> **A principal is not a permission. Authorization is the evaluated relationship between actor, action, resource, scope, policy, and impact.**

### Implementation Checklist

- [x] All requested user types are defined.
- [x] Responsibilities, capabilities, dependencies, failure modes, recovery, and future evolution are documented.
- [x] Permission and capability matrix is included with explicit scope caveat.
- [x] Role lifecycle, authorization, and enterprise boundaries are visualized.
- [x] Visual asset placeholders cover role and workspace concepts.
- [ ] Runtime identity, policy, and role schemas are defined in downstream security and architecture work.
- [ ] End-user permission UX is implemented later.

---

## 21. Product Philosophy — Why CAT Is a Platform Instead of a Tool

### Human Explanation

A **tool** helps a person perform a bounded task. A **platform** provides the reusable capabilities, contracts, participants, governance, and state needed for many tasks and products to operate coherently.

A content editor can be a tool. A link checker can be a tool. A campaign report can be a tool. CAT may contain all of those capabilities, but its identity is larger: CAT coordinates a living commerce operation in which multiple people, agents, domains, integrations, workflows, and knowledge assets work together over time.

CAT is a platform because it provides:

- a persistent operating context rather than a one-off session;
- multiple specialized capabilities rather than one isolated task;
- stable domain and integration contracts;
- identity, permissions, policy, and approval boundaries;
- agent registration, task execution, and evaluation;
- shared knowledge, memory, decisions, and outcomes;
- extension points for connectors, workflows, templates, and future modules;
- a way for multiple user types and organizations to participate;
- an ecosystem path for contributors, partners, enterprises, and future Omni products.

The platform thesis does not mean that every feature must be public, configurable, or extensible on day one. It means that the Core is designed with durable boundaries so that extension can be added without rewriting the identity and safety model.

### AI Context

An AI agent must evaluate a proposed feature at two levels:

1. **Immediate utility:** does it solve a real user or business problem?
2. **Platform fit:** does it use or strengthen a stable contract that other workflows can safely consume?

Platform fit must not become an excuse to build abstractions without evidence. The correct sequence is:

```text
Validate a useful capability → define its boundary → stabilize its contract → expose it when reuse is real.
```

An AI agent must not expose internal implementation as a public plugin surface merely because it can be called. It must identify permission scope, versioning, ownership, support, observability, and revocation.

### Business Perspective

The platform strategy compounds value. Each approved connector, evaluation pattern, workflow, knowledge relationship, and governance rule can reduce the cost of future capabilities. This creates a long-term moat in:

- accumulated commerce knowledge;
- reliable agent/domain contracts;
- integration coverage;
- trusted human/AI operating practices;
- enterprise governance;
- reusable Omni System platform primitives.

The platform strategy also changes how CAT is measured. A tool can be judged by a single task's success. A platform must be judged by the quality and safety of the ecosystem it enables.

### Technical Perspective

A platform requires stable boundaries and an evolution model:

| Platform property | CAT implication |
|---|---|
| Contract surface | APIs, events, task schemas, plugin manifests, knowledge schemas |
| Participant model | Humans, AI agents, developers, enterprises, integrations |
| Governance | Permissions, approval, audit, review, revocation |
| State | Persistent workflows, domain records, memory, decisions, outcomes |
| Compatibility | Versioning, migrations, deprecation, backward compatibility |
| Discovery | Documentation, registries, capability metadata, examples |
| Observability | Usage, failures, quality, security, financial and operational outcomes |
| Economics | Optional packaging, enterprise support, partner and marketplace paths |

The platform boundary is not the same as a public API boundary. Internal contracts can be stable without being public, and public contracts must be narrower and more governed than internal ones.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-PLAT-DEC-001 | CAT is a platform for autonomous and supervised commerce operations, not a single-purpose tool. | Product planning must consider reuse, governance, and lifecycle integration. |
| P2-PLAT-DEC-002 | Platform expansion proceeds from a strong Core through stable contracts. | Optional capabilities should not destabilize Core identity. |
| P2-PLAT-DEC-003 | Extensibility is governed. | Plugins, agents, integrations, and workflows declare permissions, ownership, compatibility, and lifecycle. |
| P2-PLAT-DEC-004 | Knowledge and outcomes are platform assets. | Participants contribute to a compounding institutional memory under review. |
| P2-PLAT-DEC-005 | Enterprise and partner customization should use policy and extension surfaces instead of Core forks. | The product remains maintainable and auditable. |

### Recommendations

- Prove a capability in CAT Core before extracting it as a reusable Omni System primitive.
- Keep public contracts smaller than internal implementation surfaces.
- Provide examples and validation tools for every supported extension type.
- Treat support, deprecation, revocation, and security as part of platform design.
- Make extension discovery useful to both developers and AI agents.
- Prefer configuration and policy for deployment variation; reserve plugins for behavior that genuinely needs code or a specialized contract.

### Experimental Ideas

- A local platform simulator that runs a complete campaign with synthetic data and no external side effect.
- A contract marketplace score based on compatibility, evaluation quality, security posture, and real usage.
- AI-generated extension plans that produce manifests, tests, documentation, and risk assessments for human review.
- A “platform maturity score” that tracks contract stability, knowledge reuse, and ecosystem health.

### Future Ideas

- Public and private extension marketplaces.
- Partner SDKs and certified integration programs.
- Enterprise solution bundles combining policy, connectors, workflows, and reports.
- Future Omni System products built on extracted identity, knowledge, agent, and governance primitives.
- Cross-product organization and billing capabilities once contracts are mature.

### Visual Overview

**Diagram ID:** P2-PLAT-004<br>
**Diagram Title:** Tool-to-Platform Progression<br>
**Purpose:** Explain the additional responsibilities CAT accepts by operating as a platform.

```mermaid
flowchart LR
    Tool[Bounded tool<br/>one task, one user, short context]
    Tool --> Workflow[Connected workflow<br/>several tasks and state]
    Workflow --> Platform[CAT platform<br/>domains, agents, governance, knowledge]
    Platform --> Ecosystem[Ecosystem<br/>partners, extensions, enterprises, future products]

    Tool -->|adds| Task[Task execution]
    Workflow -->|adds| State[Durable state and coordination]
    Platform -->|adds| Contracts[Identity, contracts, policy, learning]
    Ecosystem -->|adds| Participants[Trusted participants and reuse]
```

**Diagram ID:** P2-PLAT-005<br>
**Diagram Title:** CAT Platform Value Compounding<br>
**Purpose:** Show how use of the platform creates reusable assets without treating raw activity as knowledge automatically.

```mermaid
flowchart TB
    Capability[Capability or extension] --> Operation[Governed operation]
    Operation --> Outcome[Observed outcome]
    Outcome --> Review[Validation and interpretation]
    Review --> Knowledge[Curated knowledge and decision record]
    Knowledge --> Better[Better future context and contracts]
    Better --> Capability

    Governance[Identity, security, approval, audit] --> Operation
    Governance --> Review
```

### Visual Asset Placeholder — CAT Developer and Extension Portal

| Field | Value |
|---|---|
| **Visual Asset Name** | CAT Developer and Extension Portal |
| **Purpose** | Present APIs, events, SDKs, plugin manifests, compatibility versions, examples, evaluations, permissions, ownership, and support status to developers and AI coding agents. |
| **Recommended Resolution** | 2560 × 1600 px desktop documentation portal; 1920 × 1080 px interactive contract explorer |
| **Suggested Location inside `/design`** | `/design/DeveloperExperience/ExtensionPortal/` |
| **Mood** | Precise, welcoming, technical, and trustworthy |
| **Style** | Clean dark documentation surface with restrained cosmic accents, searchable contracts, side-by-side examples, explicit maturity badges |
| **Reference Category** | Developer portal / API explorer / platform documentation |

### Mermaid Diagram(s)

**Diagram ID:** P2-PLAT-006<br>
**Diagram Title:** Core, Contracts, and Ecosystem Layers<br>
**Purpose:** Show how the governed platform grows outward without allowing extensions to redefine Core authority.

```mermaid
graph TB
    Core[CAT Core<br/>identity, lifecycle, governance, canonical domains]
    Contracts[Stable contracts<br/>APIs, events, tasks, knowledge, extension manifests]
    FirstParty[First-party agents and modules]
    Partners[Verified partner extensions]
    Community[Community or local extensions]
    Enterprise[Enterprise policies and private catalogs]
    Future[Future Omni products]

    Core --> Contracts
    Contracts --> FirstParty
    Contracts --> Partners
    Contracts --> Community
    Contracts --> Enterprise
    Contracts --> Future

    FirstParty -->|outcomes and lessons| Core
    Partners -->|governed events and results| Core
    Community -->|scoped results| Core
    Enterprise -->|policy and feedback| Core
    Future -->|shared contracts only| Core
```

### Architecture Map

#### Platform Asset Map

| Asset | Owned by | Reused by | Governance |
|---|---|---|---|
| Core domain behavior | CAT domain owner | CAT workflows and approved clients | Architecture, security, domain review |
| API/event contract | Contract owner | Agents, modules, partners, future products | Versioning and compatibility |
| Agent role | Agent owner | Orchestrator and workflows | Evaluation, permissions, autonomy |
| Knowledge artifact | Knowledge owner plus contributor | Humans and agents | Provenance, confidence, lifecycle |
| Workflow template | Workflow owner | Users, agents, enterprises | Approval, compatibility, outcome monitoring |
| Connector | Integration/domain owner | Relevant workflows | Credentials, reliability, external policy |
| Plugin | Plugin author and marketplace owner | Approved deployments | Sandbox, permissions, review, revocation |
| Enterprise policy | Enterprise owner | Organization users and agents | Local governance within CAT safeguards |

#### Platform Health Questions

| Question | Healthy platform behavior |
|---|---|
| Can a new consumer use the capability? | Contract, documentation, example, and permission path exist |
| Can an old consumer survive a change? | Versioning, migration, or compatibility plan exists |
| Can an extension be removed? | Ownership, dependency, disable, and revocation path exists |
| Can a human understand an AI action? | Evidence, context, decision, approval, and outcome are linked |
| Can an incident be reconstructed? | Trace, event, state, and audit records are available |
| Can a new domain be added? | Domain contract and platform extension point are clear |

### Cross References

- Ecosystem philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 15.
- Developer and community ecosystem: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 15.10–15.15.
- Repository contribution model: [`../CONTRIBUTING.md`](../CONTRIBUTING.md).
- API and extension detail target: [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md), [`context/15_DIRECTORY_STRUCTURE.md`](./15_DIRECTORY_STRUCTURE.md).

### Dependencies

The platform strategy depends on stable Core identity, domain contracts, Identity and Security, versioning, documentation, observability, ownership, support, and a trustworthy knowledge lifecycle. It also depends on resisting premature public exposure of unstable internals.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Platform before product value | Complexity without validated users or outcomes | Prove Core workflows before broad ecosystem investment |
| “Everything is an extension” | Core becomes incoherent and setup becomes difficult | Keep identity-defining capabilities in Core |
| “Everything is Core” | Maintenance and security burden grow | Use placement and promotion criteria |
| Unversioned public contracts | Ecosystem breakage | Version, deprecate, and test compatibility |
| Marketplace grows faster than trust | Security and reputation incidents | Review, sandbox, monitor, and revoke |
| Platform metrics reward activity | More extensions but less value | Measure quality, reliability, outcomes, and adoption |

### Anti-patterns

- Launching a marketplace before permissions and revocation exist.
- Calling a private internal function an SDK.
- Making every enterprise request a Core branch.
- Treating ecosystem contributors as unaccountable to the platform's policies.
- Exposing sensitive data because an extension requests it.
- Measuring platform success by number of APIs or plugins alone.

### Best Practices

- Keep Core opinionated about identity, governance, lifecycle, and canonical meaning.
- Keep extensions explicit about permissions and support boundaries.
- Use compatibility tests and deprecation windows.
- Let real reuse justify abstraction.
- Record extension outcomes and failures as platform knowledge.
- Publish examples that show safe usage, not only happy-path calls.

### Extension Points

- APIs and event streams.
- Workflow and task templates.
- Agent skills and model routes.
- Connectors and provider adapters.
- Content, reporting, and policy packs.
- Enterprise private catalogs.
- Future Omni System shared services.

### AI Construction Notes

When an AI agent is asked to “make this a platform capability,” it must identify the intended consumers, contract stability, permission surface, lifecycle owner, support model, and removal strategy. If there is only one consumer and no evidence of reuse, the agent should recommend a well-bounded internal module before a public extension.

### AI Memory Anchor

> **Platform value comes from stable, governed reuse—not from exposing every internal function.**

### Implementation Checklist

- [x] Tool versus platform distinction is explicit.
- [x] Platform assets, participants, contracts, and governance are mapped.
- [x] Core-to-ecosystem expansion is visualized.
- [x] Developer/extension portal visual metadata is provided.
- [x] Risks of premature or ungoverned platformization are documented.
- [ ] Public SDK and marketplace contracts are defined only after Core maturity.

---

## 22. AI-Native Design — Every Feature Begins with AI Workflows

### Human Explanation

AI-native design means CAT is designed around AI as an active operator from the beginning. AI is not a text box added to a human-first application, and it is not a suggestion layer attached to a traditional workflow. Every feature should answer how AI discovers the need, gathers context, reasons about options, performs work, reports uncertainty, requests human judgment, observes the result, and learns.

An AI-native feature has two simultaneous interfaces:

1. **Human interface:** understandable intent, evidence, control, explanation, and correction.
2. **Agent interface:** structured context, capabilities, contracts, policies, tools, state, and evaluation.

The feature is complete only when both interfaces are coherent. A feature that humans can click but agents cannot reason about is not fully AI-native. A feature that an agent can call but humans cannot supervise is not acceptable for consequential work.

The AI-native feature loop is:

```text
Discover → contextualize → plan → authorize → execute → observe → explain → learn
```

AI-native does not mean AI-only. It means the system assumes AI participation while preserving human authority and accessible alternatives.

### AI Context

For every feature, the AI agent should identify:

- the agent or model role;
- the human intent and user types;
- the context required before reasoning;
- the tools and data scopes allowed;
- the output contract and uncertainty representation;
- the approval and escalation rule;
- the observation and evaluation signal;
- the memory and knowledge update;
- the fallback when the model, retrieval, policy, or external provider fails.

Prompts are not the entire AI architecture. Prompt behavior must be backed by permissions, schemas, evaluators, traces, state, and human review.

### Business Perspective

AI-native design is how CAT can scale beyond human attention while retaining trust. It changes product prioritization from “where can we add AI?” to “where can an intelligent, governed operator reduce a bottleneck or improve a decision?”

It also creates a durable advantage when knowledge compounds. A generic AI output can be copied by competitors. A governed system that remembers product outcomes, channel behavior, source reliability, approval patterns, and Treasury results becomes more useful through operation.

### Technical Perspective

AI-native features require a cognitive and operational substrate:

| Requirement | Technical consequence |
|---|---|
| Context before action | Retrieval, memory, source and policy resolution |
| Role-specific behavior | Agent manifests, tools, prompts, model routes, evaluators |
| Bounded autonomy | Permission and approval gateway |
| Durable work | Workflow state, task persistence, retries, checkpoints |
| Explainability | Evidence, reasoning summary, version and trace links |
| Continuous learning | Outcome capture, review state, knowledge lifecycle |
| Model substitution | Provider abstraction, routing, evaluation, cost controls |
| Safety | Input validation, output checks, sandboxing, monitoring |
| Human control | Approval, correction, rejection, override, rollback |

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-AI-DEC-001 | CAT features are designed for AI-first workflows, not merely AI-assisted user clicks. | Feature specifications must include agent participation and context requirements. |
| P2-AI-DEC-002 | AI capabilities are role-specific and bounded. | Agents declare tools, data, outputs, autonomy, and escalation. |
| P2-AI-DEC-003 | Human-readable and machine-readable interfaces are both required. | APIs, schemas, explanations, and UI surfaces evolve together. |
| P2-AI-DEC-004 | AI output is not automatically truth or authority. | Provenance, evaluation, policy, and approval remain required. |
| P2-AI-DEC-005 | Model providers are replaceable behind evaluation and routing contracts. | CAT must not define its identity by one model or vendor. |
| P2-AI-DEC-006 | Outcomes and lessons are part of AI feature completion. | A generated result without observable outcome and learning path is incomplete. |

### Recommendations

- Start feature design with a workflow and context map before a prompt.
- Separate retrieval context, tool capability, policy, and output schema.
- Use structured outputs and validation for every agent boundary.
- Make uncertainty and source evidence visible to humans.
- Evaluate agent behavior against representative tasks, edge cases, and harmful cases.
- Log model, prompt, tool, source, policy, and approval versions for consequential work.
- Design low-risk fallbacks so the system degrades gracefully without pretending success.

### Experimental Ideas

- Agents that propose their own missing-context questions before acting.
- Model routing learned from task quality, cost, latency, and risk.
- A “knowledge freshness budget” that determines when an agent must refresh evidence.
- Human feedback captured as structured evaluation labels instead of only comments.
- A sandbox in which agents can practice workflows against synthetic Commerce and Treasury records.

### Future Ideas

- Proactive agents that detect opportunities and create work without user prompts.
- Multi-agent planning with formally evaluated collaboration protocols.
- Organization-specific private models and retrieval stores.
- Adaptive user interfaces that surface only the evidence relevant to a current decision.
- AI-assisted documentation synchronization between code, contracts, and context.

### Visual Overview

**Diagram ID:** P2-AI-001<br>
**Diagram Title:** AI-Native Feature Lifecycle<br>
**Purpose:** Show the required AI, human, governance, execution, and learning steps for a CAT feature.

```mermaid
flowchart LR
    Intent[Human or system intent] --> Context[Retrieve context, evidence, memory, policy]
    Context --> Plan[Agent plan or recommendation]
    Plan --> Validate[Schema, quality, safety, and confidence checks]
    Validate --> Authority{Human approval or policy authorization?}
    Authority -->|Required| Human[Human review and decision]
    Authority -->|Not required| Execute[Bounded execution]
    Human -->|Approve| Execute
    Human -->|Reject or revise| Replan[Record and re-plan]
    Replan --> Plan
    Execute --> Observe[Observe outcome, cost, and side effect]
    Observe --> Explain[Explain status and evidence]
    Explain --> Learn[Evaluate and update knowledge]
    Learn --> Context
```

**Diagram ID:** P2-AI-002<br>
**Diagram Title:** AI Feature Contract<br>
**Purpose:** Show the artifacts that must surround an AI capability so that a prompt is not mistaken for a complete system.

```mermaid
graph TD
    Feature[AI-native feature]
    Feature --> Role[Agent role and owner]
    Feature --> Context[Knowledge and memory context]
    Feature --> Tools[Tools and permission scopes]
    Feature --> Prompt[Prompt and model route]
    Feature --> Schema[Input/output schema]
    Feature --> Policy[Approval and safety policy]
    Feature --> Eval[Evaluation and quality rubric]
    Feature --> Trace[Trace, audit, and outcome]
    Feature --> Fallback[Failure and recovery strategy]
```

**Diagram ID:** P2-AI-003<br>
**Diagram Title:** Human and Agent Dual Interface<br>
**Purpose:** Show that a CAT capability must be understandable to humans and actionable by AI systems through shared canonical state.

```mermaid
graph LR
    Canonical[Canonical workflow and domain state]
    Canonical --> HumanSurface[Human surface<br/>explanation, evidence, approval, correction]
    Canonical --> AgentSurface[Agent surface<br/>schema, tools, context, status, result]
    HumanSurface --> Canonical
    AgentSurface --> Canonical
    Policy[Shared policy and permissions] --> HumanSurface
    Policy --> AgentSurface
    Knowledge[Shared knowledge and provenance] --> HumanSurface
    Knowledge --> AgentSurface
```

### Visual Asset Placeholder — KATA AI-Native Command Center

| Field | Value |
|---|---|
| **Visual Asset Name** | KATA AI-Native Command Center |
| **Purpose** | Illustrate how human intent, agent activity, pending approvals, evidence, Treasury impact, and learning state appear as one supervised operating environment. |
| **Recommended Resolution** | 5120 × 2880 px master cinematic view; 3840 × 2160 px production desktop; accessible 1920 × 1080 px mode |
| **Suggested Location inside `/design`** | `/design/UI/KATACommandCenter/` |
| **Mood** | Living intelligence, controlled power, clarity under complexity |
| **Style** | Cosmic cat persona, 3D globe, layered glass panels, restrained particles, explicit text and accessible status indicators, configurable motion |
| **Reference Category** | AI command center / supervisory interface / spatial dashboard |

### Mermaid Diagram(s)

**Diagram ID:** P2-AI-004<br>
**Diagram Title:** AI-Native Feature Readiness Gate<br>
**Purpose:** Provide a decision tree for deciding whether an AI feature is sufficiently specified for implementation.

```mermaid
flowchart TD
    Feature[Feature proposal] --> Outcome{Clear user/business outcome?}
    Outcome -->|No| Clarify[Clarify intent and success]
    Outcome -->|Yes| Context{Required context and sources known?}
    Context -->|No| ClarifyContext[Define retrieval and provenance]
    Context -->|Yes| Role{Agent role and owner defined?}
    Role -->|No| DefineRole[Define role and accountability]
    Role -->|Yes| Tools{Tools and permissions scoped?}
    Tools -->|No| ScopeTools[Define least privilege]
    Tools -->|Yes| Approval{Risk and approval path defined?}
    Approval -->|No| DefinePolicy[Define policy and human boundary]
    Approval -->|Yes| Eval{Evaluation and fallback defined?}
    Eval -->|No| DefineEval[Define quality, safety, and recovery]
    Eval -->|Yes| Ready[AI-native implementation-ready]

    Clarify --> Feature
    ClarifyContext --> Feature
    DefineRole --> Feature
    ScopeTools --> Feature
    DefinePolicy --> Feature
    DefineEval --> Feature
```

**Diagram ID:** P2-AI-005<br>
**Diagram Title:** Model and Prompt Change Control<br>
**Purpose:** Show how a model or prompt change becomes a governed improvement rather than an invisible behavior change.

```mermaid
flowchart LR
    Change[Model or prompt change] --> Offline[Offline evaluation]
    Offline --> Compare[Compare quality, safety, cost, latency]
    Compare --> Review[Human and owner review]
    Review --> Pilot[Scoped pilot or feature flag]
    Pilot --> Monitor[Monitor production outcomes]
    Monitor --> Promote[Promote and record decision]
    Monitor --> Rollback[Rollback and investigate]
    Promote --> Knowledge[Update knowledge and documentation]
    Rollback --> Knowledge
```

### Architecture Map

#### AI-Native Capability Ownership

| Capability | Owner | AI responsibility | Human responsibility |
|---|---|---|---|
| Context retrieval | Knowledge owner | Retrieve relevant evidence within permission | Review provenance and policy for high-impact work |
| Task planning | CATA/AI owner | Decompose and route | Set intent, constraints, and priorities |
| Domain execution | Domain/agent owner | Perform scoped work | Approve material side effects |
| Output quality | Domain and evaluation owner | Validate and report uncertainty | Decide acceptable risk and quality thresholds |
| Model/prompt route | AI owner | Select approved route | Approve material route/policy changes |
| Outcome learning | Knowledge/learning owner | Propose lesson and confidence | Promote high-impact knowledge or policy |

#### AI-Native Data and Control Dependencies

| Artifact | Produced by | Consumed by | Required controls |
|---|---|---|---|
| Intent | Human, event, schedule | CATA/Automation | Identity, scope, priority |
| Context bundle | Knowledge/retrieval | Agent/model | Provenance, permissions, freshness |
| Plan | CATA/agent | Policy/human/domain | Alternatives, confidence, impact |
| Output | Agent/domain | Validator/human/executor | Schema, quality, safety, version |
| Approval | Human/policy | Executor | Exact scope, expiry, audit |
| Outcome | Domain/Analytics/Treasury | Knowledge/owners | Correlation, attribution, reconciliation |

### Cross References

- Root AI philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 12.
- AI engineering philosophy: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 14.
- Agent ecosystem target: [`context/05_AGENTS.md`](./05_AGENTS.md).
- Knowledge target: [`context/06_KNOWLEDGE_ENGINE.md`](./06_KNOWLEDGE_ENGINE.md).
- Prompting target: [`context/18_PROMPTING.md`](./18_PROMPTING.md).
- Human interaction target: [`context/10_UI_UX.md`](./10_UI_UX.md).

### Dependencies

AI-native design depends on Identity, Security, Knowledge, Automation, domain contracts, model/provider abstraction, prompt versioning, evaluation data, observability, user interfaces, and approval workflows. Removing any one can turn an apparently intelligent feature into an ungoverned or unauditable one.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| AI-first becomes prompt-first | Missing permissions, state, evaluation, and recovery | Require the full AI feature contract |
| Model output is treated as truth | Unsafe decisions and poor explainability | Provenance, validation, confidence, human review |
| Context window becomes a data boundary | Sensitive or irrelevant data leaks into prompts | Scoped retrieval and classification |
| Automation hides human choice | Accountability disappears | Explicit approval and decision records |
| Learning changes behavior silently | Unreviewed policy drift | Promotion gates and versioned knowledge |
| Model changes break quality | Production regression | Offline evaluation, pilot, rollback |
| AI interface excludes humans | Operators cannot supervise or correct | Dual interface and accessible fallback |

### Anti-patterns

- Adding a chat box to a human-first workflow and calling it AI-native.
- Putting secrets or unrestricted database access into an agent tool.
- Letting a prompt decide whether an action is authorized.
- Using a model response as a financial or legal record without validation.
- Changing prompts in production without versioning or evaluation.
- Treating “the model knows” as a provenance statement.
- Building an autonomous path with no human explanation or recovery route.

### Best Practices

- Design from intent, context, action, outcome, and learning.
- Give each agent a role, owner, tool scope, evaluator, and escalation path.
- Keep human-facing explanations linked to machine traces.
- Test uncertainty, adversarial input, missing context, and provider failure.
- Make model and prompt behavior reproducible enough to investigate.
- Capture useful human corrections as structured learning data.

### Extension Points

- New agents and skills through role manifests and evaluations.
- New models through provider and routing contracts.
- New retrieval sources through provenance-aware adapters.
- New evaluators through quality and safety interfaces.
- New human interaction surfaces through canonical workflow APIs.
- New AI coding workflows through `.ai` context and development standards.

### AI Construction Notes

An AI coding agent should never begin with “what prompt should I write?” It should begin with “what decision or operation is being automated, what context is allowed, who owns it, what can go wrong, and how will the result be verified?” Prompt design follows that analysis.

### AI Memory Anchor

> **AI-native means context before action, policy before side effect, evidence with output, and learning after outcome.**

### Implementation Checklist

- [x] AI-native design is defined beyond prompt usage.
- [x] Human and agent dual interfaces are explicit.
- [x] Feature contract and readiness gate are documented.
- [x] Model/prompt change control is visualized.
- [x] KATA command-center visual asset metadata is included.
- [ ] Detailed agent manifests, evaluators, and prompt contracts are authored downstream.
- [ ] Runtime AI feature evaluation infrastructure is implemented later.

---

## 23. Future Platform Expansion — Plugins, Contracts, and Ecosystem Evolution

### Human Explanation

CAT must be extensible, but extension must not mean uncontrolled modification. The platform expands through **contracts**: stable descriptions of what a module can do, which data it reads, which permissions it needs, which events it consumes or emits, how it fails, who owns it, and how it can be disabled or replaced.

The principal extension classes are:

- **Connector:** translates an external provider into an existing CAT domain contract.
- **Workflow template:** composes existing capabilities into a repeatable operating sequence.
- **Agent skill:** adds a bounded capability to an existing agent or introduces a new specialized agent.
- **Content or prompt pack:** adds governed templates, evaluation cases, or domain language.
- **Policy pack:** adds stricter organization or industry rules without weakening CAT safeguards.
- **Report or projection pack:** adds read models and analysis without becoming a source of truth.
- **Plugin:** a packaged, permission-scoped extension that can be installed, evaluated, updated, disabled, or revoked.
- **Future product module:** a capability that deserves separate Omni System ownership and a stable cross-product contract.

The plugin philosophy is conservative by design. A plugin may extend CAT only through an approved seam. It may not bypass Identity, Security, approval, audit, Treasury integrity, knowledge provenance, or domain ownership.

### AI Context

An AI agent proposing an extension must classify it before coding:

```text
Extension type:
Consumer(s):
Owning domain:
Permissions:
Data read:
Data write:
Events consumed:
Events emitted:
External side effects:
Approval requirement:
Compatibility target:
Failure and disable path:
Owner and support posture:
```

The agent must also state whether the extension is Official, Recommended, Experimental, or Future. “Plugin” is a packaging category, not a trust level.

### Business Perspective

Extensions let CAT cover diverse affiliate networks, markets, channels, enterprise policies, and operating styles without putting every specialized requirement into the Core roadmap. They create a way for partners and contributors to add value while giving Omni System a mechanism to protect users and brand trust.

The long-term ecosystem strategy has three concentric rings:

1. **Core ring:** identity, governance, lifecycle, canonical domain rules, knowledge principles, and essential Commerce/AI/Treasury capabilities.
2. **Trusted extension ring:** first-party and verified partner connectors, agents, workflows, policy packs, and reports.
3. **Community/private ring:** organization-specific and experimental extensions with narrower permissions and stronger isolation.

### Technical Perspective

An extension contract should include:

- manifest identity and owner;
- compatible CAT version and contract versions;
- capabilities and permissions;
- data classifications and retention;
- commands, events, queries, and tools;
- required configuration and secrets;
- human approval requirements;
- resource and rate limits;
- observability and health signals;
- evaluation and security evidence;
- upgrade, migration, disable, and revoke behavior;
- support and liability boundary.

A plugin installation is a state transition, not a file copy. CAT must inspect, authorize, install, validate, activate, monitor, update, disable, and remove it with traceability.

### Official Decisions

| ID | Official Decision | Consequence |
|---|---|---|
| P2-EXT-DEC-001 | CAT supports future expansion through governed contracts and extension points. | New modules must declare boundaries rather than modify Core invisibly. |
| P2-EXT-DEC-002 | Connectors isolate external provider behavior. | Provider API changes, credentials, and rate limits remain adapter concerns. |
| P2-EXT-DEC-003 | Plugins declare permissions, ownership, compatibility, and lifecycle. | CAT can evaluate and revoke extensions. |
| P2-EXT-DEC-004 | High-risk extension permissions require elevated review and approval. | Publishing, financial, credential, identity, and autonomy-expanding permissions are not ordinary capabilities. |
| P2-EXT-DEC-005 | Enterprise customizations should use private extensions and policies rather than Core forks. | The platform preserves upgradeability and shared safety fixes. |
| P2-EXT-DEC-006 | Public marketplace distribution is a future capability, not a current Phase A implementation claim. | Documentation and contracts may prepare for it without pretending it exists. |

### Recommendations

- Start with private/internal extensions before opening public distribution.
- Keep early extension permissions narrow, read-heavy, observable, and revocable.
- Require signed or otherwise integrity-protected packages when a distribution mechanism is implemented.
- Use semantic compatibility and migration documentation.
- Score trust using evidence, not popularity alone.
- Provide a clear owner and support status for every extension.
- Make extension health and deprecation visible to users and AI agents.

### Experimental Ideas

- Capability-based plugin sandboxing with simulated external effects.
- Automated extension review that combines static analysis, contract tests, permission inspection, and agent evaluation.
- A trust score based on security findings, uptime, compatibility, outcome quality, review history, and maintainer responsiveness.
- A plugin composition graph that warns when two extensions create conflicting policy or data behavior.
- AI-generated migration plans for extension version changes, subject to human review.

### Future Ideas

- Official marketplace with first-party, verified partner, community-reviewed, private, and experimental catalogs.
- Enterprise private marketplace and approval workflow.
- Revenue sharing and partner economics when platform governance and support are mature.
- Omni System-wide extension registry.
- Cross-product agent and knowledge packages with explicit data boundaries.
- Regional extension catalogs for market-specific programs and compliance.

### Visual Overview

**Diagram ID:** P2-EXT-001<br>
**Diagram Title:** Governed Extension Lifecycle<br>
**Purpose:** Show how a plugin or module moves from proposal through validation, installation, operation, update, and removal.

```mermaid
flowchart LR
    Proposal[Extension proposal] --> Manifest[Manifest and contract]
    Manifest --> Build[Build and package]
    Build --> Test[Compatibility, security, and evaluation]
    Test --> Review[Human/owner review]
    Review --> Catalog[Approved private or public catalog]
    Catalog --> Install[Scoped installation]
    Install --> Verify[Permission and health verification]
    Verify --> Activate[Activate under policy]
    Activate --> Observe[Monitor outcomes and risk]
    Observe --> Update[Versioned update]
    Update --> Test
    Observe --> Disable[Disable or quarantine]
    Disable --> Recover[Restore, migrate, or remove]
    Recover --> Archive[Archive evidence and ownership]
```

**Diagram ID:** P2-EXT-002<br>
**Diagram Title:** Extension Trust Rings<br>
**Purpose:** Show how trust level and capability scope should become stricter as an extension gains access or distribution.

```mermaid
graph TB
    Core[CAT Core<br/>highest trust, strongest ownership]
    FirstParty[First-party extensions]
    Verified[Verified partners]
    Community[Community-reviewed]
    Private[Private/local enterprise]
    Experimental[Sandbox experiments]

    Core --> FirstParty --> Verified --> Community
    Core --> Private
    Core --> Experimental

    Permissions[Permission scope and review depth]
    Permissions --> Core
    Permissions --> FirstParty
    Permissions --> Verified
    Permissions --> Community
    Permissions --> Private
    Permissions --> Experimental
```

**Diagram ID:** P2-EXT-003<br>
**Diagram Title:** Extension Contract Surface<br>
**Purpose:** Identify the metadata and interfaces an extension must expose before CAT can safely activate it.

```mermaid
graph TD
    Extension[Extension package]
    Extension --> Identity[Identity, owner, version]
    Extension --> Compat[Compatibility and migrations]
    Extension --> Permissions[Capabilities and permissions]
    Extension --> Data[Data classifications and retention]
    Extension --> Contracts[Commands, events, queries, tools]
    Extension --> Config[Configuration and secrets]
    Extension --> Approval[Approval and autonomy needs]
    Extension --> Health[Health, metrics, traces, alerts]
    Extension --> Security[Security and evaluation evidence]
    Extension --> Lifecycle[Disable, revoke, update, remove]
```

### Visual Asset Placeholder — CAT Extension Marketplace and Private Catalog

| Field | Value |
|---|---|
| **Visual Asset Name** | CAT Extension Marketplace and Private Catalog |
| **Purpose** | Provide a future discovery, review, installation, permission, compatibility, trust, update, and revocation experience for plugins and ecosystem assets. |
| **Recommended Resolution** | 3840 × 2160 px marketplace overview; 1920 × 1080 px extension detail; 1440 × 900 px enterprise approval view |
| **Suggested Location inside `/design`** | `/design/Marketplace/ExtensionCatalog/` |
| **Mood** | Curated, trustworthy, expansive without feeling uncontrolled |
| **Style** | Dark glass catalog, clear trust badges, permission disclosures, compatibility matrix, evidence panels, accessible filters |
| **Reference Category** | Extension marketplace / enterprise private catalog / package governance |

### Plugin/Extension Subsystem Contract

- **Purpose:** Discover, validate, install, operate, update, disable, revoke, and remove governed extensions.
- **Owner:** Platform ecosystem/marketplace owner, with Security, Architecture, and domain owners approving relevant permissions.
- **Responsibilities:** Define manifests; validate packages; assess trust; enforce permissions; manage compatibility; isolate execution; monitor health; record ownership; support rollback and revocation.
- **Inputs:** Extension package; manifest; signatures/integrity evidence; compatibility target; permissions; configuration; review decisions; telemetry.
- **Outputs:** Catalog listing; installation record; activation state; permission decision; health state; usage/outcome events; deprecation/revocation notices.
- **Dependencies:** Identity; Security; Administration; Settings; Foundation; domain contracts; evaluation; observability; package storage.
- **Extension Points:** Connector SDK; agent skill SDK; workflow template SDK; policy pack SDK; report/projection SDK; private catalog.
- **Failure Modes:** Malicious package; incompatible version; excessive permission; runtime crash; data leak; provider failure; abandoned maintenance; conflicting extension behavior.
- **Recovery Strategy:** Prevent activation; sandbox; disable; revoke; roll back version; isolate data; restore prior configuration; notify owners; preserve forensic and review evidence.
- **Future Evolution:** Marketplace economics, partner certification, cross-product registries, and federated extension governance.

### Extension Contract Example

A conceptual manifest should answer the following without reading implementation source:

```yaml
name: example-affiliate-connector
kind: connector
owner: verified-partner-or-team
version: 1.0.0
compatible_cat_contracts:
  - affiliate.link.v1
permissions:
  - affiliate.program.read
  - affiliate.link.prepare
  - network.example.request
data_access:
  reads:
    - product.public
    - campaign.scoped
  writes:
    - affiliate.link.draft
external_side_effects: false
approval_required: true
emits:
  - affiliate.link.validated
health:
  checks:
    - provider_auth
    - rate_limit
    - schema_compatibility
lifecycle:
  disable_supported: true
  rollback_supported: true
  owner_contact_required: true
```

This is a documentation example of the information contract. It is not a claim that this manifest format has been implemented.

### Mermaid Diagram(s)

**Diagram ID:** P2-EXT-004<br>
**Diagram Title:** Extension Permission Decision Tree<br>
**Purpose:** Show how CAT should classify extension access before installation or activation.

```mermaid
flowchart TD
    Extension[Extension requests capability] --> Data{What data does it access?}
    Data --> Public[Public/low sensitivity]
    Data --> Private[Private or organization data]
    Data --> Sensitive[Financial, identity, credential, or restricted data]
    Public --> Effect{External side effect?}
    Private --> Effect
    Sensitive --> HighReview[Elevated Security and owner review]
    Effect -->|No| Scoped[Scoped permission and compatibility review]
    Effect -->|Yes| RiskReview[Approval, sandbox, rate, and rollback review]
    HighReview --> RiskReview
    Scoped --> Activate[Activate if evidence passes]
    RiskReview --> Activate
    Activate --> Monitor[Continuous monitoring and revocation path]
```

**Diagram ID:** P2-EXT-005<br>
**Diagram Title:** Future Ecosystem Evolution Timeline<br>
**Purpose:** Sequence platform expansion so that governance and stable Core capabilities mature before public ecosystem scale.

```mermaid
timeline
    title CAT extension and ecosystem evolution
    Phase A : Document Core identity, domains, contracts, and governance
    Core implementation : Validate internal modules, agents, workflows, and integrations
    Private extension phase : Add internal and enterprise-private connectors and templates
    Verified partner phase : Certify selected extensions with permissions and support
    Marketplace phase : Curated distribution, trust, compatibility, and revocation
    Omni platform phase : Extract stable primitives for future Omni System products
```

### Architecture Map

#### Extension Placement Matrix

| Need | Extension type | Core change allowed? | Minimum governance |
|---|---|---:|---|
| Adapt a provider to existing Affiliate contract | Connector | No by default | Credential, reliability, data, and compatibility review |
| Reuse existing tasks in a new sequence | Workflow template | No | Approval, state, and outcome review |
| Add a bounded reasoning or execution capability | Agent skill | No by default | Role, tools, evaluation, autonomy, Security review |
| Add organization-specific restriction | Policy pack | No weakening of CAT safeguards | Administration, Security, owner approval |
| Add specialized read model | Report/projection pack | No canonical writes | Data access, freshness, and metric review |
| Change Core identity, approval model, or canonical meaning | Core module | Yes only by explicit decision | Architecture, product, Security, and affected domain review |

#### Extension Owner Matrix

| Lifecycle event | Owner | Required participants |
|---|---|---|
| Proposal | Author | Domain/architecture owner |
| Contract review | Contract owner | Security, affected domains |
| Package validation | Platform/marketplace | Automated checks, author |
| Permission approval | Security and resource owner | Enterprise/admin if local |
| Activation | Deployment/administration | Owner and support |
| Health monitoring | Extension owner | Platform observability |
| Incident response | Extension owner | Security, marketplace, affected domain |
| Deprecation/removal | Platform and extension owner | Users, enterprises, consumers |

### Cross References

- Root plugin and marketplace vision: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 15.8–15.10.
- API-first ecosystem: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 15.10.
- Third-party integrations: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Section 15.11.
- Enterprise and developer ecosystems: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md), Sections 15.13–15.14.
- Security and deployment context targets: [`context/17_SECURITY.md`](./17_SECURITY.md), [`context/16_DEPLOYMENT.md`](./16_DEPLOYMENT.md).

### Dependencies

Future expansion depends on stable Core contracts, identity and permissions, security review, package integrity, compatibility testing, ownership, support, observability, and a reliable disable/revoke path. Public marketplace economics additionally depend on legal, support, trust, and operational maturity.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Extension permission too broad | Data leak, financial loss, reputation damage | Least privilege and elevated review |
| Plugin version incompatible | Workflow failure or corrupted projections | Compatibility metadata and migration tests |
| Abandoned extension | Security and reliability debt | Owner/contact requirement, health state, deprecation |
| Extension conflicts with policy | Governance bypass | Policy evaluation at activation and execution |
| Marketplace trust is opaque | Users install unsafe assets | Evidence, permission disclosure, trust levels, revocation |
| Enterprise forks Core | Fragmented upgrades and security | Private extensions and policy surfaces |

### Anti-patterns

- Treating a package registry as a security review.
- Installing a plugin because it has a good description or rating.
- Allowing an extension to access raw Treasury or Identity data without purpose and scope.
- Letting a plugin override approval or audit behavior.
- Publishing experimental agents as stable production assets.
- Removing an extension without preserving outcome, decision, and incident history.

### Best Practices

- Make installation and activation explicit lifecycle states.
- Review permissions before code quality alone.
- Require extensions to be observable and removable.
- Use private catalogs for enterprise-specific assets.
- Publish compatibility and support posture.
- Track outcomes and security findings over the full extension lifecycle.
- Preserve a human owner even when an AI agent maintains the extension.

### Extension Points

This section itself defines the primary future extension surfaces:

- connector contracts;
- workflow templates;
- agent skills;
- policy packs;
- report/projection packs;
- content and prompt packs;
- private enterprise catalogs;
- future Omni System shared registries.

### AI Construction Notes

An AI agent must not generate an extension that requests broad permissions “for flexibility.” It should design the narrowest permission set, propose a manifest, add contract and failure tests, document the owner and removal path, and label any unverified behavior as experimental.

### AI Memory Anchor

> **Every extension must declare what it is, what it can touch, who owns it, how it fails, and how CAT can stop it.**

### Implementation Checklist

- [x] Future platform expansion is defined through contracts.
- [x] Connector, workflow, agent, policy, report, and plugin classes are distinguished.
- [x] Extension lifecycle, trust rings, contract surface, and permission decisions are visualized.
- [x] Plugin subsystem purpose, owner, responsibilities, inputs, outputs, dependencies, extension points, failures, recovery, and future evolution are documented.
- [x] Marketplace/private catalog visual asset metadata is included.
- [x] Public marketplace is clearly classified as future, not implemented.
- [ ] Extension SDKs, registries, package formats, and runtime sandboxing are defined in later architecture work.

---

## 24. Part 2 Mental Model and Implementation-Readiness Contract

### Human Explanation

A reader who has completed Part 2 should be able to describe CAT internally without opening the Architecture or Tech Stack documents:

- CAT is a platform organized into seven conceptual planes.
- Twelve domains own distinct meanings and collaborate through contracts.
- Commerce, AI, and Treasury are value stacks connected by identity, workflow, knowledge, and governance.
- KATA is the human-facing boundary; CATA coordinates internal work; domain components execute responsibilities.
- Humans, AI agents, developers, enterprises, and other user types have different responsibilities and scopes.
- Components communicate through commands, events, queries, retrieval, approvals, and adapter contracts.
- AI-native design requires context, policy, evaluation, explanation, and learning—not only prompts.
- Future expansion uses governed plugins, connectors, workflows, policies, reports, and stable extension contracts.

This is a mental model, not a replacement for implementation specifications. The next documents must make these boundaries concrete with schemas, APIs, deployment choices, coding standards, security controls, agent manifests, and tests.

### AI Context

Part 2 is sufficient to route most project-level tasks to the correct detailed context document. It is not sufficient to implement a subsystem without loading that document. An AI agent must treat the Part 2 checklists and memory anchors as routing and review aids, not as hidden runtime specifications.

### Business Perspective

The platform model gives CAT a path from a documented foundation to a durable ecosystem. It protects the flagship product from both extremes:

- a closed tool that cannot grow; and
- an ungoverned platform that cannot be trusted.

The intended path is controlled expansion: prove value, stabilize boundaries, expose contracts, evaluate extensions, and let knowledge and outcomes compound.

### Technical Perspective

Part 2 defines architectural prerequisites:

- plane and domain ownership;
- canonical versus derived data;
- component communication semantics;
- user and service-principal authorization;
- workflow state and failure recovery;
- knowledge and Treasury integration;
- AI feature contracts;
- extension lifecycle and revocation.

A detailed design that omits one of these prerequisites is incomplete even if its code path works in a happy-path demo.

### Visual Overview

**Diagram ID:** P2-CLOSE-001<br>
**Diagram Title:** CAT Part 2 Mental Model<br>
**Purpose:** Provide one final conceptual map from platform organization to domains, actors, contracts, and future extension without replacing the detailed architecture documents.

```mermaid
flowchart TB
    Platform[CAT governed platform]
    Platform --> Planes[Seven conceptual planes]
    Planes --> Domains[Twelve meaning-owning domains]
    Domains --> Trinity[Commerce, AI, Treasury value stacks]
    Actors[Humans, agents, developers, enterprises] --> Identity[Identity and policy]
    Identity --> Contracts[Commands, events, queries, retrieval, approvals]
    Contracts --> Trinity
    Trinity --> Outcomes[Commerce, financial, operational, and learning outcomes]
    Outcomes --> Knowledge[Knowledge, memory, decisions, and evidence]
    Knowledge --> Platform
    Contracts --> Extensions[Governed connectors, workflows, skills, policies, plugins]
    Extensions --> Platform
```

### Mermaid Diagram(s)

**Diagram ID:** P2-CLOSE-002<br>
**Diagram Title:** Part 2 Task Routing Map<br>
**Purpose:** Help a human or AI contributor route a future implementation task to the correct conceptual owner before loading detailed documents.

```mermaid
flowchart LR
    Task[New task] --> Classify{What changes?}
    Classify -->|Human interaction or explanation| Experience[Experience/KATA]
    Classify -->|Identity, access, policy, admin| Governance[Identity/Governance/Admin]
    Classify -->|Task state, scheduling, coordination| Automation[Automation/CATA]
    Classify -->|Market, affiliate, content, marketing| Commerce[Commerce domains]
    Classify -->|Memory, retrieval, model, learning| Intelligence[Knowledge/AI]
    Classify -->|Metrics, earnings, payouts, budget| Measure[Analytics/Treasury]
    Classify -->|Events, storage, secrets, runtime| Foundation[Foundation/Integration]
    Experience --> Load[Load owner context and contract]
    Governance --> Load
    Automation --> Load
    Commerce --> Load
    Intelligence --> Load
    Measure --> Load
    Foundation --> Load
    Load --> Review[Check policy, dependencies, failure, recovery, and status]
```

### Official Decisions

| ID | Part 2 decision area | Decision |
|---|---|---|
| P2-CLOSE-DEC-001 | Platform structure | CAT uses seven conceptual planes to orient internal organization. |
| P2-CLOSE-DEC-002 | Domain structure | Twelve domains own distinct meanings; specialized Commerce domains remain related but not merged. |
| P2-CLOSE-DEC-003 | Communication | Commands, events, queries, retrieval, approvals, and adapters have distinct semantics. |
| P2-CLOSE-DEC-004 | User authority | Human roles, AI service principals, developers, and enterprises have distinct scopes and responsibilities. |
| P2-CLOSE-DEC-005 | Platform strategy | Core identity and governance remain controlled while stable contracts enable extension. |
| P2-CLOSE-DEC-006 | AI-native design | Every feature requires context, policy, evaluation, human explanation, outcome, and learning consideration. |
| P2-CLOSE-DEC-007 | Future expansion | Public marketplace and broad Omni federation remain future capabilities, not current implementation claims. |

### Recommendations

- Use this part as the conceptual index before reading detailed architecture or stack documents.
- Turn each subsystem contract into a detailed specification before implementation.
- Resolve ownership conflicts through decisions rather than code conventions.
- Keep diagrams and tables synchronized with the written model.
- Use the memory anchors in agent bootstrap and review prompts.

### Experimental Ideas

- Generate an AI-readable platform manifest from the plane, domain, user, and extension tables.
- Build a documentation graph that checks whether every component has an owner, contract, and recovery path.
- Create synthetic end-to-end simulations from the diagrams before production runtime exists.

### Future Ideas

- Part 3 can expand ecosystem and platform relationships, implementation scenarios, and long-term evolution.
- Part 4 can integrate formal decisions, readiness criteria, and final overview closure.
- Later context documents can promote the conceptual contracts here into schemas and APIs.

### Risks

| Risk | Consequence | Control |
|---|---|---|
| Part 2 is treated as implementation detail | Future changes bypass detailed context and testing | Keep conceptual and concrete documents linked but distinct |
| Architecture and Overview diverge | Onboarding produces conflicting mental models | Review cross-document changes and authority |
| Domain contracts remain prose only | Code reintroduces accidental coupling | Convert contracts into schemas, tests, and ownership metadata |
| Visual concepts become accessibility or performance failures | Product trust and usability suffer | Validate visual assets against UI, performance, and accessibility standards |
| Status claims outrun evidence | Stakeholders misunderstand maturity | Update status only with documented evidence |

### Anti-patterns

- Starting Tech Stack decisions before the platform boundaries are understood.
- Reading a diagram as a literal deployment topology without checking Architecture.
- Treating a user-type matrix as a substitute for runtime authorization.
- Treating future plugins or marketplace concepts as current product commitments.
- Implementing a subsystem without filling its required contract fields.

### Best Practices

- Load Part 1 and Part 2 before detailed implementation planning.
- Keep product meaning, domain ownership, and technical placement separate.
- Preserve every boundary in code, tests, documentation, and observability.
- Use the diagrams as review aids and update them when relationships change.
- Capture implementation evidence in later context and status updates.

### Extension Points

- Future parts can extend the platform mental model without rewriting Part 1.
- Detailed documents can link each contract field to a schema, API, test, and owner.
- Architecture maps can be generated or validated from implementation metadata.
- The AI workspace can use the memory anchors as retrieval keys and task-routing hints.

### Cross References

- Part 1 completion contract: [Section 14](#14-part-1-completion-contract).
- Root project context: [`context/00_PROJECT_CONTEXT.md`](./00_PROJECT_CONTEXT.md).
- Next context documents: [`context/02_PROJECT_RULES.md`](./02_PROJECT_RULES.md), [`context/03_TECH_STACK.md`](./03_TECH_STACK.md), [`context/04_ARCHITECTURE.md`](./04_ARCHITECTURE.md), [`context/05_AGENTS.md`](./05_AGENTS.md).
- Current project status: [`../.ai/PROJECT_STATUS.md`](../.ai/PROJECT_STATUS.md).

### Dependencies

Part 2 depends on Part 1's identity, lifecycle, capability, value, and boundary model. Later implementation documents depend on Part 2's planes, domains, component relationships, user authority, AI-native requirements, and extension rules.

### AI Construction Notes

Before beginning a later implementation task, an AI agent should be able to answer the following from Part 2:

```text
Which plane owns the change?
Which domain owns the meaning?
Which principal initiates it?
Which contract carries it?
Which policy authorizes it?
Which component executes it?
Which records prove what happened?
How does it fail and recover?
What knowledge and Treasury outcomes are created?
```

If the agent cannot answer these questions, it should load the relevant context or request clarification before changing code.

### AI Memory Anchor

> **CAT is a governed platform: seven planes organize execution, twelve domains own meaning, three pillars create value, and contracts connect people, agents, data, and outcomes.**

### Implementation Checklist

- [x] Part 2 explains CAT's internal platform organization before detailed Architecture or Tech Stack documents.
- [x] The seven-plane structure is defined and visualized.
- [x] The Commerce AI Trinity is expanded into internal stacks with ownership and communication.
- [x] All twelve requested domains are documented with engineering contracts.
- [x] High-level component relationships, dependency directions, context maps, and sequence flows are included.
- [x] All requested user types, responsibilities, capabilities, and permissions are documented.
- [x] Platform philosophy, AI-native design, plugin philosophy, and future expansion are included.
- [x] Visual Asset Placeholders are included for future UI, workflow, role, domain, command-center, and extension surfaces.
- [x] Official Decisions, Recommendations, Experimental Ideas, and Future Ideas are separated.
- [x] AI Memory Anchors are included for each major section.
- [x] Failure modes, recovery strategies, risks, anti-patterns, best practices, extension points, dependencies, cross references, and AI construction notes are included.
- [ ] Detailed runtime implementation remains the responsibility of downstream context and engineering phases.

---

*End of Part 2 of `context/01_PROJECT_OVERVIEW.md`. Part 3 continues the executive and engineering overview with deeper ecosystem relationships, operating scenarios, and platform evolution while preserving the internal organization defined here.*

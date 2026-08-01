# CAT — Commerce AI Trinity — Project Context

---

## 1. Document Metadata

| Field | Value |
|---|---|
| **Document ID** | CAT-CTX-00 |
| **Document Name** | Project Context |
| **Version** | 1.0.0 |
| **Status** | Active |
| **Project** | CAT (Commerce AI Trinity) |
| **Company** | Omni System |
| **Owner** | Lead Repository Architect, CAT Project |
| **Source of Truth** | This document is the single source of truth for the highest-level project context. All other context documents (01–19, 99) derive from and defer to this file. |
| **Authors** | CAT Documentation Engine; Lead Software Architect; Omni System |
| **Created** | 2026-08-01 |
| **Last Updated** | 2026-08-01 |
| **Related Documents** | `context/01_PROJECT_OVERVIEW.md`, `context/02_PROJECT_RULES.md`, `context/03_TECH_STACK.md`, `context/04_ARCHITECTURE.md`, `context/05_AGENTS.md`, `context/06_KNOWLEDGE_ENGINE.md`, `context/07_TREASURY_CORE.md`, `context/08_AFFILIATE_ENGINE.md`, `context/09_CONTENT_ENGINE.md`, `context/10_UI_UX.md`, `context/11_DESIGN_LANGUAGE.md`, `context/12_DECISIONS.md`, `context/13_TERMINOLOGY.md`, `context/14_CODING_STANDARD.md`, `context/15_DIRECTORY_STRUCTURE.md`, `context/16_DEPLOYMENT.md`, `context/17_SECURITY.md`, `context/18_PROMPTING.md`, `context/19_DEVELOPMENT_GUIDE.md`, `context/99_AI_BOOTSTRAP.md`, `bible/Volume01/V01_01_Vision.md`, `.ai/README.md` |
| **Dependencies** | None. This is the root context document. All other documents depend on this one. |

### Document Hierarchy

```
00_PROJECT_CONTEXT.md   ← You are here
│
├── 01_PROJECT_OVERVIEW.md
├── 02_PROJECT_RULES.md
├── 03_TECH_STACK.md
├── 04_ARCHITECTURE.md
├── 05_AGENTS.md
├── 06_KNOWLEDGE_ENGINE.md
├── 07_TREASURY_CORE.md
├── 08_AFFILIATE_ENGINE.md
├── 09_CONTENT_ENGINE.md
├── 10_UI_UX.md
├── 11_DESIGN_LANGUAGE.md
├── 12_DECISIONS.md
├── 13_TERMINOLOGY.md
├── 14_CODING_STANDARD.md
├── 15_DIRECTORY_STRUCTURE.md
├── 16_DEPLOYMENT.md
├── 17_SECURITY.md
├── 18_PROMPTING.md
├── 19_DEVELOPMENT_GUIDE.md
└── 99_AI_BOOTSTRAP.md
```

### Intended Audience

This document is written for:

| Audience | Why They Read This |
|---|---|
| Human developers | To understand what CAT is, why it exists, and the principles that govern every implementation decision. |
| Software architects | To align all architectural choices with the project's stated philosophy and constraints. |
| AI agents (Codex, Claude, Gemini, Kimi, Minimax) | To load project context before performing any task within the CAT repository. |
| Future CAT internal agents | To bootstrap their understanding of the system they operate within. |
| Project stakeholders | To verify that development aligns with the original vision and business goals. |
| New contributors | To onboard quickly without needing oral knowledge transfer. |

### Reading Order for AI Systems

Any AI system working on the CAT repository should read documents in the following order:

1. `.ai/README.md` — AI workspace entry point
2. `context/00_PROJECT_CONTEXT.md` — This document
3. `context/01_PROJECT_OVERVIEW.md` — Detailed project overview
4. `context/02_PROJECT_RULES.md` — Project-wide rules
5. `context/03_TECH_STACK.md` — Technology decisions
6. `context/04_ARCHITECTURE.md` — System architecture
7. Remaining context documents in numerical order
8. `context/99_AI_BOOTSTRAP.md` — AI-specific bootstrap

---

## 2. Purpose

### 2.1 What This Document Is

This document is the **highest-level source of truth** for the CAT (Commerce AI Trinity) project. It defines the project's existence, its vision, its mission, and the philosophies that govern every decision made within the project — from code architecture to UI design to business strategy.

Every other document in the CAT repository is subordinate to this one. If any document conflicts with the principles, vision, or mission stated here, this document prevails, and the conflicting document must be corrected.

### 2.2 What This Document Is Not

This document is **not**:

- A technical architecture specification (see `context/04_ARCHITECTURE.md`)
- A technology stack listing (see `context/03_TECH_STACK.md`)
- An agent design document (see `context/05_AGENTS.md`)
- A UI/UX specification (see `context/10_UI_UX.md`)
- A security policy (see `context/17_SECURITY.md`)
- A coding standard (see `context/14_CODING_STANDARD.md`)
- A deployment guide (see `context/16_DEPLOYMENT.md`)

This document establishes **why** and **what**, not **how**. Implementation details belong in their respective dedicated documents.

### 2.3 Document Lifecycle

This document is designed to be maintained for the next ten years. It follows these lifecycle rules:

| Phase | Description |
|---|---|
| **Creation** | Initial drafting during Phase 0 (Foundation). |
| **Active** | The document is the current source of truth. Changes require review. |
| **Superseded** | A newer version replaces this one. The old version is archived but not deleted. |
| **Deprecated** | The document is no longer relevant. It is retained for historical reference. |

### 2.4 Change Management

Changes to this document follow these rules:

1. Any change must be committed with a conventional commit message prefixed with `docs(context):`.
2. Material changes (new sections, altered philosophy, changed scope) require a version increment.
3. The `Last Updated` field must reflect the date of the last material change.
4. A change log entry must be added to `CHANGELOG.md` for material changes.
5. All downstream documents must be reviewed for consistency after a material change to this document.

---

## 3. Scope

### 3.1 In Scope

This document covers the following domains at the highest level:

| Domain | Coverage in This Document | Detailed Document |
|---|---|---|
| Project identity | What CAT is and what it is not | `01_PROJECT_OVERVIEW.md` |
| Vision | The long-term aspiration of the project | `bible/Volume01/V01_01_Vision.md` |
| Mission | The operational mandate of the project | `01_PROJECT_OVERVIEW.md` |
| Core philosophy | The fundamental principles guiding all decisions | `02_PROJECT_RULES.md` |
| Design philosophy | The principles governing visual and interaction design | `10_UI_UX.md`, `11_DESIGN_LANGUAGE.md` |
| Engineering philosophy | The principles governing software engineering | `14_CODING_STANDARD.md`, `04_ARCHITECTURE.md` |
| Business philosophy | The principles governing commercial operations | `07_TREASURY_CORE.md`, `08_AFFILIATE_ENGINE.md` |
| AI philosophy | The principles governing artificial intelligence within CAT | `05_AGENTS.md`, `06_KNOWLEDGE_ENGINE.md` |

### 3.2 Out of Scope

The following are explicitly out of scope for this document:

- Specific technology choices and version numbers
- Detailed architecture diagrams and data flow specifications
- Individual agent designs and capabilities
- Treasury algorithms and financial models
- Affiliate network integration details
- Content generation pipelines
- Security policies and procedures
- Deployment configurations
- Prompt engineering templates
- Coding style rules

### 3.3 Scope Boundaries

```mermaid
graph TD
    A[00_PROJECT_CONTEXT.md<br/>Why & What] --> B[01–09<br/>Domain Context]
    A --> C[10–11<br/>Design Context]
    A --> D[12–19<br/>Operational Context]
    A --> E[99<br/>AI Bootstrap]
    B --> F[Architecture, Tech Stack,<br/>Agents, Treasury, etc.]
    C --> G[UI/UX, Design Language]
    D --> H[Decisions, Terminology,<br/>Coding, Deployment, Security]
    E --> I[AI-specific bootstrap]
    
    style A fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style B fill:#7c3aed,color:#fff
    style C fill:#7c3aed,color:#fff
    style D fill:#7c3aed,color:#fff
    style E fill:#7c3aed,color:#fff
```

---

## 4. Background

### 4.1 The Problem Space

Affiliate commerce is a multi-billion-dollar industry that remains fundamentally manual, fragmented, and inefficient. The typical affiliate commerce lifecycle involves:

1. **Market research** — Identifying profitable niches, trending products, and competitive landscapes.
2. **Product discovery** — Finding products with affiliate programs that match market demand.
3. **Content creation** — Writing reviews, comparisons, tutorials, and promotional content.
4. **Content publishing** — Distributing content across multiple channels (blogs, social media, video platforms).
5. **Link management** — Creating, tracking, and maintaining affiliate links.
6. **Analytics** — Monitoring clicks, conversions, revenue, and performance metrics.
7. **Optimization** — Adjusting strategies based on performance data.
8. **Treasury management** — Tracking earnings, managing payouts, and financial reporting.

Each of these steps is typically performed by human operators using disconnected tools. The result is:

| Problem | Impact |
|---|---|
| Manual research takes hours per product | Slow time-to-market |
| Content creation is a bottleneck | Limited scale |
| Multi-channel publishing is repetitive | Wasted effort |
| Link management is error-prone | Lost revenue |
| Analytics are fragmented across platforms | Incomplete picture |
| Optimization is reactive, not proactive | Suboptimal performance |
| Treasury tracking is manual | Financial blind spots |

### 4.2 The Market Opportunity

The affiliate marketing industry is characterized by:

- **Growing market size** — Global affiliate marketing spending continues to grow year over year.
- **Fragmented tooling** — No single platform covers the entire lifecycle.
- **AI gap** — Most existing tools use AI for narrow tasks (e.g., content writing) but not for end-to-end orchestration.
- **Human dependency** — Every step requires human intervention, limiting scalability.
- **Data silos** — Research, content, analytics, and financial data live in separate systems.

CAT is designed to address this gap by building an AI-native platform that automates the **entire** affiliate commerce lifecycle — not just one piece of it.

### 4.3 The Omni System Context

CAT is developed under **Omni System**, the company that owns and operates the project. Omni System's mandate is to build AI-native commerce systems that operate autonomously with human oversight. CAT is Omni System's flagship product.

Key Omni System decisions that shaped CAT:

| Decision | Rationale |
|---|---|
| AI-first, not AI-assisted | The system should be built around AI from the ground up, not have AI bolted on as a feature. |
| Human approval for critical actions | AI operates autonomously but human operators approve decisions with financial or reputational impact. |
| Modular agent ecosystem | The system is composed of specialized agents rather than a monolithic AI. |
| Knowledge-driven | The system accumulates knowledge over time and uses it to improve. |
| Security by design | Security is a foundational principle, not an afterthought. |
| Documentation first | The project is documented before it is implemented, ensuring alignment and longevity. |

### 4.4 Project Timeline

```mermaid
timeline
    title CAT Project Timeline
    Phase 0 : Foundation & Documentation
              Repository Structure
              CAT Bible
              Knowledge Base
              ADRs
              Design System
    Phase 1 : Core Platform
              Core Kernel
              Memory System
              Knowledge Graph
              Workflow Engine
              Agent Orchestrator
              Prompt Engine
    Phase 2 : AI Agent Ecosystem
              Research Agent
              Affiliate Agent
              Creative Agent
              Publisher Agent
              Treasury Agent
              Analytics Agent
              Learning Agent
              Security Agent
    Phase 3 : Commerce Platform
              Product Discovery
              Store Integrations
              Affiliate Link Management
              Content Generation
              Multi-Channel Publishing
    Phase 4 : Intelligence
              Continuous Learning
              Knowledge Refinement
              Performance Optimization
              Decision Support
    Phase 5 : Public Release
              Beta
              Production
              Enterprise Features
```

---

## 5. Why CAT Exists

### 5.1 The Core Problem

CAT exists because the affiliate commerce industry suffers from a fundamental mismatch:

> **The volume of products, markets, and content channels grows exponentially, but the human capacity to research, create, publish, and optimize grows linearly at best.**

This mismatch means:

- Human operators can only manage a fraction of available opportunities.
- Most affiliate content is low quality because quantity is prioritized over quality.
- Market opportunities are missed because humans cannot monitor all markets simultaneously.
- Financial optimization is reactive — humans respond to data after the fact, not proactively.

### 5.2 The CAT Solution

CAT solves this mismatch by replacing the human-in-the-loop model with a **human-on-the-loop** model:

| Traditional Model | CAT Model |
|---|---|
| Human researches products | AI researches products; human approves selection |
| Human writes content | AI generates content; human reviews and approves |
| Human publishes to channels | AI publishes; human monitors |
| Human tracks analytics | AI tracks and interprets analytics |
| Human optimizes strategy | AI recommends optimizations; human approves |
| Human manages treasury | AI tracks finances; human approves transactions |

The key insight is that CAT does not eliminate the human — it **elevates** the human from operator to **supervisor**. The human's role shifts from doing the work to approving the work.

### 5.3 Why "Trinity"

The name "Commerce AI Trinity" reflects the three foundational pillars of the system:

```
                    ┌─────────────────┐
                    │      CAT        │
                    │  Commerce AI    │
                    │    Trinity      │
                    └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              │              │              │
              ▼              ▼              ▼
     ┌────────────┐  ┌────────────┐  ┌────────────┐
     │  Commerce  │  │     AI     │  │  Treasury  │
     │  Engine    │  │   Engine   │  │   Engine   │
     │            │  │            │  │            │
     │ • Research │  │ • Agents   │  │ • Earnings │
     │ • Affiliate │  │ • Knowledge│  │ • Payouts  │
 │ • Content  │  │ • Memory   │  │ • Reports  │
     │ • Publish  │  │ • Reasoning│  │ • Budget   │
     └────────────┘  └────────────┘  └────────────┘
```

| Pillar | Responsibility | Key Agents |
|---|---|---|
| **Commerce Engine** | Market research, product discovery, affiliate link management, content generation, multi-channel publishing | Research Agent, Affiliate Agent, Creative Agent, Publisher Agent |
| **AI Engine** | Agent orchestration, knowledge management, memory, reasoning, continuous learning | CATA (central agent), Learning Agent, Memory Agent |
| **Treasury Engine** | Earnings tracking, payout management, financial reporting, budget optimization | Treasury Agent, Analytics Agent |

### 5.4 Why Now

Several converging trends make CAT viable at this point in time:

| Trend | Why It Matters for CAT |
|---|---|
| Multi-model AI orchestration | CAT can use different AI models for different tasks (research, writing, analysis) rather than relying on a single model. |
| Retrieval-augmented generation (RAG) | CAT can build a persistent knowledge base that improves over time. |
| Agent frameworks | Mature agent orchestration patterns enable CAT's multi-agent architecture. |
| Cloud-native infrastructure | Kubernetes, containers, and observability tools enable scalable deployment. |
| GPU acceleration | Real-time 3D rendering and particle effects for the CAT interface are feasible in the browser. |
| Affiliate market growth | The market is large enough to support a platform-level solution. |

### 5.5 What CAT Is Not

To prevent scope creep and maintain focus, the following are explicitly **not** what CAT is:

| Not CAT | Why |
|---|---|
| A chatbot | CAT is an autonomous operating system, not a conversational interface. |
| A CMS | CAT generates and publishes content but is not a content management system. |
| An ad network | CAT uses affiliate networks but is not itself an ad network. |
| A single-purpose AI tool | CAT is not "just" a content writer or "just" an analytics tool. It is an end-to-end platform. |
| A no-code platform | CAT is AI-native, not no-code. The AI does the work; humans supervise. |
| A human replacement | CAT augments human operators, it does not replace them. |

---

## 6. Vision (Expanded)

### 6.1 Vision Statement

> **CAT is an AI-native autonomous commerce operating system that researches products, creates content, manages affiliate operations, learns continuously, and assists human operators through an intelligent agent ecosystem — all within a command-center interface that feels like operating a living intelligence rather than using software.**

### 6.2 Vision Decomposition

The vision statement contains several load-bearing concepts that must be understood individually:

#### 6.2.1 AI-Native

"AI-native" means that AI is not a feature added to a traditional system. AI is the **foundation** upon which the entire system is built. Every component — from the kernel to the UI — is designed with the assumption that AI is the primary operator.

| AI-Assisted (Traditional) | AI-Native (CAT) |
|---|---|
| AI is a tool the user invokes | AI is the operator; the user supervises |
| The system waits for user input | The system proactively identifies tasks |
| AI features are optional | AI is required for the system to function |
| The UI is designed for human input | The UI is designed for human oversight of AI |
| Data flows through human workflows | Data flows through AI workflows with human checkpoints |

#### 6.2.2 Autonomous

"Autonomous" means the system operates without continuous human intervention. However, autonomy in CAT is **bounded**:

| Level | Description | Example |
|---|---|---|
| Level 0 | Human performs the task | Human writes a product review |
| Level 1 | AI suggests, human performs | AI suggests a product to review, human writes it |
| Level 2 | AI performs, human reviews | AI writes the review, human edits |
| Level 3 | AI performs, human approves | AI writes and publishes the review, human approves before it goes live |
| Level 4 | AI performs, human is notified | AI adjusts affiliate links based on performance, human is notified |
| Level 5 | AI performs autonomously | AI optimizes internal data routing without notification |

CAT targets **Level 3** for most operations, with **Level 4** for optimization tasks and **Level 5** for internal system operations. **Level 3** is the default for any action with external visibility or financial impact.

#### 6.2.3 Commerce Operating System

"Operating system" is used deliberately. CAT is not an application — it is a platform that manages resources, schedules tasks, coordinates agents, maintains memory, and provides an interface for human oversight. The analogy:

| Traditional OS | CAT |
|---|---|
| Manages CPU, memory, disk | Manages agents, knowledge, memory, treasury |
| Schedules processes | Schedules commerce tasks and agent workflows |
| Provides system calls | Provides agent APIs and commerce primitives |
| Has a shell | Has the CAT command-center interface |
| Manages permissions | Manages approval workflows |
| Logs system events | Logs all commerce and agent activities |

#### 6.2.4 Intelligent Agent Ecosystem

"Agent ecosystem" means CAT is not a single AI. It is a collection of specialized agents, each with a defined role, that collaborate through an orchestration layer.

```
┌─────────────────────────────────────────────────────────────┐
│                     CAT Agent Ecosystem                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐   │
│   │ Research  │  │ Affiliate│  │ Creative │  │ Publisher│   │
│   │  Agent    │  │  Agent   │  │  Agent   │  │  Agent   │   │
│   └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘   │
│        │              │              │              │        │
│        └──────────┬───┴──────────────┴──────────────┘       │
│                   │                                         │
│              ┌────┴────┐                                    │
│              │  CATA   │  ← Central Agent (Orchestrator)     │
│              └────┬────┘                                    │
│                   │                                         │
│        ┌──────────┼──────────────────────────┐              │
│        │          │              │            │              │
│   ┌────┴─────┐ ┌──┴───┐  ┌──────┴───┐ ┌─────┴────┐        │
│   │ Treasury │ │Memory│  │ Learning │ │ Security │        │
│   │  Agent   │ │Agent │  │  Agent   │ │  Agent   │        │
│   └──────────┘ └──────┘  └──────────┘ └──────────┘        │
│                                                             │
│   ┌──────────┐                                            │
│   │Analytics │                                            │
│   │  Agent   │                                            │
│   └──────────┘                                            │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

#### 6.2.5 Living Intelligence Interface

The CAT interface is not a dashboard. It is designed to feel like standing inside the command center of an advanced intelligence. The interface features:

- A **cosmic cat avatar** (the persona of CAT) that reacts to system state with body language, color changes, and particle effects.
- A **3D globe** at the center of the interface showing all merchant nodes, with light pulses representing transactions.
- **Glass morphism panels** with real reflection, refraction, and dispersion effects.
- **Perspective UI** where panels are angled 3–5 degrees toward the center to create a command-room feel.
- **AI thinking effects** where light strands flow between nodes when agents are processing.
- A **neon color language** with specific colors for each system state.

This is documented in detail in the CAT Visual Language specification and will be expanded in `context/10_UI_UX.md` and `context/11_DESIGN_LANGUAGE.md`.

### 6.3 Long-Term Vision (10-Year Horizon)

| Timeframe | Vision Milestone |
|---|---|
| Year 1–2 | Foundation, core platform, initial agent ecosystem |
| Year 3–4 | Full commerce platform, multi-channel publishing, treasury management |
| Year 5–6 | Continuous learning, knowledge refinement, performance optimization |
| Year 7–8 | Public release, enterprise features, multi-tenant support |
| Year 9–10 | Industry-standard autonomous commerce platform |

### 6.4 Success Criteria

The vision is achieved when:

1. CAT operates the full affiliate commerce lifecycle with minimal human intervention.
2. The knowledge base continuously improves, making CAT smarter over time.
3. The interface feels like commanding a living intelligence, not using software.
4. Treasury operations are fully tracked and optimized.
5. The system is secure, scalable, and maintainable over a decade.
6. Documentation is comprehensive enough that any AI or human can contribute.

---

## 7. Mission (Expanded)

### 7.1 Mission Statement

> **CAT's mission is to build and operate an AI-native platform that automates the complete affiliate commerce lifecycle — from market research and product discovery through content generation, publishing, analytics, continuous learning, and treasury management — while maintaining human approval for critical actions and accumulating knowledge that makes the system smarter over time.**

### 7.2 Mission Decomposition

#### 7.2.1 Automate the Complete Lifecycle

CAT's mission is not to automate *part* of the lifecycle. It is to automate the **complete** lifecycle. This means:

| Lifecycle Stage | CAT Responsibility |
|---|---|
| Market research | Identify trending products, profitable niches, and competitive landscapes |
| Product discovery | Find products with affiliate programs that match market demand |
| Content generation | Create reviews, comparisons, tutorials, and promotional content |
| Content publishing | Distribute content across blogs, social media, video platforms |
| Affiliate link management | Create, track, and maintain affiliate links |
| Analytics | Monitor clicks, conversions, revenue, and performance |
| Optimization | Adjust strategies based on performance data |
| Treasury management | Track earnings, manage payouts, generate financial reports |
| Continuous learning | Accumulate knowledge and improve over time |

If any stage is not automated, the mission is not complete.

#### 7.2.2 Human Approval for Critical Actions

Not all actions should be autonomous. CAT's mission explicitly includes maintaining human oversight for actions that have:

| Risk Type | Examples | Approval Required |
|---|---|---|
| Financial risk | Payouts, budget allocations, investments | Yes |
| Reputational risk | Publishing content, social media posts | Yes |
| Legal risk | Compliance-sensitive content, data handling | Yes |
| Irreversible risk | Deleting data, closing accounts | Yes |
| Operational risk | Internal optimization, data routing | No (notify only) |

#### 7.2.3 Accumulate Knowledge

A key part of CAT's mission is to **get smarter over time**. This is not a side effect — it is an explicit goal. The system must:

- Remember what worked and what did not.
- Learn from successful and failed campaigns.
- Build a knowledge graph of products, markets, and strategies.
- Use accumulated knowledge to make better decisions.
- Never lose knowledge (persistence is a requirement).

### 7.3 Mission vs. Vision

| Aspect | Vision | Mission |
|---|---|---|
| Timeframe | 10-year aspiration | Ongoing operational mandate |
| Focus | What CAT will become | What CAT does every day |
| Measurability | Qualitative | Quantitative |
| Scope | Aspirational | Operational |

---

## 8. Core Philosophy

### 8.1 Philosophical Foundation

CAT is built on six core principles. These principles are **non-negotiable** — they guide every decision, from architecture to UI to business strategy.

### 8.2 The Six Core Principles

```mermaid
mindmap
  root((CAT Core Philosophy))
    AI-First Architecture
      AI is the foundation
      Not a feature
      Every component designed for AI
    Human Approval for Critical Actions
      Human-on-the-loop
      Not human-in-the-loop
      Bounded autonomy
    Modular Agent Ecosystem
      Specialized agents
      Not monolithic AI
      Collaboration through orchestration
    Knowledge-Driven Continuous Improvement
      Persistent memory
      Knowledge graph
      Learning from experience
    Security by Design
      Foundational
      Not bolted on
      Defense in depth
    Scalable Cloud-Native Architecture
      Containerized
      Kubernetes-ready
      Observable
```

### 8.3 Principle 1: AI-First Architecture

**Statement:** CAT is designed around AI from the ground up. AI is not a feature added to a traditional system; it is the foundation upon which the system is built.

**Technical Implication:**
- The system architecture assumes AI agents are the primary operators.
- Data flows are designed for AI consumption, not human input.
- The API layer is designed for agent-to-agent communication, not just human-to-system.
- The UI is designed for oversight, not operation.

**Business Implication:**
- CAT can scale operations without scaling human headcount.
- The system becomes more valuable as it accumulates knowledge.
- The competitive moat is the AI infrastructure, not the UI.

**Anti-Pattern:**
- Building a traditional system and adding AI features on top.
- Designing the UI for human data entry and adding AI suggestions.
- Treating AI as an optional component.

**Decision Rationale:**
An AI-assisted system has a lower ceiling than an AI-native system because the human remains the bottleneck. By making AI the foundation, CAT removes the human bottleneck for execution while preserving human authority for approval.

### 8.4 Principle 2: Human Approval for Critical Actions

**Statement:** CAT operates autonomously for most tasks, but human operators approve decisions with financial, reputational, legal, or irreversible impact.

**Technical Implication:**
- Every action that has external impact must pass through an approval workflow.
- The system must support configurable approval levels.
- The UI must surface pending approvals prominently.
- The system must provide enough context for humans to make informed decisions.

**Business Implication:**
- The human's role is elevated from operator to supervisor.
- Risk is managed without sacrificing speed.
- The system can operate 24/7 with human oversight during business hours.

**Anti-Pattern:**
- Requiring approval for every action (creates a bottleneck).
- Allowing full autonomy for financial actions (creates risk).
- Burying approval requests in a queue (creates delay and risk).

**Decision Rationale:**
Full autonomy for critical actions creates unacceptable risk. Full human involvement for all actions negates the value of AI. The optimal balance is bounded autonomy with human approval at critical checkpoints.

### 8.5 Principle 3: Modular Agent Ecosystem

**Statement:** CAT is composed of specialized agents, each with a defined role, that collaborate through an orchestration layer. CAT is not a single, monolithic AI.

**Technical Implication:**
- Each agent has a well-defined interface and responsibility.
- Agents communicate through a structured event bus.
- Agents can be added, removed, or upgraded independently.
- The orchestrator (CATA) coordinates agent activities.

**Business Implication:**
- New capabilities can be added without rearchitecting the system.
- Agents can be specialized for specific markets or channels.
- The system can scale by adding more agents, not by making one agent bigger.

**Anti-Pattern:**
- A single AI that tries to do everything.
- Tightly coupled agents that cannot operate independently.
- Agents with overlapping responsibilities.

**Decision Rationale:**
A monolithic AI is harder to maintain, scale, and debug. A modular ecosystem allows specialization, independent development, and graceful degradation (if one agent fails, others continue).

### 8.6 Principle 4: Knowledge-Driven Continuous Improvement

**Statement:** CAT accumulates knowledge over time and uses that knowledge to make better decisions. The system gets smarter the longer it operates.

**Technical Implication:**
- The system has a persistent knowledge graph.
- The system has a memory store that survives restarts.
- The system logs outcomes (success/failure) and correlates them with decisions.
- The system uses RAG (Retrieval-Augmented Generation) to leverage accumulated knowledge.

**Business Implication:**
- The system's value compounds over time.
- Institutional knowledge is captured in the system, not in employees.
- The system can train new agents using accumulated knowledge.

**Anti-Pattern:**
- Treating each task as independent (no learning).
- Storing knowledge without using it for future decisions.
- Losing knowledge on system restart.

**Decision Rationale:**
A system that does not learn requires the same effort for every task. A system that learns reduces effort over time and improves quality. The compounding value of knowledge is the primary long-term competitive advantage.

### 8.7 Principle 5: Security by Design

**Statement:** Security is a foundational principle of CAT's architecture, not an afterthought or a layer added on top.

**Technical Implication:**
- All inter-agent communication is authenticated and authorized.
- All external API calls are validated and rate-limited.
- All data at rest is encrypted.
- All secrets are managed through a secret manager, never hardcoded.
- The system follows the principle of least privilege.

**Business Implication:**
- The system can be trusted with financial data.
- The system can be trusted with affiliate credentials.
- The system meets compliance requirements.

**Anti-Pattern:**
- Adding security after the system is built.
- Hardcoding secrets in configuration files.
- Trusting all internal communication by default.

**Decision Rationale:**
Security added after the fact is always incomplete and expensive. Security built into the foundation is more effective and cheaper to maintain. Given that CAT handles financial data and external credentials, security is existential.

### 8.8 Principle 6: Scalable Cloud-Native Architecture

**Statement:** CAT is built as a cloud-native system from the start, using containers, orchestration, and observability as foundational infrastructure.

**Technical Implication:**
- All components are containerized.
- The system is designed for Kubernetes deployment.
- Observability (metrics, logs, traces) is built in from the start.
- The system can scale horizontally by adding more instances.

**Business Implication:**
- The system can handle growth without rearchitecting.
- Infrastructure costs scale with usage.
- The system can be deployed on any cloud provider.

**Anti-Pattern:**
- Building for a single server and planning to scale later.
- Adding observability after the system is in production.
- Locking into a single cloud provider.

**Decision Rationale:**
Retrofitting cloud-native patterns onto a system not designed for them is expensive and risky. Building cloud-native from the start ensures the system can scale without architectural changes.

---

## 9. Design Philosophy

### 9.1 The CAT Interface Manifesto

> **CAT is not a dashboard. CAT is a living digital entity. The user should not feel like they are using software. They should feel like they are standing inside the command center of a super-advanced artificial intelligence.**

This is the foundational design principle for CAT's interface. Every visual and interaction decision must serve this principle.

### 9.2 Design Inspirations

CAT's visual language draws inspiration from:

| Inspiration | What CAT Borrows |
|---|---|
| Cyberpunk 2077 | Neon color language, holographic effects, dense information display |
| Tron Legacy | Light trails, geometric patterns, dark background with luminous elements |
| Iron Man HUD | Heads-up display, real-time data overlay, gesture-driven interaction |
| Jarvis | Conversational AI persona, anticipatory interface, ambient intelligence |
| Interstellar | Cosmic aesthetic, scientific precision, minimal but deep UI |
| Mass Effect | Sci-fi command interface, species-level civilization management feel |
| The Creator | AI presence, organic-digital fusion, atmospheric depth |
| Ghost in the Shell | Cybernetic identity, information density, philosophical depth |
| Deus Ex | Augmented interface, layered information, conspiracy-grade detail |
| Apple Vision Pro | Spatial computing, glass interfaces, depth and parallax |

### 9.3 The CAT Persona

CAT is not just an AI — CAT has a **personality**. The visual representation of CAT is a **cosmic cat** — not an animal, not a human, but a being between the two.

| Aspect | Specification |
|---|---|
| Body | Made of a substance like stars, nebulae, light, energy, electrons. Inside the body, galaxies, stars, particles, and light move. |
| Eyes | Blue light, animated iris, digital pupil, light trail on movement. |
| Ears | Natural movement, reactive to system events. |
| Tail | Slow, deliberate movement. |
| Breathing | Very slow, nearly imperceptible. |
| Body light | Changes based on system state (see color language below). |

### 9.4 Avatar Behavior Engine

The CAT avatar is not a static image — it is an actor. Its behavior reflects the system's state:

| System State | Avatar Behavior | Body Color |
|---|---|---|
| Normal | Calm, looking around, ears and tail moving | Blue |
| Thinking | Eyes closed, paws together, particles orbiting, tail still, brain light increasing | Purple |
| Generating | One paw moves as if creating something | Purple |
| Image creation | Paws arrange 3D shapes | Purple |
| Video creation | Video holograms appear around the avatar | Purple |
| Searching | Thousands of nodes appear around the head | Blue |
| Waiting | Blink, look around, ear and tail movement, breathing | Blue |
| Success | Subtle smile, golden light | Gold |
| Warning | Head slightly lowered | Orange |
| Error | Head lowered, orange/red light | Red |
| Learning | Turquoise light | Turquoise |
| Sleeping | Dim light | Dim blue |
| User login | Looks at user, head slightly tilted | Blue |
| User idle (few minutes) | Says "Ready when you are." | Blue |

### 9.5 Color Language

| Role | Color | Hex (Reference) |
|---|---|---|
| Primary | Electric Blue | `#00D4FF` |
| Secondary | Cosmic Purple | `#7C3AED` |
| Accent | Gold | `#FFD700` |
| Success | Emerald | `#10B981` |
| Warning | Amber | `#F59E0B` |
| Danger | Crimson | `#EF4444` |
| Learning | Turquoise | `#06B6D4` |

All colors are user-configurable through the theme engine. Users can create custom themes (e.g., Cyber Blue, Red Matrix, Emerald, Purple Galaxy, White Hologram, Amber).

### 9.5.1 Contrast and Readability

> **CRITICAL:** All text must maintain WCAG AA contrast ratios (4.5:1 for body text, 3:1 for large text) against their background at all times, including during state transitions. When the header or panel background changes color (e.g., from transparent to solid on scroll, or from blue to purple during a state change), text color must adjust automatically to maintain readability.

### 9.6 Interface Architecture

#### 9.6.1 The CAT Brain (Central Globe)

At the center of the interface is a fully 3D globe:

| Feature | Specification |
|---|---|
| Rendering | Fully 3D, real-time |
| Rotation | Slow, continuous |
| Surface | Clouds, sunlight, night lights, atmosphere |
| Particles | Particle effects on the globe surface |
| Merchant nodes | All merchants (Amazon, Impact, CJ, Digikala, Awin, AliExpress, ClickBank, ShareASale, etc.) displayed as nodes on the globe |
| Dynamic nodes | New merchants appear as new nodes automatically |
| Transaction visualization | When a sale occurs, a light pulse travels from the merchant node to the center |
| Agent activity | When an agent performs a task, light flows between Agent → Merchant → Campaign → Analytics |

#### 9.6.2 Panel System

| Property | Specification |
|---|---|
| Layout | Not flat — all panels angled 3–5 degrees toward center |
| Effect | Creates a command-room sensation |
| Layers | Background → Glass → Glow → Content → Particles → Reflection → Border |
| Glass | Real glass morphism (not simple blur): reflection, dispersion, refraction, micro noise, HDR glow |
| Materialization | Windows do not "open" — they "materialize" |
| Hover effects | Buttons gain inner light on hover |
| Click effects | Energy gathers into the button and releases on click |

#### 9.6.3 Motion System

| Property | Specification |
|---|---|
| Rendering | GPU-accelerated |
| Target FPS | 60 FPS (120 FPS on capable displays) |
| Blocking | No animation is blocking |
| Quality levels | Ultra, High, Eco |
| Background | Not pure black — subtle gradient toward blue with sparse stars, faint nebulae, light particles, slow movement |

### 9.7 Design Philosophy Principles

| Principle | Description |
|---|---|
| **Living, not static** | The interface is alive — it breathes, reacts, and responds. |
| **Depth, not flatness** | Panels have layers, depth, and perspective. |
| **Information density without clutter** | Show a lot of information, but organize it so it doesn't feel overwhelming. |
| **Sci-fi command center, not enterprise dashboard** | The aesthetic is futuristic and cinematic, not corporate. |
| **Performance is design** | Beautiful but slow is unacceptable. 60 FPS is a design requirement. |
| **Configurable beauty** | Users can customize themes, effects, and performance levels. |
| **State-aware** | The interface reflects the system's state through color, motion, and avatar behavior. |

### 9.8 Design Anti-Patterns

| Anti-Pattern | Why It's Wrong |
|---|---|
| Flat, corporate dashboard | Violates the "living intelligence" principle |
| Blocking animations | Violates the 60 FPS requirement |
| Pure black background | Violates the cosmic aesthetic |
| Static avatar | Violates the "CAT is alive" principle |
| Unconfigurable themes | Violates the user empowerment principle |
| Information overload without hierarchy | Violates the "density without clutter" principle |
| Low-contrast text on glowing backgrounds | Violates accessibility and readability requirements |

---

## 10. Engineering Philosophy

### 10.1 Engineering Principles

CAT follows the engineering principles defined in `CONTRIBUTING.md`:

| Principle | Description |
|---|---|
| Documentation First | The project is documented before it is implemented. |
| AI-First Engineering | Engineering decisions prioritize AI as the primary operator. |
| Human Approval for Critical Actions | Engineering must support configurable approval workflows. |
| Security by Design | Security is built in, not bolted on. |
| Modular Architecture | Components are independent, well-interfaced, and replaceable. |
| Continuous Learning | The system learns from every operation. |

### 10.2 Documentation First

"Documentation First" means:

1. Before any code is written, the relevant context document must be created or updated.
2. Architecture Decision Records (ADRs) must be created before architectural changes are implemented.
3. API documentation must exist before the API is consumed.
4. The documentation is the specification; code is the implementation.

**Rationale:** If the documentation is written after the code, it documents what was built. If it is written before, it specifies what should be built. The latter produces better designs because it forces thinking before doing.

### 10.3 Conventional Commits

CAT follows the Conventional Commits specification:

| Type | Use |
|---|---|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation changes |
| `style` | Code style changes (formatting, etc.) |
| `refactor` | Code changes that neither fix a bug nor add a feature |
| `test` | Adding or correcting tests |
| `chore` | Maintenance tasks, dependency updates, etc. |
| `perf` | Performance improvements |

Format: `type(scope): description`

### 10.4 Branch Strategy

| Branch | Purpose |
|---|---|
| `main` | Production-ready code |
| `develop` | Integration branch |
| `feature/*` | Feature development |
| `fix/*` | Bug fixes |
| `release/*` | Release preparation |
| `hotfix/*` | Urgent production fixes |

### 10.5 Definition of Done

A task is complete when:

1. Code is completed.
2. Tests pass.
3. Documentation is updated.
4. Code is reviewed.
5. The change is committed with a conventional commit message.

### 10.6 Engineering Trade-offs

| Decision | Chosen | Rejected | Rationale |
|---|---|---|---|
| Monolith vs. Microservices | Modular monolith (initially) | Microservices (initially) | Start simple, extract services when needed |
| SQL vs. NoSQL | SQL (Postgres/Supabase) | NoSQL | Structured data with relationships; ACID requirements for treasury |
| SSR vs. SPA | SSR (Next.js) | SPA | SEO for published content; fast initial load |
| REST vs. GraphQL | REST (primary) | GraphQL | Simpler, well-understood, sufficient for agent-to-agent communication |
| Sync vs. Async | Async (event bus) | Sync (blocking calls) | Agents operate independently; async prevents bottlenecks |

### 10.7 Engineering Anti-Patterns

| Anti-Pattern | Why It's Wrong |
|---|---|
| Writing code before documentation | Violates "Documentation First" |
| Hardcoding secrets | Violates "Security by Design" |
| Building a monolithic AI | Violates "Modular Agent Ecosystem" |
| Adding observability after deployment | Violates "Cloud-Native Architecture" |
| Skipping tests to save time | Violates "Definition of Done" |
| Non-conventional commits | Violates commit standards |

---

## 11. Business Philosophy

### 11.1 Business Model

CAT operates in the affiliate commerce industry. Its business model is:

1. **CAT operates affiliate campaigns** across multiple networks and merchants.
2. **CAT earns affiliate commissions** from successful conversions.
3. **CAT manages its own treasury** — tracking earnings, optimizing payouts, and reporting financial performance.
4. **CAT may offer enterprise features** in the future for other organizations to use the platform.

### 11.2 Revenue Streams

| Stream | Description | Phase |
|---|---|---|
| Affiliate commissions | CAT earns commissions from affiliate links it creates and publishes | Phase 3+ |
| Treasury optimization | CAT optimizes earnings through better link management and channel selection | Phase 3+ |
| Enterprise licensing | Other organizations license CAT to run their own affiliate operations | Phase 5+ |

### 11.3 Business Principles

| Principle | Description |
|---|---|
| **Revenue is a byproduct of quality** | If CAT produces high-quality content and targets the right products, revenue follows. Do not optimize for revenue directly. |
| **Long-term over short-term** | CAT prioritizes sustainable growth over quick wins. A campaign that generates $100/month for 5 years is better than one that generates $500 once. |
| **Diversification** | CAT operates across multiple merchants, networks, and channels to reduce dependency on any single source. |
| **Transparency** | All earnings, decisions, and actions are logged and auditable. |
| **Human oversight** | Financial decisions require human approval. CAT recommends; the human decides. |

### 11.4 Treasury Philosophy

The treasury is not just a ledger — it is a strategic tool. CAT's treasury management:

- Tracks all earnings in real-time.
- Correlates earnings with the content and campaigns that generated them.
- Identifies the most profitable products, channels, and strategies.
- Optimizes budget allocation based on performance.
- Maintains financial records for compliance and reporting.

Detailed treasury specifications will be in `context/07_TREASURY_CORE.md`.

### 11.5 Affiliate Philosophy

CAT's approach to affiliate marketing:

| Traditional Affiliate | CAT Affiliate |
|---|---|
| Human picks products | AI researches and recommends products |
| Human writes content | AI generates content; human approves |
| Human manages links | AI creates and manages links |
| Human tracks performance | AI tracks and interprets performance |
| Human optimizes strategy | AI recommends optimizations; human approves |
| Reactive | Proactive |
| Single channel | Multi-channel |
| Limited scale | Scalable |

Detailed affiliate specifications will be in `context/08_AFFILIATE_ENGINE.md`.

### 11.6 Business Anti-Patterns

| Anti-Pattern | Why It's Wrong |
|---|---|
| Optimizing for clicks over conversions | Clicks without conversions waste resources |
| Spamming affiliate links | Damages reputation and violates platform policies |
| Single-channel dependency | Creates fragility |
| Short-term thinking | Sacrifices sustainable growth |
| Opaque financial reporting | Violates transparency principle |
| Autonomous financial decisions | Violates human approval principle |

---

## 12. AI Philosophy

### 12.1 AI as Operator, Not Tool

In CAT, AI is the **operator** — the entity that performs the work. The human is the **supervisor** — the entity that approves the work. This is a fundamental distinction:

| AI as Tool | AI as Operator |
|---|---|
| Human invokes AI for a specific task | AI identifies tasks and performs them |
| AI output is a suggestion | AI output is the work product |
| Human does the work; AI assists | AI does the work; human approves |
| The system is human-driven | The system is AI-driven |

### 12.2 Multi-Model Orchestration

CAT does not rely on a single AI model. It orchestrates multiple models, each chosen for its strengths:

| Model Role | Why Multiple Models |
|---|---|
| Research | Different models have different knowledge cutoffs and web access capabilities |
| Content generation | Different models have different writing styles and strengths |
| Analysis | Different models have different reasoning capabilities |
| Image generation | Different models produce different aesthetic results |
| Code generation | Different models have different coding capabilities |

The orchestrator (CATA) routes tasks to the appropriate model based on the task type, required quality, and cost constraints.

### 12.3 Knowledge and Memory

CAT's AI is not stateless. It maintains:

| Store | Purpose | Persistence |
|---|---|---|
| Knowledge graph | Structured knowledge about products, markets, strategies | Permanent |
| Memory store | Context from past operations, decisions, and outcomes | Permanent |
| RAG index | Searchable index of all accumulated knowledge | Permanent |
| Session memory | Context within a single operational session | Session |

### 12.4 Bounded Autonomy

CAT's AI operates within defined boundaries:

```
┌─────────────────────────────────────────────────┐
│                 Autonomy Boundaries               │
├─────────────────────────────────────────────────┤
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Level 5: Full Autonomy                   │   │
│  │  Internal system operations, data routing │   │
│  │  No human notification required           │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Level 4: Autonomy with Notification      │   │
│  │  Optimization tasks, link adjustments    │   │
│  │  Human notified after the fact            │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Level 3: Autonomy with Approval (DEFAULT)│   │
│  │  Publishing content, creating campaigns   │   │
│  │  Human must approve before execution      │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Level 2: AI Drafts, Human Edits         │   │
│  │  Sensitive content, legal reviews         │   │
│  │  AI produces a draft for human editing    │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Level 1: AI Suggests, Human Performs     │   │
│  │  Strategic decisions, new market entry   │   │
│  │  AI provides analysis; human acts         │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
└─────────────────────────────────────────────────┘
```

### 12.5 AI Ethics

| Principle | Description |
|---|---|
| **Transparency** | All AI decisions are logged with reasoning. The human can always see why the AI made a decision. |
| **Accountability** | The human is ultimately accountable. AI recommends; human approves. |
| **No deception** | CAT never presents AI-generated content as human-created without disclosure where required. |
| **No manipulation** | CAT does not use psychological manipulation in content. Content is honest and informative. |
| **Privacy** | CAT does not collect or use personal data beyond what is necessary for operations. |
| **Bias awareness** | CAT actively monitors for and mitigates bias in its recommendations and content. |

### 12.6 AI Evolution

CAT's AI capabilities are expected to evolve over time:

| Phase | AI Capability |
|---|---|
| Phase 0–1 | Basic agent orchestration, rule-based workflows |
| Phase 2 | Multi-agent collaboration, RAG-based knowledge retrieval |
| Phase 3 | Autonomous content generation and publishing with approval |
| Phase 4 | Continuous learning, performance prediction, proactive optimization |
| Phase 5 | Advanced reasoning, strategic decision support, enterprise-grade intelligence |

### 12.7 AI Anti-Patterns

| Anti-Pattern | Why It's Wrong |
|---|---|
| Stateless AI (no memory) | Violates "Knowledge-Driven" principle |
| Single model for all tasks | Suboptimal; different models have different strengths |
| Unbounded autonomy | Violates "Human Approval" principle |
| Opaque decisions | Violates "Transparency" principle |
| AI that doesn't learn | Violates "Continuous Improvement" principle |
| AI that replaces human judgment for critical decisions | Violates "Human Approval" principle |

---

*End of Part 1. Architecture Foundation continues in Part 2 below.*

---

## 13. Architecture Foundation

### 13.1 CAT System Architecture Philosophy

CAT's architecture is not a conventional software architecture with AI features layered on top. It is an **AI-native architecture** — a system designed from the kernel outward with the assumption that AI agents are the primary operators and humans are the supervisors.

The foundational architectural philosophy can be stated as a single principle:

> **The system is an organism, not an application. It senses, thinks, decides, acts, and learns — continuously. The human does not drive the system; the human supervises it.**

This principle has profound architectural consequences. Every layer of the system must be designed to support autonomous operation, continuous learning, bounded human oversight, and graceful degradation. A traditional request-response architecture is insufficient because CAT is not a request-response system — it is a **living commerce organism** that operates continuously.

| Traditional Application Architecture | CAT AI-Native Architecture |
|---|---|
| Human initiates requests; system responds | System initiates actions; human approves |
| Stateless request handling | Stateful, persistent memory and knowledge |
| Synchronous request-response | Asynchronous event-driven workflows |
| Single application logic tier | Multi-agent orchestrated intelligence |
| Database as storage | Knowledge graph as cognitive substrate |
| UI as input form | UI as command center for oversight |
| Deployed as a single app | Deployed as an ecosystem of cooperating services |
| Scaling means more replicas | Scaling means more agents, more knowledge, more capability |

### 13.2 Why CAT Requires an AI-Native Architecture

An AI-assisted architecture — where AI is bolted onto a traditional system — has a hard ceiling. The human remains the bottleneck for every operation. The system cannot scale beyond human capacity because every action requires human initiation. The AI merely makes the human faster.

CAT's mandate is to automate the **complete** affiliate commerce lifecycle. This is impossible with an AI-assisted architecture because:

| Limitation of AI-Assisted | How AI-Native Solves It |
|---|---|
| Human must initiate every task | System identifies and initiates tasks autonomously |
| Human is the data entry point | Agents ingest and process data directly |
| Human context is lost between sessions | Persistent memory retains context across sessions |
| Human capacity limits scale | Agent capacity scales horizontally |
| Human knowledge leaves with the employee | Knowledge is captured in the knowledge graph |
| Human latency delays response | Agent latency is near-zero for routine decisions |
| Human inconsistency degrades quality | Agent consistency improves with learning |

The architectural decision to go AI-native was made in ADR-0001 and is non-negotiable. Any proposal to revert to an AI-assisted architecture must be rejected.

#### 13.2.1 Rejected Alternative: AI-Assisted Architecture

| Criterion | AI-Assisted | AI-Native (Chosen) |
|---|---|---|
| Scalability ceiling | Human capacity | Infrastructure capacity |
| Time-to-decision | Human latency (minutes to hours) | Agent latency (milliseconds to seconds) |
| Knowledge retention | Lost when employees leave | Permanent in knowledge graph |
| 24/7 operation | Impossible (humans sleep) | Native (agents run continuously) |
| Multi-market monitoring | Limited to human attention span | Unlimited (agents monitor all markets) |
| Consistency | Variable (human mood, fatigue) | Consistent (improves with training) |
| Cost scaling | Linear with headcount | Sublinear (agents scale cheaply) |

**Decision:** AI-native architecture. The scalability ceiling of AI-assisted is too low for CAT's mission.

### 13.3 Core System Layers

CAT's architecture is organized into six core layers, each with a distinct responsibility. The layers are not strict OSI-style isolation boundaries — agents and services communicate across layers through well-defined interfaces — but each layer has a clear purpose and evolution path.

```
┌─────────────────────────────────────────────────────────────────┐
│                     LAYER 6: HUMAN INTERACTION                   │
│   KATA Avatar · Command Center UI · Approval Workflows           │
│   Natural Language Interface · Theme Engine · Notifications      │
├─────────────────────────────────────────────────────────────────┤
│                     LAYER 5: AGENT ECOSYSTEM                      │
│   Research · Affiliate · Creative · Publisher · Treasury         │
│   Analytics · Learning · Memory · Security · CATA (Orchestrator) │
├─────────────────────────────────────────────────────────────────┤
│                     LAYER 4: ORCHESTRATION                        │
│   Agent Orchestrator · Workflow Engine · Task Scheduler          │
│   Approval Gateway · Agent Lifecycle Manager                     │
├─────────────────────────────────────────────────────────────────┤
│                     LAYER 3: COGNITIVE CORE                       │
│   Knowledge Graph · Memory Store · RAG Index · Reasoning Engine  │
│   Prompt Engine · Learning Pipeline · Decision Engine            │
├─────────────────────────────────────────────────────────────────┤
│                     LAYER 2: SERVICE LAYER                        │
│   Commerce Services · Treasury Services · Content Services       │
│   Analytics Services · Integration Services · Auth Services      │
├─────────────────────────────────────────────────────────────────┤
│                     LAYER 1: INFRASTRUCTURE                       │
│   Kernel · Event Bus · Database · Storage · Secrets              │
│   Observability · Container Runtime · Network · Security         │
└─────────────────────────────────────────────────────────────────┘
```

| Layer | Name | Responsibility | Key Components |
|---|---|---|---|
| 6 | Human Interaction | The interface between human supervisors and the CAT organism | KATA avatar, command center UI, approval workflows, natural language interface, theme engine |
| 5 | Agent Ecosystem | Specialized AI agents that perform commerce operations | Research, Affiliate, Creative, Publisher, Treasury, Analytics, Learning, Memory, Security agents; CATA orchestrator |
| 4 | Orchestration | Coordinates agent activities, manages workflows, enforces approval gates | Agent orchestrator, workflow engine, task scheduler, approval gateway, agent lifecycle manager |
| 3 | Cognitive Core | The brain of CAT — knowledge, memory, reasoning, learning | Knowledge graph, memory store, RAG index, reasoning engine, prompt engine, learning pipeline, decision engine |
| 2 | Service Layer | Domain-specific business logic and external integrations | Commerce services, treasury services, content services, analytics services, integration services, auth services |
| 1 | Infrastructure | Foundation — runtime, data, communication, security | Kernel, event bus, database, storage, secrets, observability, container runtime, network, security |

#### 13.3.1 Layer Interaction Rules

| Rule | Description |
|---|---|
| **Top-down dependency** | Higher layers depend on lower layers, never the reverse. Layer 5 depends on Layer 4, which depends on Layer 3, etc. |
| **No skip-layer calls** | Layer 6 cannot call Layer 1 directly. It must go through Layer 5 → Layer 4 → Layer 3 → Layer 2 → Layer 1. |
| **Horizontal communication** | Components within the same layer communicate through the layer's designated interface (e.g., agents communicate through the event bus). |
| **Cross-layer via interfaces** | Every layer exposes a well-defined interface. Cross-layer calls must go through these interfaces, not through internal implementation. |
| **Layer independence** | A layer can be replaced or upgraded without affecting layers above, as long as its interface contract is maintained. |

#### 13.3.2 Why Six Layers (Not More, Not Fewer)

| Alternative | Rejected Because |
|---|---|
| 3 layers (UI, Logic, Data) | Insufficient separation between AI cognition, agent orchestration, and business services. Conflates reasoning with execution. |
| 4 layers (UI, Agents, Logic, Data) | Missing the orchestration layer — agents would need to coordinate themselves, creating tight coupling. |
| 5 layers (UI, Agents, Orchestration, Logic, Data) | Missing the cognitive core as a distinct layer — knowledge and memory would be buried in the service layer, making them harder to evolve. |
| 7+ layers | Over-abstraction. Each layer adds interface overhead. Six layers provide clean separation without excessive indirection. |

**Decision:** Six layers. This provides clean separation between human interaction (6), agent execution (5), agent coordination (4), AI cognition (3), business logic (2), and infrastructure (1) without over-abstracting.

### 13.4 High-Level Architecture Map

```mermaid
graph TB
    subgraph "Layer 6: Human Interaction"
        KATA["KATA Avatar<br/>Natural Language Interface"]
        UI["Command Center UI<br/>3D Globe · Glass Panels"]
        APPROVAL["Approval Workflows<br/>Human-in-the-Loop Gate"]
        THEME["Theme Engine<br/>Customizable Aesthetics"]
    end

    subgraph "Layer 5: Agent Ecosystem"
        CATA["CATA<br/>Central Orchestrator Agent"]
        RESEARCH["Research Agent"]
        AFFILIATE["Affiliate Agent"]
        CREATIVE["Creative Agent"]
        PUBLISHER["Publisher Agent"]
        TREASURY["Treasury Agent"]
        ANALYTICS["Analytics Agent"]
        LEARNING["Learning Agent"]
        MEMORY["Memory Agent"]
        SECURITY["Security Agent"]
    end

    subgraph "Layer 4: Orchestration"
        ORCH["Agent Orchestrator"]
        WORKFLOW["Workflow Engine"]
        SCHEDULER["Task Scheduler"]
        APPROVAL_GATE["Approval Gateway"]
        LIFECYCLE["Agent Lifecycle Manager"]
    end

    subgraph "Layer 3: Cognitive Core"
        KG["Knowledge Graph"]
        MEM["Memory Store"]
        RAG["RAG Index"]
        REASON["Reasoning Engine"]
        PROMPT["Prompt Engine"]
        LEARN_PIPE["Learning Pipeline"]
        DECISION["Decision Engine"]
    end

    subgraph "Layer 2: Service Layer"
        COMMERCE["Commerce Services"]
        TREAS_SVC["Treasury Services"]
        CONTENT["Content Services"]
        ANALYTICS_SVC["Analytics Services"]
        INTEGRATION["Integration Services"]
        AUTH["Auth Services"]
    end

    subgraph "Layer 1: Infrastructure"
        KERNEL["Kernel"]
        EVENT_BUS["Event Bus"]
        DB["Database (Supabase/Postgres)"]
        STORAGE["Object Storage"]
        SECRETS["Secret Manager"]
        OBSERVABILITY["Observability"]
        RUNTIME["Container Runtime"]
        NET["Network Layer"]
        SEC["Security Layer"]
    end

    KATA --> CATA
    UI --> CATA
    APPROVAL --> APPROVAL_GATE
    THEME --> UI

    CATA --> ORCH
    RESEARCH --> ORCH
    AFFILIATE --> ORCH
    CREATIVE --> ORCH
    PUBLISHER --> ORCH
    TREASURY --> ORCH
    ANALYTICS --> ORCH
    LEARNING --> ORCH
    MEMORY --> ORCH
    SECURITY --> ORCH

    ORCH --> WORKFLOW
    ORCH --> SCHEDULER
    ORCH --> APPROVAL_GATE
    ORCH --> LIFECYCLE

    WORKFLOW --> KG
    WORKFLOW --> MEM
    WORKFLOW --> RAG
    WORKFLOW --> REASON
    WORKFLOW --> PROMPT
    WORKFLOW --> LEARN_PIPE
    WORKFLOW --> DECISION

    REASON --> COMMERCE
    DECISION --> TREAS_SVC
    PROMPT --> CONTENT
    LEARN_PIPE --> ANALYTICS_SVC
    INTEGRATION --> COMMERCE
    AUTH --> KERNEL

    COMMERCE --> DB
    TREAS_SVC --> DB
    CONTENT --> STORAGE
    ANALYTICS_SVC --> DB
    INTEGRATION --> NET
    AUTH --> SECRETS

    DB --> KERNEL
    STORAGE --> KERNEL
    SECRETS --> SEC
    EVENT_BUS --> KERNEL
    OBSERVABILITY --> KERNEL
```

### 13.5 Human Interaction Layer (Layer 6)

#### 13.5.1 Purpose

The Human Interaction Layer is the **only** layer that humans directly access. Its purpose is twofold:

1. **Supervision** — Provide humans with visibility into what CAT is doing, has done, and plans to do.
2. **Approval** — Provide humans with the ability to approve, reject, or modify CAT's proposed actions.

The Human Interaction Layer is **not** an input form. Humans do not enter data into CAT. They do not create campaigns, write content, or manage links. Those are agent responsibilities. Humans supervise and approve.

#### 13.5.2 Components

| Component | Responsibility |
|---|---|
| KATA Avatar | The visual persona of CAT — a cosmic cat that reflects system state through body language, color, and particle effects. Serves as the emotional and intuitive interface. |
| Command Center UI | The 3D command center with globe, glass panels, and real-time data visualization. The primary workspace for human supervisors. |
| Approval Workflows | The system that surfaces pending approvals, provides context for decision-making, and routes human decisions back to the orchestration layer. |
| Natural Language Interface | The conversational interface through which humans communicate with KATA in any language. |
| Theme Engine | The system that allows users to customize the visual aesthetic (colors, effects, performance levels). |

#### 13.5.3 Human Interaction Principles

| Principle | Description |
|---|---|
| **Oversight, not operation** | The UI is designed for monitoring and approval, not data entry. |
| **Natural communication** | Humans speak to KATA in natural language; KATA translates to system operations. |
| **Progressive disclosure** | Information is layered — summary first, detail on demand. The supervisor sees the big picture and can drill down. |
| **State-aware interface** | The UI reflects system state through color, motion, and avatar behavior. |
| **No dead ends** | Every view provides context and next actions. The supervisor is never stuck. |
| **Accessibility** | All functionality is accessible via keyboard, screen reader, and alternative input methods. |

### 13.6 KATA — The AI Avatar

#### 13.6.1 Concept

KATA is the **human interface layer** of CAT. It is a god-like AI cat avatar that serves as the visual and conversational representation of the entire CAT system.

> **The user communicates naturally with KATA in any language. KATA communicates with internal AI systems. The internal core remains protected and inaccessible. The system behaves like an intelligent autonomous commerce organism.**

KATA is not a chatbot. KATA is not a voice assistant. KATA is the **persona** of CAT — the face of a living digital intelligence. When a human interacts with CAT, they interact with KATA. They never interact with the internal agents, the orchestrator, or the cognitive core directly.

#### 13.6.2 KATA's Role in the Architecture

```mermaid
graph LR
    HUMAN["Human Supervisor"] -- "Natural Language<br/>(Any Language)" --> KATA
    KATA -- "Structured Commands" --> CATA
    CATA -- "Tasks" --> AGENTS
    AGENTS -- "Results" --> CATA
    CATA -- "Summaries" --> KATA
    KATA -- "Natural Language<br/>Visual + Text" --> HUMAN

    style KATA fill:#00d4ff,color:#000,stroke:#00d4ff,stroke-width:3px
    style HUMAN fill:#10b981,color:#fff
    style CATA fill:#7c3aed,color:#fff
    style AGENTS fill:#7c3aed,color:#fff
```

| Role | Description |
|---|---|
| **Translator** | KATA translates human natural language into structured commands for CATA (the central orchestrator). |
| **Shield** | KATA protects the internal core. Humans never access agents, the knowledge graph, or the reasoning engine directly. |
| **Persona** | KATA gives CAT a personality — making the system feel like a living intelligence rather than a collection of services. |
| **Guide** | KATA proactively suggests actions, warns of risks, and explains decisions in human-understandable terms. |
| **Messenger** | KATA delivers results, summaries, and notifications from the internal system to the human. |

#### 13.6.3 KATA's Visual Representation

KATA is represented as a **cosmic cat** — a being made of stars, nebulae, light, and energy. The visual specification is documented in the Design Philosophy section (Section 9) and will be expanded in `context/10_UI_UX.md` and `context/11_DESIGN_LANGUAGE.md`.

Key architectural implications of KATA's visual representation:

| Implication | Architectural Requirement |
|---|---|
| Real-time 3D rendering | GPU-accelerated rendering pipeline (Three.js / WebGL) |
| Reactive body language | State-driven animation system that maps system events to avatar behaviors |
| Particle effects | Particle engine capable of rendering thousands of particles at 60 FPS |
| Color changes | Theme engine that maps system state to color transitions |
| Multi-language support | Internationalization (i18n) layer for natural language processing |

#### 13.6.4 KATA's Behavioral Architecture

KATA's behavior is driven by a **state machine** that maps system events to avatar behaviors:

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Thinking: Agent starts processing
    Thinking --> Generating: Agent produces content
    Generating --> Searching: Agent searches for data
    Searching --> Thinking: Agent processes results
    Thinking --> Success: Task completed successfully
    Thinking --> Warning: Non-critical issue detected
    Thinking --> Error: Task failed
    Success --> Idle: Return to calm
    Warning --> Idle: Issue resolved
    Error --> Idle: Error handled
    Idle --> Learning: Learning pipeline active
    Learning --> Idle: Learning complete
    Idle --> Sleeping: User idle (extended)
    Sleeping --> Idle: User returns
    Idle --> Waiting: Awaiting human approval
    Waiting --> Idle: Approval received
```

| State | Trigger | Avatar Behavior | Body Color |
|---|---|---|---|
| Idle | Default state | Calm, looking around, ears and tail moving, slow breathing | Blue |
| Thinking | Agent starts processing | Eyes closed, paws together, particles orbiting, tail still | Purple |
| Generating | Agent produces content | One paw moves as if creating something | Purple |
| Searching | Agent searches data sources | Thousands of nodes appear around the head | Blue |
| Success | Task completed | Subtle smile, golden light | Gold |
| Warning | Non-critical issue | Head slightly lowered | Orange |
| Error | Task failed | Head lowered, dim light | Red |
| Learning | Learning pipeline active | Turquoise light, particles flowing inward | Turquoise |
| Waiting | Awaiting human approval | Looks at user expectantly | Blue (pulsing) |
| Sleeping | User idle (extended) | Dim light, minimal movement | Dim blue |

#### 13.6.5 KATA vs. CATA

A common confusion is the distinction between KATA and CATA. They are different components with different roles:

| Aspect | KATA | CATA |
|---|---|---|
| Full name | KATA (Avatar Interface) | CATA (Central Agent) |
| Layer | Layer 6 (Human Interaction) | Layer 5 (Agent Ecosystem) |
| Role | Human-facing persona | Internal orchestrator |
| Communicates with | Human supervisor | All internal agents |
| Direction | Outward (to human) | Inward (to agents) |
| Intelligence | Conversational, empathetic, explanatory | Strategic, coordinative, decision-making |
| Visibility | Always visible to human | Never visible to human |
| Language | Natural language (any human language) | Structured agent communication protocol |

KATA is the face. CATA is the brain. The human talks to the face. The face talks to the brain. The brain talks to the body (agents). The body does the work.

#### 13.6.6 KATA's Natural Language Processing

KATA must understand and respond in any human language. The architectural approach:

```mermaid
flowchart TD
    INPUT["Human speaks/types<br/>in any language"] --> LANG_DETECT["Language Detection"]
    LANG_DETECT --> TRANSLATE["Translate to internal<br/>representation language (English)"]
    TRANSLATE --> INTENT["Intent Classification<br/>What does the human want?"]
    INTENT --> COMMAND["Generate structured command<br/>for CATA"]
    COMMAND --> CATA["CATA processes command"]
    CATA --> RESPONSE["CATA returns result"]
    RESPONSE --> TRANSLATE_BACK["Translate response<br/>to human's language"]
    TRANSLATE_BACK --> KATA_RENDER["KATA renders response<br/>visually + textually"]
    KATA_RENDER --> OUTPUT["Human receives response<br/>in their language"]

    style INPUT fill:#10b981,color:#fff
    style OUTPUT fill:#10b981,color:#fff
    style KATA_RENDER fill:#00d4ff,color:#000,stroke-width:3px
    style CATA fill:#7c3aed,color:#fff
```

| Design Decision | Choice | Rationale |
|---|---|---|
| Internal representation language | English | Most AI models are strongest in English; simplifies internal processing |
| Human-facing language | Any language | The human should communicate in whatever language is natural to them |
| Translation layer | AI-powered translation | Near-real-time translation with high accuracy |
| Intent classification | NLU model | Maps natural language to structured commands |
| Response rendering | Visual + textual | KATA responds with both avatar body language and text/speech |

#### 13.6.7 Security Boundary

KATA is a **security boundary**. The internal core of CAT — agents, knowledge graph, memory, reasoning engine — is never directly accessible to humans. All human interaction flows through KATA, which translates and filters:

| Threat | KATA Mitigation |
|---|---|
| Direct agent manipulation | Humans cannot address agents directly; KATA routes all requests through CATA |
| Raw knowledge graph access | KATA provides summaries and explanations, not raw graph queries |
| Prompt injection via natural language | KATA's intent classification sanitizes and structures all human input before it reaches the cognitive core |
| Unauthorized operations | KATA enforces the approval workflow — no action with external impact bypasses human approval |
| Information leakage | KATA only reveals information appropriate to the human's role and the current context |

### 13.7 Central Intelligence Core (Layer 3)

#### 13.7.1 Purpose

The Cognitive Core is the brain of CAT. It is the layer where knowledge is stored, memory is maintained, reasoning occurs, and learning happens. It is the layer that makes CAT more than a workflow engine — it is what makes CAT **intelligent**.

The Cognitive Core is **not** directly accessible to humans. It is accessed by agents (Layer 5) through the orchestration layer (Layer 4). KATA (Layer 6) provides humans with summaries and explanations of what the Cognitive Core produces, but never raw access.

#### 13.7.2 Components

| Component | Responsibility | Persistence |
|---|---|---|
| Knowledge Graph | Structured knowledge about products, markets, strategies, competitors, and outcomes | Permanent (Postgres + graph extension) |
| Memory Store | Context from past operations, decisions, and outcomes | Permanent (Postgres) |
| RAG Index | Searchable vector index of all accumulated knowledge | Permanent (vector database) |
| Reasoning Engine | Applies logic, inference, and pattern matching to knowledge and memory | Stateless (uses knowledge graph and memory as input) |
| Prompt Engine | Manages prompt templates, model selection, and prompt execution | Stateless (templates stored in DB) |
| Learning Pipeline | Processes outcomes and updates the knowledge graph and memory | Continuous (background process) |
| Decision Engine | Evaluates options and recommends actions based on knowledge, memory, and reasoning | Stateless (uses all cognitive components as input) |

#### 13.7.3 Knowledge Graph Architecture

The knowledge graph is the **cognitive substrate** of CAT. Everything CAT knows is stored in the graph. The graph is not a simple key-value store — it is a semantically rich, relationship-aware structure.

```mermaid
graph TD
    PRODUCT["Product Node"] --> HAS_AFFILIATE["has_affiliate_program"]
    HAS_AFFILIATE --> AFFILIATE_PROG["Affiliate Program Node"]
    PRODUCT --> BELONGS_TO["belongs_to_category"]
    BELONGS_TO --> CATEGORY["Category Node"]
    PRODUCT --> TARGETS["targets_market"]
    TARGETS --> MARKET["Market Node"]
    PRODUCT --> REVIEWED_BY["reviewed_by"]
    REVIEWED_BY --> CONTENT["Content Node"]
    CONTENT --> PUBLISHED_ON["published_on"]
    PUBLISHED_ON --> CHANNEL["Channel Node"]
    CONTENT --> GENERATED_LINKS["generates_links"]
    GENERATED_LINKS --> LINK["Affiliate Link Node"]
    LINK --> TRACKED_BY["tracked_by"]
    TRACKED_BY --> CAMPAIGN["Campaign Node"]
    CAMPAIGN --> PRODUCES["produces"]
    PRODUCES --> EARNINGS["Earnings Node"]
    EARNINGS --> RECORDED_IN["recorded_in"]
    RECORDED_IN --> TREASURY["Treasury Node"]
    CAMPAIGN --> HAS_OUTCOME["has_outcome"]
    HAS_OUTCOME --> OUTCOME["Outcome Node<br/>(success/failure + metrics)"]
    OUTCOME --> LEARNED_BY["learned_by"]
    LEARNED_BY --> LEARNING["Learning Record Node"]

    style PRODUCT fill:#00d4ff,color:#000
    style OUTCOME fill:#10b981,color:#fff
    style LEARNING fill:#06b6d4,color:#fff
```

| Node Type | Represents | Key Properties |
|---|---|---|
| Product | A product available for affiliate promotion | Name, category, price, commission rate, merchant |
| Affiliate Program | An affiliate program for a product or merchant | Network, commission structure, cookie duration, terms |
| Category | A product category | Name, parent category, market size, competition level |
| Market | A target market or audience | Demographics, geography, language, purchasing power |
| Content | A piece of content (review, comparison, tutorial) | Type, title, body, keywords, quality score |
| Channel | A publishing channel (blog, social, video) | Platform, audience size, format requirements |
| Affiliate Link | A tracked affiliate link | URL, product, campaign, click count, conversion count |
| Campaign | A coordinated affiliate campaign | Products, content, channels, budget, timeline |
| Earnings | Revenue generated by a campaign | Amount, currency, date, source, campaign |
| Outcome | The result of a campaign or action | Success/failure, metrics, lessons learned |
| Learning Record | A learned insight from an outcome | Insight, confidence, applicability, timestamp |

#### 13.7.4 Memory Architecture

CAT's memory is not a single store — it is a **tiered memory system** modeled after human cognitive memory:

| Memory Tier | Analogy | Purpose | Persistence | Capacity |
|---|---|---|---|---|
| Working Memory | Short-term memory | Current task context, active agent state | Session (cleared on task completion) | Small (KB) |
| Episodic Memory | Personal experiences | Record of specific events, decisions, and outcomes | Permanent | Large (GB) |
| Semantic Memory | General knowledge | Structured facts about products, markets, strategies | Permanent | Large (GB) |
| Procedural Memory | Skills and procedures | How-to knowledge, successful workflows, prompt patterns | Permanent | Medium (MB) |
| Flash Memory | Recent cache | Recently accessed knowledge for fast retrieval | Ephemeral (LRU cache) | Medium (MB) |

```mermaid
graph TB
    AGENT["Agent requests<br/>knowledge/memory"] --> CHECK_FLASH{In flash memory?}
    CHECK_FLASH -- Yes --> RETURN_FLASH["Return from flash<br/>(fastest)"]
    CHECK_FLASH -- No --> CHECK_SEMANTIC{In semantic memory?}
    CHECK_SEMANTIC -- Yes --> RETURN_SEMANTIC["Return from semantic memory<br/>+ cache in flash"]
    CHECK_SEMANTIC -- No --> CHECK_EPISODIC{In episodic memory?}
    CHECK_EPISODIC -- Yes --> RETURN_EPISODIC["Return from episodic memory<br/>+ cache in flash"]
    CHECK_EPISODIC -- No --> CHECK_PROCEDURAL{In procedural memory?}
    CHECK_PROCEDURAL -- Yes --> RETURN_PROCEDURAL["Return from procedural memory<br/>+ cache in flash"]
    CHECK_PROCEDURAL -- No --> LEARN["Trigger learning pipeline<br/>to acquire knowledge"]

    style RETURN_FLASH fill:#10b981,color:#fff
    style LEARN fill:#f59e0b,color:#000
```

#### 13.7.5 Reasoning Engine

The Reasoning Engine applies logic and inference to the knowledge graph and memory to produce conclusions. It is **not** a single algorithm — it is a pipeline of reasoning strategies:

| Strategy | When Used | Example |
|---|---|---|
| Deductive reasoning | When rules and facts are known | "All products in category X have commission > 5%. Product Y is in category X. Therefore product Y has commission > 5%." |
| Inductive reasoning | When patterns exist in data | "Campaigns using channel X for product category Y have 3x conversion rate. Therefore, new campaigns for category Y should prioritize channel X." |
| Abductive reasoning | When explaining observations | "Earnings dropped 40% last week. The most likely explanation is that the primary affiliate link broke. Verify link status." |
| Analogical reasoning | When similar situations exist | "Product A is similar to product B. Campaign for product B was successful with strategy S. Try strategy S for product A." |
| Causal reasoning | When cause-effect relationships matter | "Changing the call-to-action from 'Buy Now' to 'Learn More' increased conversions by 15%. The CTA change caused the increase." |

#### 13.7.6 Decision Engine

The Decision Engine evaluates options and recommends actions. It is the component that produces the recommendations that humans approve.

```mermaid
flowchart TD
    TRIGGER["Decision Trigger<br/>(new opportunity, performance drop, scheduled review)"] --> GATHER["Gather Context<br/>from Knowledge Graph + Memory"]
    GATHER --> OPTIONS["Generate Options<br/>using Reasoning Engine"]
    OPTIONS --> EVALUATE["Evaluate Options<br/>against success criteria"]
    EVALUATE --> RANK["Rank Options<br/>by expected value"]
    RANK --> RECOMMEND["Produce Recommendation<br/>with reasoning + confidence"]
    RECOMMEND --> APPROVAL_CHECK{Requires human approval?}
    APPROVAL_CHECK -- Yes --> KATA_APPROVAL["KATA presents to human<br/>for approval"]
    APPROVAL_CHECK -- No --> EXECUTE["Execute automatically"]
    KATA_APPROVAL -- Approved --> EXECUTE
    KATA_APPROVAL -- Rejected --> LOG["Log rejection + reason"]
    KATA_APPROVAL -- Modified --> EXECUTE
    EXECUTE --> OUTCOME["Record outcome<br/>in memory + knowledge graph"]
    LOG --> LEARN["Learning pipeline<br/>updates knowledge"]

    style TRIGGER fill:#00d4ff,color:#000
    style RECOMMEND fill:#7c3aed,color:#fff
    style KATA_APPROVAL fill:#10b981,color:#fff
    style EXECUTE fill:#10b981,color:#fff
    style LEARN fill:#06b6d4,color:#fff
```

### 13.8 Agent Ecosystem (Layer 5)

#### 13.8.1 Agent Architecture

Each agent in CAT's ecosystem follows a common internal architecture:

```mermaid
graph TB
    subgraph "Agent Internal Architecture"
        INPUT["Input Handler<br/>Receives tasks from orchestrator"] --> CONTEXT["Context Builder<br/>Gathers knowledge + memory"]
        CONTEXT --> PROCESS["Processing Core<br/>Agent-specific logic"]
        PROCESS --> OUTPUT["Output Handler<br/>Returns results to orchestrator"]
        OUTPUT --> LEARN["Learning Hook<br/>Reports outcome to learning pipeline"]
    end

    ORCHESTRATOR["Layer 4: Orchestrator"] --> INPUT
    OUTPUT --> ORCHESTRATOR
    LEARN --> COGNITIVE_CORE["Layer 3: Cognitive Core"]

    style PROCESS fill:#7c3aed,color:#fff
    style ORCHESTRATOR fill:#00d4ff,color:#000
    style COGNITIVE_CORE fill:#06b6d4,color:#fff
```

| Component | Responsibility |
|---|---|
| Input Handler | Receives structured tasks from the orchestrator, validates input, and manages task lifecycle |
| Context Builder | Queries the knowledge graph and memory store for relevant context before processing |
| Processing Core | Agent-specific logic — research, content generation, link management, etc. |
| Output Handler | Formats results, returns them to the orchestrator, and triggers downstream tasks |
| Learning Hook | Reports the outcome (success/failure + metrics) to the learning pipeline for knowledge updates |

#### 13.8.2 Agent Inventory

| Agent | Role | Primary Inputs | Primary Outputs | Autonomy Level |
|---|---|---|---|---|
| CATA | Central orchestrator — coordinates all agents | Tasks from KATA, events from agents | Task assignments to agents, status to KATA | Level 5 |
| Research | Market research and product discovery | Market parameters, trending data | Product recommendations, market analysis | Level 3 |
| Affiliate | Affiliate program management and link creation | Product selections, affiliate network APIs | Affiliate links, program terms | Level 3 |
| Creative | Content generation (text, image, video) | Product info, content requirements, brand guidelines | Articles, reviews, images, videos | Level 3 |
| Publisher | Multi-channel content publishing | Content, channel specifications | Published content, distribution reports | Level 3 |
| Treasury | Earnings tracking and financial management | Earnings data, payout schedules | Financial reports, payout recommendations | Level 3 |
| Analytics | Performance monitoring and interpretation | Click data, conversion data, revenue data | Performance reports, optimization recommendations | Level 4 |
| Learning | Continuous learning and knowledge refinement | Outcomes from all agents | Updated knowledge graph entries, learned patterns | Level 5 |
| Memory | Memory management and retrieval | Memory queries from agents | Relevant memories, context | Level 5 |
| Security | Threat detection and access control | System events, access requests | Security alerts, access decisions | Level 5 |

#### 13.8.3 Agent Communication Protocol

Agents do not communicate with each other directly. All communication flows through the orchestrator (CATA) via the event bus:

```mermaid
sequenceDiagram
    participant KATA
    participant CATA
    participant Research
    participant Affiliate
    participant Creative
    participant Publisher
    participant EventBus

    KATA->>CATA: "Find profitable products in electronics"
    CATA->>Research: Task: research_market(electronics)
    Research->>CATA: Result: 5 products recommended
    CATA->>KATA: "Found 5 products. Approve selection?"
    KATA->>CATA: Human approves product selection
    CATA->>Affiliate: Task: create_links(products)
    Affiliate->>CATA: Result: affiliate links created
    CATA->>Creative: Task: generate_content(products, links)
    Creative->>CATA: Result: content generated
    CATA->>KATA: "Content ready. Approve for publishing?"
    KATA->>CATA: Human approves content
    CATA->>Publisher: Task: publish_content(content, channels)
    Publisher->>CATA: Result: content published
    CATA->>KATA: "Content published to 3 channels"
```

| Communication Rule | Description |
|---|---|
| No direct agent-to-agent calls | Agents never call each other. All coordination goes through CATA. |
| Event bus for async communication | Agents publish and subscribe to events on the event bus. CATA routes events. |
| Structured task format | All tasks are structured objects with type, input, priority, and deadline. |
| Structured result format | All results are structured objects with status, output, metrics, and outcome. |
| Idempotent operations | Agents must handle duplicate tasks gracefully (idempotency). |

#### 13.8.4 Agent Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Registered: Agent deployed
    Registered --> Idle: Initialization complete
    Idle --> Active: Task assigned
    Active --> Idle: Task completed
    Active --> Waiting: Awaiting external response
    Waiting --> Active: External response received
    Idle --> Paused: Maintenance mode
    Paused --> Idle: Maintenance complete
    Idle --> Degraded: Partial failure
    Degraded --> Idle: Recovery complete
    Active --> Failed: Critical error
    Failed --> Idle: Error handled
    Failed --> Unregistered: Unrecoverable
    Idle --> Unregistered: Decommissioned
    Unregistered --> [*]
```

| State | Description | Orchestrator Action |
|---|---|---|
| Registered | Agent is deployed but not yet initialized | Wait for initialization |
| Idle | Agent is ready and waiting for tasks | Assign tasks as needed |
| Active | Agent is processing a task | Monitor progress |
| Waiting | Agent is awaiting an external response (e.g., human approval, API response) | Route response when available |
| Paused | Agent is in maintenance mode | Do not assign tasks; redirect to backup |
| Degraded | Agent is partially functional | Assign reduced workload; attempt recovery |
| Failed | Agent encountered a critical error | Attempt recovery or failover |
| Unregistered | Agent has been decommissioned | Remove from active roster |

### 13.9 Service Communication Philosophy

#### 13.9.1 Event-Driven Architecture

CAT's service communication is **event-driven**, not request-response. This is a fundamental architectural decision, not an implementation detail.

| Request-Response | Event-Driven (CAT) |
|---|---|
| Caller blocks until response arrives | Caller publishes event and continues |
| Tight coupling between caller and responder | Loose coupling — publisher doesn't know subscribers |
| One-to-one communication | One-to-many communication (event fan-out) |
| Synchronous error handling | Asynchronous error handling (dead letter queue) |
| Difficult to add new consumers | New consumers subscribe without affecting publisher |
| Caller failure blocks responder | Publisher and subscriber failures are isolated |

#### 13.9.2 Event Bus Architecture

The event bus is the **nervous system** of CAT. All inter-component communication flows through it.

```mermaid
graph TB
    subgraph "Event Bus"
        TOPIC_RESEARCH["research.events"]
        TOPIC_AFFILIATE["affiliate.events"]
        TOPIC_CREATIVE["creative.events"]
        TOPIC_PUBLISHER["publisher.events"]
        TOPIC_TREASURY["treasury.events"]
        TOPIC_ANALYTICS["analytics.events"]
        TOPIC_LEARNING["learning.events"]
        TOPIC_SECURITY["security.events"]
        TOPIC_SYSTEM["system.events"]
        TOPIC_APPROVAL["approval.events"]
    end

    PUBLISHER_1["Research Agent"] --> TOPIC_RESEARCH
    PUBLISHER_2["Affiliate Agent"] --> TOPIC_AFFILIATE
    PUBLISHER_3["Creative Agent"] --> TOPIC_CREATIVE
    PUBLISHER_4["Publisher Agent"] --> TOPIC_PUBLISHER
    PUBLISHER_5["Treasury Agent"] --> TOPIC_TREASURY

    TOPIC_RESEARCH --> SUBSCRIBER_1["CATA (Orchestrator)"]
    TOPIC_AFFILIATE --> SUBSCRIBER_1
    TOPIC_CREATIVE --> SUBSCRIBER_1
    TOPIC_PUBLISHER --> SUBSCRIBER_1
    TOPIC_TREASURY --> SUBSCRIBER_1
    TOPIC_RESEARCH --> SUBSCRIBER_2["Learning Agent"]
    TOPIC_ANALYTICS --> SUBSCRIBER_2
    TOPIC_SYSTEM --> SUBSCRIBER_3["Security Agent"]
    TOPIC_APPROVAL --> SUBSCRIBER_4["KATA (Human Interface)"]
```

| Event Property | Description |
|---|---|
| Event ID | Unique identifier for the event |
| Event Type | Category of event (e.g., `research.product_found`, `creative.content_ready`) |
| Source | Agent or service that published the event |
| Timestamp | When the event was published |
| Payload | Structured data specific to the event type |
| Correlation ID | Links related events across a workflow |
| Priority | Event priority (critical, high, normal, low) |
| TTL | Time-to-live — event expires after this duration if not consumed |

#### 13.9.3 Event Categories

| Category | Topic Prefix | Example Events |
|---|---|---|
| Research | `research.*` | `research.market_scanned`, `research.product_found`, `research.trend_detected` |
| Affiliate | `affiliate.*` | `affiliate.link_created`, `affiliate.program_joined`, `affiliate.link_expired` |
| Creative | `creative.*` | `creative.content_generated`, `creative.content_reviewed`, `creative.image_created` |
| Publisher | `publisher.*` | `publisher.content_published`, `publisher.publish_failed`, `publisher.channel_connected` |
| Treasury | `treasury.*` | `treasury.earnings_recorded`, `treasury.payout_processed`, `treasury.report_generated` |
| Analytics | `analytics.*` | `analytics.performance_updated`, `analytics.anomaly_detected`, `analytics.threshold_reached` |
| Learning | `learning.*` | `learning.pattern_discovered`, `learning.knowledge_updated`, `learning.model_retrained` |
| Security | `security.*` | `security.threat_detected`, `security.access_denied`, `security.audit_logged` |
| System | `system.*` | `system.agent_started`, `system.agent_stopped`, `system.health_check` |
| Approval | `approval.*` | `approval.requested`, `approval.granted`, `approval.rejected` |

#### 13.9.4 Communication Reliability

| Requirement | Implementation |
|---|---|
| At-least-once delivery | Events are persisted before delivery; consumers must be idempotent |
| Event ordering | Events within the same topic are ordered by timestamp; cross-topic ordering is not guaranteed |
| Dead letter queue | Events that cannot be processed after N retries are moved to a dead letter queue for investigation |
| Event replay | Events can be replayed from the event store for debugging or recovery |
| Backpressure | If a consumer is overwhelmed, the event bus applies backpressure (buffer or shed load) |

### 13.10 Knowledge-Driven Architecture

#### 13.10.1 Knowledge as First-Class Citizen

In CAT, knowledge is not a byproduct — it is a **first-class architectural citizen**. Every operation produces knowledge. Every decision consumes knowledge. The system is designed around the flow of knowledge.

```mermaid
graph LR
    subgraph "Knowledge Flow"
        ACTION["Agent Action"] --> OUTCOME["Outcome<br/>(success/failure + metrics)"]
        OUTCOME --> LEARNING["Learning Pipeline"]
        LEARNING --> KNOWLEDGE["Knowledge Graph"]
        KNOWLEDGE --> CONTEXT["Context for Next Action"]
        CONTEXT --> ACTION2["Next Agent Action"]
    end

    style KNOWLEDGE fill:#06b6d4,color:#fff,stroke-width:3px
    style LEARNING fill:#06b6d4,color:#fff
```

#### 13.10.2 Knowledge Lifecycle

| Stage | Description | Component |
|---|---|---|
| Acquisition | Knowledge is acquired through agent actions, external data ingestion, or human input | Agents, integration services |
| Validation | New knowledge is validated for accuracy, relevance, and non-duplication | Learning pipeline |
| Integration | Validated knowledge is integrated into the knowledge graph | Knowledge graph |
| Retrieval | Knowledge is retrieved when needed for decisions or actions | RAG index, memory store |
| Application | Retrieved knowledge is applied to improve decision-making | Reasoning engine, decision engine |
| Refinement | Knowledge is refined over time based on new evidence | Learning pipeline |
| Expiration | Outdated knowledge is marked as stale (not deleted) | Learning pipeline |

#### 13.10.3 Knowledge Never Deleted

CAT follows a **soft-delete** philosophy for knowledge. Knowledge is never hard-deleted. Instead, outdated or incorrect knowledge is marked as stale with a timestamp and reason. This preserves the historical record and allows the learning pipeline to understand what was believed, when it was believed, and why it changed.

| Operation | Allowed | Rationale |
|---|---|---|
| Create new knowledge | Yes | New knowledge is always welcome |
| Update existing knowledge | Yes | Knowledge evolves with new evidence |
| Mark knowledge as stale | Yes | Outdated knowledge is preserved but flagged |
| Hard delete knowledge | No | Historical record must be preserved for learning |

### 13.11 Event-Driven Architecture (Deep Dive)

#### 13.11.1 Why Event-Driven

CAT's architecture is event-driven because the system is **continuous**, not transactional. A traditional request-response architecture assumes that a user initiates an action and waits for a response. CAT's agents operate continuously — they discover opportunities, generate content, publish, track, and optimize without waiting for human initiation.

| Scenario | Request-Response | Event-Driven |
|---|---|---|
| Research agent finds a trending product | Must wait for someone to ask | Publishes `research.product_found` event immediately |
| Creative agent finishes content | Must wait for someone to request it | Publishes `creative.content_ready` event immediately |
| Treasury detects earnings anomaly | Must wait for someone to query | Publishes `treasury.anomaly_detected` event immediately |
| Analytics detects performance drop | Must wait for someone to check | Publishes `analytics.threshold_reached` event immediately |

#### 13.11.2 Event Sourcing

CAT uses **event sourcing** for critical workflows. Event sourcing means that the current state of the system is derived from the sequence of events that led to it, not from a mutable state store.

| Benefit | Description |
|---|---|
| Complete audit trail | Every state change is recorded as an immutable event |
| Replay capability | The system can be rebuilt by replaying events from the event store |
| Debugging | Any issue can be traced by examining the event sequence |
| Time travel | The system state at any point in time can be reconstructed |
| Decoupling | Producers and consumers are fully decoupled |

#### 13.11.3 CQRS Pattern

CAT applies the **Command Query Responsibility Segregation (CQRS)** pattern:

```mermaid
graph TB
    subgraph "Command Side"
        CMD["Command<br/>(create, update, approve)"] --> CMD_HANDLER["Command Handler"]
        CMD_HANDLER --> EVENT_STORE["Event Store"]
        EVENT_STORE --> PROJECTION["Projection"]
    end

    subgraph "Query Side"
        QUERY["Query<br/>(read, search, analyze)"] --> READ_MODEL["Read Model"]
        PROJECTION --> READ_MODEL
    end

    style CMD fill:#7c3aed,color:#fff
    style QUERY fill:#00d4ff,color:#000
    style EVENT_STORE fill:#10b981,color:#fff
```

| Side | Responsibility | Optimization |
|---|---|---|
| Command side | Processes writes (create, update, approve) | Optimized for writes; validates and persists events |
| Query side | Processes reads (search, analyze, display) | Optimized for reads; pre-computed projections |
| Event store | Source of truth | Append-only log of all events |
| Projections | Derived read models | Updated from events; optimized for specific queries |

#### 13.11.4 Rejected Alternative: Pure Request-Response

| Criterion | Request-Response | Event-Driven (Chosen) |
|---|---|---|
| Continuous operation | Requires polling | Native (event-driven) |
| Scalability | Tight coupling limits scale | Loose coupling enables horizontal scale |
| Auditability | Only final state is recorded | Complete event history |
| Fault tolerance | Caller failure blocks system | Isolated failures |
| Extensibility | New consumers require caller changes | New consumers subscribe independently |

**Decision:** Event-driven architecture with event sourcing for critical workflows and CQRS for read/write separation.

### 13.12 Scalable Infrastructure Philosophy

#### 13.12.1 Cloud-Native from Day One

CAT is designed as a cloud-native system from the start. This is not a "we'll add it later" decision — it is a foundational architectural commitment.

| Cloud-Native Principle | CAT Implementation |
|---|---|
| Containerized | All components (agents, services, UI) run in containers |
| Orchestrated | Kubernetes manages container lifecycle, scaling, and scheduling |
| Observable | Metrics, logs, and traces are collected from all components |
| Declarative | Infrastructure is defined as code (Terraform) |
| Immutable | Containers are not modified at runtime; new versions are deployed |
| Microservice-ready | Components are independently deployable (though initially deployed as a modular monolith) |

#### 13.12.2 Scaling Strategy

```mermaid
graph TB
    subgraph "Scaling Dimensions"
        direction TB
        AGENT_SCALE["Agent Scaling<br/>More agent instances for parallel work"]
        KNOWLEDGE_SCALE["Knowledge Scaling<br/>Knowledge graph grows with operations"]
        SERVICE_SCALE["Service Scaling<br/>Services scale with request volume"]
        UI_SCALE["UI Scaling<br/>CDN + edge rendering for global access"]
    end

    AGENT_SCALE --> ORCHESTRATOR["Orchestrator distributes<br/>work across instances"]
    KNOWLEDGE_SCALE --> SHARDING["Knowledge graph sharding<br/>by domain or time"]
    SERVICE_SCALE --> HPA["Horizontal Pod Autoscaler<br/>CPU/memory/request-based"]
    UI_SCALE --> CDN["CDN + edge rendering<br/>Static assets + SSR"]

    style AGENT_SCALE fill:#00d4ff,color:#000
    style KNOWLEDGE_SCALE fill:#06b6d4,color:#fff
    style SERVICE_SCALE fill:#7c3aed,color:#fff
    style UI_SCALE fill:#10b981,color:#fff
```

| Component | Scaling Strategy | Trigger |
|---|---|---|
| Agents | Horizontal — more instances | Task queue depth |
| Knowledge graph | Sharding by domain | Graph size / query latency |
| Memory store | Horizontal — read replicas | Read query volume |
| RAG index | Sharding by vector partition | Index size / search latency |
| Event bus | Partitioning by topic | Event throughput |
| Database (Supabase/Postgres) | Read replicas + connection pooling | Connection count / query latency |
| UI | CDN + edge SSR | Geographic distribution of users |
| Observability | Sampling + retention policies | Data volume |

#### 13.12.3 Performance Requirements

| Component | Target | Rationale |
|---|---|---|
| UI frame rate | 60 FPS (120 FPS on capable displays) | Smooth, cinematic experience |
| KATA avatar response | < 100ms from system state change | Avatar must feel responsive |
| Agent task dispatch | < 50ms from orchestrator to agent | Near-instant task routing |
| Knowledge graph query | < 200ms for typical queries | Agents need fast context retrieval |
| RAG search | < 500ms for similarity search | Fast knowledge retrieval |
| Event bus latency | < 10ms publish-to-subscribe | Near-real-time event propagation |
| Database query (typical) | < 100ms | Fast data access |
| Page load (initial) | < 2s | Fast initial render |
| Page transition | < 300ms | Smooth navigation |

#### 13.12.4 Performance Quality Levels

Because CAT's UI is GPU-intensive, the system supports configurable performance quality levels:

| Level | Target Hardware | Features |
|---|---|---|
| Ultra | High-end GPU (RTX 4070+), 120Hz display | Full particle effects, real-time reflections, 120 FPS |
| High | Mid-range GPU (RTX 3060+), 60Hz display | Reduced particle count, baked reflections, 60 FPS |
| Eco | Integrated GPU, 60Hz display | Minimal particles, no reflections, static background, 60 FPS |

The quality level is auto-detected on first load and user-adjustable through the theme engine.

#### 13.12.5 Security Notes

| Security Requirement | Implementation |
|---|---|
| All inter-service communication encrypted | TLS 1.3 for all internal traffic |
| All external API calls validated | Schema validation + rate limiting + allowlist |
| All secrets in secret manager | No secrets in code, config files, or environment variables in the repository |
| All agent actions logged | Immutable audit log with agent ID, action, timestamp, and outcome |
| All human approvals logged | Immutable audit log with human ID, decision, timestamp, and reasoning |
| Principle of least privilege | Each agent and service has only the permissions it needs |
| Defense in depth | Multiple security layers: network, service, agent, data |
| Regular security audits | Automated scanning + manual review |

#### 13.12.6 Observability Architecture

```mermaid
graph TB
    subgraph "Observability Stack"
        METRICS["Metrics<br/>(Prometheus)"] --> GRAFANA["Grafana<br/>(Dashboards)"]
        LOGS["Logs<br/>(Loki)"] --> GRAFANA
        TRACES["Traces<br/>(OpenTelemetry)"] --> GRAFANA
        ALERTS["Alerts<br/>(Alertmanager)"] --> PAGER["Pager<br/>(Notification)"]
    end

    AGENTS["Agents"] --> METRICS
    AGENTS --> LOGS
    AGENTS --> TRACES
    SERVICES["Services"] --> METRICS
    SERVICES --> LOGS
    SERVICES --> TRACES
    INFRA["Infrastructure"] --> METRICS
    INFRA --> LOGS
    INFRA --> TRACES

    style GRAFANA fill:#00d4ff,color:#000
```

| Pillar | What It Tracks | Tool |
|---|---|---|
| Metrics | Quantitative measurements (CPU, memory, request rate, error rate, agent task completion) | Prometheus |
| Logs | Discrete events (agent actions, system events, errors, approvals) | Loki |
| Traces | Request flows across services (end-to-end latency, bottlenecks) | OpenTelemetry |
| Alerts | Threshold-based notifications (error rate, latency, resource usage) | Alertmanager |

### 13.13 Future Multi-Node Architecture

#### 13.13.1 Single-Node to Multi-Node Evolution

CAT is initially deployed as a single-node system (one Kubernetes cluster). However, the architecture is designed to evolve into a **multi-node** system where CAT instances in different geographic regions collaborate.

```mermaid
graph TB
    subgraph "Phase 1: Single Node"
        NODE_1["CAT Instance<br/>(Single Cluster)"]
    end

    subgraph "Phase 3: Multi-Node"
        NODE_US["CAT Instance<br/>(US Region)"]
        NODE_EU["CAT Instance<br/>(EU Region)"]
        NODE_ASIA["CAT Instance<br/>(Asia Region)"]
        NODE_US <--> NODE_EU
        NODE_EU <--> NODE_ASIA
        NODE_US <--> NODE_ASIA
    end

    subgraph "Phase 5: Federation"
        NODE_PRIMARY["CAT Primary<br/>(Global Coordinator)"]
        NODE_REGIONAL_1["CAT Regional<br/>(Market-Specific)"]
        NODE_REGIONAL_2["CAT Regional<br/>(Market-Specific)"]
        NODE_REGIONAL_3["CAT Regional<br/>(Market-Specific)"]
        NODE_PRIMARY <--> NODE_REGIONAL_1
        NODE_PRIMARY <--> NODE_REGIONAL_2
        NODE_PRIMARY <--> NODE_REGIONAL_3
    end

    style NODE_1 fill:#00d4ff,color:#000
    style NODE_PRIMARY fill:#7c3aed,color:#fff
```

| Phase | Architecture | Rationale |
|---|---|---|
| Phase 1 | Single node | Simplicity; validate the architecture |
| Phase 3 | Multi-node (geo-distributed) | Reduce latency for regional markets; handle larger scale |
| Phase 5 | Federation (primary + regional) | Market-specific intelligence with global coordination |

#### 13.13.2 Multi-Node Communication

When CAT evolves to multi-node, nodes will communicate through a **federation protocol**:

| Aspect | Specification |
|---|---|
| Communication | Inter-node event bus (federated) |
| Knowledge sharing | Knowledge graph sync (eventual consistency) |
| Task distribution | Primary node assigns regional tasks; regional nodes execute |
| Conflict resolution | Last-writer-wins for non-critical; primary arbitrates for critical |
| Failover | If a regional node fails, primary reassigns to another node |

#### 13.13.3 Future Evolution Path

| Component | Phase 1 (Current) | Phase 3 (Multi-Node) | Phase 5 (Federation) |
|---|---|---|---|
| Deployment | Single Kubernetes cluster | Multiple clusters (geo-distributed) | Primary + regional clusters |
| Agent execution | All agents on one cluster | Agents distributed across clusters | Regional agents + global coordinator |
| Knowledge graph | Single instance | Replicated across nodes | Federated with sync protocol |
| Memory store | Single instance | Replicated | Federated with regional memory |
| Event bus | Single instance | Federated across clusters | Global event backbone |
| Database | Single Supabase instance | Read replicas per region | Federated with sync |
| UI | Single CDN | Regional CDN edges | Global CDN with regional personalization |

#### 13.13.4 Architectural Decisions for Future Evolution

| Decision | Phase 1 Choice | Future-Proofing |
|---|---|---|
| Modular monolith vs. microservices | Modular monolith | Module boundaries are service boundaries — extract to services when needed |
| Single database vs. sharded | Single database | Schema designed for future sharding by domain |
| Single event bus vs. federated | Single event bus | Event schema is versioned and backward-compatible |
| Single knowledge graph vs. federated | Single knowledge graph | Graph schema supports future federation |
| Single UI vs. regional | Single UI | UI is stateless and CDN-deployable |

---

*End of Part 2. The next part will cover Technology Stack, Agent Ecosystem Details, and Knowledge Engine specifications.*

# CAT OMNISYSTEM — UI/UX Bible V1 — Part 1 Continuation

> **Status:** Draft / Part 1 build
>
> This document is an append-only continuation of `context/10_UI_UX.md`. The existing V1 document is the frozen visual baseline. This continuation must not rewrite or reinterpret the approved CAT Command Center and CAT Global Performance / CAT Brain direction.

## Part 1 Scope

Part 1 expands the approved UI/UX baseline into the first implementation-grade architecture stratum. It defines the operational shell, navigation, workspace composition, responsive behavior, component hierarchy, dashboard grammar, CAT Brain surface, agent experience, approval states, data freshness, provenance, and accessibility foundations.

The interface remains a **governed operational control plane**, not a canonical source of domain truth. The existing V1 explicitly defines the UI as the presentation/interaction surface between humans, AI agents, engines, canonical truth, intelligence, recommendations, decisions, execution, and audit. citeturn71file0

---

# Section 21 — UI/UX Constitutional Architecture

## 21.1 Purpose

CAT's UI must make the system's authority boundaries visible rather than hiding them behind a generic dashboard. Every surface therefore belongs to an explicit operational layer.

```text
┌──────────────────────────────────────────────────────────────┐
│ HUMAN INTENT                                                 │
├──────────────────────────────────────────────────────────────┤
│ COMMAND / QUERY / FILTER / APPROVAL                          │
├──────────────────────────────────────────────────────────────┤
│ PRESENTATION STATE                                            │
├──────────────────────────────────────────────────────────────┤
│ DERIVED INTELLIGENCE                                          │
├──────────────────────────────────────────────────────────────┤
│ RECOMMENDATION                                                │
├──────────────────────────────────────────────────────────────┤
│ DECISION                                                      │
├──────────────────────────────────────────────────────────────┤
│ EXECUTION                                                     │
├──────────────────────────────────────────────────────────────┤
│ AUDIT / OBSERVABILITY                                         │
└──────────────────────────────────────────────────────────────┘
```

The UI may visualize every layer but may never silently collapse them into one status.

## 21.2 Authority Rules

1. A visualization is not evidence merely because it is rendered.
2. A recommendation card is not a decision record.
3. A decision card is not proof of execution.
4. An execution status is not equivalent to settlement or monetary truth.
5. Forecasts are visually marked as forecasts.
6. Simulations are visually marked as simulations.
7. AI-generated explanations retain provenance.
8. Human approval gates are explicit, actionable, and auditable.
9. Stale data is visible rather than silently presented as current.
10. Permission scope follows the authenticated actor and active workspace.

## 21.3 Visual Authority Vocabulary

The design language should use consistent semantic labels:

| State | UI meaning | Required behavior |
|---|---|---|
| `TRUTH` | authoritative domain value | show provenance/freshness |
| `EVIDENCE` | supporting observation | show source/context |
| `INTELLIGENCE` | derived interpretation | show derivation class |
| `RECOMMENDATION` | suggested action | never imply execution |
| `DECISION` | authorized decision | show policy/authority |
| `EXECUTION` | runtime action | show live state |
| `AUDIT` | historical proof | immutable presentation |
| `SIMULATION` | hypothetical | strong simulation marker |
| `FORECAST` | predicted future | confidence/freshness |
| `BLOCKED` | prevented by policy/system | show reason and next action |

## 21.4 Architecture Contract

```json
{
  "record_type": "ui_authority_state",
  "surface": "command_center",
  "state": "RECOMMENDATION",
  "source_class": "derived_intelligence",
  "canonical_truth": false,
  "execution_status": "not_executed",
  "human_approval_required": true,
  "freshness": "current",
  "provenance_required": true
}
```

## 21.5 Acceptance Criteria

- Users can distinguish recommendation, decision, execution, and audit without opening a secondary screen.
- Simulated values cannot visually masquerade as production values.
- Every high-impact action exposes authority and approval state.
- The CAT Brain visualization never claims canonical authority.

---

# Section 22 — Global Application Shell

The global shell is the persistent frame around CAT operational workspaces. It must remain recognizable across Command Center, Analytics, Agents, Campaigns, Content Factory, Knowledge, Automation, and Settings.

## 22.1 Shell Anatomy

```text
┌────────────────────────────────────────────────────────────────────┐
│ CAT MARK │ GLOBAL SEARCH │ WORKSPACE │ SYSTEM HEALTH │ USER/ROLE   │
├──────────┬─────────────────────────────────────────────────────────┤
│          │                                                         │
│ PRIMARY  │                   CONTEXTUAL WORKSPACE                 │
│ NAV      │                                                         │
│          │                                                         │
│          ├──────────────────────────────────────────┬──────────────┤
│          │ Main content                              │ Agent rail   │
│          │                                            │ / context    │
│          │                                            │              │
└──────────┴──────────────────────────────────────────┴──────────────┘
```

The existing baseline specifies a global top bar, left navigation rail, contextual page header, central workspace, optional right-side agent/context rail, command palette, and notifications/audit access. citeturn71file0

## 22.2 Top Bar

The top bar contains only persistent controls:

- CAT identity mark
- active workspace/tenant
- global command/search entry
- system health indicator
- notifications
- approval queue indicator
- audit access
- user identity and role

It must not become a second navigation system.

## 22.3 Left Navigation

Navigation is capability-aware and permission-aware. Disabled modules should not appear as fake interactive features.

Recommended groups:

```text
COMMAND
  Command Center
  Global Performance

OPERATIONS
  Campaigns
  Affiliates
  Content Factory
  Automation

INTELLIGENCE
  Analytics & BI
  Knowledge Brain
  CAT Brain
  AI Agents

SYSTEM
  Marketplace
  Plugins
  Settings
  Audit
```

The baseline already lists Command Center, Dashboard, AI Agents, Campaigns, Affiliate Intelligence, Content Factory, Ad OS, Analytics & BI, Knowledge Brain, Automation, Settings, Plugins, and Marketplace as conceptual navigation modules. citeturn71file0

## 22.4 Responsive Shell Rules

Desktop uses the full rail. Tablet collapses secondary navigation while preserving the active context. Mobile replaces the persistent rail with a compact navigation drawer and retains the top-level operational status bar.

The responsive system must preserve **priority**, not pixel geometry.

---

# Section 23 — Command Center Workspace

The Command Center is the primary executive-operational surface. It is not a collection of unrelated widgets; it is a prioritized operational narrative.

## 23.1 Priority Stack

```text
P0 — Critical health / blocked operations
P1 — Money / operational truth
P2 — Approvals / active decisions
P3 — Agent execution
P4 — Intelligence / anomalies
P5 — Forecast / trend
P6 — Recommendations
P7 — Secondary context
```

The baseline explicitly prioritizes system health, financial/operational truth, decisions and approvals, agent execution, intelligence, trends/forecasts, recommendations, and secondary metadata. citeturn71file0

## 23.2 Primary Layout

```text
┌─────────────────────────────────────────────────────────────────┐
│ COMMAND CENTER · LIVE · WORKSPACE: GLOBAL                       │
├──────────────┬──────────────┬──────────────┬─────────────────────┤
│ Revenue      │ Profit       │ Conversions  │ Active Campaigns    │
├──────────────┴──────────────┴──────────────┴─────────────────────┤
│                                                                 │
│                     CAT BRAIN                                   │
│              network / intelligence core                        │
│                                                                 │
├──────────────────────────────────────┬──────────────────────────┤
│ Performance / revenue trend           │ AI Agent Activity        │
├──────────────────────────────────────┼──────────────────────────┤
│ Channel / market performance          │ Approvals / Decisions    │
├──────────────────────────────────────┼──────────────────────────┤
│ Products / offers                     │ Live Feed                │
└──────────────────────────────────────┴──────────────────────────┘
```

## 23.3 KPI Card Contract

Every KPI card supports:

- current value
- comparison period
- delta
- trend direction
- freshness
- confidence where applicable
- provenance
- drill-down target

```yaml
component: kpi_card
required:
  - metric_id
  - value
  - unit
  - freshness
  - source_class
  - display_state
optional:
  - comparison_period
  - delta
  - confidence
  - provenance
  - drilldown_route
constraints:
  simulated_data_requires_label: true
  forecast_requires_label: true
  color_only_status: false
```

## 23.4 Core KPI Families

The approved Command Center direction calls for revenue, profit, clicks/traffic, conversions, active campaigns, and EPC/unit-economic signals, plus performance, channel, product, activity, AI Brain, and live-feed surfaces. citeturn71file0

The implementation must distinguish financial truth owned by the appropriate engine from UI-computed presentation metrics.

---

# Section 24 — Global Performance and Strategic Intelligence Surface

The Global Performance surface is the strategic counterpart to the operational Command Center.

## 24.1 Strategic Layout

```text
┌─────────────────────────────────────────────────────────────┐
│ GLOBAL PERFORMANCE · WORLD / NETWORK CONTEXT                │
├──────────────────────┬──────────────────────────────────────┤
│ Revenue / Profit     │ Markets / Countries                  │
├──────────────────────┼──────────────────────────────────────┤
│ Channel Mix          │ Affiliate Network                     │
├──────────────────────┴──────────────────────────────────────┤
│                         CAT BRAIN                             │
│        global graph / world / intelligence flows              │
├──────────────────────────────┬───────────────────────────────┤
│ Top Products                 │ AI Insights / Council         │
├──────────────────────────────┼───────────────────────────────┤
│ Content Factory              │ Knowledge Graph               │
├──────────────────────────────┼───────────────────────────────┤
│ Workflow Engine              │ Model Router / System Status  │
└──────────────────────────────┴───────────────────────────────┘
```

The V1 baseline names global KPIs, active markets, channel mix, affiliate network mix, top products, recent activity, AI insights, CAT AI Council, Content Factory, Knowledge Graph, Workflow Engine, Model Router, System Status, and CAT Brain/world visualization as conceptual zones. citeturn71file0

## 24.2 World Visualization

The world view is a strategic spatial metaphor. It can represent:

- market activity
- affiliate network density
- content distribution
- opportunity propagation
- risk concentration
- agent activity
- knowledge relationships

It must never encode exact geographic truth unless the underlying data contract provides verified geography.

## 24.3 Strategic Drill-Down

Clicking a market should preserve global context while opening a scoped workspace:

`Global → Region → Country → Market → Channel → Campaign → Offer → Evidence`

Back navigation must preserve filters, time range, and selected entities.

---

# Section 25 — CAT Brain Experience

CAT Brain is the visual center of the system-level intelligence fabric. The existing UI/UX baseline explicitly states that it may visualize global network state, knowledge density, intelligence flows, engine health, agent activity, economic signals, opportunity propagation, and learning state, while never becoming the source of truth itself. citeturn71file0

## 25.1 Visual Anatomy

```text
                       KNOWLEDGE
                          ·
                    ·     │     ·
              AGENTS ─── CAT BRAIN ─── ENGINES
                    ·     │     ·
                  ECONOMY │  OPPORTUNITY
                          │
                       LEARNING
```

The core should feel alive without becoming visually noisy. Activity is represented through restrained motion, signal pulses, graph expansion, and controlled particle/network behavior.

## 25.2 Brain Modes

| Mode | Meaning |
|---|---|
| Operational | current runtime activity |
| Intelligence | derived signals and relationships |
| Knowledge | graph density and retrieval activity |
| Economic | revenue/value/capital signals |
| Agent | agent coordination and workload |
| Risk | anomalies, threats, blocked actions |
| Learning | experiments, feedback, model improvement |
| Simulation | hypothetical future state |

## 25.3 Interaction Rules

The Brain supports progressive disclosure:

1. overview
2. hover/selection
3. node context
4. relationship inspection
5. evidence panel
6. source/provenance
7. related decision or recommendation
8. authorized action

The visualization must not jump directly from a visual signal to an irreversible action.

## 25.4 Motion Principles

- idle motion is subtle
- active nodes pulse briefly
- critical events use controlled attention motion
- transitions communicate causality
- no perpetual decorative animation that competes with data
- reduced-motion mode disables non-essential animation

## 25.5 CAT Brain JSON Contract

```json
{
  "record_type": "cat_brain_signal",
  "signal_id": "SIG-EXAMPLE",
  "mode": "INTELLIGENCE",
  "entity_ids": ["entity-1", "entity-2"],
  "source_class": "derived_intelligence",
  "confidence": 0.91,
  "freshness_seconds": 8,
  "provenance_ref": "PROV-EXAMPLE",
  "recommended_action": null,
  "execution_authorized": false
}
```

## 25.6 Constitutional Rule

**CAT Brain may explain and expose relationships; it may not silently promote a visualization into canonical truth, a recommendation into a decision, or a decision into execution.**

---

# Part 1 Completion Direction

Sections 21–25 establish the foundation for the remaining UI/UX continuation: agent operations, approval workflows, command palette, analytics surfaces, workflow visualization, knowledge graph interaction, notification architecture, data freshness, responsive/mobile behavior, accessibility, error/recovery states, and the final Part 1 completion contract.

The visual baseline remains the approved CAT Command Center + CAT Global Performance / CAT Brain direction. Contemporary agent control-plane research reinforces the same architectural trend: production agent interfaces increasingly need live execution state, approvals, exception routing, auditability, and unified operational visibility rather than a generic chat surface. citeturn0search1turn0search3turn0search4

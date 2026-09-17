# CAT OMNISYSTEM — UI/UX Bible V1 — Part 1 Continuation II

> **Status:** Draft / implementation-grade continuation
>
> **Frozen visual baseline:** `context/10_UI_UX.md`
>
> **Design rule:** preserve the approved CAT Command Center + CAT Global Performance / CAT Brain visual language. This document extends interaction architecture; it does not replace the visual identity.

## 26 — Agent Operations Surface

### Purpose

AI agents are first-class operational entities. The approved baseline requires agent cards to expose name, role, state, task, health, confidence where applicable, authority scope, recent activity, escalation state, and model/runtime information. fileciteturn71file0L2-L6

### Agent Command Center

```text
┌────────────────────────────────────────────────────────────────────┐
│ AI AGENTS · 24 ONLINE · 3 ATTENTION · 2 APPROVALS                 │
├──────────────┬──────────────────────────────────┬──────────────────┤
│ FILTERS      │ ACTIVE RUN                        │ CONTEXT          │
│ role         │ Product Hunter                    │ authority        │
│ state        │ ███████░░ 72%                    │ model            │
│ health       │ task: discover high-EPC offers    │ budget           │
│ risk         │ tools: search → rank → propose    │ policy           │
├──────────────┼──────────────────────────────────┼──────────────────┤
│ AGENT LIST   │ EVENT TIMELINE                    │ ACTIONS          │
│ Hunter       │ 12:41 search completed            │ pause            │
│ Analyst      │ 12:42 evidence attached          │ inspect          │
│ Writer       │ 12:43 recommendation generated   │ escalate         │
│ Publisher    │ 12:44 awaiting approval          │ revoke           │
└──────────────┴──────────────────────────────────┴──────────────────┘
```

### Agent Card Contract

```json
{
  "record_type":"agent_presence",
  "agent_id":"agent.product-hunter",
  "display_name":"Product Hunter",
  "role":"discovery",
  "state":"WAITING_APPROVAL",
  "task_id":"TASK-001",
  "health":"healthy",
  "authority_scope":["read:catalog","propose:offer"],
  "approval_required":true,
  "model":"MODEL-ROUTER-SELECTED",
  "runtime":"agent-runtime-01",
  "last_event_at":"2026-08-17T00:00:00Z"
}
```

### Interaction Rules

- Selecting an agent opens its run context without losing the global command context.
- `Thinking`, `waiting`, `executing`, `blocked`, and `completed` are distinct states.
- The UI must never imply that an agent is executing when it is only generating a recommendation.
- Authority scope is visible before consequential actions.
- Pause, revoke, and escalation are control-plane operations and require confirmation according to risk.

---

## 27 — Approval Inbox and Human-in-the-Loop Control Plane

Approval must be a **decision surface**, not a confirmation modal. Recent agent-control-plane research emphasizes that consequential approval needs action payload, evidence, authority, uncertainty, alternatives, expiry, audit, and enforcement—not merely an Approve button. citeturn0search1turn0search3turn0search10

### Approval Queue

```text
┌──────────────────────────────────────────────────────────────────┐
│ APPROVAL INBOX                                                  │
├──────┬───────────────┬────────┬──────────┬───────────┬──────────┤
│ RISK │ ACTION        │ AGENT  │ IMPACT   │ EXPIRES   │ STATE    │
├──────┼───────────────┼────────┼──────────┼───────────┼──────────┤
│ HIGH │ publish offer │ Writer │ market   │ 04:12     │ PENDING  │
│ HIGH │ increase bid  │ Optim. │ €2,400   │ 01:41     │ PENDING  │
│ MED  │ create draft  │ Writer │ content  │ 18:30     │ PENDING  │
└──────┴───────────────┴────────┴──────────┴───────────┴──────────┘
```

### Decision Packet

Every approval view should expose:

1. exact proposed action
2. target and scope
3. current state
4. proposed delta
5. evidence
6. policy result
7. agent identity
8. model/runtime identity
9. confidence/uncertainty
10. financial impact if applicable
11. alternatives considered
12. reversibility
13. approval expiry
14. previous related decisions
15. audit destination

### Approval State Machine

```text
PROPOSED
   ↓
CLASSIFIED
   ↓
EVIDENCE_READY
   ↓
ROUTED
   ↓
PENDING_APPROVAL
   ├── APPROVED → EXECUTION_REQUESTED → EXECUTING → COMPLETED
   ├── DENIED ───────────────────────────────→ CLOSED
   ├── EDITED → RECLASSIFIED
   ├── EXPIRED ──────────────────────────────→ CLOSED
   └── ESCALATED → HIGHER_AUTHORITY

EXECUTING → FAILED → ROLLBACK_RECOMMENDED → ROLLED_BACK / ACCEPTED_EXCEPTION
```

The backend control plane must remain authoritative for these transitions. Contemporary agent-control-plane implementations similarly separate approval enforcement from probabilistic agent behavior. citeturn0search0turn0search6

### Approval Contract

```yaml
component: approval_request
required:
  - approval_id
  - action_id
  - agent_id
  - authority_scope
  - risk_class
  - evidence_refs
  - proposed_payload
  - policy_result
  - expires_at
  - audit_ref
states:
  - proposed
  - classified
  - evidence_ready
  - routed
  - pending
  - approved
  - denied
  - edited
  - expired
  - escalated
  - execution_requested
  - executing
  - completed
  - failed
  - rolled_back
constraints:
  frontend_is_not_state_machine_authority: true
  approved_payload_must_match_execution_payload: true
  expired_approval_cannot_execute: true
```

---

## 28 — Command Palette and Global Intent Interface

The command palette is CAT's universal intent entry point. It is not a chatbot replacement for structured navigation; it is a fast route into governed operations.

### Interaction Model

```text
USER INTENT
   ↓
PARSE
   ↓
SCOPE RESOLUTION
   ↓
CAPABILITY CHECK
   ↓
CONTEXT / EVIDENCE
   ↓
PREVIEW
   ↓
RECOMMENDATION or ACTION
   ↓
POLICY / APPROVAL
   ↓
EXECUTION
```

### Example

`Increase Germany campaign budget by 10% for the next 24h`

The UI should transform that intent into a preview:

```text
ACTION PREVIEW
────────────────────────────────────
Target: Campaign DE-SEARCH-04
Current budget: €2,000/day
Proposed: €2,200/day
Duration: 24h
Expected impact: forecast +€310 revenue
Risk: MEDIUM
Authority: recommendation → approval required
Evidence: 7 signals / 3 forecasts
[ REVIEW ] [ CANCEL ]
```

### Command Palette Rules

- Never execute an ambiguous command.
- Show interpreted scope before execution.
- Preserve user wording in the audit record.
- Display resolved entities before mutation.
- Commands that mutate canonical truth require the relevant engine's policy boundary.
- Destructive commands require explicit confirmation and, where mandated, human approval.

---

## 29 — Notifications, Alerts, and Attention Architecture

Notifications are operational routing, not decoration.

### Severity Model

| Level | Meaning | UX |
|---|---|---|
| CRITICAL | immediate operational threat | persistent banner + inbox |
| HIGH | consequential action/approval | approval queue + alert |
| MEDIUM | action recommended | intelligence feed |
| LOW | informational | activity feed |
| INFO | routine event | quiet feed |

### Attention Budget

CAT must prevent alert fatigue. A notification is promoted to a stronger surface only when urgency, impact, confidence, and user responsibility justify it.

```text
SIGNAL
  ↓
DEDUPLICATE
  ↓
CORRELATE
  ↓
SEVERITY
  ↓
ROUTE
  ├── Banner
  ├── Approval Inbox
  ├── Agent Rail
  ├── Activity Feed
  └── Audit Only
```

### Notification Object

```json
{
  "record_type":"ui_notification",
  "notification_id":"NTF-001",
  "severity":"HIGH",
  "category":"approval",
  "title":"Bid increase requires approval",
  "entity_refs":["campaign-04"],
  "action_route":"/approvals/APP-001",
  "expires_at":"2026-08-17T03:00:00Z",
  "dedupe_key":"campaign-04-bid-approval",
  "source_event":"EVT-123",
  "read":false
}
```

---

## 30 — Agent Run Timeline and Execution Trace UX

A production agent surface needs a run-aware timeline showing events, approvals, tool activity, resumes, failures, and outcomes in order. This pattern is also visible in current AgentOps/control-plane implementations. citeturn0search4turn0search15

### Timeline

```text
00:00  RUN CREATED
00:01  CONTEXT LOADED
00:02  KNOWLEDGE RETRIEVED     12 sources
00:04  PLAN GENERATED
00:05  TOOL CALL               catalog.search
00:07  EVIDENCE ATTACHED        8 records
00:08  RECOMMENDATION CREATED
00:08  POLICY CHECK             PASS
00:09  APPROVAL REQUESTED
00:18  HUMAN APPROVED
00:19  EXECUTION STARTED
00:21  TOOL RESULT               SUCCESS
00:22  POST-CONDITION VERIFIED
00:23  RUN COMPLETED
```

### Event Types

- lifecycle
- model
- retrieval
- reasoning-summary
- tool-call
- tool-result
- policy
- approval
- execution
- rollback
- error
- escalation
- outcome

The UI should expose **structured reasoning summaries and evidence references**, not hidden chain-of-thought.

---

## 31 — Workflow Visualization

Workflow visualization converts complex automation into an inspectable graph.

```text
[Trigger]
   ↓
[Context]
   ↓
[Research] ───────→ [Knowledge]
   ↓
[Decision]
   ↓
◇ Approval Required?
 ├─ No ─→ [Execute]
 └─ Yes → [Human Gate]
             ↓
          [Execute]
             ↓
        [Verify Result]
          ├── Success → [Close]
          └── Failure → [Recover]
```

### Node Requirements

Each node can expose:

- state
- owner
- duration
- input/output references
- policy result
- retry count
- confidence where meaningful
- linked audit events

### Graph Interaction

- zoom/pan
- focus selected node
- collapse completed branches
- inspect evidence
- replay historical execution
- compare runs
- identify bottlenecks

The graph is a visual representation of backend workflow state; it is not the workflow authority.

---

## 32 — Knowledge Graph Interaction Surface

Knowledge Graph UI connects CAT Brain, evidence, entities, content, affiliates, campaigns, products, agents, and decisions.

### Graph Surface

```text
                    [MERCHANT]
                    /        \
             [PROGRAM]      [PRODUCT]
                 |              |
             [OFFER]──────[CONTENT]
                |             |
          [AFFILIATE]────[CAMPAIGN]
                \             /
                 [DECISION]
                      |
                   [AGENT]
```

### Evidence-first Inspection

Selecting an edge should reveal:

- relationship type
- source
- timestamp
- confidence
- provenance
- validity interval
- who/what created it
- whether it is canonical or derived

### Graph Safety

The UI must visually distinguish:

`canonical edge` vs `inferred edge` vs `predicted edge` vs `simulation edge`.

An inferred relationship must never look identical to a canonical relationship.

---

## 33 — Search, Retrieval, and Evidence UX

Search in CAT is entity-aware and evidence-aware.

### Search Result Anatomy

```text
QUERY: "best high-EPC offers in Germany"

┌────────────────────────────────────────────────────────────┐
│ OFFER A                                                     │
│ EPC €4.21 · VERIFIED · updated 4m ago                     │
│ Evidence 12 · Confidence 0.93                             │
│ [Inspect evidence] [Compare] [Recommend]                   │
├────────────────────────────────────────────────────────────┤
│ OFFER B                                                     │
│ EPC €3.87 · INFERRED · updated 19m ago                    │
│ Evidence 6 · Confidence 0.76                              │
└────────────────────────────────────────────────────────────┘
```

The result ranking may be intelligent, but ranking signals must not be represented as canonical facts.

---

## 34 — Data Freshness and Temporal UX

Freshness is a first-class visual property.

### Freshness States

```text
LIVE          < 10s
CURRENT       < 5m
RECENT        < 1h
STALE         >= 1h
EXPIRED       outside validity interval
UNKNOWN       no trustworthy timestamp
```

Exact thresholds are metric-specific and must come from the data contract; the above values are UX examples, not universal engine rules.

### Display Rules

- Never hide stale data behind a green status.
- Show last-updated time on high-value operational metrics.
- Forecasts show generation time and forecast horizon.
- Cached data shows cache age.
- Conflicting sources expose the conflict rather than silently choosing one.

---

## 35 — Responsive and Mobile Experience

CAT's mobile UI is not a miniature desktop dashboard.

### Mobile Priority

```text
1. system health
2. approvals
3. critical alerts
4. active agent runs
5. key KPIs
6. decisions
7. intelligence
8. detailed analytics
```

### Mobile Shell

```text
┌──────────────────────────────┐
│ CAT · HEALTH · ALERT · USER  │
├──────────────────────────────┤
│ Revenue   Profit   Conv.     │
├──────────────────────────────┤
│ APPROVAL REQUIRED            │
│ Bid increase · €2,400        │
│ [Review]                     │
├──────────────────────────────┤
│ ACTIVE AGENTS                │
│ Hunter  ● running            │
│ Writer  ◐ waiting            │
├──────────────────────────────┤
│ CAT BRAIN                    │
│ compact intelligence view    │
├──────────────────────────────┤
│ Home Agents Approvals More   │
└──────────────────────────────┘
```

Mobile actions must be optimized for quick inspection and safe approval, not dense data manipulation.

---

## 36 — Accessibility and Inclusive Interaction

Accessibility is a structural requirement, not a later polish phase.

### Requirements

- keyboard-complete navigation
- visible focus state
- semantic landmarks
- screen-reader labels
- logical heading hierarchy
- no color-only status communication
- reduced-motion mode
- sufficient contrast
- touch targets appropriate for mobile
- confirmation flows usable without pointer precision
- charts accompanied by accessible textual summaries
- graph nodes reachable without drag-only interaction

Current approval UX implementations also treat keyboard navigation, focus trapping, and ARIA labels as first-class dashboard requirements. citeturn0search8

### Accessibility State Example

```yaml
component: approval_modal
accessibility:
  role: dialog
  labelled_by: approval-title
  described_by: approval-summary
  focus_trap: true
  escape_behavior: close_without_action
  keyboard_actions:
    approve: ctrl+enter
    deny: ctrl+backspace
  color_only_status: false
```

---

## 37 — Loading, Empty, Error, and Recovery States

Every production surface requires explicit non-happy-path design.

### State Matrix

| State | Required UI |
|---|---|
| Loading | skeleton preserving layout |
| Partial | loaded regions + explicit pending regions |
| Empty | explanation + next useful action |
| Stale | freshness warning |
| Error | cause class + retry/recovery |
| Forbidden | permission explanation |
| Blocked | policy explanation + resolution route |
| Offline | cached state + offline indicator |
| Recovering | progress + affected scope |
| Unknown | uncertainty rather than fabricated value |

### Recovery Pattern

```text
ERROR
 ↓
CLASSIFY
 ↓
EXPLAIN
 ↓
SAFE ACTION
 ├─ Retry
 ├─ Resume
 ├─ Rollback
 ├─ Escalate
 └─ Inspect
 ↓
AUDIT
```

The interface must not make a failed execution appear successful merely because the request was accepted by the UI.

---

## 38 — Analytics and BI Surface Grammar

Analytics screens must follow the same visual hierarchy as the Command Center while allowing deeper analysis.

### Analytical Layers

```text
KPI
 ↓
TREND
 ↓
SEGMENT
 ↓
DRILLDOWN
 ↓
EVIDENCE
 ↓
DECISION / RECOMMENDATION
```

### Chart Rules

- chart titles describe the business question
- axes expose units
- time ranges are explicit
- simulated/demo data is labeled
- forecast ranges distinguish historical and predicted regions
- annotations reference evidence/events
- interactions preserve selected filters
- visual emphasis should follow operational significance

### Required Analytical Interactions

- time-range selection
- comparison period
- segment filters
- drill-through
- anomaly inspection
- export
- saved view
- shareable scoped URL

---

## 39 — Executive Decision Surface

Executives need a compressed view of what changed, why it matters, what CAT recommends, what has already been decided, and what requires human authority.

```text
┌──────────────────────────────────────────────────────────────┐
│ EXECUTIVE BRIEF · 09:30                                      │
├──────────────────────────────────────────────────────────────┤
│ WHAT CHANGED                                                 │
│ Germany EPC +18%                                             │
├──────────────────────────────────────────────────────────────┤
│ WHY                                                           │
│ 3 verified signals + 1 forecast                              │
├──────────────────────────────────────────────────────────────┤
│ CAT RECOMMENDS                                                │
│ increase budget 8%                                           │
│ confidence 0.88 · risk MED                                  │
├──────────────────────────────────────────────────────────────┤
│ DECISION                                                     │
│ awaiting VP Marketing approval                               │
├──────────────────────────────────────────────────────────────┤
│ EXECUTION                                                    │
│ not started                                                  │
└──────────────────────────────────────────────────────────────┘
```

This directly implements the project's core visual separation between insight, recommendation, decision, and execution.

---

## 40 — UI State, URL State, and Persistence

Navigation state must be deterministic and recoverable.

### State Classes

```text
SERVER STATE
  canonical/derived/runtime data

VIEW STATE
  filters / tabs / sort / expanded nodes

SESSION STATE
  current workspace / recent entities

USER PREFERENCE
  density / theme / reduced motion / defaults

URL STATE
  shareable scoped navigation
```

Only appropriate state classes may persist. Canonical domain truth never becomes browser-owned truth.

### URL Contract

Example:

`/command-center?market=DE&range=30d&view=network&entity=campaign-04`

Refreshing the browser must preserve safe view context without replaying an action.

---

## 41 — Audit and Human-Readable History

Audit is an operational UI, not a database dump.

### Audit Timeline

```text
WHO / WHAT / WHEN / WHY / AUTHORITY / RESULT

09:11  Agent Optimizer proposed bid +8%
09:11  Policy classified MEDIUM risk
09:12  Evidence packet attached
09:13  Afshiiin approved within authority scope
09:13  Broker executed approved payload
09:14  Campaign API returned success
09:14  Post-condition verified
09:15  Audit receipt sealed
```

The history must allow users to move from human-readable event → technical event → evidence → execution result.

### Immutable Audit View

Users may filter, inspect, export, and correlate audit events, but the UI must not offer editing of immutable history.

---

## 42 — Trust, Risk, and Authority Visualization

Trust is built through visible boundaries.

### Agent Trust Panel

```text
AGENT: Ad Optimizer
──────────────────────────────
AUTHORITY
  Read campaigns       ✓
  Propose budget       ✓
  Execute budget       ✕
  Spend money          ✕

RISK
  Current: MEDIUM
  Policy: APPROVAL_REQUIRED

BUDGET
  Proposed: €2,400
  Limit: €5,000

EVIDENCE
  12 verified
  2 inferred
  1 forecast

LAST ACTION
  11m ago · completed
```

A trust surface must answer three questions quickly: **what can this agent do, what can it spend/change, and what has it already done?** This pattern is increasingly emphasized in current agent safety UX. citeturn0search11

---

## 43 — Design Tokens and Visual Language

The existing baseline defines a dark near-black/navy environment, electric blue network illumination, controlled cyan/violet/green/amber/gold accents, premium CAT gold identity, glass/holographic panels, dense information architecture, and restrained depth. fileciteturn71file0L2-L6

### Token Families

```yaml
color:
  canvas: near-black-navy
  surface: translucent-navy
  border: cool-blue-low-alpha
  primary: electric-blue
  intelligence: cyan
  agent: violet
  success: controlled-green
  warning: amber
  critical: red
  identity: cat-gold
  text_primary: cool-white
  text_secondary: muted-blue-white
motion:
  fast: 120ms
  normal: 180ms
  deliberate: 280ms
  reduced_motion: 0ms
radius:
  card: 14px
  panel: 18px
  control: 10px
```

These are implementation starting tokens, not a replacement for the final design-system specification.

### Glass Rules

Glass surfaces must maintain readable contrast. Blur and transparency are subordinate to information clarity. Avoid stacking many translucent layers over animated backgrounds.

---

## 44 — Visual Density and Information Architecture

CAT should feel **dense, intelligent, and premium**, but never chaotic.

### Density Modes

| Mode | Intended user |
|---|---|
| Executive | leadership |
| Standard | operations |
| Dense | analysts/operators |
| Focus | single-task approval |
| Mobile | field/urgent |

Density changes spacing and secondary metadata, not the semantic hierarchy.

### Progressive Disclosure

```text
LEVEL 0 — signal
LEVEL 1 — summary
LEVEL 2 — context
LEVEL 3 — evidence
LEVEL 4 — technical detail
LEVEL 5 — audit / raw event
```

Users should not need to absorb all technical complexity to perform routine tasks.

---

## 45 — Part 1 Completion Contract

Part 1 is complete when the UI/UX foundation provides a coherent operational shell that preserves the approved CAT visual identity while making authority, agent state, approval, evidence, freshness, execution, audit, responsive behavior, and accessibility structurally visible.

### Closed Registry — Primary Surfaces

```yaml
registry: CAT-UI-P1-SURFACES
entries:
  - global_shell
  - command_center
  - global_performance
  - cat_brain
  - agent_operations
  - approval_inbox
  - command_palette
  - notifications
  - run_timeline
  - workflow_visualizer
  - knowledge_graph
  - search_evidence
  - analytics_bi
  - executive_decision
  - audit_history
  - trust_authority
```

### Closed Registry — Core Interaction States

```json
{
  "record_type":"ui_interaction_state_registry",
  "states":[
    "idle","loading","active","thinking","waiting","pending_approval",
    "approved","denied","executing","completed","failed","blocked",
    "escalated","expired","rolled_back","stale","offline","forbidden"
  ],
  "immutable":true
}
```

### Constitutional Acceptance Tests

1. A recommendation cannot visually appear as an executed action.
2. A high-impact approval exposes action, scope, evidence, authority, risk, expiry, and outcome.
3. Browser refresh cannot replay a mutation.
4. CAT Brain remains a visualization layer, not canonical truth.
5. Agent authority is visible before consequential execution.
6. Stale and forecast data are explicitly represented.
7. Audit history cannot be edited through UI controls.
8. Mobile preserves critical operational priorities.
9. Keyboard and reduced-motion modes remain functional.
10. Error and recovery states are explicit.

### Architecture Invariant

```text
             HUMAN
               │
        ┌──────▼──────┐
        │     UI      │
        └──────┬──────┘
               │ intent / review
        ┌──────▼──────┐
        │ CONTROL     │
        │ PLANE       │
        └──────┬──────┘
               │ policy / authority
        ┌──────▼──────┐
        │ AGENT /     │
        │ WORKFLOW    │
        └──────┬──────┘
               │ execution
        ┌──────▼──────┐
        │ DOMAIN      │
        │ ENGINES     │
        └─────────────┘
```

The UI informs, routes, previews, requests approval, observes, and explains. Deterministic backend control planes enforce authority and state. This separation is consistent with current agent-control-plane architectures that place policy enforcement, approvals, budgets, and audit outside probabilistic execution. citeturn0search0turn0search6

### Part 1 Status

**CAT UI/UX Foundation — Part 1 — COMPLETE FOR THIS ITERATION**

**Next:** Part 2 should define detailed page-level UX for Campaigns, Affiliate Intelligence, Content Factory, Agent Studio, Knowledge Brain, Automation, Analytics/BI, Marketplace/Plugins, Settings, and cross-engine operational workflows, while preserving every constitutional UI rule defined above.

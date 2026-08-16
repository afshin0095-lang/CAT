# CAT OMNISYSTEM — UI/UX Bible V1

> Canonical UI/UX design reference for CAT OMNISYSTEM. This document is based on the two approved CAT Command Center and Global Performance / CAT Brain reference designs supplied for the project.

## 1. Design Identity

CAT is not a conventional SaaS dashboard. The interface is a governed operational control plane between humans, AI agents, engines, canonical truth, derived intelligence, recommendations, decisions, execution, and audit.

Core visual identity:
- futuristic command-center / AI operations aesthetic
- deep near-black/navy environment
- electric blue network illumination
- controlled cyan, violet, green, amber, and gold accents
- premium CAT gold identity mark
- glass / holographic panels with restrained depth
- dense information architecture without visual clutter
- cinematic CAT Brain as the central visual anchor
- responsive layouts that preserve hierarchy rather than simply collapsing desktop panels

## 2. Approved Reference Designs

The two supplied reference images are authoritative visual references for the UI direction:

1. **CAT Command Center** — primary operational shell.
2. **CAT Global Performance / CAT Brain** — strategic/global intelligence shell.

Reference assets are intended to live at:

```text
assets/ui/cat-command-center-reference.jpg
assets/ui/cat-global-performance-reference.jpg
```

The references define the visual language, composition, information density, panel hierarchy, CAT Brain treatment, agent rail, KPI cards, command navigation, and strategic intelligence surfaces. They are references, not pixel-locked implementation screenshots.

## 3. Constitutional UI Boundary

The UI is a presentation and interaction surface. It is never the canonical authority source.

```text
Canonical Truth
      ↓
Derived Intelligence
      ↓
Recommendation
      ↓
Decision
      ↓
Execution
      ↓
Audit / Observability
```

The interface must visually distinguish these states. In particular:
- AI recommendation ≠ executed action.
- Prediction ≠ fact.
- Insight ≠ decision.
- Decision ≠ execution.
- UI state ≠ canonical domain truth.
- Human approval gates must be structurally visible where required.

## 4. Primary Shell

The primary CAT shell consists of:
- global top bar
- left navigation rail
- contextual page header
- central workspace
- optional right-side agent / context rail
- bottom intelligence/status surfaces where appropriate
- command palette
- notifications and audit access

### 4.1 Global Navigation

Primary navigation follows the approved Command Center composition and may include:
- Command Center
- Dashboard
- AI Agents
- Campaigns
- Affiliate Intelligence
- Content Factory
- Ad OS
- Analytics & BI
- Knowledge Brain
- Automation
- Settings
- Plugins
- Marketplace

The exact navigation is permission-aware and capability-aware; unavailable modules must not appear as fake active features.

## 5. Command Center

The Command Center is the default executive-operational surface.

Required visual zones:
- revenue KPI
- profit KPI
- clicks / traffic KPI
- conversions KPI
- active campaigns KPI
- EPC / unit-economic KPI
- CAT Brain central visualization
- engine online/offline status
- AI agent activity rail
- campaign performance chart
- revenue breakdown
- channel performance
- top products
- system activity
- AI Brain status
- live feed
- AI insights
- quick actions

The CAT Brain is the primary focal point and must not be reduced to a decorative background image.

## 6. Global Performance Surface

The strategic surface expands the Command Center into a global operating view.

Required conceptual zones:
- global performance KPIs
- active countries / markets
- channel mix
- affiliate network mix
- top products
- recent activity
- AI insights
- CAT AI Council
- Content Factory production state
- Knowledge Graph
- Workflow Engine
- Model Router
- System Status
- CAT Brain / world visualization

## 7. CAT Brain

CAT Brain represents the system-level intelligence fabric.

It may visualize:
- global network state
- knowledge graph density
- active intelligence flows
- engine health
- agent activity
- economic signals
- opportunity propagation
- learning state

It must not imply that the visualization itself is the source of truth.

## 8. AI Agent Experience

Agents are first-class operational entities.

Agent cards should expose at minimum:
- agent name
- role
- state
- current task
- health
- confidence where applicable
- authority scope
- recent activity
- escalation state
- model/runtime information where appropriate

Examples from the reference direction include Product Hunter, Trend Analyzer, Content Writer, Image Creator, Video Creator, Publisher, Ad Optimizer, Analytics Agent, Research Agent, Marketing Agent, Content Agent, Creative Agent, Finance Agent, and CEO Agent.

Agent identity must be visually distinct from human identity.

## 9. Human / Agent Responsibility

The UI must clearly show when an action is:
- autonomous and policy-authorized
- recommendation-only
- awaiting human approval
- blocked by policy
- escalated
- executed
- failed / rolled back

Approval states must never be represented only by a tiny notification.

## 10. Information Hierarchy

Priority order:
1. system health / blocking conditions
2. financial and operational truth
3. active decisions and approvals
4. agent execution state
5. intelligence and anomalies
6. trends and forecasts
7. recommendations
8. secondary metadata

Visual hierarchy should reflect operational importance rather than decoration.

## 11. KPI Cards

KPI cards should support:
- current value
- delta
- comparison period
- directionality
- confidence / freshness where relevant
- drill-down
- provenance where required

Do not display fabricated real-time numbers in production surfaces. Demo data must be explicitly identified as demo/simulated data.

## 12. Intelligence Surfaces

AI Insights should distinguish:
- observation
- detected anomaly
- forecast
- recommendation
- opportunity
- risk
- decision request

Every actionable insight should expose provenance and an appropriate path to the underlying evidence.

## 13. Charts and Data Visualization

Charts should follow the CAT visual language while remaining readable:
- dark plotting surfaces
- restrained luminous accents
- high contrast axes and labels
- tooltips with exact values
- clear units
- comparison periods
- confidence bands where relevant
- no ornamental 3D charts when a simpler chart communicates better

## 14. Knowledge Graph UI

The Knowledge Brain / Knowledge Graph surface must support:
- node exploration
- relationship inspection
- provenance
- confidence
- temporal state
- entity type
- evidence links
- graph filtering
- graph-to-decision traceability

Graph visualization is derived intelligence and must not silently overwrite canonical records.

## 15. Workflow Engine UI

Workflow views should expose:

```text
Trigger → Plan → Generate → Validate → Approve → Execute → Observe → Learn
```

Each step must have a state and timestamp. Failed steps must expose recovery/escalation actions where authorized.

## 16. Model Router UI

Model routing surfaces should expose:
- selected model
- fallback model
- task class
- quality / latency / cost trade-off
- availability
- privacy/security classification
- routing reason

The router is an operational mechanism, not a marketing badge.

## 17. System Status

System status should expose meaningful infrastructure state:
- CPU
- memory
- storage
- uptime
- database connectivity
- event bus
- agent runtime
- backup state
- degraded services
- active incidents

Status must be evidence-backed by observability infrastructure.

## 18. Responsive / Mobile Strategy

The UI must preserve the command-center hierarchy on mobile rather than merely shrinking desktop.

Mobile priorities:
1. critical alerts
2. KPI strip
3. current decision / approval
4. active agents
5. core intelligence
6. charts
7. secondary modules

The CAT Brain may become a compact intelligence header on narrow screens.

## 19. Accessibility

Required baseline:
- keyboard navigation
- visible focus states
- sufficient contrast
- non-color status indicators
- readable typography
- reduced-motion mode
- screen-reader labels for interactive controls
- semantic headings
- accessible chart summaries

## 20. UI/UX Completion Contract

The UI/UX system is complete only when the implementation preserves the approved visual language and the operational constitutional boundaries.

### Closed Principles
- Command Center is an operational control plane.
- CAT Brain is the central intelligence visualization.
- AI agents are first-class operational entities.
- Human authority and AI authority are visually distinguishable.
- Recommendation, decision, execution, and audit states are distinct.
- Canonical truth is never represented as merely a visual state.
- Reference designs guide composition and visual identity without forcing pixel duplication.
- Demo/simulated data is explicitly identified.
- Accessibility and responsive behavior are constitutional requirements.

### Reference Asset Registry
| ID | Asset | Purpose |
|---|---|---|
| CAT-UI-REF-001 | `assets/ui/cat-command-center-reference.jpg` | Primary operational shell reference |
| CAT-UI-REF-002 | `assets/ui/cat-global-performance-reference.jpg` | Strategic global intelligence reference |

### Acceptance Criteria
- Command Center hierarchy is preserved.
- Global Performance hierarchy is preserved.
- CAT Brain remains a meaningful system-intelligence surface.
- Agent authority and execution states are explicit.
- Canonical truth is not confused with UI state.
- Human approval gates are visible and actionable.
- Responsive/mobile hierarchy is intentionally designed.
- Accessibility requirements are implemented.
- Reference assets remain traceable from the repository.

**Status:** Planned — canonical UI/UX specification.

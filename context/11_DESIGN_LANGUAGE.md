# CAT OMNISYSTEM — Design Language Bible V1

> Canonical visual design language for CAT OMNISYSTEM. This document converts the approved CAT Command Center and CAT Global Performance / CAT Brain references into reusable visual rules, tokens, component behavior, and AI implementation constraints.

**Status:** Canonical Draft V1
**Version:** 1.0.0
**Project:** CAT (Commerce AI Trinity)
**Company:** Omni System
**Governing UI/UX:** `context/10_UI_UX.md`
**Last Updated:** 2026-08-17

---

## 1. Design Language Identity

CAT must look and behave like an **AI-driven autonomous commerce command system**, not a generic SaaS dashboard.

The design language combines:

- futuristic command-center architecture
- near-black / deep-navy spatial surfaces
- electric-blue network illumination
- controlled cyan, violet, green, amber, and CAT-gold semantic accents
- premium metallic CAT identity
- restrained glass and holographic depth
- dense operational information without visual noise
- cinematic CAT Brain as a meaningful intelligence surface
- enterprise-grade data clarity
- visible distinction between observation, intelligence, recommendation, decision, execution, and audit

The two approved visual references define the direction:

1. **CAT Command Center** — operational control-plane language.
2. **CAT Global Performance / CAT Brain** — strategic intelligence and global-scale language.

These references are **design authorities**, not pixel-copy targets. New screens must inherit their grammar while adapting composition to the task.

---

## 2. Constitutional Design Principles

### DL-001 — Command System, Not Dashboard

Every major surface must communicate operational purpose. Decorative complexity must never compete with the user's current decision or task.

### DL-002 — Intelligence Has Layers

The visual system must distinguish:

```text
Observation
   ↓
Evidence
   ↓
Intelligence
   ↓
Prediction / Forecast
   ↓
Recommendation
   ↓
Decision
   ↓
Execution
   ↓
Audit
```

### DL-003 — Truth Is Visually Protected

Canonical truth must never look identical to an AI-generated insight, estimate, prediction, or recommendation.

### DL-004 — Authority Is Visible

Human authority, agent authority, policy authority, and system state must be distinguishable without requiring the user to inspect raw metadata.

### DL-005 — Density Without Chaos

CAT may be information-dense, but every dense surface must have a dominant hierarchy, grouping logic, and clear visual rhythm.

### DL-006 — Glow Is Semantic

Glow, illumination, and luminous borders communicate state, focus, connectivity, or system activity. They are not generic decoration.

### DL-007 — Gold Is Identity

CAT gold is reserved primarily for brand identity, premium emphasis, major system anchors, and selected high-authority states.

### DL-008 — Blue Is Infrastructure / Intelligence

Electric blue and cyan are the primary visual language for system connectivity, network intelligence, active infrastructure, and interactive focus.

### DL-009 — Semantic Colors Are Controlled

Green, amber, red, and violet have defined meanings and must not be used arbitrarily for visual variety.

### DL-010 — No Generic AI Slop

Do not introduce generic purple gradients, random neon gradients, excessive rounded cards, decorative blobs, meaningless glassmorphism, or template-like SaaS layouts that conflict with the approved CAT references.

### DL-011 — Evidence Before Ornament

A visual effect is justified only when it improves hierarchy, state recognition, navigation, or comprehension.

### DL-012 — Responsive by Hierarchy

Mobile is not a shrunken desktop. Content priority must remain explicit at every breakpoint.

---

## 3. Visual Grammar

CAT visual grammar is built from six recurring primitives:

1. **Frame** — structural shell, navigation, viewport boundaries.
2. **Panel** — bounded operational information surface.
3. **Signal** — status, alert, metric delta, event, or state indicator.
4. **Node** — agent, engine, entity, model, market, or graph object.
5. **Flow** — directional relationship between nodes, states, or pipeline stages.
6. **Field** — large-scale contextual visualization such as CAT Brain, world map, network, or knowledge graph.

A new screen should be explainable primarily through these primitives.

---

## 4. Color System

### 4.1 Base Palette

| Token | Value | Role |
|---|---|---|
| `cat-bg-950` | `#020611` | deepest application background |
| `cat-bg-900` | `#050B18` | primary shell background |
| `cat-bg-850` | `#081122` | elevated navigation / workspace background |
| `cat-surface-900` | `#0A1426` | primary panel |
| `cat-surface-850` | `#0D1930` | elevated panel |
| `cat-surface-800` | `#11213B` | hover / selected surface |
| `cat-border` | `#173454` | default structural border |
| `cat-border-active` | `#168BFF` | active luminous border |
| `cat-blue-500` | `#168BFF` | primary interaction / system signal |
| `cat-cyan-400` | `#20D7FF` | intelligence / network highlight |
| `cat-cyan-300` | `#63E7FF` | secondary luminous highlight |
| `cat-violet-500` | `#7A5CFF` | AI / analytics secondary signal |
| `cat-green-500` | `#19D889` | healthy / successful / running |
| `cat-amber-500` | `#FFB84A` | warning / pending / attention |
| `cat-red-500` | `#FF4D67` | error / blocked / critical |
| `cat-gold-500` | `#D8A94A` | CAT brand identity |
| `cat-gold-300` | `#F3D487` | premium highlight |
| `cat-text-100` | `#F4F8FF` | primary text |
| `cat-text-300` | `#B9C8DE` | secondary text |
| `cat-text-500` | `#71839E` | tertiary / metadata text |

Values are canonical defaults. Implementation may use derived alpha values from these tokens but must not introduce unrelated brand colors without a documented decision.

### 4.2 Semantic Color Contract

| Semantic | Primary token | Meaning |
|---|---|---|
| Running / Healthy | `cat-green-500` | operationally healthy |
| Success | `cat-green-500` | completed positive outcome |
| Active | `cat-blue-500` | current interactive/system activity |
| Intelligence | `cat-cyan-400` | intelligence/network/evidence |
| AI | `cat-violet-500` | AI/model/learning context |
| Pending | `cat-amber-500` | awaiting action or attention |
| Warning | `cat-amber-500` | elevated risk without failure |
| Blocked | `cat-red-500` | policy/security/system block |
| Critical | `cat-red-500` | urgent operational failure |
| Brand | `cat-gold-500` | CAT identity / premium anchor |

Never communicate status using color alone. Pair semantic color with iconography, text, shape, or motion.

### 4.3 Gradients

Gradients are permitted only when they describe depth, energy, or a controlled brand transition.

Preferred gradient families:

- deep navy → transparent navy
- blue → cyan for active intelligence
- gold → warm gold for CAT identity
- violet → blue for AI system surfaces

Avoid rainbow gradients and high-saturation multi-hue backgrounds.

---

## 5. Typography

CAT typography must prioritize **operational readability first and cinematic identity second**.

### 5.1 Roles

| Role | Weight | Use |
|---|---:|---|
| Display | 600–700 | page titles, major CAT Brain labels |
| Heading | 600 | section titles, panel headings |
| Body | 400–500 | descriptions and normal content |
| Label | 500–600 | KPI labels, navigation, state labels |
| Metric | 600–700 | financial / operational values |
| Mono | 400–500 | IDs, technical values, event IDs, timestamps |

### 5.2 Typography Rules

- Large metrics must dominate their cards.
- Labels must never compete with the metric they describe.
- Metadata uses reduced contrast, not tiny unreadable text.
- Technical identifiers may use monospace.
- Avoid excessive all-caps copy.
- Uppercase is reserved for short labels, status chips, section eyebrows, and brand-style microcopy.
- Line height must remain comfortable in dense panels.
- Never use typography as decoration at the expense of scanability.

### 5.3 Number Formatting

Financial and operational metrics must have consistent formatting:

- currency: explicit currency symbol / ISO context
- percentages: `%`
- rates: explicit unit
- large counts: compact notation only when the exact value remains accessible
- timestamps: localized display plus exact timestamp on inspection where required

---

## 6. Spacing and Layout Tokens

Base spacing unit: **4px**.

Recommended scale:

| Token | Value |
|---|---:|
| `space-1` | 4px |
| `space-2` | 8px |
| `space-3` | 12px |
| `space-4` | 16px |
| `space-5` | 20px |
| `space-6` | 24px |
| `space-8` | 32px |
| `space-10` | 40px |
| `space-12` | 48px |
| `space-16` | 64px |
| `space-20` | 80px |

Use spacing to create hierarchy rather than adding borders everywhere.

### 6.1 Layout Rules

- Preserve a strong outer frame.
- Establish a primary workspace axis before adding secondary rails.
- Group related KPIs tightly.
- Separate unrelated systems through whitespace before using visual decoration.
- Maintain consistent panel gutters.
- Do not allow every panel to become visually equal.
- The primary task or decision gets the largest visual territory.

### 6.2 Desktop Composition

The canonical desktop pattern is:

```text
┌─────────────── Global Top Bar ───────────────────────────────┐
├──────┬───────────────────────────────┬────────────────────────┤
│ Nav  │       Primary Workspace       │ Context / Agent Rail  │
│ Rail │                               │                        │
│      │                               │                        │
├──────┴───────────────────────────────┴────────────────────────┤
│             Intelligence / Status / Activity Surface          │
└───────────────────────────────────────────────────────────────┘
```

This is a grammar, not a mandatory fixed pixel layout.

---

## 7. Panel Language

Panels are the fundamental CAT information container.

### 7.1 Panel Anatomy

```text
Panel
├── eyebrow / category
├── title
├── optional state
├── primary content
├── secondary metadata
└── action / drill-down
```

### 7.2 Panel Surface

Default panel characteristics:

- deep translucent navy surface
- subtle 1px structural border
- restrained corner radius
- low-opacity internal highlight
- minimal shadow
- optional luminous edge when active

### 7.3 Panel Hierarchy

Three visual tiers:

**Tier A — Primary**

Major workspace, CAT Brain, decision surface, active approval, critical operational state.

**Tier B — Secondary**

KPI groups, charts, agent lists, workflow stages, intelligence panels.

**Tier C — Supporting**

Metadata, recent events, logs, secondary filters, auxiliary controls.

Do not give Tier C the same visual weight as Tier A.

---

## 8. Borders, Glow, and Depth

### 8.1 Borders

Default borders are structural and subtle.

- default: low-opacity blue-gray
- hover: brighter blue
- selected: electric blue
- critical: red
- brand: gold only for identity-oriented surfaces

### 8.2 Glow

Glow is allowed for:

- active nodes
- selected controls
- running agents
- network connections
- CAT Brain energy fields
- system transitions
- critical alerts

Glow must have a clear semantic owner.

### 8.3 Depth

CAT uses **layered depth**, not heavy floating-card shadows.

Depth hierarchy:

```text
Background Field
    ↓
Ambient Grid / Network
    ↓
Workspace Surface
    ↓
Primary Panel
    ↓
Focused Component
    ↓
Transient Overlay
```

---

## 9. Corner Radius and Shape Language

Recommended radius vocabulary:

- `radius-sm`: 4px
- `radius-md`: 8px
- `radius-lg`: 12px
- `radius-xl`: 16px
- `radius-pill`: 999px

Default CAT operational cards should prefer `radius-md` or `radius-lg`.

Avoid making every component a pill or heavily rounded container.

Circular geometry is reserved for:

- avatars
- agent status indicators
- graph nodes
- system gauges
- CAT Brain orbital structures
- compact action controls where appropriate

---

## 10. Iconography

Icons are operational symbols, not decoration.

Rules:

- use one coherent icon family
- maintain consistent stroke/weight
- use filled icons selectively for strong semantic states
- pair unfamiliar icons with labels
- never rely on icon color alone for meaning
- navigation icons should remain visually subordinate to labels
- agent icons may use role-specific visual identity

Preferred conceptual categories:

- navigation
- analytics
- intelligence
- agent
- content
- affiliate
- finance
- security
- workflow
- governance
- system

---

## 11. Status Indicators

Every operational state has a consistent visual treatment.

```text
● Running      green + label
● Healthy      green + label
● Active       blue + label
● Learning     violet/cyan + label
● Pending      amber + label
● Approval     amber + explicit human-action label
● Blocked      red + reason
● Failed       red + recovery path
● Offline      muted + explicit offline state
```

A small dot alone is insufficient for important decisions or security states.

---

## 12. KPI Design Language

A KPI is not merely a number card.

Canonical structure:

```text
CATEGORY
Primary Value
Δ + Direction
Comparison Period
Freshness / Confidence
Drill-down
```

Examples:

- Revenue
- Profit
- Clicks
- Conversions
- EPC
- ROAS
- Active Campaigns
- Agent Utilization
- Content Throughput
- Forecast Confidence

### KPI Rules

- One dominant metric per card.
- Delta must have a comparison context.
- Positive/negative color must respect semantic meaning.
- Financial metrics require provenance where the surface is decision-relevant.
- Simulated data must be explicitly labeled.
- Never fabricate real-time operational status.

---

## 13. Data Visualization Language

Charts must feel like part of the CAT command system while preserving analytical clarity.

### Preferred

- line charts for ordered trends
- bars for categorical comparisons
- stacked bars for composition over an ordered dimension
- restrained donut charts for simple part-to-whole summaries
- scatter plots for relationships
- network/graph views for graph-native problems
- heatmaps only when the matrix relationship is meaningful

### Rules

- dark plotting field
- high-contrast axes
- readable labels
- exact tooltip values
- explicit units
- consistent time intervals
- meaningful legends
- minimal chart decoration
- no 3D charts when 2D communicates the same information

### Intelligence Overlays

Forecasts, confidence bands, anomalies, and targets must have distinct visual treatments from observed truth.

---

## 14. CAT Brain Language

CAT Brain is the signature visual element of the system.

It must communicate **living intelligence**, not a decorative sci-fi orb.

Possible visual layers:

1. world / market field
2. network topology
3. knowledge graph activity
4. agent activity
5. economic signals
6. opportunity signals
7. learning activity
8. system health

### CAT Brain Rules

- central position on executive surfaces
- never overwhelm primary financial truth
- animated only when activity is meaningful
- motion can indicate data flow, learning, or connectivity
- no random particles solely for visual effect
- always provide an accessible textual/structured alternative
- never imply that the visualization itself is canonical truth

---

## 15. Agent Visual Language

Agents are operational entities and need a recognizable visual grammar.

Agent cards should expose:

```text
Identity
Role
State
Task
Authority
Confidence
Health
Model
Runtime
Escalation
Recent Activity
```

### Agent State Visualization

- avatar / role mark
- state indicator
- current task
- execution progress
- authority badge when relevant
- approval requirement
- escalation indicator

Human and agent identities must not be visually interchangeable.

---

## 16. AI Council Language

The CAT AI Council represents specialized agents cooperating under governance.

Council views should communicate:

```text
Agent → Position / Evidence → Confidence → Recommendation → Decision Gate
```

Do not display a council as a collection of decorative character portraits. Each participant must have an operational role.

---

## 17. Workflow Language

Workflows use a visible state pipeline.

Canonical sequence:

```text
Trigger
  ↓
Plan
  ↓
Generate
  ↓
Validate
  ↓
Approve
  ↓
Execute
  ↓
Observe
  ↓
Learn
  ↓
Optimize
```

Each node requires:

- status
- timestamp
- owner/agent
- input/output where relevant
- failure state
- recovery or escalation path

Completed stages must not visually resemble pending stages.

---

## 18. Model Router Language

Model routing is an infrastructure and governance surface.

A model card may show:

- model identity
- provider/runtime
- task class
- quality profile
- latency
- cost
- privacy classification
- availability
- fallback
- routing reason

Do not present model names as decorative brand badges.

---

## 19. Knowledge Graph Language

Knowledge graph surfaces should use node/edge visual grammar rather than card grids pretending to be graphs.

Nodes may represent:

- products
- merchants
- offers
- affiliates
- campaigns
- audiences
- content
- trends
- agents
- evidence
- decisions
- markets

Edges must communicate relationship type where it matters.

Graph UI must expose:

- provenance
- confidence
- time
- entity type
- evidence
- filters
- graph-to-decision traceability

---

## 20. Command and Navigation Language

### Navigation

Primary navigation is persistent on desktop and intentionally prioritized on mobile.

Navigation labels must use canonical product terminology from `context/13_TERMINOLOGY.md` once established.

### Command Palette

The command palette is a first-class operational interface.

It should support:

- navigation
- search
- actions
- entity lookup
- agent commands
- workflow commands
- report access
- settings

Destructive or financial actions require explicit confirmation and authority checks.

---

## 21. Interaction States

Every interactive component must define:

```text
Default
Hover
Focus
Active
Selected
Disabled
Loading
Success
Warning
Error
Blocked
```

Do not design only the happy path.

### Focus

Keyboard focus must be clearly visible and must not depend exclusively on color.

### Disabled

Disabled controls must communicate both unavailable state and, where appropriate, why the action is unavailable.

### Loading

Loading states should preserve layout geometry to prevent disruptive shifts.

---

## 22. Motion Language

Motion should explain system behavior.

### Motion Categories

**Micro** — button feedback, selection, hover.

**Transition** — navigation and panel state changes.

**Operational** — workflow progression, agent activity, event arrival.

**Intelligence** — knowledge graph propagation, CAT Brain activity, learning signals.

### Rules

- no gratuitous perpetual animation
- no motion that obscures data
- respect reduced-motion preferences
- important state changes must not rely on animation alone
- motion duration should scale with semantic distance
- fast feedback for direct interaction
- slower motion for ambient system transitions

---

## 23. Glass / Holographic Treatment

Glass is a secondary material, not the entire visual identity.

Use for:

- floating context rails
- overlays
- transient intelligence surfaces
- selected CAT Brain layers
- modal command surfaces

Avoid:

- every card being translucent
- excessive blur
- low-contrast text on glass
- glass over busy graphs without readability testing

The hierarchy must survive if blur effects are disabled.

---

## 24. Background and Ambient Field

The background may include:

- subtle grid
- world/network field
- orbital lines
- faint topology
- restrained stars/particles
- architectural framing

Ambient elements must remain below the information layer.

They must never compete with:

- financial truth
- approvals
- active incidents
- decisions
- agent execution state

---

## 25. Gold Identity Language

CAT gold is a brand anchor.

Use gold for:

- CAT logo
- primary brand mark
- major CAT Brain identity
- premium system identity
- selected executive emphasis

Do not use gold for every important metric. Importance is not equivalent to branding.

---

## 26. Error, Risk, and Security Language

Security and governance states must be unmistakable.

### Error

Red + explicit failure message + recovery path.

### Risk

Amber + quantified or described risk + next action.

### Block

Red or strong amber + policy reason + authority required.

### Security Incident

Dedicated high-priority treatment with:

- incident ID
- severity
- affected surface
- detection time
- evidence
- containment state
- owner
- escalation

Never hide security information inside a generic notification drawer.

---

## 27. Financial Surface Language

Financial information has elevated visual authority because Treasury Core owns monetary truth.

Financial surfaces must distinguish:

- observed monetary truth
- derived economic metrics
- forecast
- recommendation
- planned allocation
- executed transaction

Canonical monetary values should use stronger typography and clearer provenance than secondary analytics.

---

## 28. Affiliate / Content Surface Language

Affiliate and Content surfaces must remain visually related but domain-distinct.

Affiliate surfaces emphasize:

- attribution
- conversions
- commissions
- partner networks
- offers
- revenue influence

Content surfaces emphasize:

- content lifecycle
- provenance
- quality
- discovery
- influence
- licensing
- distribution

Cross-engine views must make ownership boundaries visible.

---

## 29. Responsive Design Contract

### Breakpoint Philosophy

Breakpoints are chosen from information failure, not device marketing labels.

Minimum audit states:

- narrow mobile
- mobile / tablet transition
- desktop
- wide desktop / command center

### Mobile Priority

```text
Critical Alert
↓
Decision / Approval
↓
Financial KPI
↓
Active Agents
↓
Primary Intelligence
↓
Core Charts
↓
Secondary Modules
```

The left navigation becomes a compact navigation mechanism. The right agent/context rail becomes contextual or collapsible.

CAT Brain becomes a compact intelligence header or focused visualization rather than an oversized globe.

---

## 30. Accessibility Contract

Accessibility is constitutional.

Required:

- keyboard navigation
- visible focus
- semantic HTML
- accessible names
- sufficient contrast
- non-color status indicators
- reduced-motion support
- screen-reader labels
- accessible tables where data is dense
- text alternatives for complex visualizations
- chart summaries
- no interaction that depends exclusively on hover

Visual density never overrides accessibility.

---

## 31. Content and Microcopy Language

CAT copy should sound:

- precise
- operational
- calm
- intelligent
- concise
- evidence-aware

Avoid:

- hype without evidence
- anthropomorphic claims that imply unsupported autonomy
- vague AI marketing language
- fake certainty
- unexplained technical jargon

Prefer:

> Recommendation generated from 4 evidence sources.

over:

> AI knows the perfect move.

Prefer:

> Awaiting human approval.

over:

> Almost done!

---

## 32. Empty, Loading, and Recovery States

Every major surface requires designed states for:

- first load
- loading
- no data
- filtered no results
- unavailable service
- partial data
- stale data
- permission denied
- policy blocked
- failed action
- recovered action

Empty states should explain what is missing and what the user can do next.

---

## 33. Data Freshness Language

Data freshness must be visible where stale information could affect a decision.

Examples:

- Live
- Updated 12s ago
- Updated 8m ago
- Stale
- Forecast
- Simulated
- Historical

Do not imply real-time state when the underlying source is delayed or simulated.

---

## 34. Demo and Simulation Language

Demo data must never be visually indistinguishable from production truth when the distinction matters.

Use explicit labels such as:

- Demo
- Simulated
- Sandbox
- Synthetic
- Example
- Replay

The visual system may remain cinematic, but the epistemic status of data must remain honest.

---

## 35. Component Composition Rules

Components should compose from shared primitives.

Preferred hierarchy:

```text
Design Tokens
   ↓
Primitives
   ↓
Operational Components
   ↓
Domain Components
   ↓
Workspace Surfaces
   ↓
Command Center / Global Surfaces
```

Do not create one-off styling for every screen.

A new visual pattern should either reuse an existing component or establish a documented reusable pattern.

---

## 36. Token Governance

Design tokens are the implementation bridge between this document and code.

Token categories:

- color
- typography
- spacing
- radius
- border
- elevation
- glow
- motion
- z-index
- breakpoint
- chart semantics
- state semantics

Component code should consume semantic tokens rather than hard-coding visual values repeatedly.

---

## 37. Theme Architecture

CAT should support a canonical dark command-center theme first.

Future themes may exist, but must preserve:

- semantic meaning
- contrast
- information hierarchy
- CAT identity
- state distinctions
- accessibility

Theme switching must not change domain semantics.

---

## 38. Anti-Patterns

The following are prohibited unless explicitly approved by a design decision:

1. Generic SaaS white-card dashboard layout.
2. Random purple AI gradients.
3. Rainbow accent systems.
4. Excessive glassmorphism.
5. Every component being rounded.
6. Every panel glowing.
7. Decorative 3D charts.
8. Decorative AI avatars with no operational role.
9. Fake live metrics.
10. Status communicated only by color.
11. Tiny unreadable metadata.
12. Dense screens without hierarchy.
13. Permanent animation everywhere.
14. Giant hero graphics that hide the actual task.
15. UI states that imply execution when the action is only a recommendation.
16. Pixel-copying the reference images when the product context requires a different layout.
17. Introducing a new visual language for individual modules.
18. Treating AI-generated content as canonical truth because it passed a quality gate.

---

## 39. Design Review Checklist

Before accepting a new CAT surface, verify:

### Identity
- Does it look recognizably CAT?
- Does it inherit the approved Command Center / CAT Brain language?

### Hierarchy
- Is the primary task obvious within seconds?
- Are high-authority states visually dominant?

### Semantics
- Can the user distinguish truth, intelligence, recommendation, decision, and execution?
- Are statuses understandable without color alone?

### Density
- Is the screen information-rich without becoming visually chaotic?
- Are secondary details subordinate?

### Data
- Are freshness, provenance, and simulation state clear where required?
- Are charts readable and correctly labeled?

### Interaction
- Are hover, focus, active, loading, success, warning, error, and blocked states defined?
- Are destructive actions protected by authority and confirmation?

### Responsive
- Does hierarchy survive mobile?
- Does the layout adapt rather than merely shrink?

### Accessibility
- Is keyboard navigation complete?
- Is focus visible?
- Are complex visuals explained accessibly?

### Craft
- Are glow, blur, gradients, and animation purposeful?
- Does anything look like generic AI-generated UI?

---

## 40. AI Design-Agent Contract

Any AI coding/design agent working on CAT must read:

1. `context/10_UI_UX.md`
2. `context/11_DESIGN_LANGUAGE.md`
3. relevant domain/context documents
4. relevant decisions once `context/12_DECISIONS.md` exists

The agent must:

- preserve canonical tokens
- preserve semantic colors
- preserve authority boundaries
- preserve responsive hierarchy
- preserve accessibility requirements
- reuse existing component grammar
- avoid prohibited anti-patterns
- distinguish generated/demo data from canonical data
- never invent product capabilities merely to complete a visual mockup
- record genuinely new design decisions for later registration in `context/12_DECISIONS.md`

When reference images and textual rules appear to conflict, the textual constitutional rules govern product behavior and the approved reference images govern visual direction.

---

## 41. Reference Asset Registry

| ID | Asset | Authority |
|---|---|---|
| `CAT-UI-REF-001` | `assets/ui/cat-command-center-reference.jpg` | Primary operational shell |
| `CAT-UI-REF-002` | `assets/ui/cat-global-performance-reference.jpg` | Strategic/global intelligence shell |

Reference images are design evidence. They do not override canonical domain truth, security, governance, or architecture rules.

---

## 42. Design Language Completion Contract

The Design Language Bible is complete when:

- CAT visual identity is tokenized.
- Color semantics are closed.
- Typography roles are defined.
- Spacing and geometry vocabulary is defined.
- Panel, border, glow, and depth grammar is defined.
- Agent, CAT Brain, workflow, graph, KPI, and intelligence surfaces have visual rules.
- Responsive and accessibility rules are explicit.
- Demo/simulation state is explicit.
- Anti-patterns are documented.
- AI design-agent operating rules are explicit.
- Reference assets are traceable.
- New design decisions can be registered without silently changing the language.

**Status:** Complete — Design Language V1 canonical draft.

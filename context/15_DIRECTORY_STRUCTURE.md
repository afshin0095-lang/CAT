# CAT OMNISYSTEM — Directory Structure Bible V1

> Canonical repository organization and ownership rules for CAT OMNISYSTEM.

**Status:** Canonical Draft V1
**Version:** 1.0.0
**Project:** CAT (Commerce AI Trinity)
**Company:** Omni System
**Repository:** `afshin0095-lang/CAT`
**Default Branch:** `main`
**Governing Documents:** `context/04_ARCHITECTURE.md`, `context/13_TERMINOLOGY.md`, `context/14_CODING_STANDARD.md`

---

## 1. Purpose

The CAT repository is a multi-core AI commerce platform. Directory structure is therefore an architectural boundary, not merely a file-management preference.

The structure must make five things obvious:

1. where executable system cores live;
2. where shared contracts and platform foundations live;
3. where documentation and project context live;
4. where tests and verification live;
5. where infrastructure, automation, and generated artifacts belong.

A developer or AI coding agent should be able to determine the intended ownership of a file from its path before opening the implementation.

---

## 2. Constitutional Rules

### DIR-001 — Path Implies Ownership

Every production path belongs to a defined architectural domain.

### DIR-002 — Core and Context Are Different

`core/` contains implementation. `context/` contains the durable project contracts, standards, terminology, and decisions used to govern implementation.

### DIR-003 — No Unowned Root Folders

A new top-level directory requires an explicit architectural reason and a recorded decision when it changes repository boundaries.

### DIR-004 — Tests Stay Near the Contract They Verify

Unit and component tests live with the relevant core where practical. Cross-core tests belong in the appropriate integration/test surface.

### DIR-005 — Generated Output Is Never Canonical Source

Generated files must identify their generator and must not become the authoritative source of domain behavior.

### DIR-006 — Domain Truth Has One Owner

A directory may consume another domain's public contract, but it must not silently become an alternate owner of that domain's canonical truth.

### DIR-007 — Language Follows Workload

Rust, Go, Python, and TypeScript remain specialized implementation languages according to the coding standard. Directory structure must not imply that one language owns the entire platform.

### DIR-008 — Internal Modules Are Not Public APIs

The existence of a source directory does not grant other domains permission to import its internals. Public boundaries must be explicit.

---

## 3. Canonical Repository Map

The repository is organized conceptually as follows:

```text
CAT/
├── .ai/                         # AI development context and project status
├── .github/                     # GitHub automation and repository workflows
├── context/                     # Canonical project context and engineering bibles
├── architecture/                # Architecture documents and system maps
├── knowledge/                   # Human-readable knowledge and architecture maps
├── core/                        # Executable CAT platform cores
│   ├── knowledge/               # Knowledge Core
│   │   └── rust/                # Rust implementation + tests
│   └── platform/                # Shared platform foundation
│       └── rust/                # Rust platform implementation
├── scripts/                     # Repository automation and generators
├── tests/                       # Cross-domain / repository-level verification
├── docs/                        # Additional technical/product documentation
└── README.md                    # Repository entry point
```

This map separates **governance**, **architecture knowledge**, **executable cores**, **automation**, and **verification**.

The repository currently contains active Rust implementation under `core/knowledge/rust/` and `core/platform/rust/`; the Knowledge Core is the current active implementation surface according to `.ai/PROJECT_STATUS.md`.

---

## 4. `.ai/` — AI Development Control Plane

```text
.ai/
├── README.md
└── PROJECT_STATUS.md
```

### Responsibility

`.ai/` contains machine-oriented development context used by AI coding agents and project automation.

### Rules

- project status must reflect repository reality;
- AI instructions must not silently override canonical architecture decisions;
- generated status must be reviewable;
- secrets and credentials are forbidden;
- project state should identify current and next tasks explicitly.

`PROJECT_STATUS.md` is the operational handoff surface for the current implementation phase.

---

## 5. `context/` — Project Constitution

`context/` contains durable project contracts.

```text
context/
├── 00_PROJECT_CONTEXT.md
├── 01_PROJECT_OVERVIEW.md
├── 02_PROJECT_RULES.md
├── 03_TECH_STACK.md
├── 04_ARCHITECTURE.md
├── 05_AGENTS.md
├── 06_KNOWLEDGE_ENGINE.md
├── 07_MEMORY_SYSTEM.md
├── 08_EVENTS_SYSTEM.md
├── 09_REASONING_ENGINE.md
├── 10_DECISION_ENGINE.md
├── 11_PLANNING_ENGINE.md
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
└── 19_DEVELOPMENT_GUIDE.md
```

> Note: the repository currently contains overlapping numeric namespaces in the older core-context sequence and the later governance sequence. These names are preserved as existing contracts. Renaming or renumbering them requires an explicit decision and reference migration.

### Responsibility

- architecture contracts;
- project rules;
- technology decisions;
- agent definitions;
- knowledge/memory/event semantics;
- treasury/affiliate/content contracts;
- UI/UX and design language;
- terminology;
- coding standards;
- deployment/security/prompting/development guides.

### Rules

Context documents are versioned project knowledge. Implementation changes that materially contradict them require a decision record or an explicit update to the governing document.

---

## 6. `architecture/` — Architecture Reference Surface

Known architecture documents include:

```text
architecture/
├── README.md
├── System_Architecture.md
├── Backend_Architecture.md
└── Frontend_Architecture.md
```

### Responsibility

This directory describes how CAT's major technical surfaces fit together.

It is explanatory architecture documentation and must remain aligned with the executable implementation and the canonical context bibles.

### Rule

Architecture documents describe contracts; source code remains the implementation authority for behavior.

---

## 7. `knowledge/` — Knowledge Documentation Surface

Known repository content includes:

```text
knowledge/
└── Architecture_Map.md
```

### Responsibility

Human-readable knowledge maps and cross-domain architectural knowledge belong here when they are not themselves executable source.

The Knowledge Core's runtime implementation remains under `core/knowledge/`.

---

## 8. `core/` — Executable Platform Cores

`core/` is the primary executable architecture boundary.

```text
core/
├── knowledge/
│   └── rust/
│       ├── Cargo.toml
│       ├── src/
│       └── tests/
│
└── platform/
    └── rust/
        ├── Cargo.toml
        └── src/
```

### Core Rule

Each core owns a coherent responsibility and exposes explicit contracts.

A core should not reach into another core's private source tree to bypass its public API.

### 8.1 Knowledge Core

Current verified implementation surface:

```text
core/knowledge/rust/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── model.rs
│   ├── persistence.rs
│   ├── provenance.rs
│   ├── query.rs
│   ├── snapshot.rs
│   ├── store.rs
│   ├── temporal.rs
│   ├── traversal.rs
│   └── validation.rs
└── tests/
    ├── graph.rs
    ├── graph_invariants.rs
    ├── query_snapshot.rs
    └── temporal_provenance_persistence.rs
```

The current Knowledge Core surface includes graph modeling, persistence, provenance, querying, snapshots, temporal validity, traversal, and validation.

The current project status identifies traversal, evidence validation, provenance-aware retrieval, and graph consistency contracts as the active hardening stage.

### 8.2 Platform Core

```text
core/platform/rust/
├── Cargo.toml
└── src/
```

The Platform Core contains reusable platform-level Rust foundations and must not become a miscellaneous utility package.

---

## 9. Language Placement

CAT uses language-specific implementation directories where the domain requires them.

### Rust

Preferred for:

- correctness-sensitive cores;
- high-throughput runtime paths;
- concurrency-heavy infrastructure;
- foundational platform components.

### Go

Preferred for:

- network services;
- control-plane services;
- workers;
- adapters;
- orchestration components.

### Python

Preferred for:

- AI/data workflows;
- research;
- model adapters;
- automation;
- non-latency-critical services.

### TypeScript / React

Preferred for:

- web interfaces;
- command center;
- admin surfaces;
- visual intelligence interfaces;
- typed frontend SDK consumers.

The coding standard is authoritative for language selection; directory structure must not force a language where the workload calls for another.

---

## 10. Source Module Rules

Within a core:

```text
src/
├── lib.rs / entrypoint
├── domain modules
├── application/service modules
├── adapters
├── infrastructure boundaries
└── internal helpers
```

Prefer semantic module names over generic folders such as:

```text
utils/
helpers/
common/
misc/
```

If shared behavior is truly cross-domain, it belongs in an explicit platform contract rather than an unowned helper folder.

---

## 11. Test Placement

### Unit Tests

Test local behavior close to the owning module when language conventions support it.

### Integration Tests

For Rust cores, repository-visible integration tests may live under:

```text
core/<core-name>/<language>/tests/
```

This is already used by the Knowledge Core.

### Cross-Core Tests

Cross-core tests belong in a dedicated integration/test surface when the test validates a contract between independently owned cores.

### Test Naming

Names must communicate the invariant being protected.

Good:

```text
temporal_provenance_persistence.rs
graph_invariants.rs
```

Avoid:

```text
misc.rs
test2.rs
stuff.rs
```

---

## 12. Scripts and Generators

Known automation lives under:

```text
scripts/
└── generator/
    ├── generate_content_engine_part4.py
    └── validate_content_engine_part4.py
```

### Rules

- generators have deterministic inputs and outputs;
- generated artifacts identify their source;
- validators fail closed;
- generators must not embed credentials;
- changes to generator behavior are code changes and require tests;
- do not hand-edit generated output when the generator is authoritative.

---

## 13. `.github/` — Repository Automation

`.github/` owns GitHub-specific automation:

```text
.github/
├── workflows/
├── CODEOWNERS
├── issue templates
└── pull-request templates
```

Exact files may evolve as CI/CD grows.

Repository automation must remain separate from application runtime code.

---

## 14. Documentation vs Runtime Source

The following rule prevents architectural confusion:

```text
context/      = governing contract
architecture/ = explanatory architecture
knowledge/    = knowledge maps
core/         = executable implementation
scripts/      = repository automation
.github/      = repository automation platform
```

A document describing a feature does not make that feature implemented.

Likewise, an implementation that contradicts a governing contract requires either correction or a recorded architectural decision.

---

## 15. Public API Boundaries

A public API boundary should be visible through one or more of:

- exported library types;
- service API definitions;
- Protobuf/JSON Schema contracts;
- documented adapter interfaces;
- explicit SDK packages.

A path such as `core/foo/rust/src/internal/` is not a public contract merely because another module can technically import it.

---

## 16. Generated, Temporary, and Local Files

The following do not belong in canonical source directories unless explicitly required:

```text
*.log
.env
.env.*
coverage/
target/
node_modules/
__pycache__/
.tmp/
local secrets
machine-specific caches
```

Build artifacts must be produced by CI/release tooling rather than treated as source.

---

## 17. Naming Rules for Directories

Directory names must:

- use stable semantic terminology;
- avoid unexplained abbreviations;
- match canonical terminology;
- remain language-appropriate;
- avoid version numbers unless the version is part of the architectural identity;
- avoid dates unless representing immutable historical artifacts.

Recommended style:

```text
knowledge/
platform/
traversal.rs
provenance.rs
content_engine/
```

Avoid:

```text
misc2/
new_final/
helpers_old/
version3_really_final/
```

---

## 18. Ownership Matrix

| Area | Primary Owner | Canonical Source |
|---|---|---|
| Project context | Project Governance | `context/` |
| Architecture | Architecture | `architecture/` + governing context |
| Knowledge runtime | Knowledge Core | `core/knowledge/` |
| Platform runtime | Platform Core | `core/platform/` |
| Repository automation | Engineering | `scripts/` |
| GitHub automation | DevOps | `.github/` |
| UI/UX rules | Design / Product | `context/10_UI_UX.md` |
| Design tokens/language | Design System | `context/11_DESIGN_LANGUAGE.md` |
| Decisions | Governance / Architecture | `context/12_DECISIONS.md` |
| Terminology | Project Governance | `context/13_TERMINOLOGY.md` |
| Coding standards | Engineering | `context/14_CODING_STANDARD.md` |
| Directory structure | Architecture | `context/15_DIRECTORY_STRUCTURE.md` |

---

## 19. Change Rules

A directory change requires additional review when it:

- creates a new top-level folder;
- changes core ownership;
- moves a public API;
- changes language ownership;
- moves canonical truth;
- changes test boundaries;
- changes generated-source ownership;
- changes CI/CD or deployment boundaries.

Small local refactors may remain normal code review.

Architectural moves require a decision record when they change system contracts.

---

## 20. AI Coding Agent Rules

AI coding agents operating in CAT must follow this sequence:

```text
Read PROJECT_STATUS
      ↓
Read governing context
      ↓
Locate owning directory
      ↓
Inspect neighboring implementation/tests
      ↓
Make the smallest coherent change
      ↓
Run relevant validation
      ↓
Update status/documentation when required
      ↓
Commit with a semantic message
```

AI agents must not:

- create duplicate cores because an existing core is difficult to understand;
- move files merely to make a task easier;
- create generic `utils` folders as escape hatches;
- place production code in documentation directories;
- bypass public contracts by importing private modules;
- overwrite unrelated files;
- mark a feature complete without implementation evidence.

---

## 21. Directory Decision Contract

Any proposed structural change should record:

```yaml
record_type: directory_change
change_id: CAT-DIR-0001
reason: "Why the current structure is insufficient"
affected_paths: []
new_owner: "domain owner"
public_contract_changed: false
canonical_truth_moved: false
migration_required: false
test_impact: []
rollback_plan: "..."
approval_tier: T1
```

For T2+ structural changes, the decision system in `context/12_DECISIONS.md` applies.

---

## 22. Verification Contract

The directory structure is considered healthy when:

- every production path has an identifiable owner;
- core boundaries are respected;
- tests remain associated with their contracts;
- generated output is distinguishable from source;
- scripts do not become application runtime code;
- documentation does not claim implementation that does not exist;
- AI agents can locate the correct implementation surface from project status and architecture context;
- no new root directory exists without a recorded architectural reason.

### Required Structural Checks

```text
DIR-CHECK-001  Root directory ownership
DIR-CHECK-002  Core boundary validation
DIR-CHECK-003  Forbidden generic folders
DIR-CHECK-004  Generated-artifact isolation
DIR-CHECK-005  Test placement
DIR-CHECK-006  Public API boundary validation
DIR-CHECK-007  Documentation/source separation
```

---

## 23. Repository Evolution

The structure is designed to scale from the current foundation to a multi-core, multi-service, multi-agent CAT platform.

The preferred evolution is:

```text
Current foundation
      ↓
More specialized cores
      ↓
Service / adapter boundaries
      ↓
Independent scaling
      ↓
Distributed execution
      ↓
Kubernetes / multi-node deployment
```

Scaling the repository must not require flattening domain boundaries or turning the root into an unstructured monolith.

---

## 24. Completion Contract

`context/15_DIRECTORY_STRUCTURE.md` is the canonical reference for repository organization.

### Acceptance Criteria

- directory ownership is explicit;
- `core/` is reserved for executable platform cores;
- `context/` remains the project governance layer;
- architecture and knowledge documentation remain separate from runtime implementation;
- test placement rules are explicit;
- generated/local artifacts are isolated;
- AI coding-agent navigation rules are explicit;
- structural changes have an auditable decision path;
- the structure can scale with additional CAT cores without collapsing domain boundaries.

### Memory Anchor

`CAT-DIR-MEM-S01-001` — **Directory structure is an architectural map: the path of a file must communicate its ownership, contract boundary, and operational role.**

### Registry

```json
{
  "record_type": "directory_structure_contract",
  "record_version": "1.0",
  "contract_id": "CAT-DIR-CONTRACT-001",
  "root_domains": [".ai", ".github", "context", "architecture", "knowledge", "core", "scripts", "tests", "docs"],
  "core_boundary": "core/*",
  "governance_boundary": "context/*",
  "implementation_languages": ["rust", "go", "python", "typescript"],
  "generated_source_policy": "generated_output_is_not_canonical_source",
  "ai_agent_policy": "read_status_and_governing_context_before_edit"
}
```

**Status:** COMPLETE — V1 Foundation

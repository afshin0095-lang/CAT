# CAT Contract Architecture Index

**Status:** L2 — Architecture specified

## Contract architecture chapters

| ID | Document | Scope |
|---:|---|---|
| 55 | Canonical Schema System | Core machine-readable object model |
| 56 | Schema Registry & Validation | Registry and validation gates |
| 57 | Agent Bootstrap & Discovery | Runtime context and authority discovery |
| 58 | Contract-to-Code Generation | Executable language representations |
| 59 | Registry Operating Model | Control-plane registry operations |
| 60 | Contract Quality Gates | Activation requirements |
| 61 | Machine-Readable Architecture Guide | Universal CAT mental model |
| 62 | Schema Implementation Roadmap | Documentation → runtime sequence |
| 63 | Affiliate Discovery Example | Business capability example |
| 64 | Content Generation Example | Content capability example |
| 65 | Advertising Action Example | High-risk economic capability example |
| 66 | Naming & Identity Rules | Canonical identifier policy |
| 67 | Contract Migration Playbook | Safe contract evolution |
| 68 | Canonical Contract Checklist | Implementation/review checklist |
| 69 | Contract Language Guide | Normative terminology |
| 70 | Contract Architecture Definition of Done | Completion boundary |
| 71 | Schema Implementation Bridge | Mapping to existing Rust packages |
| 72 | Contract Architecture Review Gate | Final architecture review |

## Canonical chain

```text
AGENT
 → CAPABILITY
 → DOMAIN SERVICE
 → TOOL
 → CONNECTOR
 → PROVIDER
 → EXTERNAL SYSTEM
 → EVIDENCE
 → MEASUREMENT
```

## Next implementation boundary

The documentation establishes the target. The next engineering phase should inspect the current Rust workspace and introduce the smallest shared contract primitives needed to make this architecture executable, with tests and compatibility checks at every step.

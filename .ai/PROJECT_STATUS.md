| 14_CODING_STANDARD.md | Part 3 Prepared | 75% |
| 15_DIRECTORY_STRUCTURE.md | Not Started | 0% |
| 16_DEPLOYMENT.md | Not Started | 0% |
| 17_SECURITY.md | Not Started | 0% |
| 18_PROMPTING.md | Not Started | 0% |
| 19_DEVELOPMENT_GUIDE.md | Not Started | 0% |

# Active Task

**Current Task:** Platform Adapter Layer — provider health probes, deterministic health transitions, and resilient adapter registry wiring

**Document:** `core/platform/rust/src/provider_adapters.rs` + `core/platform/rust/src/resilient_adapter.rs`

**Status:** Provider adapters now expose a provider-neutral health probe boundary. The registry can probe one provider or all providers in deterministic provider-id order, while the resilient adapter is directly registerable and derives health from its circuit state (`Ready` / `Degraded` / `Unavailable`). Canonical provider semantics remain outside the resilience layer.

# Next Task

Platform adapter contract hardening — add concrete provider transport adapters and capability-specific health probes, then verify failure classification and recovery semantics against disposable external-provider fixtures.

# Next Tasks

1. Core Event Bus — foundation implementation complete; hardening history preserved
2. PostgreSQL Event Store — durable implementation + projection checkpoint foundation complete; integration history preserved
3. Knowledge Core — traversal + evidence validation stage completed
4. LLM / AI Core — P0 foundation + P1 provider adapter layer completed
5. Memory Core — P0 foundation completed
6. Reasoning Core — P1 deterministic evidence reasoning foundation completed
7. Decision Core — deterministic policy-gated decision foundation completed
8. Planning Core — foundation implementation completed; platform execution wiring completed
9. Orchestrator / Workflow Core — durable workflow state, leases, scheduling, retries, compensation, and public workflow API completed; platform scheduling wiring completed
10. Retrieval Core — provider-neutral chunk/index/retrieval/ranking foundation completed
11. Platform Integration Core — typed boundary implemented; planning/orchestration and remaining LLM/reasoning/decision/retrieval concrete wiring completed
12. Platform Adapter Layer — provider contract/registry hardening active; retry + circuit resilience + health probing implemented

# Development Rules

After every completed implementation task:

- Update this file
- Update progress percentages where measurable
- Commit
- Push only when explicitly requested

Never leave this file outdated.

# Completion Progress

Phase A Documentation

██████████████████████████ 100% ✅

Implementation

██████████████████░░░░ 60% — provider health probes and resilient registry wiring implemented

Overall Repository

███████████████████░ 84% — architecture/documentation complete; implementation active
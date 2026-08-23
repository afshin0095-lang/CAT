# CAT Platform Integration Core

`cat-platform` is the first cross-core composition boundary for CAT's Phase B implementation.

It deliberately does **not** become a second domain engine. Its responsibility is to carry stable integration context and compose already-owned capabilities without moving canonical truth between domains.

## Responsibilities

- carry request, correlation, causation, actor, and workflow identity;
- represent explicit cross-core integration targets;
- validate integration commands at the platform boundary;
- compose deterministic Decision, Planning, and Orchestrator capabilities;
- expose a small scheduling boundary for platform work;
- keep future Event Bus, Knowledge, Memory, LLM, Reasoning, and Retrieval adapters behind stable boundaries.

## Ownership rules

- Event Bus owns transport and delivery semantics.
- Event Store owns durable event persistence.
- Knowledge owns knowledge-domain truth.
- Memory owns memory-domain truth.
- LLM owns provider/model execution boundaries.
- Reasoning owns evidence reasoning.
- Decision owns policy-gated decision proposals.
- Planning owns plans and plan validation.
- Orchestrator owns workflow execution state and scheduling.
- Retrieval owns retrieval/chunk/index/ranking semantics.
- Platform owns composition, correlation, and cross-core coordination only.

The platform layer must never silently become the owner of monetary truth, affiliate truth, content truth, knowledge truth, or memory truth.

## Design reference

The transport boundary remains compatible with the existing Event Bus design. NATS/JetStream is appropriate for the durable messaging layer because it provides persistence, replay, acknowledgement, deduplication, and clustered high availability, while the platform crate remains broker-neutral at the composition level. urlNATS architecture referencehttps://nats.io/about/

## Verification

Contract tests cover:

1. correlation and workflow identity propagation;
2. deterministic scheduler readiness;
3. queue-depth accounting;
4. command validation at the platform boundary.

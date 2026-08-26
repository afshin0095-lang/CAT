# CAT LLM Core

`cat-llm` is the provider-neutral LLM boundary for CAT OMNISYSTEM. It defines contracts for model identity, chat messages, generation requests/responses, incremental streaming chunks, usage accounting, safety classification, routing policy, health-aware routing, and deterministic local execution.

## Constitutional boundaries

- Provider SDKs do not become CAT domain truth.
- Model output is untrusted derived intelligence until validated by the consuming engine.
- Routing policy selects an allowed provider/model; it does not authorize business decisions.
- Secrets, API keys, prompts containing sensitive data, and provider-specific credentials remain outside the core contract objects.
- Usage is observable accounting metadata, not monetary truth; Treasury remains the owner of monetary truth.
- Provider health suppresses failing routes but never invents a provider, model, or safety downgrade.
- Streaming chunks are derived output only; they do not establish canonical business truth.
- The core is deterministic when used with the local test provider, making higher-level engines testable without external APIs.

## Streaming contract

`LlmProvider::generate_stream` exposes a provider-neutral stream of `GenerationChunk` values. Native adapters may override it with true provider streaming; the default implementation remains backward-compatible by converting the normal generation response into one terminal chunk.

The deterministic provider emits multiple word-boundary chunks for contract testing. Only the terminal chunk carries usage and finish metadata. `collect_stream` is provided for consumers and tests that need to reconstruct the derived response text.

## Current scope

Production provider adapters, health-aware resilient routing, streaming transports, tool execution, prompt registries, evaluation pipelines, and model-specific optimizers are separate stages. This module owns the contracts and safety boundaries, not business decisions or monetary truth.

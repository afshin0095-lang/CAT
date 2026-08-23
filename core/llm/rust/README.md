# CAT LLM Core

`cat-llm` is the provider-neutral LLM boundary for CAT OMNISYSTEM. It defines contracts for model identity, chat messages, generation requests/responses, usage accounting, safety classification, routing policy, and deterministic local execution.

## Constitutional boundaries

- Provider SDKs do not become CAT domain truth.
- Model output is untrusted derived intelligence until validated by the consuming engine.
- Routing policy selects an allowed provider/model; it does not authorize business decisions.
- Secrets, API keys, prompts containing sensitive data, and provider-specific credentials remain outside the core contract objects.
- Usage is observable accounting metadata, not monetary truth; Treasury remains the owner of monetary truth.
- The core is deterministic when used with the local test provider, making higher-level engines testable without external APIs.

Production provider adapters, streaming transports, tool execution, prompt registries, evaluation pipelines, and model-specific optimizers are separate stages.

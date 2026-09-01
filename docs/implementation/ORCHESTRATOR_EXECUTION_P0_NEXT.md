# Orchestrator Execution P0 — Next Integration

The next implementation boundary is durable worker dispatch: connect execution requests to EventBus-backed delivery, lease ownership, persistence, retries, and recovery while preserving the deterministic core contracts.

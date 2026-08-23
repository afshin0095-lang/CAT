# CAT Orchestrator / Workflow Core

The orchestration foundation owns workflow lifecycle and execution control. It does not own domain truth or monetary truth.

## Current foundation

- explicit workflow and step state machines
- deterministic scheduling queue
- bounded exponential retry policy
- execution leases with ownership and expiry
- reverse-order compensation planning
- immutable-friendly workflow revision tracking
- contract tests for lease, scheduling, retry, compensation, and terminal states

## Constitutional boundaries

1. The orchestrator coordinates; domain engines own domain truth.
2. Treasury remains the authority for monetary truth.
3. Workflow execution is policy-bounded and observable.
4. External, financial, or irreversible actions require the existing approval boundaries.
5. Durable persistence is an adapter boundary; this crate does not silently become a database.

## Next integration layers

The foundation is intentionally transport- and storage-agnostic. Subsequent increments can bind durable workflow state, event-driven execution, leases, retries, and compensation to the existing Event Bus and PostgreSQL Event Store without changing the workflow model.

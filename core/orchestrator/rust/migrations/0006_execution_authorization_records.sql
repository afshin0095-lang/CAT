CREATE TABLE IF NOT EXISTS cat_execution_authorizations (
    execution_id UUID PRIMARY KEY REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    invocation_id UUID NOT NULL,
    agent_id UUID NOT NULL,
    capability_id TEXT NOT NULL,
    requested_side_effect TEXT NOT NULL CHECK (
        requested_side_effect IN ('S0', 'S1', 'S2', 'S3')
    ),
    required_policies JSONB NOT NULL,
    approval_reference TEXT,
    idempotency_key TEXT NOT NULL,
    correlation_id UUID NOT NULL,
    admitted_at TIMESTAMPTZ NOT NULL,
    UNIQUE (execution_id, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_agent
    ON cat_execution_authorizations (agent_id, admitted_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_capability
    ON cat_execution_authorizations (capability_id, admitted_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_invocation
    ON cat_execution_authorizations (invocation_id);

CREATE TABLE IF NOT EXISTS cat_execution_audit_events (
    audit_sequence BIGSERIAL PRIMARY KEY,
    audit_id UUID NOT NULL UNIQUE,
    event_key TEXT NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    workflow_id UUID NOT NULL,
    step_id TEXT NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    action TEXT NOT NULL CHECK (
        action IN ('noop', 'continue', 'confirm_success', 'confirm_failure', 'manual_review')
    ),
    agent_id UUID NOT NULL,
    capability_id TEXT NOT NULL,
    requested_side_effect TEXT NOT NULL CHECK (
        requested_side_effect IN ('S0', 'S1', 'S2', 'S3')
    ),
    approval_reference TEXT,
    correlation_id UUID NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    evidence JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_events_execution
    ON cat_execution_audit_events (execution_id, audit_sequence DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_events_agent
    ON cat_execution_audit_events (agent_id, recorded_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_events_capability
    ON cat_execution_audit_events (capability_id, recorded_at DESC);

CREATE TABLE IF NOT EXISTS cat_execution_audit_read_model (
    execution_id UUID PRIMARY KEY REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    event_key TEXT NOT NULL,
    workflow_id UUID NOT NULL,
    step_id TEXT NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'failed', 'cancelled')),
    action TEXT NOT NULL CHECK (
        action IN ('noop', 'continue', 'confirm_success', 'confirm_failure', 'manual_review')
    ),
    agent_id UUID NOT NULL,
    capability_id TEXT NOT NULL,
    requested_side_effect TEXT NOT NULL CHECK (
        requested_side_effect IN ('S0', 'S1', 'S2', 'S3')
    ),
    approval_reference TEXT,
    correlation_id UUID NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL,
    source_audit_sequence BIGINT NOT NULL,
    evidence JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_read_agent
    ON cat_execution_audit_read_model (agent_id, recorded_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_read_capability
    ON cat_execution_audit_read_model (capability_id, recorded_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_read_action
    ON cat_execution_audit_read_model (action, recorded_at DESC);

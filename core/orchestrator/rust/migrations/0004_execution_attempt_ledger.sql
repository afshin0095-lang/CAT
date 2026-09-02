CREATE TABLE IF NOT EXISTS cat_execution_attempts (
    execution_id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES cat_workflows(id),
    step_id TEXT NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt > 0),
    status TEXT NOT NULL CHECK (status IN ('running', 'succeeded', 'failed', 'cancelled')),
    owner TEXT NOT NULL,
    fencing_token BIGINT NOT NULL CHECK (fencing_token >= 0),
    started_at TIMESTAMPTZ NOT NULL,
    heartbeat_at TIMESTAMPTZ NOT NULL,
    finished_at TIMESTAMPTZ,
    result JSONB,
    error TEXT,
    UNIQUE (workflow_id, step_id, attempt)
);

CREATE INDEX IF NOT EXISTS idx_cat_execution_attempts_workflow_step
    ON cat_execution_attempts (workflow_id, step_id, attempt DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_attempts_running_heartbeat
    ON cat_execution_attempts (heartbeat_at, execution_id)
    WHERE status = 'running';

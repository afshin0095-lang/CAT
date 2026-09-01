CREATE TABLE IF NOT EXISTS cat_workflows (
    id UUID PRIMARY KEY,
    workflow_type TEXT NOT NULL,
    workflow_version INTEGER NOT NULL,
    state JSONB NOT NULL,
    revision BIGINT NOT NULL CHECK (revision >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cat_workflow_outbox (
    event_id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES cat_workflows(id),
    event_type TEXT NOT NULL,
    version INTEGER NOT NULL,
    occurred_at_ms BIGINT NOT NULL,
    payload JSONB NOT NULL,
    attempt INTEGER NOT NULL DEFAULT 0 CHECK (attempt >= 0),
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    claimed_by TEXT,
    claimed_until TIMESTAMPTZ,
    last_error TEXT
);

CREATE INDEX IF NOT EXISTS idx_cat_workflow_outbox_ready
    ON cat_workflow_outbox (available_at, event_id)
    WHERE claimed_by IS NULL;

CREATE TABLE IF NOT EXISTS cat_execution_leases (
    resource TEXT PRIMARY KEY,
    owner TEXT NOT NULL,
    fencing_token BIGINT NOT NULL CHECK (fencing_token >= 0),
    expires_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cat_execution_leases_expiry
    ON cat_execution_leases (expires_at);

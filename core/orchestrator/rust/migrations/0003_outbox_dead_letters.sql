CREATE TABLE IF NOT EXISTS cat_workflow_dead_letters (
    event_id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL,
    event_type TEXT NOT NULL,
    version INTEGER NOT NULL,
    event_kind TEXT NOT NULL,
    occurred_at_ms BIGINT NOT NULL,
    producer TEXT NOT NULL,
    correlation_id UUID,
    causation_id UUID,
    subject_id UUID,
    payload JSONB NOT NULL,
    attempt INTEGER NOT NULL CHECK (attempt >= 0),
    last_error TEXT NOT NULL,
    dead_lettered_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cat_workflow_dead_letters_workflow
    ON cat_workflow_dead_letters (workflow_id, dead_lettered_at, event_id);

CREATE INDEX IF NOT EXISTS idx_cat_workflow_outbox_claimable
    ON cat_workflow_outbox (available_at, claimed_until, event_id);

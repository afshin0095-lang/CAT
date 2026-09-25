CREATE TABLE IF NOT EXISTS cat_provider_execution_journal (
    journal_sequence BIGSERIAL PRIMARY KEY,
    journal_id UUID NOT NULL UNIQUE,
    execution_id UUID NOT NULL REFERENCES cat_execution_attempts(execution_id) ON DELETE CASCADE,
    event_key TEXT NOT NULL UNIQUE,
    event_type TEXT NOT NULL CHECK (event_type IN ('submitted', 'observed')),
    provider TEXT NOT NULL,
    provider_execution_id TEXT NOT NULL,
    request_hash TEXT NOT NULL,
    outcome_state TEXT CHECK (outcome_state IN ('succeeded', 'failed', 'unknown')),
    result JSONB,
    error TEXT,
    recorded_at TIMESTAMPTZ NOT NULL,
    CHECK (
        (event_type = 'submitted' AND outcome_state IS NULL)
        OR event_type = 'observed'
    )
);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_journal_execution
    ON cat_provider_execution_journal (execution_id, journal_sequence);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_journal_provider
    ON cat_provider_execution_journal (provider, provider_execution_id, journal_sequence);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_journal_pending
    ON cat_provider_execution_journal (event_type, recorded_at, execution_id)
    WHERE event_type = 'submitted';

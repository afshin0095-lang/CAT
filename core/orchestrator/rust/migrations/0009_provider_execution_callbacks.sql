CREATE TABLE IF NOT EXISTS cat_provider_execution_callbacks (
    callback_sequence BIGSERIAL PRIMARY KEY,
    callback_id UUID NOT NULL UNIQUE,
    event_key TEXT NOT NULL UNIQUE,
    provider TEXT NOT NULL,
    provider_execution_id TEXT NOT NULL,
    request_hash TEXT,
    outcome_state TEXT NOT NULL CHECK (outcome_state IN ('succeeded', 'failed', 'unknown')),
    result JSONB,
    error TEXT,
    received_at TIMESTAMPTZ NOT NULL,
    execution_id UUID REFERENCES cat_execution_attempts(execution_id) ON DELETE SET NULL,
    correlation_state TEXT NOT NULL CHECK (correlation_state IN ('unmatched', 'correlated')) DEFAULT 'unmatched',
    correlated_at TIMESTAMPTZ,
    CHECK (
        (correlation_state = 'unmatched' AND execution_id IS NULL AND correlated_at IS NULL)
        OR
        (correlation_state = 'correlated' AND execution_id IS NOT NULL AND correlated_at IS NOT NULL)
    )
);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_callbacks_lookup
    ON cat_provider_execution_callbacks (provider, provider_execution_id, callback_sequence);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_callbacks_unmatched
    ON cat_provider_execution_callbacks (provider, callback_sequence)
    WHERE correlation_state = 'unmatched';

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_callbacks_execution
    ON cat_provider_execution_callbacks (execution_id, callback_sequence);

CREATE UNIQUE INDEX IF NOT EXISTS idx_cat_provider_execution_callbacks_event_key
    ON cat_provider_execution_callbacks (event_key);
CREATE TABLE IF NOT EXISTS cat_provider_execution_results (
    execution_id UUID PRIMARY KEY REFERENCES cat_execution_attempts(execution_id),
    provider TEXT NOT NULL,
    provider_execution_id TEXT NOT NULL,
    submission_state TEXT NOT NULL CHECK (submission_state IN ('submitted', 'unknown')) DEFAULT 'submitted',
    outcome_state TEXT CHECK (outcome_state IN ('succeeded', 'failed', 'unknown')),
    request_hash TEXT NOT NULL,
    submitted_at TIMESTAMPTZ NOT NULL,
    observed_at TIMESTAMPTZ,
    result JSONB,
    error TEXT,
    UNIQUE (provider, provider_execution_id),
    CHECK ((outcome_state IS NULL AND observed_at IS NULL) OR (outcome_state IS NOT NULL AND observed_at IS NOT NULL))
);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_results_lookup
    ON cat_provider_execution_results (provider, provider_execution_id);

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_results_pending
    ON cat_provider_execution_results (submitted_at, execution_id)
    WHERE outcome_state IS NULL;

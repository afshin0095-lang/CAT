ALTER TABLE cat_provider_execution_callbacks
    ADD COLUMN IF NOT EXISTS correlation_error TEXT;

ALTER TABLE cat_provider_execution_callbacks
    DROP CONSTRAINT IF EXISTS cat_provider_execution_callbacks_correlation_state_check;

ALTER TABLE cat_provider_execution_callbacks
    ADD CONSTRAINT cat_provider_execution_callbacks_correlation_state_check
    CHECK (
        (correlation_state = 'unmatched' AND execution_id IS NULL AND correlated_at IS NULL)
        OR
        (correlation_state = 'correlated' AND execution_id IS NOT NULL AND correlated_at IS NOT NULL)
        OR
        (correlation_state = 'rejected' AND correlated_at IS NULL)
    );

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_callbacks_rejected
    ON cat_provider_execution_callbacks (provider, callback_sequence)
    WHERE correlation_state = 'rejected';
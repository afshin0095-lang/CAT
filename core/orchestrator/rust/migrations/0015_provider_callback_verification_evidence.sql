ALTER TABLE cat_provider_execution_callbacks
    ADD COLUMN IF NOT EXISTS verification_method TEXT;

ALTER TABLE cat_provider_execution_callbacks
    ADD COLUMN IF NOT EXISTS verification_algorithm TEXT;

ALTER TABLE cat_provider_execution_callbacks
    ADD COLUMN IF NOT EXISTS verification_key_reference TEXT;

ALTER TABLE cat_provider_execution_callbacks
    ADD COLUMN IF NOT EXISTS verification_version INTEGER;

ALTER TABLE cat_provider_execution_callbacks
    ADD COLUMN IF NOT EXISTS verified_at TIMESTAMPTZ;

CREATE INDEX IF NOT EXISTS idx_cat_provider_execution_callbacks_verification
    ON cat_provider_execution_callbacks (provider, verified_at DESC);
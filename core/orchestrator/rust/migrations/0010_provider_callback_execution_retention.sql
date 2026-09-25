-- Provider callback evidence is an external observation record.
-- It may outlive a transient execution-attempt row, so it keeps the
-- canonical execution UUID as evidence without an FK that can erase it.
ALTER TABLE cat_provider_execution_callbacks
    DROP CONSTRAINT IF EXISTS cat_provider_execution_callbacks_execution_id_fkey;
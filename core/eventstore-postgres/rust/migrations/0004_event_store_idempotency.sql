CREATE INDEX IF NOT EXISTS idx_cat_event_idempotency_event
    ON cat_event_idempotency (event_id);

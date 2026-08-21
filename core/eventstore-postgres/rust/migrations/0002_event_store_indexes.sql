CREATE INDEX IF NOT EXISTS idx_cat_events_type_order
    ON cat_events (event_type, occurred_at_ms, sequence);

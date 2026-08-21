CREATE INDEX IF NOT EXISTS idx_cat_events_causation
    ON cat_events (causation_id, occurred_at_ms, sequence);

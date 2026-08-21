CREATE INDEX IF NOT EXISTS idx_cat_events_actor_order
    ON cat_events (actor_id, occurred_at_ms, sequence);

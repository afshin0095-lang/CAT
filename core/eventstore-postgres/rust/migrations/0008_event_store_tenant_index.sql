CREATE INDEX IF NOT EXISTS idx_cat_events_tenant_stream
    ON cat_events (tenant_id, stream_id, sequence);

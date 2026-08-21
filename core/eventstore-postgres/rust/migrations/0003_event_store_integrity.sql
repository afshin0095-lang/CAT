CREATE UNIQUE INDEX IF NOT EXISTS uq_cat_events_stream_event
    ON cat_events (stream_id, event_id);

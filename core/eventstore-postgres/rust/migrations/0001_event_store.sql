CREATE TABLE IF NOT EXISTS cat_event_streams (
    stream_id UUID PRIMARY KEY,
    current_sequence BIGINT NOT NULL DEFAULT 0,
    CONSTRAINT cat_event_streams_sequence_nonnegative CHECK (current_sequence >= 0)
);

CREATE TABLE IF NOT EXISTS cat_events (
    stream_id UUID NOT NULL REFERENCES cat_event_streams(stream_id),
    sequence BIGINT NOT NULL,
    event_id UUID NOT NULL UNIQUE,
    event_type TEXT NOT NULL,
    event_version INTEGER NOT NULL,
    tenant_id UUID NOT NULL,
    correlation_id UUID NOT NULL,
    causation_id UUID,
    actor_id UUID NOT NULL,
    occurred_at_ms BIGINT NOT NULL,
    envelope JSONB NOT NULL,
    PRIMARY KEY (stream_id, sequence),
    CONSTRAINT cat_events_sequence_positive CHECK (sequence > 0),
    CONSTRAINT cat_events_version_positive CHECK (event_version > 0)
);

CREATE INDEX IF NOT EXISTS idx_cat_events_stream_order
    ON cat_events (stream_id, sequence);

CREATE INDEX IF NOT EXISTS idx_cat_events_tenant_order
    ON cat_events (tenant_id, occurred_at_ms, sequence);

CREATE TABLE IF NOT EXISTS cat_event_idempotency (
    idempotency_key TEXT PRIMARY KEY,
    stream_id UUID NOT NULL,
    event_id UUID NOT NULL UNIQUE,
    sequence BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT cat_event_idempotency_sequence_positive CHECK (sequence > 0)
);

CREATE INDEX IF NOT EXISTS idx_cat_event_idempotency_stream
    ON cat_event_idempotency (stream_id, sequence);

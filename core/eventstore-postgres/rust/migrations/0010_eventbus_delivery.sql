CREATE TABLE IF NOT EXISTS cat_event_outbox (
    event_id UUID PRIMARY KEY,
    stream_id UUID NOT NULL,
    sequence BIGINT NOT NULL,
    event_type TEXT NOT NULL,
    envelope JSONB NOT NULL,
    state TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    available_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    claimed_at TIMESTAMPTZ,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT cat_event_outbox_attempts_nonnegative CHECK (attempts >= 0),
    CONSTRAINT cat_event_outbox_state_valid CHECK (
        state IN ('pending', 'in_flight', 'succeeded', 'retry_scheduled', 'dead_lettered')
    )
);

CREATE INDEX IF NOT EXISTS idx_cat_event_outbox_ready
    ON cat_event_outbox (available_at, created_at)
    WHERE state IN ('pending', 'retry_scheduled');

CREATE INDEX IF NOT EXISTS idx_cat_event_outbox_type
    ON cat_event_outbox (event_type, created_at);

CREATE TABLE IF NOT EXISTS cat_event_inbox (
    event_id UUID NOT NULL,
    consumer_name TEXT NOT NULL,
    state TEXT NOT NULL DEFAULT 'in_flight',
    attempts INTEGER NOT NULL DEFAULT 1,
    first_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    last_error TEXT,
    PRIMARY KEY (event_id, consumer_name),
    CONSTRAINT cat_event_inbox_attempts_positive CHECK (attempts > 0),
    CONSTRAINT cat_event_inbox_state_valid CHECK (
        state IN ('in_flight', 'succeeded', 'failed')
    )
);

CREATE INDEX IF NOT EXISTS idx_cat_event_inbox_consumer_state
    ON cat_event_inbox (consumer_name, state, first_seen_at);

COMMENT ON TABLE cat_event_outbox IS 'Transactional publication boundary: durable work is committed with canonical events and published later.';
COMMENT ON TABLE cat_event_inbox IS 'Consumer-side idempotency boundary for at-least-once delivery; each consumer owns an independent claim.';

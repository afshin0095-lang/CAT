CREATE TABLE IF NOT EXISTS cat_projection_checkpoints (
    projection_id TEXT NOT NULL,
    stream_id UUID NOT NULL,
    sequence BIGINT NOT NULL,
    event_id UUID NOT NULL,
    phase TEXT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (projection_id, stream_id),
    CONSTRAINT cat_projection_checkpoints_sequence_nonnegative CHECK (sequence >= 0),
    CONSTRAINT cat_projection_checkpoints_phase_valid CHECK (phase IN ('catchup', 'live'))
);

CREATE INDEX IF NOT EXISTS idx_cat_projection_checkpoints_projection
    ON cat_projection_checkpoints (projection_id, sequence);

ALTER TABLE cat_event_inbox
    ADD COLUMN IF NOT EXISTS claimed_at TIMESTAMPTZ;

ALTER TABLE cat_event_inbox
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

UPDATE cat_event_inbox
SET claimed_at = COALESCE(claimed_at, first_seen_at)
WHERE state = 'in_flight';

CREATE INDEX IF NOT EXISTS idx_cat_event_inbox_stale_claims
    ON cat_event_inbox (consumer_name, claimed_at)
    WHERE state = 'in_flight';

COMMENT ON COLUMN cat_event_inbox.claimed_at IS 'Start of the current delivery attempt; used to recover abandoned claims after worker failure.';
COMMENT ON COLUMN cat_event_inbox.updated_at IS 'Last durable state transition for this consumer claim.';

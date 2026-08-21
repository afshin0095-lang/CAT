COMMENT ON TABLE cat_event_streams IS 'CAT canonical append-only stream version registry; updated transactionally with event append.';
COMMENT ON TABLE cat_events IS 'CAT immutable event journal; rows are never updated in place.';
COMMENT ON TABLE cat_event_idempotency IS 'CAT durable idempotency receipts for exactly-once semantic acceptance.';

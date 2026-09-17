-- Sprint 0 — Affiliate Opportunity Revalidation Requests
--
-- Additive, idempotent migration. Persists revalidation REQUEST facts only:
-- a request is the intent to refresh an opportunity against a source.
-- Attempts and results are execution-boundary concerns (Sprint 1) and are
-- deliberately not modeled here.
--
-- Design rules:
-- - lifecycle state (Active/Stale/Expired) is NEVER persisted here: it is a
--   derived projection over observation facts (see opportunity_freshness).
-- - reasons are stored as forward-compatible snake_case strings without a
--   CHECK constraint so newer schema versions can introduce new reasons;
--   unknown values decode into the domain's `Unknown` reason on read.
-- - statuses ARE a closed set and CHECK-constrained.
-- - the partial unique index enforces idempotent deduplication: only ACTIVE
--   requests (pending/claimed/running) suppress duplicates; terminal
--   requests never block re-issue.
--
-- Reversal (repository convention: manual, additive-only up migrations):
--   DROP INDEX IF EXISTS uq_cat_affiliate_revalidation_active_dedup;
--   DROP INDEX IF EXISTS idx_cat_affiliate_revalidation_claim;
--   DROP INDEX IF EXISTS idx_cat_affiliate_revalidation_identity;
--   DROP TABLE IF EXISTS cat_affiliate_revalidation_requests;

CREATE TABLE IF NOT EXISTS cat_affiliate_revalidation_requests (
    request_id UUID PRIMARY KEY,
    opportunity_id UUID NOT NULL,
    identity TEXT NOT NULL,
    source TEXT NOT NULL,
    reason TEXT NOT NULL,
    priority TEXT NOT NULL CHECK (priority IN ('low','normal','high','critical')),
    status TEXT NOT NULL CHECK (status IN ('pending','claimed','running','succeeded','failed','cancelled','dead_lettered')),
    dedup_key TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL CHECK (created_at_ms >= 0),
    scheduled_at_ms BIGINT NOT NULL CHECK (scheduled_at_ms >= 0),
    started_at_ms BIGINT,
    completed_at_ms BIGINT,
    attempt INTEGER NOT NULL DEFAULT 0 CHECK (attempt >= 0),
    last_error TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Active-request deduplication (idempotency contract; see
-- AFFILIATE_OPPORTUNITY_REVALIDATION_P0.md): one active request per
-- (identity, source, reason, window bucket).
CREATE UNIQUE INDEX IF NOT EXISTS uq_cat_affiliate_revalidation_active_dedup
    ON cat_affiliate_revalidation_requests (dedup_key)
    WHERE status IN ('pending', 'claimed', 'running');

-- Claim path: due pending requests in scheduled order.
CREATE INDEX IF NOT EXISTS idx_cat_affiliate_revalidation_claim
    ON cat_affiliate_revalidation_requests (status, scheduled_at_ms ASC);

-- History per opportunity.
CREATE INDEX IF NOT EXISTS idx_cat_affiliate_revalidation_identity
    ON cat_affiliate_revalidation_requests (identity, created_at_ms DESC);

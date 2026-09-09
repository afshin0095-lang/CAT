CREATE TABLE IF NOT EXISTS cat_affiliate_opportunities (
    identity TEXT PRIMARY KEY,
    opportunity_id UUID NOT NULL UNIQUE,
    merchant_name TEXT NOT NULL,
    product_name TEXT NOT NULL,
    category TEXT,
    best_source TEXT NOT NULL,
    best_score INTEGER NOT NULL CHECK (best_score BETWEEN 0 AND 10000),
    first_observed_at_ms BIGINT NOT NULL CHECK (first_observed_at_ms >= 0),
    last_observed_at_ms BIGINT NOT NULL CHECK (last_observed_at_ms >= first_observed_at_ms),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cat_affiliate_opportunity_observations (
    identity TEXT NOT NULL REFERENCES cat_affiliate_opportunities(identity) ON DELETE CASCADE,
    source TEXT NOT NULL,
    external_id TEXT NOT NULL,
    destination_url TEXT NOT NULL,
    currency TEXT NOT NULL,
    price_minor BIGINT,
    commission_bps INTEGER CHECK (commission_bps BETWEEN 0 AND 100000),
    score INTEGER NOT NULL CHECK (score BETWEEN 0 AND 10000),
    observed_at_ms BIGINT NOT NULL CHECK (observed_at_ms >= 0),
    PRIMARY KEY (identity, source)
);

CREATE INDEX IF NOT EXISTS idx_cat_affiliate_opportunities_best_score
    ON cat_affiliate_opportunities (best_score DESC, identity ASC);

CREATE INDEX IF NOT EXISTS idx_cat_affiliate_observations_external
    ON cat_affiliate_opportunity_observations (source, external_id);

-- CAT Content Engine: publication receipt persistence.
-- A receipt records an observed publication outcome; it does not authorize
-- publication and must never become the source of canonical content truth.

CREATE TABLE IF NOT EXISTS content_publication_receipts (
    tenant_id UUID NOT NULL,
    receipt_id UUID NOT NULL,
    content_id UUID NOT NULL,
    revision_id UUID NOT NULL,
    destination TEXT NOT NULL,
    outcome TEXT NOT NULL,
    policy_version TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    observed_at TIMESTAMPTZ NOT NULL,
    actor_id TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    PRIMARY KEY (tenant_id, receipt_id),
    CONSTRAINT content_publication_receipts_outcome_check
        CHECK (outcome IN ('published', 'rejected', 'failed', 'rolled_back'))
);

CREATE INDEX IF NOT EXISTS idx_content_publication_receipts_content
    ON content_publication_receipts (tenant_id, content_id, observed_at DESC);

CREATE INDEX IF NOT EXISTS idx_content_publication_receipts_revision
    ON content_publication_receipts (tenant_id, revision_id, observed_at DESC);

CREATE INDEX IF NOT EXISTS idx_content_publication_receipts_destination
    ON content_publication_receipts (tenant_id, destination, observed_at DESC);

COMMENT ON TABLE content_publication_receipts IS
    'Observed publication outcomes for Content Engine auditability; not an authorization source.';

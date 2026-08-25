-- CAT Content Engine: durable immutable revision store.
-- Canonical content truth remains owned by core/content; this adapter persists
-- revisions and their provenance without granting the event store mutation rights
-- over content-domain decisions.

CREATE TABLE IF NOT EXISTS content_revisions (
    tenant_id UUID NOT NULL,
    content_id UUID NOT NULL,
    revision_id UUID NOT NULL,
    parent_revision_id UUID NULL,
    revision_number BIGINT NOT NULL,
    content_type TEXT NOT NULL,
    lifecycle_state TEXT NOT NULL,
    canonical_payload JSONB NOT NULL,
    content_hash TEXT NOT NULL,
    provenance JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    created_by TEXT NOT NULL,
    PRIMARY KEY (tenant_id, revision_id),
    UNIQUE (tenant_id, content_id, revision_number),
    UNIQUE (tenant_id, revision_id, content_hash),
    CONSTRAINT content_revisions_self_parent_check
        CHECK (parent_revision_id IS NULL OR parent_revision_id <> revision_id)
);

CREATE INDEX IF NOT EXISTS idx_content_revisions_content
    ON content_revisions (tenant_id, content_id, revision_number DESC);

CREATE INDEX IF NOT EXISTS idx_content_revisions_state
    ON content_revisions (tenant_id, lifecycle_state, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_content_revisions_hash
    ON content_revisions (tenant_id, content_hash);

COMMENT ON TABLE content_revisions IS
    'Immutable persistence boundary for Content Engine revisions; canonical domain policy remains in core/content.';

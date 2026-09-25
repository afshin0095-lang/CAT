ALTER TABLE cat_execution_authorizations
    ADD COLUMN IF NOT EXISTS tenant_id UUID;

ALTER TABLE cat_execution_authorizations
    ADD COLUMN IF NOT EXISTS project_id UUID;

-- Existing authorization rows may predate durable tenant propagation. They remain
-- intentionally nullable until their canonical invocation context can be reconstructed.
CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_tenant
    ON cat_execution_authorizations (tenant_id, admitted_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_project
    ON cat_execution_authorizations (project_id, admitted_at DESC)
    WHERE project_id IS NOT NULL;
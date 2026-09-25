ALTER TABLE cat_execution_authorizations
    ADD COLUMN IF NOT EXISTS tenant_id UUID;

ALTER TABLE cat_execution_authorizations
    ADD COLUMN IF NOT EXISTS project_id UUID;

UPDATE cat_execution_authorizations a
SET tenant_id = w.state->'definition'->>'tenant_id'
FROM cat_execution_attempts e
JOIN cat_workflows w ON w.id = e.workflow_id
WHERE a.execution_id = e.execution_id AND a.tenant_id IS NULL;

CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_tenant
    ON cat_execution_authorizations (tenant_id, admitted_at DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_authorizations_project
    ON cat_execution_authorizations (project_id, admitted_at DESC)
    WHERE project_id IS NOT NULL;
ALTER TABLE cat_execution_audit_events
    ADD COLUMN IF NOT EXISTS tenant_id UUID;

ALTER TABLE cat_execution_audit_events
    ADD COLUMN IF NOT EXISTS project_id UUID;

ALTER TABLE cat_execution_audit_read_model
    ADD COLUMN IF NOT EXISTS tenant_id UUID;

ALTER TABLE cat_execution_audit_read_model
    ADD COLUMN IF NOT EXISTS project_id UUID;

UPDATE cat_execution_audit_events a
SET tenant_id = (a.evidence->'authorization'->>'tenant_id')::uuid,
    project_id = NULLIF(a.evidence->'authorization'->>'project_id', '')::uuid
WHERE a.tenant_id IS NULL;

UPDATE cat_execution_audit_read_model a
SET tenant_id = (a.evidence->'authorization'->>'tenant_id')::uuid,
    project_id = NULLIF(a.evidence->'authorization'->>'project_id', '')::uuid
WHERE a.tenant_id IS NULL;

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_events_scope
    ON cat_execution_audit_events (tenant_id, project_id, audit_sequence DESC);

CREATE INDEX IF NOT EXISTS idx_cat_execution_audit_read_model_scope
    ON cat_execution_audit_read_model (tenant_id, project_id, source_audit_sequence DESC);
ALTER TABLE cat_workflow_outbox ADD COLUMN IF NOT EXISTS event_kind TEXT NOT NULL DEFAULT 'integration';
ALTER TABLE cat_workflow_outbox ADD COLUMN IF NOT EXISTS producer TEXT NOT NULL DEFAULT 'orchestrator.postgres_outbox';
ALTER TABLE cat_workflow_outbox ADD COLUMN IF NOT EXISTS correlation_id UUID;
ALTER TABLE cat_workflow_outbox ADD COLUMN IF NOT EXISTS causation_id UUID;
ALTER TABLE cat_workflow_outbox ADD COLUMN IF NOT EXISTS subject_id UUID;

CREATE INDEX IF NOT EXISTS idx_cat_workflow_outbox_ready_v2
    ON cat_workflow_outbox (available_at, claimed_until, event_id)
    WHERE claimed_by IS NULL OR claimed_until IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_cat_workflow_outbox_claim_expiry
    ON cat_workflow_outbox (claimed_until, event_id)
    WHERE claimed_by IS NOT NULL;

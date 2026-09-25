CREATE TABLE IF NOT EXISTS cat_operator_authorization_decisions (
    decision_sequence BIGSERIAL PRIMARY KEY,
    decision_id UUID NOT NULL UNIQUE,
    principal_id UUID NOT NULL,
    session_id UUID NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('owner', 'operator', 'auditor')),
    permission TEXT NOT NULL CHECK (permission IN ('read_audit', 'read_audit_evidence', 'rebuild_audit_read_model')),
    outcome TEXT NOT NULL CHECK (outcome IN ('allowed', 'denied')),
    policy_version TEXT NOT NULL,
    tenant_id UUID,
    project_id UUID,
    authentication_method TEXT NOT NULL,
    reasons JSONB NOT NULL DEFAULT '[]'::jsonb,
    recorded_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_cat_operator_authorization_decisions_principal
    ON cat_operator_authorization_decisions (principal_id, decision_sequence DESC);

CREATE INDEX IF NOT EXISTS idx_cat_operator_authorization_decisions_session
    ON cat_operator_authorization_decisions (session_id, decision_sequence DESC);

CREATE INDEX IF NOT EXISTS idx_cat_operator_authorization_decisions_scope
    ON cat_operator_authorization_decisions (tenant_id, project_id, decision_sequence DESC);
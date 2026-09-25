CREATE TABLE IF NOT EXISTS cat_operator_identities (
    principal_id UUID PRIMARY KEY,
    external_subject TEXT NOT NULL UNIQUE,
    role TEXT NOT NULL CHECK (role IN ('owner', 'operator', 'auditor')),
    tenant_id UUID NOT NULL,
    project_id UUID,
    resource_scopes JSONB NOT NULL DEFAULT '[]'::jsonb,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_cat_operator_identities_tenant_project
    ON cat_operator_identities (tenant_id, project_id);

CREATE TABLE IF NOT EXISTS cat_operator_sessions (
    session_id UUID PRIMARY KEY,
    principal_id UUID NOT NULL REFERENCES cat_operator_identities(principal_id) ON DELETE CASCADE,
    auth_method TEXT NOT NULL,
    issued_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (expires_at > issued_at),
    CHECK (revoked_at IS NULL OR revoked_at >= issued_at)
);

CREATE INDEX IF NOT EXISTS idx_cat_operator_sessions_principal
    ON cat_operator_sessions (principal_id, expires_at);

CREATE INDEX IF NOT EXISTS idx_cat_operator_sessions_active
    ON cat_operator_sessions (expires_at, session_id)
    WHERE revoked_at IS NULL;
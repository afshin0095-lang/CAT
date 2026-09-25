use async_trait::async_trait;
use cat_kernel::{EntityId, TenantId};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    AuthenticationEvidence, OperatorPrincipal, OperatorRole, OperatorScope, OrchestratorError,
    OrchestratorResult, PostgresExecutionStore,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorIdentityRecord {
    pub principal_id: Uuid,
    pub external_subject: String,
    pub role: OperatorRole,
    pub tenant_id: TenantId,
    pub project_id: Option<EntityId>,
    pub resources: Vec<String>,
    pub enabled: bool,
}

impl OperatorIdentityRecord {
    pub fn to_principal(&self, authentication: AuthenticationEvidence) -> OrchestratorResult<OperatorPrincipal> {
        let scope = OperatorScope::new(self.tenant_id, self.project_id)?;
        let scope = self.resources.iter().try_fold(scope, |scope, resource| {
            scope.with_resource(resource.clone())
        })?;

        Ok(OperatorPrincipal::new(self.principal_id, self.role, authentication)?
            .with_scope(scope))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OperatorSessionRecord {
    pub session_id: Uuid,
    pub principal_id: Uuid,
    pub auth_method: String,
    pub issued_at_ms: u64,
    pub expires_at_ms: u64,
    pub revoked_at_ms: Option<u64>,
}

impl OperatorSessionRecord {
    pub fn validate_at(&self, now_ms: u64) -> OrchestratorResult<()> {
        if self.session_id.is_nil() || self.principal_id.is_nil() {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator session identity must not be nil".into(),
            ));
        }
        if self.auth_method.trim().is_empty() || self.issued_at_ms > self.expires_at_ms {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator session lifetime or authentication method is invalid".into(),
            ));
        }
        if self.issued_at_ms > now_ms || self.expires_at_ms <= now_ms {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator session is not currently valid".into(),
            ));
        }
        if self.revoked_at_ms.is_some_and(|revoked_at| revoked_at <= now_ms) {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator session has been revoked".into(),
            ));
        }
        Ok(())
    }
}

#[async_trait]
pub trait OperatorIdentityStore: Send + Sync {
    async fn load_principal_for_session(
        &self,
        session_id: Uuid,
        now_ms: u64,
    ) -> OrchestratorResult<OperatorPrincipal>;

    async fn revoke_session(
        &self,
        session_id: Uuid,
        revoked_at_ms: u64,
    ) -> OrchestratorResult<()>;
}

impl PostgresExecutionStore {
    async fn load_operator_principal_for_session(
        &self,
        session_id: Uuid,
        now_ms: u64,
    ) -> OrchestratorResult<OperatorPrincipal> {
        if session_id.is_nil() || now_ms == 0 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "invalid operator session lookup".into(),
            ));
        }

        let row = sqlx::query(
            "SELECT i.principal_id, i.external_subject, i.role, i.tenant_id, i.project_id,
                    i.resource_scopes, i.enabled,
                    s.session_id, s.auth_method,
                    EXTRACT(EPOCH FROM s.issued_at) * 1000 AS issued_at_ms,
                    EXTRACT(EPOCH FROM s.expires_at) * 1000 AS expires_at_ms,
                    EXTRACT(EPOCH FROM s.revoked_at) * 1000 AS revoked_at_ms
             FROM cat_operator_sessions s
             JOIN cat_operator_identities i ON i.principal_id = s.principal_id
             WHERE s.session_id = $1
             FOR UPDATE OF i, s",
        )
        .bind(session_id)
        .fetch_optional(self.pool())
        .await
        .map_err(db_error)?
        .ok_or_else(|| OrchestratorError::InvalidAuthorizationInput(
            "operator session or identity was not found".into(),
        ))?;

        let session = decode_session(&row)?;
        session.validate_at(now_ms)?;
        let enabled: bool = row.try_get("enabled").map_err(row_error)?;
        if !enabled {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator identity is disabled".into(),
            ));
        }

        let resources: Vec<String> = serde_json::from_value(
            row.try_get::<serde_json::Value, _>("resource_scopes").map_err(row_error)?
        )
        .map_err(json_error)?;

        let identity = OperatorIdentityRecord {
            principal_id: row.try_get("principal_id").map_err(row_error)?,
            external_subject: row.try_get("external_subject").map_err(row_error)?,
            role: parse_role(&row.try_get::<String, _>("role").map_err(row_error)?)?,
            tenant_id: TenantId::from_uuid(row.try_get("tenant_id").map_err(row_error)?),
            project_id: row.try_get("project_id").map(row_error)?.map(EntityId::from_uuid),
            resources,
            enabled,
        };

        let auth = AuthenticationEvidence::new(
            session.auth_method.clone(),
            session.session_id,
            session.issued_at_ms,
        )?;
        identity.to_principal(auth)
    }

    async fn revoke_operator_session(
        &self,
        session_id: Uuid,
        revoked_at_ms: u64,
    ) -> OrchestratorResult<()> {
        if session_id.is_nil() || revoked_at_ms == 0 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "invalid operator session revocation".into(),
            ));
        }

        let result = sqlx::query(
            "UPDATE cat_operator_sessions
             SET revoked_at = TO_TIMESTAMP($2 / 1000.0)
             WHERE session_id = $1 AND (revoked_at IS NULL OR revoked_at > TO_TIMESTAMP($2 / 1000.0))",
        )
        .bind(session_id)
        .bind(revoked_at_ms as f64)
        .execute(self.pool())
        .await
        .map_err(db_error)?;

        if result.rows_affected() != 1 {
            return Err(OrchestratorError::InvalidAuthorizationInput(
                "operator session was not active".into(),
            ));
        }
        Ok(())
    }
}

#[async_trait]
impl OperatorIdentityStore for PostgresExecutionStore {
    async fn load_principal_for_session(
        &self,
        session_id: Uuid,
        now_ms: u64,
    ) -> OrchestratorResult<OperatorPrincipal> {
        self.load_operator_principal_for_session(session_id, now_ms).await
    }

    async fn revoke_session(&self, session_id: Uuid, revoked_at_ms: u64) -> OrchestratorResult<()> {
        self.revoke_operator_session(session_id, revoked_at_ms).await
    }
}

fn decode_session(row: &sqlx::postgres::PgRow) -> OrchestratorResult<OperatorSessionRecord> {
    let issued_at_ms: f64 = row.try_get("issued_at_ms").map_err(row_error)?;
    let expires_at_ms: f64 = row.try_get("expires_at_ms").map_err(row_error)?;
    let revoked_at_ms: Option<f64> = row.try_get("revoked_at_ms").map_err(row_error)?;
    Ok(OperatorSessionRecord {
        session_id: row.try_get("session_id").map_err(row_error)?,
        principal_id: row.try_get("principal_id").map_err(row_error)?,
        auth_method: row.try_get("auth_method").map_err(row_error)?,
        issued_at_ms: issued_at_ms.max(0.0) as u64,
        expires_at_ms: expires_at_ms.max(0.0) as u64,
        revoked_at_ms: revoked_at_ms.map(|value| value.max(0.0) as u64),
    })
}

fn parse_role(value: &str) -> OrchestratorResult<OperatorRole> {
    match value {
        "owner" => Ok(OperatorRole::Owner),
        "operator" => Ok(OperatorRole::Operator),
        "auditor" => Ok(OperatorRole::Auditor),
        other => Err(OrchestratorError::InvalidAuthorizationInput(format!(
            "unknown operator role: {other}"
        ))),
    }
}

fn db_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql operator identity error: {error}"))
}

fn row_error(error: sqlx::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("postgresql operator identity row error: {error}"))
}

fn json_error(error: serde_json::Error) -> OrchestratorError {
    OrchestratorError::Serialization(format!("operator identity JSON error: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_session_is_accepted_at_boundary() {
        let session = OperatorSessionRecord {
            session_id: Uuid::new_v4(),
            principal_id: Uuid::new_v4(),
            auth_method: "oidc".into(),
            issued_at_ms: 1_000,
            expires_at_ms: 2_000,
            revoked_at_ms: None,
        };
        assert!(session.validate_at(1_500).is_ok());
    }

    #[test]
    fn expired_session_is_rejected() {
        let session = OperatorSessionRecord {
            session_id: Uuid::new_v4(),
            principal_id: Uuid::new_v4(),
            auth_method: "oidc".into(),
            issued_at_ms: 1_000,
            expires_at_ms: 2_000,
            revoked_at_ms: None,
        };
        assert!(session.validate_at(2_000).is_err());
    }

    #[test]
    fn revoked_session_is_rejected() {
        let session = OperatorSessionRecord {
            session_id: Uuid::new_v4(),
            principal_id: Uuid::new_v4(),
            auth_method: "oidc".into(),
            issued_at_ms: 1_000,
            expires_at_ms: 3_000,
            revoked_at_ms: Some(2_000),
        };
        assert!(session.validate_at(2_000).is_err());
    }

    #[test]
    fn identity_builds_scoped_principal_without_credentials() {
        let tenant = TenantId::new();
        let identity = OperatorIdentityRecord {
            principal_id: Uuid::new_v4(),
            external_subject: "subject".into(),
            role: OperatorRole::Auditor,
            tenant_id: tenant,
            project_id: Some(EntityId::new()),
            resources: vec!["audit/read".into()],
            enabled: true,
        };
        let principal = identity
            .to_principal(AuthenticationEvidence::new("oidc", Uuid::new_v4(), 1_000).unwrap())
            .unwrap();
        assert_eq!(principal.scope.as_ref().unwrap().tenant_id, tenant);
        assert!(principal.scope.as_ref().unwrap().allows_resource("audit/read"));
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{OrchestratorError, OrchestratorResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lease {
    pub id: Uuid,
    pub resource: String,
    pub owner: String,
    pub expires_at_ms: u64,
}

impl Lease {
    pub fn acquire(
        resource: impl Into<String>,
        owner: impl Into<String>,
        now_ms: u64,
        ttl_ms: u64,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            resource: resource.into(),
            owner: owner.into(),
            expires_at_ms: now_ms.saturating_add(ttl_ms),
        }
    }

    pub fn valid_for(&self, owner: &str, now_ms: u64) -> OrchestratorResult<()> {
        if self.owner != owner {
            return Err(OrchestratorError::LeaseOwnerMismatch {
                lease_id: self.id.to_string(),
                owner: owner.to_owned(),
            });
        }
        if now_ms >= self.expires_at_ms {
            return Err(OrchestratorError::LeaseExpired {
                lease_id: self.id.to_string(),
            });
        }
        Ok(())
    }

    pub fn renew(&mut self, owner: &str, now_ms: u64, ttl_ms: u64) -> OrchestratorResult<()> {
        self.valid_for(owner, now_ms)?;
        self.expires_at_ms = now_ms.saturating_add(ttl_ms);
        Ok(())
    }
}

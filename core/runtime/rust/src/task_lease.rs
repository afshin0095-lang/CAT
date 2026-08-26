use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskLease {
    pub owner: String,
    pub epoch: u64,
    pub expires_at_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LeaseError {
    InvalidOwner,
    InvalidDuration,
    AlreadyHeld,
    NotOwner,
    Expired,
}

impl TaskLease {
    pub fn acquire(owner: impl Into<String>, epoch: u64, ttl: Duration) -> Result<Self, LeaseError> {
        let owner = owner.into();
        if owner.trim().is_empty() {
            return Err(LeaseError::InvalidOwner);
        }
        if ttl.is_zero() {
            return Err(LeaseError::InvalidDuration);
        }
        Ok(Self {
            owner,
            epoch,
            expires_at_ms: now_ms().saturating_add(ttl.as_millis() as u64),
        })
    }

    pub fn is_expired_at(&self, now_ms: u64) -> bool {
        now_ms >= self.expires_at_ms
    }

    pub fn renew(&mut self, owner: &str, ttl: Duration, now_ms: u64) -> Result<(), LeaseError> {
        if ttl.is_zero() {
            return Err(LeaseError::InvalidDuration);
        }
        if self.is_expired_at(now_ms) {
            return Err(LeaseError::Expired);
        }
        if self.owner != owner {
            return Err(LeaseError::NotOwner);
        }
        self.expires_at_ms = now_ms.saturating_add(ttl.as_millis() as u64);
        Ok(())
    }

    pub fn release(&self, owner: &str, now_ms: u64) -> Result<(), LeaseError> {
        if self.is_expired_at(now_ms) {
            return Err(LeaseError::Expired);
        }
        if self.owner != owner {
            return Err(LeaseError::NotOwner);
        }
        Ok(())
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}

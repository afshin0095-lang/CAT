use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Lease, OrchestratorError, OrchestratorResult};

/// Monotonically increasing token used to fence stale workers after lease turnover.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct FencingToken(u64);

impl FencingToken {
    pub const fn value(self) -> u64 {
        self.0
    }
    pub(crate) const fn from_value(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FencedLease {
    pub lease: Lease,
    pub fencing_token: FencingToken,
}

impl FencedLease {
    pub fn valid_for(
        &self,
        owner: &str,
        fencing_token: FencingToken,
        now_ms: u64,
    ) -> OrchestratorResult<()> {
        self.lease.valid_for(owner, now_ms)?;
        if fencing_token != self.fencing_token {
            return Err(OrchestratorError::FencingTokenMismatch {
                resource: self.lease.resource.clone(),
                expected: self.fencing_token.value(),
                actual: fencing_token.value(),
            });
        }
        Ok(())
    }
}

/// Distributed implementations must persist and atomically advance the token per resource.
pub trait FencedLeaseProvider {
    fn acquire(
        &mut self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<FencedLease>;
    fn validate(
        &self,
        resource: &str,
        owner: &str,
        fencing_token: FencingToken,
        now_ms: u64,
    ) -> OrchestratorResult<()>;
}

#[derive(Default)]
pub struct InMemoryFencedLeaseProvider {
    leases: HashMap<String, FencedLease>,
    next_tokens: HashMap<String, u64>,
}

impl FencedLeaseProvider for InMemoryFencedLeaseProvider {
    fn acquire(
        &mut self,
        resource: &str,
        owner: &str,
        now_ms: u64,
        ttl_ms: u64,
    ) -> OrchestratorResult<FencedLease> {
        if let Some(existing) = self.leases.get(resource)
            && existing.lease.expires_at_ms > now_ms
            && existing.lease.owner != owner
        {
                return Err(OrchestratorError::LeaseUnavailable {
                    resource: resource.to_owned(),
                });
            }
        }

        let next = self.next_tokens.entry(resource.to_owned()).or_insert(0);
        *next = next.saturating_add(1);
        let fenced = FencedLease {
            lease: Lease::acquire(resource, owner, now_ms, ttl_ms),
            fencing_token: FencingToken(*next),
        };
        self.leases.insert(resource.to_owned(), fenced.clone());
        Ok(fenced)
    }

    fn validate(
        &self,
        resource: &str,
        owner: &str,
        fencing_token: FencingToken,
        now_ms: u64,
    ) -> OrchestratorResult<()> {
        let current =
            self.leases
                .get(resource)
                .ok_or_else(|| OrchestratorError::LeaseUnavailable {
                    resource: resource.to_owned(),
                })?;
        current.valid_for(owner, fencing_token, now_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_increments_after_lease_turnover() {
        let mut provider = InMemoryFencedLeaseProvider::default();
        let first = provider.acquire("workflow/1", "worker-a", 100, 10).unwrap();
        assert_eq!(first.fencing_token.value(), 1);
        let second = provider.acquire("workflow/1", "worker-b", 110, 10).unwrap();
        assert_eq!(second.fencing_token.value(), 2);
        assert_ne!(first.lease.id, second.lease.id);
    }

    #[test]
    fn stale_token_is_rejected() {
        let mut provider = InMemoryFencedLeaseProvider::default();
        let first = provider.acquire("workflow/1", "worker-a", 100, 10).unwrap();
        let second = provider.acquire("workflow/1", "worker-b", 110, 10).unwrap();
        let error = provider
            .validate("workflow/1", "worker-b", first.fencing_token, 111)
            .unwrap_err();
        assert!(matches!(
            error,
            OrchestratorError::FencingTokenMismatch { .. }
        ));
        provider
            .validate("workflow/1", "worker-b", second.fencing_token, 111)
            .unwrap();
    }

    #[test]
    fn validation_rejects_expired_fenced_lease() {
        let mut provider = InMemoryFencedLeaseProvider::default();
        let lease = provider.acquire("workflow/1", "worker-a", 100, 10).unwrap();
        assert!(matches!(
            provider.validate("workflow/1", "worker-a", lease.fencing_token, 110),
            Err(OrchestratorError::LeaseExpired { .. })
        ));
    }

    #[test]
    fn token_order_is_monotonic() {
        let mut provider = InMemoryFencedLeaseProvider::default();
        let first = provider.acquire("workflow/1", "worker-a", 0, 1).unwrap();
        let second = provider.acquire("workflow/1", "worker-b", 1, 1).unwrap();
        assert!(second.fencing_token > first.fencing_token);
    }
}

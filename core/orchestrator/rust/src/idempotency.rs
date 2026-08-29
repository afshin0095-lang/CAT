use std::collections::HashSet;

use uuid::Uuid;

use crate::error::{OrchestratorError, OrchestratorResult};

#[derive(Clone, Debug, Default)]
pub struct IdempotencyRegistry {
    keys: HashSet<String>,
}

impl IdempotencyRegistry {
    pub fn claim(&mut self, key: impl Into<String>) -> OrchestratorResult<()> {
        let key = key.into();
        if key.trim().is_empty() {
            return Err(OrchestratorError::Serialization(
                "idempotency key must not be empty".to_string(),
            ));
        }
        if !self.keys.insert(key.clone()) {
            return Err(OrchestratorError::Serialization(format!(
                "idempotency key already claimed: {key}"
            )));
        }
        Ok(())
    }

    pub fn contains(&self, key: &str) -> bool {
        self.keys.contains(key)
    }
}

pub fn workflow_key(workflow_id: Uuid, operation: &str, revision: u64) -> String {
    format!("{workflow_id}:{operation}:{revision}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_duplicate_claims() {
        let mut registry = IdempotencyRegistry::default();
        registry.claim("x").unwrap();
        assert!(registry.claim("x").is_err());
    }

    #[test]
    fn workflow_key_is_stable() {
        let id = Uuid::nil();
        assert_eq!(workflow_key(id, "start", 2), format!("{id}:start:2"));
    }
}

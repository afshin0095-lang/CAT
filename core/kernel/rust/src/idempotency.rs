use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{EventId, KernelError, KernelResult, SequenceNumber};

/// Stable key supplied by a command or delivery attempt to make side effects safe to retry.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    pub fn new(value: impl Into<String>) -> KernelResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(KernelError::InvalidIdentifier(
                "idempotency key must not be empty".to_owned(),
            ));
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Result recorded for a previously accepted operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdempotencyReceipt {
    pub event_id: EventId,
    pub sequence: SequenceNumber,
}

/// In-memory idempotency ledger used by the kernel contract and tests.
///
/// A production adapter can persist the same semantic contract in a durable store.
#[derive(Clone, Debug, Default)]
pub struct IdempotencyLedger {
    receipts: HashMap<IdempotencyKey, IdempotencyReceipt>,
}

impl IdempotencyLedger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup(&self, key: &IdempotencyKey) -> Option<IdempotencyReceipt> {
        self.receipts.get(key).copied()
    }

    pub fn record(
        &mut self,
        key: IdempotencyKey,
        receipt: IdempotencyReceipt,
    ) -> KernelResult<IdempotencyReceipt> {
        if let Some(existing) = self.receipts.get(&key).copied() {
            return Ok(existing);
        }
        self.receipts.insert(key, receipt);
        Ok(receipt)
    }

    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_keys() {
        assert!(IdempotencyKey::new("  ").is_err());
    }

    #[test]
    fn returns_the_original_receipt_for_duplicate_keys() {
        let mut ledger = IdempotencyLedger::new();
        let key = IdempotencyKey::new("command-1").unwrap();
        let first = IdempotencyReceipt {
            event_id: EventId::new(),
            sequence: SequenceNumber::new(1).unwrap(),
        };
        let second = IdempotencyReceipt {
            event_id: EventId::new(),
            sequence: SequenceNumber::new(2).unwrap(),
        };

        assert_eq!(ledger.record(key.clone(), first).unwrap(), first);
        assert_eq!(ledger.record(key.clone(), second).unwrap(), first);
        assert_eq!(ledger.lookup(&key), Some(first));
        assert_eq!(ledger.len(), 1);
    }
}

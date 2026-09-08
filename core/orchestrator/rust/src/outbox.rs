use std::collections::VecDeque;
use cat_eventbus::EventEnvelope;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{OrchestratorError, OrchestratorResult, RetryPolicy};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OutboxRecord { pub event: EventEnvelope, pub attempt: u32, pub available_at_ms: u64, pub claimed_by: Option<String> }
pub trait DurableOutboxStore { fn enqueue(&mut self, event: EventEnvelope) -> OrchestratorResult<()>; fn claim_next(&mut self, owner: &str, now_ms: u64) -> OrchestratorResult<Option<OutboxRecord>>; fn acknowledge(&mut self, event_id: Uuid, owner: &str) -> OrchestratorResult<()>; fn fail(&mut self, event_id: Uuid, owner: &str, now_ms: u64, policy: RetryPolicy, error: &str) -> OrchestratorResult<OutboxDisposition>; }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboxDisposition { RetryScheduled { attempt: u32, available_at_ms: u64 }, DeadLettered { attempt: u32 } }
#[derive(Default)] pub struct InMemoryDurableOutbox { records: VecDeque<OutboxRecord> }
impl InMemoryDurableOutbox { pub fn pending(&self) -> usize { self.records.len() } }
impl DurableOutboxStore for InMemoryDurableOutbox {
    fn enqueue(&mut self, event: EventEnvelope) -> OrchestratorResult<()> { if self.records.iter().any(|record| record.event.event_id == event.event_id) { return Err(OrchestratorError::Serialization("duplicate outbox event id".into())); } self.records.push_back(OutboxRecord { event, attempt: 0, available_at_ms: 0, claimed_by: None }); Ok(()) }
    fn claim_next(&mut self, owner: &str, now_ms: u64) -> OrchestratorResult<Option<OutboxRecord>> { let Some(position) = self.records.iter().position(|record| record.available_at_ms <= now_ms && record.claimed_by.as_deref().is_none_or(|claimant| claimant == owner)) else { return Ok(None); }; let record = self.records.get_mut(position).expect("position was produced from the same queue"); record.claimed_by = Some(owner.to_owned()); Ok(Some(record.clone())) }
    fn acknowledge(&mut self, event_id: Uuid, owner: &str) -> OrchestratorResult<()> { let position = self.records.iter().position(|record| record.event.event_id == event_id).ok_or_else(|| OrchestratorError::Serialization("outbox event not found".into()))?; if self.records[position].claimed_by.as_deref() != Some(owner) { return Err(OrchestratorError::LeaseOwnerMismatch { lease_id: event_id.to_string(), owner: owner.to_owned() }); } self.records.remove(position); Ok(()) }
    fn fail(&mut self, event_id: Uuid, owner: &str, now_ms: u64, policy: RetryPolicy, error: &str) -> OrchestratorResult<OutboxDisposition> {
        let position = self.records.iter().position(|record| record.event.event_id == event_id).ok_or_else(|| OrchestratorError::Serialization("outbox event not found".into()))?;
        let attempt = self.records[position].attempt.saturating_add(1);
        if self.records[position].claimed_by.as_deref() != Some(owner) { return Err(OrchestratorError::LeaseOwnerMismatch { lease_id: event_id.to_string(), owner: owner.to_owned() }); }
        if policy.retryable(attempt) {
            let available_at = now_ms.saturating_add(policy.delay_ms(attempt));
            let record = self.records.get_mut(position).expect("position was produced from the same queue");
            record.attempt = attempt; record.claimed_by = None; record.available_at_ms = available_at;
            Ok(OutboxDisposition::RetryScheduled { attempt, available_at_ms: available_at })
        } else {
            let _ = error;
            self.records.remove(position);
            Ok(OutboxDisposition::DeadLettered { attempt })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn event() -> EventEnvelope { EventEnvelope { event_id: Uuid::now_v7(), event_type: "orchestrator.test".into(), version: 1, kind: cat_eventbus::EventKind::Domain, occurred_at_ms: 1, producer: "test".into(), correlation_id: None, causation_id: None, subject_id: None, payload: serde_json::json!({}) } }
    #[test] fn claim_ack_removes_record() { let mut outbox = InMemoryDurableOutbox::default(); let event = event(); let id = event.event_id; outbox.enqueue(event).unwrap(); let record = outbox.claim_next("dispatcher-a", 0).unwrap().unwrap(); assert_eq!(record.event.event_id, id); outbox.acknowledge(id, "dispatcher-a").unwrap(); assert_eq!(outbox.pending(), 0); }
    #[test] fn failed_record_is_rescheduled_before_exhaustion() { let mut outbox = InMemoryDurableOutbox::default(); let event = event(); let id = event.event_id; outbox.enqueue(event).unwrap(); outbox.claim_next("dispatcher-a", 100).unwrap(); let disposition = outbox.fail(id, "dispatcher-a", 100, RetryPolicy::default(), "timeout").unwrap(); assert!(matches!(disposition, OutboxDisposition::RetryScheduled { .. })); assert!(outbox.claim_next("dispatcher-a", 100).unwrap().is_none()); }
    #[test] fn claim_cannot_be_acked_by_wrong_owner() { let mut outbox = InMemoryDurableOutbox::default(); let event = event(); let id = event.event_id; outbox.enqueue(event).unwrap(); outbox.claim_next("dispatcher-a", 0).unwrap(); assert!(matches!(outbox.acknowledge(id, "dispatcher-b"), Err(OrchestratorError::LeaseOwnerMismatch { .. }))); }
}

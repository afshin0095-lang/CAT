use uuid::Uuid;

use crate::{
    DurableOutboxStore, ExecutionEventSink, OrchestratorResult, OutboxDisposition, RetryPolicy,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutboxDispatchOutcome {
    Idle,
    Delivered,
    RetryScheduled { attempt: u32, available_at_ms: u64 },
    DeadLettered { attempt: u32 },
}

/// Drains the durable outbox without making persistence or transport choices.
pub struct OutboxDispatcher<'a, O, P> {
    pub outbox: &'a mut O,
    pub publisher: &'a mut P,
    pub retry_policy: RetryPolicy,
    pub owner: String,
}

impl<'a, O, P> OutboxDispatcher<'a, O, P>
where
    O: DurableOutboxStore,
    P: ExecutionEventSink,
{
    pub fn dispatch_one(&mut self, now_ms: u64) -> OrchestratorResult<OutboxDispatchOutcome> {
        let Some(record) = self.outbox.claim_next(&self.owner, now_ms)? else {
            return Ok(OutboxDispatchOutcome::Idle);
        };

        let event_id = record.event.event_id;
        match self.publisher.publish(record.event) {
            Ok(()) => {
                self.outbox.acknowledge(event_id, &self.owner)?;
                Ok(OutboxDispatchOutcome::Delivered)
            }
            Err(error) => match self.outbox.fail(
                event_id,
                &self.owner,
                now_ms,
                self.retry_policy,
                &error.to_string(),
            )? {
                OutboxDisposition::RetryScheduled {
                    attempt,
                    available_at_ms,
                } => Ok(OutboxDispatchOutcome::RetryScheduled {
                    attempt,
                    available_at_ms,
                }),
                OutboxDisposition::DeadLettered { attempt } => {
                    Ok(OutboxDispatchOutcome::DeadLettered { attempt })
                }
            },
        }
    }

    pub fn dispatch_until_idle(
        &mut self,
        now_ms: u64,
        max_events: usize,
    ) -> OrchestratorResult<usize> {
        let mut delivered = 0usize;
        for _ in 0..max_events {
            match self.dispatch_one(now_ms)? {
                OutboxDispatchOutcome::Idle => break,
                _ => delivered = delivered.saturating_add(1),
            }
        }
        Ok(delivered)
    }
}

#[allow(dead_code)]
fn event_id(record: &crate::OutboxRecord) -> Uuid {
    record.event.event_id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ExecutionEventSink, InMemoryDurableOutbox, OutboxRecord, WorkflowEventFactory};

    struct RecordingPublisher {
        events: Vec<cat_eventbus::EventEnvelope>,
        fail_once: bool,
    }

    impl ExecutionEventSink for RecordingPublisher {
        fn publish(&mut self, event: cat_eventbus::EventEnvelope) -> crate::OrchestratorResult<()> {
            if self.fail_once {
                self.fail_once = false;
                return Err(crate::OrchestratorError::Serialization(
                    "temporary transport failure".into(),
                ));
            }
            self.events.push(event);
            Ok(())
        }
    }

    fn event() -> cat_eventbus::EventEnvelope {
        WorkflowEventFactory::started(uuid::Uuid::now_v7(), "affiliate.test", 1, "test").unwrap()
    }

    #[test]
    fn dispatcher_delivers_and_acknowledges() {
        let mut outbox = InMemoryDurableOutbox::default();
        outbox.enqueue(event()).unwrap();
        let mut publisher = RecordingPublisher {
            events: Vec::new(),
            fail_once: false,
        };
        let mut dispatcher = OutboxDispatcher {
            outbox: &mut outbox,
            publisher: &mut publisher,
            retry_policy: RetryPolicy::default(),
            owner: "dispatcher-a".into(),
        };
        assert_eq!(
            dispatcher.dispatch_one(100).unwrap(),
            OutboxDispatchOutcome::Delivered
        );
        assert_eq!(dispatcher.publisher.events.len(), 1);
        assert_eq!(dispatcher.outbox.pending(), 0);
    }

    #[test]
    fn dispatcher_preserves_failed_event_for_retry() {
        let mut outbox = InMemoryDurableOutbox::default();
        outbox.enqueue(event()).unwrap();
        let mut publisher = RecordingPublisher {
            events: Vec::new(),
            fail_once: true,
        };
        let mut dispatcher = OutboxDispatcher {
            outbox: &mut outbox,
            publisher: &mut publisher,
            retry_policy: RetryPolicy::default(),
            owner: "dispatcher-a".into(),
        };
        assert!(matches!(
            dispatcher.dispatch_one(100).unwrap(),
            OutboxDispatchOutcome::RetryScheduled { .. }
        ));
        assert_eq!(dispatcher.outbox.pending(), 1);
        assert!(matches!(
            dispatcher.dispatch_one(100).unwrap(),
            OutboxDispatchOutcome::Idle
        ));
        assert_eq!(
            dispatcher.dispatch_one(350).unwrap(),
            OutboxDispatchOutcome::Delivered
        );
    }

    #[test]
    fn dispatcher_drains_to_limit() {
        let mut outbox = InMemoryDurableOutbox::default();
        outbox.enqueue(event()).unwrap();
        outbox.enqueue(event()).unwrap();
        let mut publisher = RecordingPublisher {
            events: Vec::new(),
            fail_once: false,
        };
        let mut dispatcher = OutboxDispatcher {
            outbox: &mut outbox,
            publisher: &mut publisher,
            retry_policy: RetryPolicy::default(),
            owner: "dispatcher-a".into(),
        };
        assert_eq!(dispatcher.dispatch_until_idle(100, 1).unwrap(), 1);
        assert_eq!(dispatcher.outbox.pending(), 1);
    }

    #[test]
    fn event_id_helper_is_stable() {
        let record = OutboxRecord {
            event: event(),
            attempt: 0,
            available_at_ms: 0,
            claimed_by: None,
        };
        assert_eq!(event_id(&record), record.event.event_id);
    }
}

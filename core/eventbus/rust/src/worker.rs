use crate::{DeliveryOutcome, EventBusResult, EventTransport, OutboxDispatcher, OutboxStore, DeadLetterStore, RetryPolicy};
use std::time::Duration;

/// Runtime policy for a durable delivery worker.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeliveryWorkerConfig {
    pub max_attempts_per_run: u32,
    pub idle_poll_delay: Duration,
}

impl Default for DeliveryWorkerConfig {
    fn default() -> Self {
        Self { max_attempts_per_run: 1024, idle_poll_delay: Duration::from_millis(250) }
    }
}

/// Deterministic lifecycle wrapper around the durable outbox dispatcher.
pub struct DeliveryWorker<O, T, D> {
    pub dispatcher: OutboxDispatcher<O, T, D>,
    pub config: DeliveryWorkerConfig,
}

impl<O, T, D> DeliveryWorker<O, T, D>
where O: OutboxStore, T: EventTransport, D: DeadLetterStore {
    pub fn new(outbox: O, transport: T, dead_letters: D, retry_policy: RetryPolicy) -> Self {
        Self { dispatcher: OutboxDispatcher { outbox, transport, dead_letters, retry_policy }, config: DeliveryWorkerConfig::default() }
    }

    pub fn tick(&mut self, attempt: u32) -> EventBusResult<DeliveryOutcome> {
        self.dispatcher.dispatch_one(attempt)
    }

    pub fn run_until_idle(&mut self) -> EventBusResult<Vec<DeliveryOutcome>> {
        let mut outcomes = Vec::new();
        for attempt in 0..self.config.max_attempts_per_run {
            let outcome = self.tick(attempt)?;
            let idle = matches!(outcome, DeliveryOutcome::Idle);
            outcomes.push(outcome);
            if idle { break; }
        }
        Ok(outcomes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InMemoryDeadLetterStore, InMemoryOutbox, RecordingTransport};

    #[test]
    fn worker_stops_when_outbox_is_idle() {
        let mut worker = DeliveryWorker::new(InMemoryOutbox::default(), RecordingTransport::default(), InMemoryDeadLetterStore::default(), RetryPolicy::default());
        assert_eq!(worker.run_until_idle().unwrap(), vec![DeliveryOutcome::Idle]);
    }

    #[test]
    fn default_config_has_bounded_drain() {
        let config = DeliveryWorkerConfig::default();
        assert_eq!(config.max_attempts_per_run, 1024);
        assert_eq!(config.idle_poll_delay, Duration::from_millis(250));
    }
}

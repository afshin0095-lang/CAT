use crate::{DeliveryOutcome, EventBusResult, OutboxDispatcher, OutboxStore, EventTransport, DeadLetterStore};
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct DeliveryMetrics {
    idle: AtomicU64,
    delivered: AtomicU64,
    retries: AtomicU64,
    dead_lettered: AtomicU64,
    last_attempt: AtomicU64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DeliveryMetricsSnapshot {
    pub idle: u64,
    pub delivered: u64,
    pub retries: u64,
    pub dead_lettered: u64,
    pub total_attempts: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeliveryHealth { Healthy, Degraded, Unhealthy }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeliveryHealthSnapshot {
    pub status: DeliveryHealth,
    pub successful: u64,
    pub retries: u64,
    pub dead_lettered: u64,
    pub total_attempts: u64,
}

impl DeliveryMetrics {
    pub fn record(&self, outcome: DeliveryOutcome) {
        match outcome {
            DeliveryOutcome::Idle => { self.idle.fetch_add(1, Ordering::Relaxed); }
            DeliveryOutcome::Delivered => {
                self.delivered.fetch_add(1, Ordering::Relaxed);
                self.last_attempt.fetch_add(1, Ordering::Relaxed);
            }
            DeliveryOutcome::RetryScheduled { .. } => {
                self.retries.fetch_add(1, Ordering::Relaxed);
                self.last_attempt.fetch_add(1, Ordering::Relaxed);
            }
            DeliveryOutcome::DeadLettered { .. } => {
                self.dead_lettered.fetch_add(1, Ordering::Relaxed);
                self.last_attempt.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn snapshot(&self) -> DeliveryMetricsSnapshot {
        DeliveryMetricsSnapshot {
            idle: self.idle.load(Ordering::Relaxed),
            delivered: self.delivered.load(Ordering::Relaxed),
            retries: self.retries.load(Ordering::Relaxed),
            dead_lettered: self.dead_lettered.load(Ordering::Relaxed),
            total_attempts: self.last_attempt.load(Ordering::Relaxed),
        }
    }

    pub fn health(&self) -> DeliveryHealthSnapshot {
        let snapshot = self.snapshot();
        let status = if snapshot.dead_lettered > 0 { DeliveryHealth::Unhealthy }
        else if snapshot.retries > 0 { DeliveryHealth::Degraded }
        else { DeliveryHealth::Healthy };
        DeliveryHealthSnapshot { status, successful: snapshot.delivered, retries: snapshot.retries, dead_lettered: snapshot.dead_lettered, total_attempts: snapshot.total_attempts }
    }

    pub fn reset(&self) {
        self.idle.store(0, Ordering::Relaxed);
        self.delivered.store(0, Ordering::Relaxed);
        self.retries.store(0, Ordering::Relaxed);
        self.dead_lettered.store(0, Ordering::Relaxed);
        self.last_attempt.store(0, Ordering::Relaxed);
    }
}

pub struct ObservedDispatcher<O, T, D> {
    pub dispatcher: OutboxDispatcher<O, T, D>,
    pub metrics: DeliveryMetrics,
}

impl<O, T, D> ObservedDispatcher<O, T, D>
where O: OutboxStore, T: EventTransport, D: DeadLetterStore,
{
    pub fn new(dispatcher: OutboxDispatcher<O, T, D>) -> Self {
        Self { dispatcher, metrics: DeliveryMetrics::default() }
    }

    pub fn dispatch_one(&mut self, attempt: u32) -> EventBusResult<DeliveryOutcome> {
        let outcome = self.dispatcher.dispatch_one(attempt)?;
        self.metrics.record(outcome);
        Ok(outcome)
    }

    pub fn metrics(&self) -> &DeliveryMetrics { &self.metrics }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InMemoryDeadLetterStore, InMemoryOutbox, RecordingTransport, RetryPolicy};

    #[test]
    fn metrics_classify_delivery_health() {
        let metrics = DeliveryMetrics::default();
        assert_eq!(metrics.health().status, DeliveryHealth::Healthy);
        metrics.record(DeliveryOutcome::RetryScheduled { attempt: 1, delay: std::time::Duration::from_millis(10) });
        assert_eq!(metrics.health().status, DeliveryHealth::Degraded);
        metrics.record(DeliveryOutcome::DeadLettered { attempt: 2 });
        assert_eq!(metrics.health().status, DeliveryHealth::Unhealthy);
    }

    #[test]
    fn observed_dispatcher_records_idle_ticks() {
        let dispatcher = OutboxDispatcher { outbox: InMemoryOutbox::default(), transport: RecordingTransport::default(), dead_letters: InMemoryDeadLetterStore::default(), retry_policy: RetryPolicy::default() };
        let mut observed = ObservedDispatcher::new(dispatcher);
        assert_eq!(observed.dispatch_one(0).unwrap(), DeliveryOutcome::Idle);
        assert_eq!(observed.metrics().snapshot().idle, 1);
    }

    #[test]
    fn reset_returns_metrics_to_clean_state() {
        let metrics = DeliveryMetrics::default();
        metrics.record(DeliveryOutcome::Delivered);
        metrics.record(DeliveryOutcome::RetryScheduled { attempt: 1, delay: std::time::Duration::from_millis(1) });
        metrics.reset();
        assert_eq!(metrics.snapshot(), DeliveryMetricsSnapshot::default());
        assert_eq!(metrics.health().status, DeliveryHealth::Healthy);
    }
}

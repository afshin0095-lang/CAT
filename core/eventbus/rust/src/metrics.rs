use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct EventBusMetrics {
    published: AtomicU64,
    delivered: AtomicU64,
    acknowledged: AtomicU64,
    retried: AtomicU64,
    dead_lettered: AtomicU64,
    rejected: AtomicU64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EventBusMetricsSnapshot {
    pub published: u64,
    pub delivered: u64,
    pub acknowledged: u64,
    pub retried: u64,
    pub dead_lettered: u64,
    pub rejected: u64,
}

impl EventBusMetrics {
    pub fn record_published(&self) {
        self.published.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_delivered(&self) {
        self.delivered.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_acknowledged(&self) {
        self.acknowledged.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_retried(&self) {
        self.retried.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_dead_lettered(&self) {
        self.dead_lettered.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_rejected(&self) {
        self.rejected.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> EventBusMetricsSnapshot {
        EventBusMetricsSnapshot {
            published: self.published.load(Ordering::Relaxed),
            delivered: self.delivered.load(Ordering::Relaxed),
            acknowledged: self.acknowledged.load(Ordering::Relaxed),
            retried: self.retried.load(Ordering::Relaxed),
            dead_lettered: self.dead_lettered.load(Ordering::Relaxed),
            rejected: self.rejected.load(Ordering::Relaxed),
        }
    }

    pub fn reset(&self) -> EventBusMetricsSnapshot {
        EventBusMetricsSnapshot {
            published: self.published.swap(0, Ordering::Relaxed),
            delivered: self.delivered.swap(0, Ordering::Relaxed),
            acknowledged: self.acknowledged.swap(0, Ordering::Relaxed),
            retried: self.retried.swap(0, Ordering::Relaxed),
            dead_lettered: self.dead_lettered.swap(0, Ordering::Relaxed),
            rejected: self.rejected.swap(0, Ordering::Relaxed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_and_reset_preserve_counter_semantics() {
        let metrics = EventBusMetrics::default();
        metrics.record_published();
        metrics.record_delivered();
        metrics.record_retried();
        metrics.record_rejected();

        assert_eq!(
            metrics.snapshot(),
            EventBusMetricsSnapshot {
                published: 1,
                delivered: 1,
                acknowledged: 0,
                retried: 1,
                dead_lettered: 0,
                rejected: 1,
            }
        );

        let previous = metrics.reset();
        assert_eq!(previous.published, 1);
        assert_eq!(metrics.snapshot(), EventBusMetricsSnapshot::default());
    }
}

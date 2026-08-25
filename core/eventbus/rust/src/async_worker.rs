use crate::{AsyncEventHandler, AsyncInboxStore, AsyncNatsJetStreamConsumer, DeadLetterStore, EventBusResult, InMemoryDeadLetterStore, NatsConsumerConfig};
use async_nats::jetstream::Context;
use std::time::Duration;

/// Runtime policy for the asynchronous JetStream delivery worker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AsyncDeliveryWorkerConfig {
    /// Maximum number of consumer batches processed by one bounded run.
    pub max_batches_per_run: usize,
    /// Delay between empty polls when the caller chooses a polling loop.
    pub idle_poll_delay: Duration,
}

impl Default for AsyncDeliveryWorkerConfig {
    fn default() -> Self {
        Self {
            max_batches_per_run: 1024,
            idle_poll_delay: Duration::from_millis(250),
        }
    }
}

/// Async worker facade that keeps broker, inbox, retry, and DLQ semantics inside
/// the EventBus adapter while leaving application handlers transport-independent.
pub struct AsyncDeliveryWorker<I, D = InMemoryDeadLetterStore> {
    pub consumer: AsyncNatsJetStreamConsumer<I, D>,
    pub config: AsyncDeliveryWorkerConfig,
}

impl<I> AsyncDeliveryWorker<I, InMemoryDeadLetterStore>
where
    I: AsyncInboxStore,
{
    pub fn new(
        context: Context,
        stream_name: impl Into<String>,
        consumer_config: NatsConsumerConfig,
        inbox: I,
    ) -> Self {
        Self::with_dead_letters(
            context,
            stream_name,
            consumer_config,
            inbox,
            InMemoryDeadLetterStore::default(),
        )
    }
}

impl<I, D> AsyncDeliveryWorker<I, D>
where
    I: AsyncInboxStore,
    D: DeadLetterStore,
{
    pub fn with_dead_letters(
        context: Context,
        stream_name: impl Into<String>,
        consumer_config: NatsConsumerConfig,
        inbox: I,
        dead_letters: D,
    ) -> Self {
        Self {
            consumer: AsyncNatsJetStreamConsumer::with_dead_letters(
                context,
                stream_name,
                consumer_config,
                inbox,
                dead_letters,
            ),
            config: AsyncDeliveryWorkerConfig::default(),
        }
    }

    pub fn config_mut(&mut self) -> &mut AsyncDeliveryWorkerConfig {
        &mut self.config
    }

    pub async fn tick<H>(&mut self, handler: &H) -> EventBusResult<usize>
    where
        H: AsyncEventHandler,
    {
        self.consumer.process_batch(handler).await
    }

    /// Drain a bounded number of batches. A zero-message batch terminates the run.
    pub async fn run_until_idle<H>(&mut self, handler: &H) -> EventBusResult<usize>
    where
        H: AsyncEventHandler,
    {
        let mut processed = 0usize;
        for _ in 0..self.config.max_batches_per_run {
            let batch = self.tick(handler).await?;
            processed += batch;
            if batch == 0 {
                break;
            }
        }
        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_worker_policy_is_bounded() {
        let config = AsyncDeliveryWorkerConfig::default();
        assert_eq!(config.max_batches_per_run, 1024);
        assert_eq!(config.idle_poll_delay, Duration::from_millis(250));
    }
}

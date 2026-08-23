use crate::{
    consumer::DeliveryState,
    AsyncEventHandler,
    DeadLetterStore,
    EventBusError,
    EventBusResult,
    EventEnvelope,
    InboxStore,
};
use async_nats::jetstream::{consumer, AckKind, Context};
use async_trait::async_trait;
use futures_util::StreamExt;
use std::time::Duration;

/// Application handler invoked only after inbox admission succeeds.
#[async_trait]
pub trait AsyncEventHandler: Send + Sync {
    async fn handle(&self, event: EventEnvelope) -> EventBusResult<()>;
}

/// Retry and dead-letter policy applied at the CAT consumer boundary.
#[derive(Clone, Debug)]
pub struct NatsRetryPolicy {
    /// Delays indexed by delivery attempt. The last delay is reused for later attempts.
    pub backoff: Vec<Duration>,
    /// If true, an exhausted event is parked in the CAT DLQ before JetStream TERM.
    pub dead_letter_on_exhaustion: bool,
}

impl Default for NatsRetryPolicy {
    fn default() -> Self {
        Self {
            backoff: vec![
                Duration::from_secs(1),
                Duration::from_secs(5),
                Duration::from_secs(15),
                Duration::from_secs(30),
            ],
            dead_letter_on_exhaustion: true,
        }
    }
}

impl NatsRetryPolicy {
    pub fn delay_for(&self, delivered: i64) -> Duration {
        let index = delivered.saturating_sub(1) as usize;
        self.backoff
            .get(index)
            .copied()
            .or_else(|| self.backoff.last().copied())
            .unwrap_or_default()
    }

    pub fn exhausted(&self, delivered: i64, max_deliver: i64) -> bool {
        max_deliver > 0 && delivered >= max_deliver
    }
}

/// Durable pull-consumer configuration for CAT event processing.
#[derive(Clone, Debug)]
pub struct NatsConsumerConfig {
    pub durable_name: String,
    pub filter_subject: String,
    pub ack_wait: Duration,
    pub max_deliver: i64,
    pub max_ack_pending: i64,
    pub batch_size: usize,
    pub retry: NatsRetryPolicy,
}

impl Default for NatsConsumerConfig {
    fn default() -> Self {
        Self {
            durable_name: "cat-eventbus-worker".to_owned(),
            filter_subject: "cat.events.>".to_owned(),
            ack_wait: Duration::from_secs(30),
            max_deliver: 5,
            max_ack_pending: 1024,
            batch_size: 64,
            retry: NatsRetryPolicy::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NatsConsumerStats {
    pub received: u64,
    pub succeeded: u64,
    pub duplicates: u64,
    pub retried: u64,
    pub dead_lettered: u64,
    pub malformed: u64,
    pub handler_failures: u64,
}

/// Pull-based JetStream consumer that couples broker delivery to CAT inbox semantics.
///
/// JetStream remains at-least-once: pull consumers require explicit acknowledgements,
/// and unacknowledged messages are redelivered. CAT therefore records application
/// success in the inbox before acknowledging the broker delivery. citeturn1search3turn1search4
pub struct NatsJetStreamConsumer<I, D = crate::InMemoryDeadLetterStore> {
    context: Context,
    stream_name: String,
    config: NatsConsumerConfig,
    inbox: I,
    dead_letters: D,
    stats: NatsConsumerStats,
}

impl<I> NatsJetStreamConsumer<I> {
    pub fn new(
        context: Context,
        stream_name: impl Into<String>,
        config: NatsConsumerConfig,
        inbox: I,
    ) -> Self {
        Self::with_dead_letters(context, stream_name, config, inbox, D::default())
    }
}

impl<I, D> NatsJetStreamConsumer<I, D>
where
    I: InboxStore,
    D: DeadLetterStore,
{
    pub fn with_dead_letters(
        context: Context,
        stream_name: impl Into<String>,
        config: NatsConsumerConfig,
        inbox: I,
        dead_letters: D,
    ) -> Self {
        Self {
            context,
            stream_name: stream_name.into(),
            config,
            inbox,
            dead_letters,
            stats: NatsConsumerStats::default(),
        }
    }

    pub fn inbox(&self) -> &I {
        &self.inbox
    }

    pub fn inbox_mut(&mut self) -> &mut I {
        &mut self.inbox
    }

    pub fn dead_letters(&self) -> &D {
        &self.dead_letters
    }

    pub fn dead_letters_mut(&mut self) -> &mut D {
        &mut self.dead_letters
    }

    pub fn stats(&self) -> NatsConsumerStats {
        self.stats
    }

    pub async fn ensure_consumer(&self) -> EventBusResult<consumer::Consumer<consumer::pull::Config>> {
        let stream = self
            .context
            .get_stream(&self.stream_name)
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;

        let config = consumer::pull::Config {
            durable_name: Some(self.config.durable_name.clone()),
            filter_subject: self.config.filter_subject.clone(),
            ack_wait: self.config.ack_wait,
            max_deliver: self.config.max_deliver,
            max_ack_pending: self.config.max_ack_pending,
            ..Default::default()
        };

        stream
            .get_or_create_consumer(&self.config.durable_name, config)
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))
    }

    /// Process up to `batch_size` messages and return the number consumed.
    ///
    /// Handler failure follows a strict path:
    /// `Inbox::RetryScheduled -> NAK(delay) -> redelivery`, and after exhaustion:
    /// `DLQ park -> TERM`. A DLQ write failure never TERM-acks the message, so the
    /// broker can redeliver it instead of silently losing it.
    pub async fn process_batch<H>(&mut self, handler: &H) -> EventBusResult<usize>
    where
        H: AsyncEventHandler,
    {
        let consumer = self.ensure_consumer().await?;
        let mut messages = consumer
            .fetch()
            .max_messages(self.config.batch_size)
            .messages()
            .await
            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;

        let mut processed = 0usize;

        while let Some(message) = messages.next().await {
            let message = message
                .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
            self.stats.received += 1;

            let delivered = message
                .info()
                .map(|info| info.delivered)
                .unwrap_or(1);

            let event: EventEnvelope = match serde_json::from_slice(&message.payload) {
                Ok(event) => event,
                Err(error) => {
                    self.stats.malformed += 1;
                    message
                        .ack_with(AckKind::Term)
                        .await
                        .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                    return Err(EventBusError::Serialization(error.to_string()));
                }
            };

            // A successful duplicate is safe to ACK. An in-flight duplicate must not
            // be ACKed, because doing so could suppress the retry of the original worker.
            if !self.inbox.accept(event.event_id)? {
                match self.inbox.state(event.event_id) {
                    Some(DeliveryState::Succeeded) | Some(DeliveryState::DeadLettered) | None => {
                        self.stats.duplicates += 1;
                        message
                            .ack()
                            .await
                            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
                        processed += 1;
                    }
                    Some(DeliveryState::InFlight) | Some(DeliveryState::RetryScheduled) => {
                        message
                            .ack_with(AckKind::Nak(Some(self.config.retry.delay_for(delivered))))
                            .await
                            .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
                        self.stats.retried += 1;
                    }
                }
                continue;
            }

            match handler.handle(event.clone()).await {
                Ok(()) => {
                    self.inbox.mark_succeeded(event.event_id)?;
                    // The broker ACK is deliberately last: application success is recorded
                    // before JetStream advances its acknowledgement floor. JetStream's
                    // double_ack waits for broker confirmation. citeturn1search4turn3view0
                    message
                        .double_ack()
                        .await
                        .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
                    self.stats.succeeded += 1;
                    processed += 1;
                }
                Err(error) => {
                    self.stats.handler_failures += 1;
                    self.inbox.mark_failed(event.event_id)?;

                    if self.config.retry.exhausted(delivered, self.config.max_deliver) {
                        if self.config.retry.dead_letter_on_exhaustion {
                            self.dead_letters.park(
                                event.clone(),
                                format!("handler failure after {delivered} delivery attempts: {error}"),
                            )?;
                        }

                        self.inbox.mark_failed(event.event_id)?;
                        message
                            .ack_with(AckKind::Term)
                            .await
                            .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                        self.stats.dead_lettered += 1;
                        processed += 1;
                        continue;
                    }

                    message
                        .ack_with(AckKind::Nak(Some(self.config.retry.delay_for(delivered))))
                        .await
                        .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                    self.stats.retried += 1;
                }
            }
        }

        Ok(processed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_consumer_policy_is_bounded() {
        let config = NatsConsumerConfig::default();
        assert_eq!(config.max_deliver, 5);
        assert_eq!(config.max_ack_pending, 1024);
        assert_eq!(config.batch_size, 64);
        assert_eq!(config.durable_name, "cat-eventbus-worker");
        assert_eq!(config.retry.delay_for(1), Duration::from_secs(1));
        assert_eq!(config.retry.delay_for(4), Duration::from_secs(30));
    }

    #[test]
    fn exhaustion_is_deterministic() {
        let policy = NatsRetryPolicy::default();
        assert!(!policy.exhausted(4, 5));
        assert!(policy.exhausted(5, 5));
        assert!(!policy.exhausted(100, -1));
    }
}

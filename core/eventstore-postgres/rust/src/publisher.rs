use cat_eventbus::{AsyncEventTransport, DeliveryState, EventBusError, EventBusResult, RetryPolicy};
use std::time::Duration;
use tokio::time::sleep;

use crate::PostgresOutbox;

/// Result of one PostgreSQL outbox publication cycle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PublicationOutcome {
    Idle,
    Published { event_id: uuid::Uuid, attempts: u32 },
    RetryScheduled { event_id: uuid::Uuid, attempts: u32, delay: Duration },
    DeadLettered { event_id: uuid::Uuid, attempts: u32 },
}

/// Configuration for the durable outbox publisher.
#[derive(Clone, Debug)]
pub struct OutboxPublisherConfig {
    pub retry_policy: RetryPolicy,
    pub stale_after: Duration,
    pub idle_delay: Duration,
}

impl Default for OutboxPublisherConfig {
    fn default() -> Self {
        Self {
            retry_policy: RetryPolicy::default(),
            stale_after: Duration::from_secs(300),
            idle_delay: Duration::from_millis(250),
        }
    }
}

/// Bridges the transactional PostgreSQL outbox to an asynchronous transport.
///
/// The worker acknowledges the database row only after the transport acknowledges
/// publication. This is deliberately an at-least-once boundary; the event ID is
/// the stable deduplication identity for transports such as JetStream.
pub struct PostgresOutboxPublisher<T> {
    outbox: PostgresOutbox,
    transport: T,
    config: OutboxPublisherConfig,
}

impl<T> PostgresOutboxPublisher<T> {
    pub fn new(outbox: PostgresOutbox, transport: T, config: OutboxPublisherConfig) -> Self {
        Self { outbox, transport, config }
    }

    pub fn outbox(&self) -> &PostgresOutbox { &self.outbox }
    pub fn transport(&self) -> &T { &self.transport }
    pub fn config(&self) -> &OutboxPublisherConfig { &self.config }
}

impl<T> PostgresOutboxPublisher<T>
where
    T: AsyncEventTransport,
{
    /// Publishes at most one ready event.
    pub async fn publish_one(&self) -> EventBusResult<PublicationOutcome> {
        let Some(record) = self.outbox.claim_next().await? else {
            return Ok(PublicationOutcome::Idle);
        };
        let event_id = record.event.event_id;
        let attempts = record.attempts;
        match self.transport.publish(&record.event).await {
            Ok(()) => {
                self.outbox.acknowledge(event_id).await?;
                Ok(PublicationOutcome::Published { event_id, attempts })
            }
            Err(error) => {
                let state = self.outbox.fail(event_id, attempts, &self.config.retry_policy, &error.to_string()).await?;
                match state {
                    DeliveryState::DeadLettered => Ok(PublicationOutcome::DeadLettered { event_id, attempts }),
                    DeliveryState::RetryScheduled => Ok(PublicationOutcome::RetryScheduled {
                        event_id,
                        attempts,
                        delay: self.config.retry_policy.delay_for(attempts),
                    }),
                    other => Err(EventBusError::InvalidConfiguration(format!(
                        "unexpected outbox state after failed publication: {other:?}"
                    ))),
                }
            }
        }
    }

    /// Runs until the caller cancels the task.
    pub async fn run(&self) -> EventBusResult<()> {
        loop {
            self.outbox.requeue_stale(self.config.stale_after).await?;
            match self.publish_one().await? {
                PublicationOutcome::Idle => sleep(self.config.idle_delay).await,
                PublicationOutcome::Published { .. }
                | PublicationOutcome::RetryScheduled { .. }
                | PublicationOutcome::DeadLettered { .. } => {}
            }
        }
    }
}

use crate::{
    AsyncEventHandler, AsyncInboxStore, DeadLetterStore, DeliveryState, EventBusError,
    EventBusResult, EventEnvelope, InMemoryDeadLetterStore, NatsConsumerConfig,
};
use async_nats::jetstream::{AckKind, Context, consumer};
use futures_util::StreamExt;

/// Async JetStream consumer backed by an async durable inbox.
pub struct AsyncNatsJetStreamConsumer<I, D = InMemoryDeadLetterStore> {
    context: Context,
    stream_name: String,
    config: NatsConsumerConfig,
    inbox: I,
    dead_letters: D,
}

impl<I> AsyncNatsJetStreamConsumer<I, InMemoryDeadLetterStore>
where
    I: AsyncInboxStore,
{
    pub fn new(
        context: Context,
        stream_name: impl Into<String>,
        config: NatsConsumerConfig,
        inbox: I,
    ) -> Self {
        Self::with_dead_letters(
            context,
            stream_name,
            config,
            inbox,
            InMemoryDeadLetterStore::default(),
        )
    }
}

impl<I, D> AsyncNatsJetStreamConsumer<I, D>
where
    I: AsyncInboxStore,
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
        }
    }

    pub fn inbox(&self) -> &I {
        &self.inbox
    }
    pub fn dead_letters(&self) -> &D {
        &self.dead_letters
    }

    pub async fn ensure_consumer(
        &self,
    ) -> EventBusResult<consumer::Consumer<consumer::pull::Config>> {
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
            let message =
                message.map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
            let delivered = message.info().map(|info| info.delivered).unwrap_or(1);
            let event: EventEnvelope = match serde_json::from_slice(&message.payload) {
                Ok(event) => event,
                Err(error) => {
                    message
                        .ack_with(AckKind::Term)
                        .await
                        .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                    return Err(EventBusError::Serialization(error.to_string()));
                }
            };

            if !self.inbox.accept(event.event_id).await? {
                match self.inbox.state(event.event_id).await? {
                    Some(DeliveryState::Succeeded) | Some(DeliveryState::DeadLettered) | None => {
                        message.ack().await.map_err(|error| {
                            EventBusError::TransportUnavailable(error.to_string())
                        })?;
                        processed += 1;
                    }
                    Some(DeliveryState::InFlight)
                    | Some(DeliveryState::RetryScheduled)
                    | Some(DeliveryState::Pending) => {
                        message
                            .ack_with(AckKind::Nak(Some(self.config.retry.delay_for(delivered))))
                            .await
                            .map_err(|error| {
                                EventBusError::TransportUnavailable(error.to_string())
                            })?;
                    }
                }
                continue;
            }

            match handler.handle(&event).await {
                Ok(()) => {
                    self.inbox.mark_succeeded(event.event_id).await?;
                    message
                        .double_ack()
                        .await
                        .map_err(|error| EventBusError::TransportUnavailable(error.to_string()))?;
                    processed += 1;
                }
                Err(error) => {
                    self.inbox.mark_failed(event.event_id).await?;
                    if self
                        .config
                        .retry
                        .exhausted(delivered, self.config.max_deliver)
                    {
                        if self.config.retry.dead_letter_on_exhaustion {
                            self.dead_letters.park(
                                event.clone(),
                                format!(
                                    "handler failure after {delivered} delivery attempts: {error}"
                                ),
                            )?;
                        }
                        message
                            .ack_with(AckKind::Term)
                            .await
                            .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                        processed += 1;
                    } else {
                        message
                            .ack_with(AckKind::Nak(Some(self.config.retry.delay_for(delivered))))
                            .await
                            .map_err(|ack| EventBusError::TransportUnavailable(ack.to_string()))?;
                    }
                }
            }
        }
        Ok(processed)
    }
}

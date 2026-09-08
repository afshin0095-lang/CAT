use crate::{AsyncEventHandler, AsyncInboxStore, EventBusError, EventBusResult, EventEnvelope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AsyncInboxDeliveryOutcome { Duplicate, Delivered, RetryableFailure }

pub struct AsyncInboxDeliveryWorker<I, H> {
    pub inbox: I,
    pub handler: H,
}

impl<I, H> AsyncInboxDeliveryWorker<I, H>
where
    I: AsyncInboxStore,
    H: AsyncEventHandler,
{
    pub async fn process(&self, event: &EventEnvelope) -> EventBusResult<AsyncInboxDeliveryOutcome> {
        if !self.inbox.accept(event.event_id).await? {
            return Ok(AsyncInboxDeliveryOutcome::Duplicate);
        }
        match self.handler.handle(event).await {
            Ok(()) => {
                self.inbox.mark_succeeded(event.event_id).await?;
                Ok(AsyncInboxDeliveryOutcome::Delivered)
            }
            Err(error) => {
                self.inbox.mark_failed(event.event_id).await?;
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AsyncInMemoryInbox, EventKind};
    use async_trait::async_trait;

    #[derive(Clone)] struct Succeeds;
    #[async_trait]
    impl AsyncEventHandler for Succeeds {
        async fn handle(&self, _event: &EventEnvelope) -> EventBusResult<()> { Ok(()) }
    }

    #[derive(Clone)] struct Fails;
    #[async_trait]
    impl AsyncEventHandler for Fails {
        async fn handle(&self, _event: &EventEnvelope) -> EventBusResult<()> {
            Err(EventBusError::Storage("handler failed".to_owned()))
        }
    }

    fn event() -> EventEnvelope {
        EventEnvelope {
            event_id: uuid::Uuid::now_v7(), event_type: "cat.event.inbox.v1".to_owned(), version: 1,
            kind: EventKind::Integration, occurred_at_ms: 1, producer: "eventbus-test".to_owned(),
            correlation_id: None, causation_id: None, subject_id: None, payload: serde_json::json!({}),
        }
    }

    #[tokio::test]
    async fn successful_delivery_is_deduplicated_after_completion() {
        let worker = AsyncInboxDeliveryWorker { inbox: AsyncInMemoryInbox::new(), handler: Succeeds };
        let event = event();
        assert_eq!(worker.process(&event).await.unwrap(), AsyncInboxDeliveryOutcome::Delivered);
        assert_eq!(worker.process(&event).await.unwrap(), AsyncInboxDeliveryOutcome::Duplicate);
    }

    #[tokio::test]
    async fn failed_delivery_returns_handler_error_and_can_retry() {
        let inbox = AsyncInMemoryInbox::new();
        let worker = AsyncInboxDeliveryWorker { inbox: inbox.clone(), handler: Fails };
        let event = event();
        assert!(worker.process(&event).await.is_err());
        assert!(inbox.accept(event.event_id).await.unwrap());
    }
}

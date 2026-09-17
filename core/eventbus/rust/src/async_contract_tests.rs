#[cfg(test)]
mod tests {
    use crate::{AsyncInMemoryInbox, AsyncInboxStore, DeliveryState};
    use uuid::Uuid;

    #[test]
    fn async_inbox_preserves_the_same_idempotency_state_machine() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .expect("tokio runtime should build");

        runtime.block_on(async {
            let inbox = AsyncInMemoryInbox::new();
            let event_id = Uuid::now_v7();

            assert!(inbox.accept(event_id).await.unwrap());
            assert!(!inbox.accept(event_id).await.unwrap());
            assert_eq!(
                inbox.state(event_id).await.unwrap(),
                Some(DeliveryState::InFlight)
            );

            inbox.mark_failed(event_id).await.unwrap();
            assert_eq!(
                inbox.state(event_id).await.unwrap(),
                Some(DeliveryState::RetryScheduled)
            );

            inbox.mark_succeeded(event_id).await.unwrap();
            assert_eq!(
                inbox.state(event_id).await.unwrap(),
                Some(DeliveryState::Succeeded)
            );
            assert!(!inbox.accept(event_id).await.unwrap());
        });
    }
}

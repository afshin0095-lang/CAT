use cat_eventbus::{AsyncInboxStore, AsyncInMemoryInbox, DeliveryState};
use uuid::Uuid;

#[tokio::test]
async fn async_inbox_preserves_retry_state_machine() {
    let inbox = AsyncInMemoryInbox::new();
    let event_id = Uuid::now_v7();

    assert!(inbox.accept(event_id).await.unwrap());
    inbox.mark_failed(event_id).await.unwrap();
    assert_eq!(inbox.state(event_id).await.unwrap(), Some(DeliveryState::RetryScheduled));
    assert!(inbox.accept(event_id).await.unwrap());

    inbox.mark_succeeded(event_id).await.unwrap();
    assert_eq!(inbox.state(event_id).await.unwrap(), Some(DeliveryState::Succeeded));
    assert!(!inbox.accept(event_id).await.unwrap());
}

#[tokio::test]
async fn async_inbox_isolated_event_ids_do_not_collide() {
    let inbox = AsyncInMemoryInbox::new();
    let first = Uuid::now_v7();
    let second = Uuid::now_v7();

    assert!(inbox.accept(first).await.unwrap());
    assert!(inbox.accept(second).await.unwrap());
    assert_eq!(inbox.state(first).await.unwrap(), Some(DeliveryState::InFlight));
    assert_eq!(inbox.state(second).await.unwrap(), Some(DeliveryState::InFlight));
}

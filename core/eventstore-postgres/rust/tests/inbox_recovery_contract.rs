use cat_eventstore_postgres::PostgresInbox;
use std::time::Duration;
use uuid::Uuid;

async fn inbox() -> Option<PostgresInbox> {
    let url = std::env::var("DATABASE_URL").ok()?;
    let store = cat_eventstore_postgres::PostgresEventStore::connect(&url).await.ok()?;
    store.ensure_schema().await.ok()?;
    Some(PostgresInbox::new(store.pool().clone(), format!("recovery-contract-{}", Uuid::now_v7())))
}

#[tokio::test]
async fn failed_claim_can_be_reclaimed_but_succeeded_claim_stays_suppressed() {
    let Some(inbox) = inbox().await else { return; };
    let event_id = Uuid::now_v7();

    assert!(inbox.accept(event_id).await.unwrap());
    inbox.fail(event_id, "transient failure").await.unwrap();
    assert!(inbox.accept(event_id).await.unwrap());

    inbox.succeed(event_id).await.unwrap();
    assert!(!inbox.accept(event_id).await.unwrap());
}

#[tokio::test]
async fn stale_recovery_is_consumer_scoped() {
    let Some(inbox) = inbox().await else { return; };
    let event_id = Uuid::now_v7();

    assert!(inbox.accept(event_id).await.unwrap());
    assert_eq!(inbox.requeue_stale(Duration::from_millis(0)).await.unwrap(), 1);
    assert!(inbox.accept(event_id).await.unwrap());
}

use cat_eventbus::{DeliveryState, RetryPolicy};
use cat_eventstore_postgres::{PostgresInbox, PostgresOutbox};
use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

async fn pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").ok()?;
    let pool = PgPool::connect(&url).await.ok()?;
    sqlx::migrate!("./migrations").run(&pool).await.ok()?;
    Some(pool)
}

#[tokio::test]
async fn inbox_isolated_by_consumer_and_idempotent_per_consumer() {
    let Some(pool) = pool().await else {
        return;
    };
    let event_id = Uuid::now_v7();
    let first = PostgresInbox::new(pool.clone(), format!("contract-consumer-a-{event_id}"));
    let second = PostgresInbox::new(pool.clone(), format!("contract-consumer-b-{event_id}"));

    assert!(first.accept(event_id).await.unwrap());
    assert!(!first.accept(event_id).await.unwrap());
    assert!(second.accept(event_id).await.unwrap());

    first.succeed(event_id).await.unwrap();
    second.fail(event_id, "transient").await.unwrap();

    assert_eq!(
        first.state(event_id).await.unwrap(),
        Some(DeliveryState::Succeeded)
    );
    assert_eq!(
        second.state(event_id).await.unwrap(),
        Some(DeliveryState::RetryScheduled)
    );
}

#[tokio::test]
async fn outbox_retry_policy_reaches_dead_letter_state() {
    let Some(pool) = pool().await else {
        return;
    };
    let outbox = PostgresOutbox::new(pool);
    let policy = RetryPolicy::new(1, Duration::from_millis(10), Duration::from_millis(100));
    let event_id = Uuid::now_v7();

    assert_eq!(
        outbox
            .fail(event_id, 1, &policy, "permanent")
            .await
            .unwrap(),
        DeliveryState::DeadLettered
    );
}

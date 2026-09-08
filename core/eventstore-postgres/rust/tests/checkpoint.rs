use cat_eventstore_postgres::PostgresProjectionCheckpointStore;
use cat_kernel::{EntityId, EventId, ProjectionCheckpoint, ProjectionPhase, SequenceNumber};
use sqlx::postgres::PgPoolOptions;

fn database_url() -> Option<String> { std::env::var("CAT_TEST_DATABASE_URL").ok() }

fn checkpoint(projection_id: &str, stream_id: EntityId, sequence: u64, event_id: EventId, phase: ProjectionPhase) -> ProjectionCheckpoint {
    ProjectionCheckpoint::new(projection_id, stream_id, SequenceNumber::new(sequence), event_id, phase).expect("checkpoint must be valid")
}

#[tokio::test]
#[ignore = "requires a PostgreSQL instance configured by CAT_TEST_DATABASE_URL"]
async fn checkpoint_store_is_monotonic_and_idempotent() {
    let url = database_url().expect("CAT_TEST_DATABASE_URL must be configured");
    let pool = PgPoolOptions::new().max_connections(2).connect(&url).await.expect("connect to PostgreSQL");
    sqlx::migrate!("./migrations").run(&pool).await.expect("apply CAT event-store migrations");
    let store = PostgresProjectionCheckpointStore::new(pool.clone());
    let stream_id = EntityId::new();
    let stream_key = stream_id.as_uuid();
    let projection_id = format!("checkpoint-contract-{stream_key}");
    let first_event = EventId::new();
    let second_event = EventId::new();
    let conflicting_event = EventId::new();

    let first = checkpoint(&projection_id, stream_id, 10, first_event, ProjectionPhase::Catchup);
    store.save(&first).await.expect("save first checkpoint");
    let replay = checkpoint(&projection_id, stream_id, 10, first_event, ProjectionPhase::Catchup);
    store.save(&replay).await.expect("replay same checkpoint");
    let conflicting = checkpoint(&projection_id, stream_id, 10, conflicting_event, ProjectionPhase::Live);
    store.save(&conflicting).await.expect("equal-sequence conflict must be safely ignored");
    let advanced = checkpoint(&projection_id, stream_id, 11, second_event, ProjectionPhase::Live);
    store.save(&advanced).await.expect("advance checkpoint");
    let stale = checkpoint(&projection_id, stream_id, 9, EventId::new(), ProjectionPhase::Catchup);
    store.save(&stale).await.expect("stale write must be ignored");

    let loaded = store.load(&projection_id, stream_id).await.expect("load checkpoint").expect("checkpoint should exist");
    assert_eq!(loaded.sequence, SequenceNumber::new(11));
    assert_eq!(loaded.event_id, second_event);
    assert_eq!(loaded.phase, ProjectionPhase::Live);

    sqlx::query("DELETE FROM cat_projection_checkpoints WHERE projection_id = $1 AND stream_id = $2").bind(&projection_id).bind(stream_id.as_uuid()).execute(&pool).await.expect("cleanup checkpoint contract row");
}

#[tokio::test]
#[ignore = "requires a PostgreSQL instance configured by CAT_TEST_DATABASE_URL"]
async fn checkpoint_store_supports_independent_projection_stream_pairs() {
    let url = database_url().expect("CAT_TEST_DATABASE_URL must be configured");
    let pool = PgPoolOptions::new().max_connections(2).connect(&url).await.expect("connect to PostgreSQL");
    sqlx::migrate!("./migrations").run(&pool).await.expect("apply CAT event-store migrations");
    let store = PostgresProjectionCheckpointStore::new(pool.clone());
    let stream_a = EntityId::new();
    let stream_b = EntityId::new();
    let projection_a = format!("projection-a-{}", stream_a.as_uuid());
    let projection_b = format!("projection-b-{}", stream_b.as_uuid());

    let checkpoint_a = checkpoint(&projection_a, stream_a, 3, EventId::new(), ProjectionPhase::Live);
    let checkpoint_b = checkpoint(&projection_b, stream_b, 7, EventId::new(), ProjectionPhase::Catchup);
    store.save(&checkpoint_a).await.expect("save projection A");
    store.save(&checkpoint_b).await.expect("save projection B");

    let loaded_a = store.load(&projection_a, stream_a).await.expect("load projection A").expect("projection A exists");
    let loaded_b = store.load(&projection_b, stream_b).await.expect("load projection B").expect("projection B exists");
    assert_eq!(loaded_a.sequence, SequenceNumber::new(3));
    assert_eq!(loaded_b.sequence, SequenceNumber::new(7));
    assert_ne!(loaded_a.stream_id, loaded_b.stream_id);
    assert_ne!(loaded_a.projection_id, loaded_b.projection_id);

    sqlx::query("DELETE FROM cat_projection_checkpoints WHERE (projection_id, stream_id) IN (($1,$2),($3,$4))").bind(&projection_a).bind(stream_a.as_uuid()).bind(&projection_b).bind(stream_b.as_uuid()).execute(&pool).await.expect("cleanup checkpoint isolation rows");
}

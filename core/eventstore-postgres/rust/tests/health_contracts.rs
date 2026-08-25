use cat_eventstore_postgres::EventStoreHealth;

#[test]
fn health_is_not_ready_until_database_and_schema_are_available() {
    let health = EventStoreHealth::not_ready();
    assert!(!health.database_reachable);
    assert!(!health.schema_ready);
    assert!(!health.is_ready());
}

#[test]
fn ready_health_requires_both_dependencies() {
    let ready = EventStoreHealth::ready();
    assert!(ready.database_reachable);
    assert!(ready.schema_ready);
    assert!(ready.is_ready());
}

#[test]
fn readiness_is_strictly_conjunctive() {
    let database_only = EventStoreHealth {
        database_reachable: true,
        schema_ready: false,
    };
    let schema_only = EventStoreHealth {
        database_reachable: false,
        schema_ready: true,
    };

    assert!(!database_only.is_ready());
    assert!(!schema_only.is_ready());
}

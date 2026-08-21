use cat_eventstore_postgres::PostgresEventStore;

#[test]
fn adapter_type_is_constructible_at_compile_time() {
    let _ = std::any::type_name::<PostgresEventStore>();
}

use cat_eventbus::NatsStreamConfig;

#[test]
fn stream_config_is_constructible_without_network() {
    let config = NatsStreamConfig::default();
    assert!(config.validate().is_ok());
    assert_eq!(config.name, "CAT_EVENTS");
}

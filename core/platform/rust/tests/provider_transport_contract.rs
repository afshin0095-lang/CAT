use cat_platform::{ProviderFailure, ProviderFailureClass, ProviderTelemetry};
use reqwest::StatusCode;

#[test]
fn http_statuses_have_stable_retry_semantics() {
    let cases = [
        (
            StatusCode::BAD_REQUEST,
            ProviderFailureClass::InvalidRequest,
            false,
        ),
        (
            StatusCode::UNAUTHORIZED,
            ProviderFailureClass::Unauthorized,
            false,
        ),
        (
            StatusCode::TOO_MANY_REQUESTS,
            ProviderFailureClass::RateLimited,
            true,
        ),
        (StatusCode::BAD_GATEWAY, ProviderFailureClass::Server, true),
    ];
    for (status, expected, retryable) in cases {
        let failure = ProviderFailure::http(status, "generate", "contract");
        assert_eq!(failure.class, expected);
        assert_eq!(failure.class.retryable(), retryable);
        assert_eq!(failure.status_code, Some(status.as_u16()));
    }
}

#[test]
fn telemetry_is_provider_local_and_deterministic() {
    let mut telemetry = ProviderTelemetry::default();
    telemetry.record(ProviderFailureClass::Timeout);
    telemetry.record(ProviderFailureClass::Server);
    telemetry.record(ProviderFailureClass::Unauthorized);
    telemetry.record(ProviderFailureClass::Server);

    assert_eq!(telemetry.count(ProviderFailureClass::Server), 2);
    assert_eq!(telemetry.retryable_failures(), 3);
    assert_eq!(
        telemetry
            .snapshot()
            .get(&ProviderFailureClass::Unauthorized),
        Some(&1)
    );
}

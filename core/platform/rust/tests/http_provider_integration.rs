use cat_platform::{
    AdapterRequest, ExternalProviderAdapter, HttpJsonProviderAdapter, IntegrationCommand,
    IntegrationContext, IntegrationTarget, PlatformError, PlatformResult, ProviderCircuitConfig,
    ProviderHealth, ProviderHealthProbe, ProviderRetryConfig, ResilientProviderAdapter,
};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct FixtureServer {
    address: String,
    state: Arc<Mutex<FixtureState>>,
    join: Option<thread::JoinHandle<()>>,
}

#[derive(Default)]
struct FixtureState {
    requests: usize,
    fail_remaining: usize,
    slow: bool,
    unhealthy: bool,
}

impl FixtureServer {
    fn start(fail_remaining: usize, slow: bool, unhealthy: bool) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(false).unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let state = Arc::new(Mutex::new(FixtureState {
            fail_remaining,
            slow,
            unhealthy,
            ..Default::default()
        }));
        let shared = Arc::clone(&state);
        let join = thread::spawn(move || {
            for stream in listener.incoming().take(8) {
                if let Ok(mut stream) = stream {
                    handle(&mut stream, &shared);
                }
            }
        });
        Self {
            address,
            state,
            join: Some(join),
        }
    }

    fn requests(&self) -> usize {
        self.state.lock().unwrap().requests
    }
}

impl Drop for FixtureServer {
    fn drop(&mut self) {
        let _ = TcpStream::connect(self.address.trim_start_matches("http://"));
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

fn handle(stream: &mut TcpStream, state: &Arc<Mutex<FixtureState>>) {
    let mut buffer = [0_u8; 4096];
    let size = stream.read(&mut buffer).unwrap_or(0);
    let request = String::from_utf8_lossy(&buffer[..size]);
    let path = request
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("/");
    let (status, body, slow) = {
        let mut state = state.lock().unwrap();
        state.requests += 1;
        if path == "/health" {
            if state.unhealthy {
                (503, json!({"status":"degraded"}), false)
            } else {
                (200, json!({"status":"ready"}), false)
            }
        } else if path == "/fail" {
            if state.fail_remaining > 0 {
                state.fail_remaining -= 1;
                (503, json!({"error":"temporary"}), false)
            } else {
                (200, json!({"ok":true}), false)
            }
        } else if path == "/slow" {
            (200, json!({"ok":true}), state.slow)
        } else if path == "/generate" {
            (200, json!({"ok":true,"provider":"fixture"}), false)
        } else {
            (404, json!({"error":"not_found"}), false)
        }
    };
    if slow {
        thread::sleep(Duration::from_millis(150));
    }
    let payload = serde_json::to_string(&body).unwrap();
    let reason = match status {
        200 => "OK",
        503 => "Service Unavailable",
        404 => "Not Found",
        _ => "Error",
    };
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{payload}",
        payload.len()
    );
    let _ = stream.write_all(response.as_bytes());
}

fn request(operation: &str) -> AdapterRequest {
    AdapterRequest::from_command(
        IntegrationCommand::new(
            IntegrationTarget::Llm,
            operation,
            IntegrationContext::new("platform-http-fixture"),
        ),
        json!({"prompt":"fixture"}),
    )
}

#[test]
fn http_provider_success_and_health_are_verified_against_local_fixture() {
    let server = FixtureServer::start(0, false, false);
    let adapter = HttpJsonProviderAdapter::new(
        "fixture",
        IntegrationTarget::Llm,
        &server.address,
        ["generate"],
        "/health",
        Duration::from_secs(1),
    )
    .unwrap();
    assert_eq!(
        ProviderHealthProbe::probe_health(&adapter).unwrap(),
        ProviderHealth::Ready
    );
    let response = adapter.execute(&request("generate")).unwrap();
    assert!(response.accepted);
    assert_eq!(response.payload["provider"], "fixture");
    assert_eq!(server.requests(), 2);
}

#[test]
fn http_provider_classifies_non_2xx_as_transport_failure() {
    let server = FixtureServer::start(0, false, false);
    let adapter = HttpJsonProviderAdapter::new(
        "fixture",
        IntegrationTarget::Llm,
        &server.address,
        ["fail"],
        "/health",
        Duration::from_secs(1),
    )
    .unwrap();
    let error = adapter.execute(&request("fail")).unwrap_err();
    assert!(
        matches!(error, PlatformError::TransportUnavailable(message) if message.contains("503"))
    );
}

#[test]
fn http_provider_timeout_is_bounded() {
    let server = FixtureServer::start(0, true, false);
    let adapter = HttpJsonProviderAdapter::new(
        "fixture",
        IntegrationTarget::Llm,
        &server.address,
        ["slow"],
        "/health",
        Duration::from_millis(25),
    )
    .unwrap();
    let started = std::time::Instant::now();
    let error = adapter.execute(&request("slow")).unwrap_err();
    assert!(matches!(error, PlatformError::TransportUnavailable(_)));
    assert!(started.elapsed() < Duration::from_millis(120));
}

#[test]
fn http_provider_health_degrades_without_external_credentials() {
    let server = FixtureServer::start(0, false, true);
    let adapter = HttpJsonProviderAdapter::new(
        "fixture",
        IntegrationTarget::Llm,
        &server.address,
        ["generate"],
        "/health",
        Duration::from_secs(1),
    )
    .unwrap();
    assert_eq!(
        ProviderHealthProbe::probe_health(&adapter).unwrap(),
        ProviderHealth::Degraded
    );
}

#[test]
fn resilient_provider_retries_and_recovers_after_fixture_failure_window() {
    let server = FixtureServer::start(2, false, false);
    let adapter = HttpJsonProviderAdapter::new(
        "fixture",
        IntegrationTarget::Llm,
        &server.address,
        ["fail"],
        "/health",
        Duration::from_secs(1),
    )
    .unwrap();
    let resilient = ResilientProviderAdapter::new(
        Arc::new(adapter),
        ProviderRetryConfig {
            max_attempts: 3,
            retry_delay: Duration::ZERO,
            max_retry_delay: Duration::ZERO,
        },
        ProviderCircuitConfig {
            failure_threshold: 5,
            recovery_after: Duration::ZERO,
        },
    )
    .unwrap();
    let response = resilient.execute(&request("fail")).unwrap();
    assert!(response.accepted);
    assert_eq!(
        resilient.circuit_state(),
        cat_platform::ProviderCircuitState::Closed
    );
    assert!(server.requests() >= 3);
}

#[test]
fn resilient_provider_opens_circuit_after_repeated_fixture_failures() {
    let server = FixtureServer::start(8, false, false);
    let adapter = HttpJsonProviderAdapter::new(
        "fixture",
        IntegrationTarget::Llm,
        &server.address,
        ["fail"],
        "/health",
        Duration::from_secs(1),
    )
    .unwrap();
    let resilient = ResilientProviderAdapter::new(
        Arc::new(adapter),
        ProviderRetryConfig {
            max_attempts: 3,
            retry_delay: Duration::ZERO,
            max_retry_delay: Duration::ZERO,
        },
        ProviderCircuitConfig {
            failure_threshold: 2,
            recovery_after: Duration::from_secs(60),
        },
    )
    .unwrap();
    assert!(resilient.execute(&request("fail")).is_err());
    assert_eq!(
        resilient.circuit_state(),
        cat_platform::ProviderCircuitState::Open
    );
    assert!(resilient.execute(&request("fail")).is_err());
    assert!(server.requests() <= 2);
}

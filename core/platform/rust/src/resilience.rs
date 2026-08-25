use crate::{AdapterRequest, AdapterResponse, ExternalProviderAdapter, PlatformError, PlatformResult, ProviderCapabilities};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderCircuitState {
    Closed,
    Open,
    HalfOpen,
}

#[derive(Clone, Debug)]
pub struct ProviderCircuitConfig {
    pub failure_threshold: u32,
    pub recovery_after: Duration,
}

impl Default for ProviderCircuitConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 3,
            recovery_after: Duration::from_secs(30),
        }
    }
}

#[derive(Debug)]
struct CircuitState {
    state: ProviderCircuitState,
    consecutive_failures: u32,
    opened_at: Option<Instant>,
}

/// Provider-neutral circuit breaker for external infrastructure adapters.
///
/// The breaker is deliberately outside the domain cores: it protects the transport/provider
/// boundary without changing canonical domain semantics. A successful half-open probe closes
/// the circuit; a failed probe reopens it and increments the failure count.
pub struct ProviderCircuitBreaker {
    adapter: Arc<dyn ExternalProviderAdapter>,
    config: ProviderCircuitConfig,
    state: Mutex<CircuitState>,
}

impl ProviderCircuitBreaker {
    pub fn new(
        adapter: Arc<dyn ExternalProviderAdapter>,
        config: ProviderCircuitConfig,
    ) -> PlatformResult<Self> {
        if config.failure_threshold == 0 {
            return Err(PlatformError::InvalidCommand(
                "provider circuit failure threshold must be greater than zero".into(),
            ));
        }

        Ok(Self {
            adapter,
            config,
            state: Mutex::new(CircuitState {
                state: ProviderCircuitState::Closed,
                consecutive_failures: 0,
                opened_at: None,
            }),
        })
    }

    pub fn capabilities(&self) -> ProviderCapabilities {
        self.adapter.capabilities()
    }

    pub fn state(&self) -> ProviderCircuitState {
        self.state.lock().expect("provider circuit mutex poisoned").state
    }

    pub fn consecutive_failures(&self) -> u32 {
        self.state
            .lock()
            .expect("provider circuit mutex poisoned")
            .consecutive_failures
    }

    pub fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        {
            let mut state = self.state.lock().expect("provider circuit mutex poisoned");
            match state.state {
                ProviderCircuitState::Closed => {}
                ProviderCircuitState::Open => {
                    let ready = state
                        .opened_at
                        .map(|opened| opened.elapsed() >= self.config.recovery_after)
                        .unwrap_or(false);
                    if !ready {
                        return Err(PlatformError::TransportUnavailable(format!(
                            "provider {} circuit is open",
                            self.capabilities().provider_id.as_str()
                        )));
                    }
                    state.state = ProviderCircuitState::HalfOpen;
                }
                ProviderCircuitState::HalfOpen => {
                    return Err(PlatformError::TransportUnavailable(format!(
                        "provider {} circuit probe already in progress",
                        self.capabilities().provider_id.as_str()
                    )));
                }
            }
        }

        match self.adapter.execute(request) {
            Ok(response) => {
                let mut state = self.state.lock().expect("provider circuit mutex poisoned");
                state.state = ProviderCircuitState::Closed;
                state.consecutive_failures = 0;
                state.opened_at = None;
                Ok(response)
            }
            Err(error) => {
                let mut state = self.state.lock().expect("provider circuit mutex poisoned");
                state.consecutive_failures = state.consecutive_failures.saturating_add(1);
                if state.consecutive_failures >= self.config.failure_threshold {
                    state.state = ProviderCircuitState::Open;
                    state.opened_at = Some(Instant::now());
                } else {
                    state.state = ProviderCircuitState::Closed;
                }
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IntegrationCommand, IntegrationContext, IntegrationTarget, ProviderId, ProviderHealth};
    use serde_json::json;

    #[derive(Clone)]
    struct FailingAdapter {
        capabilities: ProviderCapabilities,
    }

    impl FailingAdapter {
        fn new() -> Self {
            Self {
                capabilities: ProviderCapabilities {
                    provider_id: ProviderId::new("failing").unwrap(),
                    target: IntegrationTarget::Llm,
                    operations: vec!["generate".into()],
                    health: ProviderHealth::Ready,
                },
            }
        }
    }

    impl ExternalProviderAdapter for FailingAdapter {
        fn capabilities(&self) -> ProviderCapabilities { self.capabilities.clone() }

        fn execute(&self, _request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
            Err(PlatformError::TransportUnavailable("synthetic failure".into()))
        }
    }

    fn request() -> AdapterRequest {
        AdapterRequest::from_command(
            IntegrationCommand::new(
                IntegrationTarget::Llm,
                "generate",
                IntegrationContext::new("circuit-test"),
            ),
            json!({"prompt":"ping"}),
        )
    }

    #[test]
    fn threshold_opens_circuit_and_blocks_follow_up_calls() {
        let breaker = ProviderCircuitBreaker::new(
            Arc::new(FailingAdapter::new()),
            ProviderCircuitConfig {
                failure_threshold: 1,
                recovery_after: Duration::from_secs(60),
            },
        )
        .unwrap();

        assert!(breaker.execute(&request()).is_err());
        assert_eq!(breaker.state(), ProviderCircuitState::Open);
        assert_eq!(breaker.consecutive_failures(), 1);
        assert!(matches!(breaker.execute(&request()), Err(PlatformError::TransportUnavailable(_))));
    }

    #[test]
    fn zero_recovery_window_allows_a_half_open_probe() {
        let breaker = ProviderCircuitBreaker::new(
            Arc::new(FailingAdapter::new()),
            ProviderCircuitConfig {
                failure_threshold: 1,
                recovery_after: Duration::ZERO,
            },
        )
        .unwrap();

        assert!(breaker.execute(&request()).is_err());
        assert_eq!(breaker.state(), ProviderCircuitState::Open);
        assert!(breaker.execute(&request()).is_err());
        assert_eq!(breaker.state(), ProviderCircuitState::Open);
        assert_eq!(breaker.consecutive_failures(), 2);
    }
}

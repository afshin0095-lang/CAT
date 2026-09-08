use crate::{AdapterRequest, AdapterResponse, ExternalProviderAdapter, PlatformError, PlatformResult, ProviderCapabilities, ProviderCircuitBreaker, ProviderCircuitConfig, ProviderHealth, ProviderHealthProbe};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderRetryConfig { pub max_attempts: u32, pub retry_delay: Duration, pub max_retry_delay: Duration }
impl Default for ProviderRetryConfig { fn default() -> Self { Self { max_attempts: 3, retry_delay: Duration::from_millis(50), max_retry_delay: Duration::from_millis(500) } } }
impl ProviderRetryConfig {
    pub fn validate(self) -> PlatformResult<Self> { if self.max_attempts == 0 { return Err(PlatformError::InvalidCommand("provider retry max_attempts must be greater than zero".into())); } if self.max_retry_delay < self.retry_delay { return Err(PlatformError::InvalidCommand("provider retry max_retry_delay must be >= retry_delay".into())); } Ok(self) }
    pub fn delay_for(self, attempt: u32) -> Duration { if attempt == 0 || self.retry_delay.is_zero() { return Duration::ZERO; } let multiplier = 2_u32.saturating_pow(attempt.saturating_sub(1)); self.retry_delay.checked_mul(multiplier).unwrap_or(self.max_retry_delay).min(self.max_retry_delay) }
}

pub struct ResilientProviderAdapter { circuit: ProviderCircuitBreaker, retry: ProviderRetryConfig }
impl ResilientProviderAdapter {
    pub fn new(adapter: Arc<dyn ExternalProviderAdapter>, retry: ProviderRetryConfig, circuit: ProviderCircuitConfig) -> PlatformResult<Self> { Ok(Self { circuit: ProviderCircuitBreaker::new(adapter, circuit)?, retry: retry.validate()? }) }
    pub fn capabilities(&self) -> ProviderCapabilities { let mut capabilities = self.circuit.capabilities(); capabilities.health = match self.circuit.state() { crate::ProviderCircuitState::Closed => ProviderHealth::Ready, crate::ProviderCircuitState::HalfOpen => ProviderHealth::Degraded, crate::ProviderCircuitState::Open => ProviderHealth::Unavailable }; capabilities }
    pub fn circuit_state(&self) -> crate::ProviderCircuitState { self.circuit.state() }
    pub fn consecutive_failures(&self) -> u32 { self.circuit.consecutive_failures() }
    pub fn retry_config(&self) -> ProviderRetryConfig { self.retry }
    pub fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> { let mut last_error = None; for attempt in 1..=self.retry.max_attempts { match self.circuit.execute(request) { Ok(response) => return Ok(response), Err(PlatformError::TransportUnavailable(message)) if message.contains("circuit is open") || message.contains("probe already in progress") => return Err(PlatformError::TransportUnavailable(message)), Err(error) => { last_error = Some(error); if attempt < self.retry.max_attempts { let delay = self.retry.delay_for(attempt); if !delay.is_zero() { thread::sleep(delay); } } } } } Err(last_error.unwrap_or_else(|| PlatformError::TransportUnavailable("provider execution failed without an error".into()))) }
}
impl ProviderHealthProbe for ResilientProviderAdapter { fn probe_health(&self) -> PlatformResult<ProviderHealth> { Ok(self.capabilities().health) } }
impl ExternalProviderAdapter for ResilientProviderAdapter { fn capabilities(&self) -> ProviderCapabilities { ResilientProviderAdapter::capabilities(self) } fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> { ResilientProviderAdapter::execute(self, request) } fn probe_health(&self) -> PlatformResult<ProviderHealth> { ProviderHealthProbe::probe_health(self) } }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AdapterRequest, DeterministicProviderAdapter, IntegrationCommand, IntegrationContext, IntegrationTarget, PlatformError, ProviderHealth, ProviderId};
    use serde_json::json;
    use std::sync::atomic::{AtomicU32, Ordering};
    struct FlakyAdapter { capabilities: ProviderCapabilities, failures_before_success: AtomicU32 }
    impl FlakyAdapter { fn new(failures_before_success: u32) -> Self { Self { capabilities: ProviderCapabilities { provider_id: ProviderId::new("flaky").unwrap(), target: IntegrationTarget::Llm, operations: vec!["generate".into()], health: ProviderHealth::Ready }, failures_before_success: AtomicU32::new(failures_before_success) } } }
    impl ExternalProviderAdapter for FlakyAdapter { fn capabilities(&self) -> ProviderCapabilities { self.capabilities.clone() } fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> { if self.failures_before_success.load(Ordering::SeqCst) > 0 { self.failures_before_success.fetch_sub(1, Ordering::SeqCst); return Err(PlatformError::TransportUnavailable("temporary provider failure".into())); } Ok(AdapterResponse { request_id: request.context.request_id, target: request.target, operation: request.operation.clone(), accepted: true, payload: json!({"provider":"flaky","attempt":"success"}) }) } }
    fn request() -> AdapterRequest { AdapterRequest::from_command(IntegrationCommand::new(IntegrationTarget::Llm, "generate", IntegrationContext::new("resilience-test")), json!({"prompt":"ping"})) }
    #[test] fn retry_recovers_from_transient_provider_failure() { let resilient = ResilientProviderAdapter::new(Arc::new(FlakyAdapter::new(2)), ProviderRetryConfig { max_attempts: 3, retry_delay: Duration::ZERO, max_retry_delay: Duration::ZERO }, ProviderCircuitConfig { failure_threshold: 5, recovery_after: Duration::ZERO }).unwrap(); assert!(resilient.execute(&request()).unwrap().accepted); assert_eq!(resilient.circuit_state(), crate::ProviderCircuitState::Closed); assert_eq!(ProviderHealthProbe::probe_health(&resilient).unwrap(), ProviderHealth::Ready); }
    #[test] fn retry_budget_is_bounded() { let resilient = ResilientProviderAdapter::new(Arc::new(FlakyAdapter::new(10)), ProviderRetryConfig { max_attempts: 3, retry_delay: Duration::ZERO, max_retry_delay: Duration::ZERO }, ProviderCircuitConfig { failure_threshold: 10, recovery_after: Duration::ZERO }).unwrap(); assert!(matches!(resilient.execute(&request()), Err(PlatformError::TransportUnavailable(_)))); }
    #[test] fn open_circuit_short_circuits_nested_retry_loop() { let resilient = ResilientProviderAdapter::new(Arc::new(FlakyAdapter::new(10)), ProviderRetryConfig { max_attempts: 5, retry_delay: Duration::ZERO, max_retry_delay: Duration::ZERO }, ProviderCircuitConfig { failure_threshold: 1, recovery_after: Duration::from_secs(60) }).unwrap(); assert!(resilient.execute(&request()).is_err()); assert_eq!(resilient.circuit_state(), crate::ProviderCircuitState::Open); assert_eq!(ProviderHealthProbe::probe_health(&resilient).unwrap(), ProviderHealth::Unavailable); assert!(resilient.execute(&request()).is_err()); assert_eq!(resilient.consecutive_failures(), 1); }
    #[test] fn invalid_retry_configuration_is_rejected() { let adapter = Arc::new(DeterministicProviderAdapter::new("local", IntegrationTarget::Llm, ["generate"]).unwrap()); assert!(ResilientProviderAdapter::new(adapter, ProviderRetryConfig { max_attempts: 0, retry_delay: Duration::ZERO, max_retry_delay: Duration::ZERO }, ProviderCircuitConfig::default()).is_err()); }
    #[test] fn retry_delay_grows_exponentially_and_is_capped() { let config = ProviderRetryConfig { max_attempts: 8, retry_delay: Duration::from_millis(50), max_retry_delay: Duration::from_millis(200) }; assert_eq!(config.delay_for(1), Duration::from_millis(50)); assert_eq!(config.delay_for(2), Duration::from_millis(100)); assert_eq!(config.delay_for(3), Duration::from_millis(200)); assert_eq!(config.delay_for(7), Duration::from_millis(200)); assert_eq!(config.delay_for(0), Duration::ZERO); }
    #[test] fn invalid_retry_delay_bounds_are_rejected() { let adapter = Arc::new(DeterministicProviderAdapter::new("local", IntegrationTarget::Llm, ["generate"]).unwrap()); let result = ResilientProviderAdapter::new(adapter, ProviderRetryConfig { max_attempts: 3, retry_delay: Duration::from_millis(200), max_retry_delay: Duration::from_millis(100) }, ProviderCircuitConfig::default()); assert!(matches!(result, Err(PlatformError::InvalidCommand(_)))); }
}

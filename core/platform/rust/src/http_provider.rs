use crate::{
    AdapterRequest, AdapterResponse, ExternalProviderAdapter, IntegrationTarget, PlatformError,
    PlatformResult, ProviderCapabilities, ProviderFailure, ProviderHealth, ProviderHealthProbe,
    ProviderId,
};
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue};
use serde_json::{Value, json};
use std::time::Duration;

/// Concrete JSON-over-HTTP provider adapter. Provider failures remain classified at the transport boundary;
/// CAT domain truth, decisions, and monetary semantics never depend on provider-specific error text.
pub struct HttpJsonProviderAdapter {
    client: Client,
    capabilities: ProviderCapabilities,
    base_url: String,
    health_path: String,
    headers: HeaderMap,
}
impl HttpJsonProviderAdapter {
    pub fn new(
        provider_id: impl Into<String>,
        target: IntegrationTarget,
        base_url: impl Into<String>,
        operations: impl IntoIterator<Item = impl Into<String>>,
        health_path: impl Into<String>,
        timeout: Duration,
    ) -> PlatformResult<Self> {
        let base_url = base_url.into().trim_end_matches('/').to_owned();
        if !(base_url.starts_with("https://") || base_url.starts_with("http://"))
            || base_url.contains(' ')
        {
            return Err(PlatformError::InvalidCommand(
                "provider base_url must use http(s) and contain no whitespace".into(),
            ));
        }
        if timeout.is_zero() {
            return Err(PlatformError::InvalidCommand(
                "provider HTTP timeout must be greater than zero".into(),
            ));
        }
        let operations: Vec<String> = operations.into_iter().map(Into::into).collect();
        if operations.is_empty() {
            return Err(PlatformError::InvalidCommand(
                "HTTP provider must declare at least one operation".into(),
            ));
        }
        let health_path = health_path.into();
        if !health_path.starts_with('/') {
            return Err(PlatformError::InvalidCommand(
                "provider health_path must start with '/'".into(),
            ));
        }
        let client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|error| {
                PlatformError::TransportUnavailable(format!(
                    "provider HTTP client build failed: {error}"
                ))
            })?;
        Ok(Self {
            client,
            capabilities: ProviderCapabilities {
                provider_id: ProviderId::new(provider_id)?,
                target,
                operations,
                health: ProviderHealth::Unknown,
            },
            base_url,
            health_path,
            headers: HeaderMap::new(),
        })
    }
    pub fn with_header(
        mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> PlatformResult<Self> {
        let name = HeaderName::from_bytes(name.as_ref().as_bytes()).map_err(|error| {
            PlatformError::InvalidCommand(format!("invalid provider header name: {error}"))
        })?;
        let value = HeaderValue::from_str(value.as_ref()).map_err(|error| {
            PlatformError::InvalidCommand(format!("invalid provider header value: {error}"))
        })?;
        self.headers.insert(name, value);
        Ok(self)
    }
    fn operation_url(&self, operation: &str) -> String {
        format!("{}/{}", self.base_url, operation.trim_start_matches('/'))
    }
    fn health_url(&self) -> String {
        format!("{}{}", self.base_url, self.health_path)
    }
    fn request_json(&self, operation: &str, payload: &Value) -> PlatformResult<Value> {
        let response = self
            .client
            .post(self.operation_url(operation))
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, "application/json")
            .json(payload)
            .send()
            .map_err(|error| {
                if error.is_timeout() {
                    PlatformError::ProviderFailure(ProviderFailure::timeout(
                        operation,
                        error.to_string(),
                    ))
                } else {
                    PlatformError::ProviderFailure(ProviderFailure::transport(
                        operation,
                        error.to_string(),
                    ))
                }
            })?;
        let status = response.status();
        let body: Value = response.json().map_err(|error| {
            PlatformError::Serialization(format!("provider response JSON decode failed: {error}"))
        })?;
        if !status.is_success() {
            return Err(PlatformError::ProviderFailure(ProviderFailure::http(
                status,
                operation,
                format!("provider returned HTTP {status}: {body}"),
            )));
        }
        Ok(body)
    }
}
impl ProviderHealthProbe for HttpJsonProviderAdapter {
    fn probe_health(&self) -> PlatformResult<ProviderHealth> {
        let response = self
            .client
            .get(self.health_url())
            .headers(self.headers.clone())
            .send()
            .map_err(|error| {
                if error.is_timeout() {
                    PlatformError::ProviderFailure(ProviderFailure::timeout(
                        "health",
                        error.to_string(),
                    ))
                } else {
                    PlatformError::ProviderFailure(ProviderFailure::transport(
                        "health",
                        error.to_string(),
                    ))
                }
            })?;
        Ok(if response.status().is_success() {
            ProviderHealth::Ready
        } else {
            ProviderHealth::Degraded
        })
    }
}
impl ExternalProviderAdapter for HttpJsonProviderAdapter {
    fn capabilities(&self) -> ProviderCapabilities {
        self.capabilities.clone()
    }
    fn execute(&self, request: &AdapterRequest) -> PlatformResult<AdapterResponse> {
        if request.target != self.capabilities.target {
            return Err(PlatformError::TargetMismatch {
                expected: self.capabilities.target,
                actual: request.target,
            });
        }
        if !self
            .capabilities
            .operations
            .iter()
            .any(|operation| operation == &request.operation)
        {
            return Err(PlatformError::ProviderFailure(ProviderFailure {
                class: crate::ProviderFailureClass::UnsupportedOperation,
                status_code: None,
                operation: Some(request.operation.clone()),
                provider_id: Some(self.capabilities.provider_id.as_str().to_owned()),
                message: format!("provider does not support operation {}", request.operation),
            }));
        }
        let payload = json!({ "request_id": request.context.request_id, "command_id": request.command_id, "operation": request.operation, "context": request.context, "payload": request.payload });
        Ok(AdapterResponse {
            request_id: request.context.request_id,
            target: request.target,
            operation: request.operation.clone(),
            accepted: true,
            payload: self.request_json(&request.operation, &payload)?,
        })
    }
    fn probe_health(&self) -> PlatformResult<ProviderHealth> {
        ProviderHealthProbe::probe_health(self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_invalid_configuration() {
        assert!(
            HttpJsonProviderAdapter::new(
                "x",
                IntegrationTarget::Llm,
                "ftp://example.test",
                ["generate"],
                "/health",
                Duration::from_secs(1)
            )
            .is_err()
        );
        assert!(
            HttpJsonProviderAdapter::new(
                "x",
                IntegrationTarget::Llm,
                "https://example.test",
                [],
                "/health",
                Duration::from_secs(1)
            )
            .is_err()
        );
        assert!(
            HttpJsonProviderAdapter::new(
                "x",
                IntegrationTarget::Llm,
                "https://example.test",
                ["generate"],
                "health",
                Duration::from_secs(1)
            )
            .is_err()
        );
        assert!(
            HttpJsonProviderAdapter::new(
                "x",
                IntegrationTarget::Llm,
                "https://example.test",
                ["generate"],
                "/health",
                Duration::ZERO
            )
            .is_err()
        );
    }
    #[test]
    fn constructs_deterministic_urls() {
        let adapter = HttpJsonProviderAdapter::new(
            "x",
            IntegrationTarget::Llm,
            "https://example.test/",
            ["generate"],
            "/health",
            Duration::from_secs(1),
        )
        .unwrap();
        assert_eq!(
            adapter.operation_url("generate"),
            "https://example.test/generate"
        );
        assert_eq!(adapter.health_url(), "https://example.test/health");
    }
    #[test]
    fn validates_custom_headers() {
        let adapter = HttpJsonProviderAdapter::new(
            "x",
            IntegrationTarget::Llm,
            "https://example.test",
            ["generate"],
            "/health",
            Duration::from_secs(1),
        )
        .unwrap();
        assert!(adapter.with_header("authorization", "Bearer test").is_ok());
        assert!(adapter.with_header("not a header", "x").is_err());
    }
}

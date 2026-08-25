use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ProviderFailureClass {
    InvalidRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    RateLimited,
    Server,
    Timeout,
    Transport,
    Serialization,
    TargetMismatch,
    UnsupportedOperation,
}

impl ProviderFailureClass {
    pub const fn retryable(self) -> bool {
        matches!(self, Self::RateLimited | Self::Server | Self::Timeout | Self::Transport)
    }

    pub const fn terminal(self) -> bool { !self.retryable() }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderFailure {
    pub class: ProviderFailureClass,
    pub status_code: Option<u16>,
    pub operation: Option<String>,
    pub provider_id: Option<String>,
    pub message: String,
}

impl ProviderFailure {
    pub fn http(status: StatusCode, operation: impl Into<String>, message: impl Into<String>) -> Self {
        let class = match status {
            StatusCode::BAD_REQUEST => ProviderFailureClass::InvalidRequest,
            StatusCode::UNAUTHORIZED => ProviderFailureClass::Unauthorized,
            StatusCode::FORBIDDEN => ProviderFailureClass::Forbidden,
            StatusCode::NOT_FOUND => ProviderFailureClass::NotFound,
            StatusCode::TOO_MANY_REQUESTS => ProviderFailureClass::RateLimited,
            s if s.is_server_error() => ProviderFailureClass::Server,
            _ => ProviderFailureClass::InvalidRequest,
        };
        Self { class, status_code: Some(status.as_u16()), operation: Some(operation.into()), provider_id: None, message: message.into() }
    }

    pub fn transport(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self { class: ProviderFailureClass::Transport, status_code: None, operation: Some(operation.into()), provider_id: None, message: message.into() }
    }

    pub fn timeout(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self { class: ProviderFailureClass::Timeout, status_code: None, operation: Some(operation.into()), provider_id: None, message: message.into() }
    }
}

impl fmt::Display for ProviderFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provider failure {:?}: {}", self.class, self.message)
    }
}

impl std::error::Error for ProviderFailure {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_http_failures_deterministically() {
        assert_eq!(ProviderFailure::http(StatusCode::TOO_MANY_REQUESTS, "x", "busy").class, ProviderFailureClass::RateLimited);
        assert!(ProviderFailure::http(StatusCode::SERVICE_UNAVAILABLE, "x", "down").class.retryable());
        assert!(ProviderFailure::http(StatusCode::UNAUTHORIZED, "x", "bad key").class.terminal());
    }

    #[test]
    fn transport_and_timeout_are_retryable() {
        assert!(ProviderFailure::transport("x", "io").class.retryable());
        assert!(ProviderFailure::timeout("x", "deadline").class.retryable());
    }
}

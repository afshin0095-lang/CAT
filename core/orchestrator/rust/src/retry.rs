use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self { Self { max_attempts: 5, base_delay_ms: 250, max_delay_ms: 30_000 } }
}

impl RetryPolicy {
    pub fn delay_ms(&self, attempt: u32) -> u64 {
        let exponent = attempt.saturating_sub(1).min(20);
        self.base_delay_ms.saturating_mul(1u64 << exponent).min(self.max_delay_ms)
    }

    pub fn retryable(&self, attempt: u32) -> bool { attempt < self.max_attempts }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exponential_backoff_is_bounded() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.delay_ms(1), 250);
        assert_eq!(policy.delay_ms(3), 1000);
        assert_eq!(policy.delay_ms(99), 30_000);
        assert!(!policy.retryable(5));
    }
}

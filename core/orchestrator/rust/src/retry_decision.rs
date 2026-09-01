use crate::RetryPolicy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetryDecision {
    Retry { delay_ms: u64 },
    Exhausted,
}

/// Decides whether a failed attempt should be retried without performing the retry.
pub fn decide_retry(policy: RetryPolicy, attempt: u32) -> RetryDecision {
    if policy.retryable(attempt) {
        RetryDecision::Retry { delay_ms: policy.delay_ms(attempt) }
    } else {
        RetryDecision::Exhausted
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_decision_uses_bounded_backoff() {
        let policy = RetryPolicy::default();
        assert_eq!(decide_retry(policy, 2), RetryDecision::Retry { delay_ms: 500 });
        assert_eq!(decide_retry(policy, 5), RetryDecision::Exhausted);
    }
}

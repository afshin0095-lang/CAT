use std::time::Duration;

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub exponential: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(30),
            exponential: true,
        }
    }
}

impl RetryPolicy {
    pub fn delay_for(&self, attempt: u32) -> Duration {
        if !self.exponential || attempt <= 1 {
            return self.initial_delay.min(self.max_delay);
        }
        let factor = 2u32.saturating_pow(attempt.saturating_sub(1));
        self.initial_delay.saturating_mul(factor).min(self.max_delay)
    }
}

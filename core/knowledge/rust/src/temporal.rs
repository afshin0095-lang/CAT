use serde::{Deserialize, Serialize};

/// A half-open validity interval for derived knowledge.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ValidityWindow {
    pub valid_from_ms: u64,
    pub valid_until_ms: Option<u64>,
}

impl ValidityWindow {
    pub fn from(start_ms: u64) -> Self { Self { valid_from_ms: start_ms, valid_until_ms: None } }

    pub fn until(mut self, end_ms: u64) -> Self {
        self.valid_until_ms = Some(end_ms);
        self
    }

    pub fn is_valid_at(&self, timestamp_ms: u64) -> bool {
        timestamp_ms >= self.valid_from_ms
            && self.valid_until_ms.is_none_or(|end| timestamp_ms < end)
    }

    pub fn is_well_formed(&self) -> bool {
        self.valid_until_ms.is_none_or(|end| end > self.valid_from_ms)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidityWindow;

    #[test]
    fn half_open_window_excludes_end_boundary() {
        let window = ValidityWindow::from(100).until(200);
        assert!(!window.is_valid_at(99));
        assert!(window.is_valid_at(100));
        assert!(window.is_valid_at(199));
        assert!(!window.is_valid_at(200));
    }

    #[test]
    fn malformed_window_is_detectable() {
        assert!(!ValidityWindow::from(200).until(100).is_well_formed());
    }
}

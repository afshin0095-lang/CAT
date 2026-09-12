use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variant {
    pub id: String,
    pub name: String,
    pub content: String,
    pub traffic_allocation: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VariantMetrics {
    pub impressions: u64,
    pub conversions: u64,
    pub clicks: u64,
    pub revenue: f64,
}
impl VariantMetrics {
    pub fn conversion_rate(&self) -> f64 {
        if self.impressions == 0 {
            0.0
        } else {
            self.conversions as f64 / self.impressions as f64
        }
    }
    pub fn ctr(&self) -> f64 {
        if self.impressions == 0 {
            0.0
        } else {
            self.clicks as f64 / self.impressions as f64
        }
    }
    pub fn rpi(&self) -> f64 {
        if self.impressions == 0 {
            0.0
        } else {
            self.revenue / self.impressions as f64
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ABTest {
    pub id: String,
    pub name: String,
    pub variants: HashMap<String, Variant>,
    pub metrics: HashMap<String, VariantMetrics>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub is_active: bool,
}
impl ABTest {
    pub fn new(id: String, name: String, variants: Vec<Variant>) -> Self {
        let mut vm = HashMap::new();
        let mut mm = HashMap::new();
        for v in variants {
            let id = v.id.clone();
            vm.insert(id.clone(), v);
            mm.insert(id, VariantMetrics::default());
        }
        Self {
            id,
            name,
            variants: vm,
            metrics: mm,
            start_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            end_time: None,
            is_active: true,
        }
    }
    pub fn record_impression(&mut self, id: &str) {
        if let Some(m) = self.metrics.get_mut(id) {
            m.impressions += 1;
        }
    }
    pub fn record_click(&mut self, id: &str) {
        if let Some(m) = self.metrics.get_mut(id) {
            m.clicks += 1;
        }
    }
    pub fn record_conversion(&mut self, id: &str, revenue: f64) {
        if let Some(m) = self.metrics.get_mut(id) {
            m.conversions += 1;
            m.revenue += revenue;
        }
    }
    pub fn winner(&self) -> Option<(String, f64)> {
        if !self.is_active {
            return None;
        }
        self.metrics
            .iter()
            .max_by(|a, b| {
                a.1.conversion_rate()
                    .partial_cmp(&b.1.conversion_rate())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, m)| (id.clone(), m.conversion_rate()))
    }
    pub fn end_test(&mut self) {
        self.is_active = false;
        self.end_time = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        );
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TestRunner {
    tests: HashMap<String, ABTest>,
}
impl TestRunner {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn create_test(&mut self, test: ABTest) {
        self.tests.insert(test.id.clone(), test);
    }
    pub fn get_test(&self, id: &str) -> Option<&ABTest> {
        self.tests.get(id)
    }
    pub fn get_test_mut(&mut self, id: &str) -> Option<&mut ABTest> {
        self.tests.get_mut(id)
    }
    pub fn list_active(&self) -> Vec<&ABTest> {
        self.tests.values().filter(|t| t.is_active).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metrics_are_calculated() {
        let m = VariantMetrics {
            impressions: 100,
            conversions: 10,
            clicks: 25,
            revenue: 150.0,
        };
        assert_eq!(m.conversion_rate(), 0.1);
        assert_eq!(m.ctr(), 0.25);
        assert_eq!(m.rpi(), 1.5);
    }
}

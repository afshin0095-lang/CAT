use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteCandidate {
    pub id: String,
    pub destination: String,
    pub network: String,
    pub geography: Option<String>,
    pub active: bool,
    pub epc: f64,
    pub conversion_rate: f64,
    pub commission_rate: f64,
    pub latency_ms: u32,
    pub failures: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RouteRequest {
    pub geography: Option<String>,
    pub network: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LinkHealth {
    pub checks: u32,
    pub successes: u32,
    pub failures: u32,
    pub last_status: Option<u16>,
}
impl LinkHealth {
    pub fn availability(&self) -> f64 {
        if self.checks == 0 {
            0.0
        } else {
            self.successes as f64 / self.checks as f64
        }
    }
    pub fn record(&mut self, status: u16) {
        self.checks += 1;
        self.last_status = Some(status);
        if (200..400).contains(&status) {
            self.successes += 1;
        } else {
            self.failures += 1;
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SmartRouter {
    candidates: HashMap<String, RouteCandidate>,
    health: HashMap<String, LinkHealth>,
}
impl SmartRouter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, candidate: RouteCandidate) {
        self.health.entry(candidate.id.clone()).or_default();
        self.candidates.insert(candidate.id.clone(), candidate);
    }
    pub fn record_health(&mut self, id: &str, status: u16) {
        if let Some(h) = self.health.get_mut(id) {
            h.record(status);
        }
    }
    pub fn choose(&self, request: &RouteRequest) -> Option<&RouteCandidate> {
        self.candidates
            .values()
            .filter(|c| c.active)
            .filter(|c| {
                request
                    .network
                    .as_ref()
                    .map(|n| n == &c.network)
                    .unwrap_or(true)
            })
            .filter(|c| {
                request.geography.is_none()
                    || c.geography.is_none()
                    || request.geography == c.geography
            })
            .filter(|c| {
                self.health
                    .get(&c.id)
                    .map(|h| h.failures == 0 || h.availability() >= 0.95)
                    .unwrap_or(true)
            })
            .max_by(|a, b| {
                self.score(a)
                    .partial_cmp(&self.score(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
    pub fn score(&self, c: &RouteCandidate) -> f64 {
        let availability = self
            .health
            .get(&c.id)
            .map(|h| h.availability())
            .unwrap_or(1.0);
        (c.epc * 0.45)
            + (c.conversion_rate * 100.0 * 0.30)
            + (c.commission_rate * 100.0 * 0.15)
            + (availability * 0.10)
            - ((c.latency_ms as f64 / 1000.0) * 0.02)
    }
    pub fn unhealthy(&self) -> Vec<&RouteCandidate> {
        self.candidates
            .values()
            .filter(|c| {
                self.health
                    .get(&c.id)
                    .map(|h| h.checks > 0 && h.availability() < 0.95)
                    .unwrap_or(false)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn c(id: &str, epc: f64) -> RouteCandidate {
        RouteCandidate {
            id: id.into(),
            destination: "https://example.com".into(),
            network: "network-a".into(),
            geography: None,
            active: true,
            epc,
            conversion_rate: 0.1,
            commission_rate: 0.1,
            latency_ms: 100,
            failures: 0,
        }
    }
    #[test]
    fn chooses_highest_yield() {
        let mut r = SmartRouter::new();
        r.register(c("low", 1.0));
        r.register(c("high", 3.0));
        assert_eq!(
            r.choose(&RouteRequest {
                geography: None,
                network: None
            })
            .unwrap()
            .id,
            "high"
        );
    }
    #[test]
    fn excludes_unhealthy_route() {
        let mut r = SmartRouter::new();
        r.register(c("bad", 10.0));
        for _ in 0..20 {
            r.record_health("bad", 500);
        }
        assert!(
            r.choose(&RouteRequest {
                geography: None,
                network: None
            })
            .is_none()
        );
        assert_eq!(r.unhealthy().len(), 1);
    }
}

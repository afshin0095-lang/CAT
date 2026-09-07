use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentVariant { pub id: String, pub template_id: String, pub audience: String, pub channel: String, pub impressions: u64, pub clicks: u64, pub conversions: u64, pub revenue: f64 }
impl ContentVariant { pub fn ctr(&self) -> f64 { if self.impressions == 0 { 0.0 } else { self.clicks as f64 / self.impressions as f64 } } pub fn conversion_rate(&self) -> f64 { if self.clicks == 0 { 0.0 } else { self.conversions as f64 / self.clicks as f64 } } pub fn revenue_per_impression(&self) -> f64 { if self.impressions == 0 { 0.0 } else { self.revenue / self.impressions as f64 } } }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ContentOptimizer { variants: HashMap<String, ContentVariant> }
impl ContentOptimizer { pub fn new() -> Self { Self::default() } pub fn record(&mut self, variant: ContentVariant) { self.variants.insert(variant.id.clone(), variant); } pub fn best_for(&self, audience: &str, channel: &str) -> Option<&ContentVariant> { self.variants.values().filter(|v| v.audience == audience && v.channel == channel).max_by(|a,b| a.revenue_per_impression().partial_cmp(&b.revenue_per_impression()).unwrap_or(std::cmp::Ordering::Equal)) } pub fn snapshot(&self) -> Vec<&ContentVariant> { self.variants.values().collect() } }

#[cfg(test)]
mod tests { use super::*; fn v(id: &str, revenue: f64) -> ContentVariant { ContentVariant { id: id.into(), template_id: "t".into(), audience: "buyers".into(), channel: "email".into(), impressions: 100, clicks: 20, conversions: 4, revenue } } #[test] fn selects_highest_yield() { let mut o = ContentOptimizer::new(); o.record(v("a", 10.0)); o.record(v("b", 25.0)); assert_eq!(o.best_for("buyers", "email").unwrap().id, "b"); } }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExperimentOutcome { pub experiment_id: String, pub hypothesis: String, pub baseline: f64, pub observed: f64, pub sample_size: u64, pub confidence: f64 }
impl ExperimentOutcome { pub fn uplift(&self) -> f64 { if self.baseline == 0.0 { 0.0 } else { (self.observed - self.baseline) / self.baseline } } pub fn is_actionable(&self, minimum_sample: u64, minimum_confidence: f64) -> bool { self.sample_size >= minimum_sample && self.confidence >= minimum_confidence } }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImprovementProposal { pub id: String, pub source_experiment: String, pub change: String, pub expected_uplift: f64, pub approved: bool }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SelfImprover { outcomes: HashMap<String, ExperimentOutcome>, proposals: Vec<ImprovementProposal>, minimum_sample: u64, minimum_confidence: f64 }
impl SelfImprover {
    pub fn new(minimum_sample: u64, minimum_confidence: f64) -> Self { Self { outcomes: HashMap::new(), proposals: Vec::new(), minimum_sample, minimum_confidence } }
    pub fn record(&mut self, outcome: ExperimentOutcome) -> Option<&ImprovementProposal> { let actionable = outcome.is_actionable(self.minimum_sample, self.minimum_confidence); let id = outcome.experiment_id.clone(); self.outcomes.insert(id.clone(), outcome); if !actionable { return None; } let o = self.outcomes.get(&id).unwrap(); let proposal = ImprovementProposal { id: format!("proposal:{id}"), source_experiment: id, change: o.hypothesis.clone(), expected_uplift: o.uplift(), approved: false }; self.proposals.push(proposal); self.proposals.last() }
    pub fn approve(&mut self, id: &str) -> bool { if let Some(p) = self.proposals.iter_mut().find(|p| p.id == id) { p.approved = true; true } else { false } }
    pub fn approved(&self) -> Vec<&ImprovementProposal> { self.proposals.iter().filter(|p| p.approved).collect() }
    pub fn proposals(&self) -> &[ImprovementProposal] { &self.proposals }
}

#[cfg(test)]
mod tests { use super::*; fn o(n: u64, confidence: f64) -> ExperimentOutcome { ExperimentOutcome { experiment_id: "exp-1".into(), hypothesis: "use winning template".into(), baseline: 0.10, observed: 0.13, sample_size: n, confidence } } #[test] fn creates_only_actionable_proposals() { let mut s = SelfImprover::new(100, 0.9); assert!(s.record(o(20, 0.99)).is_none()); assert!(s.record(o(200, 0.95)).is_some()); assert!(s.proposals()[0].expected_uplift > 0.29); } #[test] fn approval_is_explicit() { let mut s = SelfImprover::new(1, 0.5); let id = s.record(o(2, 0.8)).unwrap().id.clone(); assert!(s.approved().is_empty()); assert!(s.approve(&id)); assert_eq!(s.approved().len(), 1); } }

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VolumeMode { pub max_parallel: usize, pub max_daily_actions: u32, pub budget_limit: f64, pub stop_on_error: bool }
impl Default for VolumeMode { fn default() -> Self { Self { max_parallel: 4, max_daily_actions: 100, budget_limit: 50.0, stop_on_error: false } } }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VolumeLedger { pub active: usize, pub actions_today: u32, pub spent_today: f64 }
impl VolumeMode { pub fn can_start(&self, ledger: &VolumeLedger, estimated_cost: f64) -> bool { ledger.active < self.max_parallel && ledger.actions_today < self.max_daily_actions && ledger.spent_today + estimated_cost <= self.budget_limit } pub fn reserve(&self, ledger: &mut VolumeLedger, estimated_cost: f64) -> Result<(), String> { if !self.can_start(ledger, estimated_cost) { return Err("volume mode limit reached".into()); } ledger.active += 1; ledger.actions_today += 1; ledger.spent_today += estimated_cost; Ok(()) } pub fn release(&self, ledger: &mut VolumeLedger) { ledger.active = ledger.active.saturating_sub(1); } }

#[cfg(test)]
mod tests { use super::*; #[test] fn enforces_parallel_and_budget_limits() { let mode = VolumeMode { max_parallel: 1, max_daily_actions: 2, budget_limit: 5.0, stop_on_error: false }; let mut l = VolumeLedger::default(); assert!(mode.reserve(&mut l, 3.0).is_ok()); assert!(mode.reserve(&mut l, 1.0).is_err()); mode.release(&mut l); assert!(mode.reserve(&mut l, 3.0).is_err()); } }

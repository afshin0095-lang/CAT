use std::cmp::Ordering;
use std::collections::BinaryHeap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduleRequest {
    pub workflow_id: Uuid,
    pub not_before_ms: u64,
    pub priority: i32,
}

#[derive(Clone, Debug)]
struct QueueItem(ScheduleRequest);
impl PartialEq for QueueItem { fn eq(&self, other: &Self) -> bool { self.0.not_before_ms == other.0.not_before_ms && self.0.priority == other.0.priority && self.0.workflow_id == other.0.workflow_id } }
impl Eq for QueueItem {}
impl PartialOrd for QueueItem { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }
impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.not_before_ms.cmp(&self.0.not_before_ms).then_with(|| self.0.priority.cmp(&other.0.priority)).then_with(|| self.0.workflow_id.cmp(&other.0.workflow_id))
    }
}

#[derive(Default)]
pub struct Scheduler { queue: BinaryHeap<QueueItem> }
impl Scheduler {
    pub fn schedule(&mut self, request: ScheduleRequest) { self.queue.push(QueueItem(request)); }
    pub fn pop_ready(&mut self, now_ms: u64) -> Option<ScheduleRequest> { match self.queue.peek() { Some(item) if item.0.not_before_ms <= now_ms => self.queue.pop().map(|item| item.0), _ => None } }
    pub fn len(&self) -> usize { self.queue.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn earliest_schedule_is_ready_first() {
        let mut scheduler = Scheduler::default();
        let id = Uuid::now_v7();
        scheduler.schedule(ScheduleRequest { workflow_id: id, not_before_ms: 100, priority: 1 });
        assert!(scheduler.pop_ready(99).is_none());
        assert_eq!(scheduler.pop_ready(100).unwrap().workflow_id, id);
    }
}

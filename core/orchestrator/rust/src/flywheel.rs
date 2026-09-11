use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum FlywheelStage {
    Discover,
    Evaluate,
    Create,
    Validate,
    Distribute,
    Measure,
    Optimize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlywheelNode {
    pub id: String,
    pub stage: FlywheelStage,
    pub depends_on: Vec<String>,
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlywheelPlan {
    pub id: String,
    pub nodes: Vec<FlywheelNode>,
    pub revision: u64,
}

impl FlywheelPlan {
    pub fn new(id: impl Into<String>, nodes: Vec<FlywheelNode>) -> Result<Self, String> {
        let plan = Self {
            id: id.into(),
            nodes,
            revision: 0,
        };
        plan.validate()?;
        Ok(plan)
    }
    pub fn validate(&self) -> Result<(), String> {
        let ids: HashSet<&str> = self.nodes.iter().map(|n| n.id.as_str()).collect();
        if ids.len() != self.nodes.len() {
            return Err("duplicate flywheel node id".into());
        }
        for node in &self.nodes {
            for dep in &node.depends_on {
                if !ids.contains(dep.as_str()) {
                    return Err(format!("unknown dependency: {dep}"));
                }
            }
        }
        self.topological_order().map(|_| ())
    }
    pub fn topological_order(&self) -> Result<Vec<String>, String> {
        let mut indegree: HashMap<&str, usize> =
            self.nodes.iter().map(|n| (n.id.as_str(), 0)).collect();
        let mut next: HashMap<&str, Vec<&str>> = HashMap::new();
        for n in &self.nodes {
            for dep in &n.depends_on {
                *indegree.get_mut(n.id.as_str()).unwrap() += 1;
                next.entry(dep.as_str()).or_default().push(n.id.as_str());
            }
        }
        let mut queue: VecDeque<&str> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(id, _)| *id)
            .collect();
        let mut result = Vec::new();
        while let Some(id) = queue.pop_front() {
            result.push(id.to_string());
            if let Some(children) = next.get(id) {
                for child in children {
                    let d = indegree.get_mut(child).unwrap();
                    *d -= 1;
                    if *d == 0 {
                        queue.push_back(child);
                    }
                }
            }
        }
        if result.len() != self.nodes.len() {
            return Err("flywheel plan contains a cycle".into());
        }
        Ok(result)
    }
    pub fn ready_nodes(&self, completed: &HashSet<String>) -> Vec<&FlywheelNode> {
        self.nodes
            .iter()
            .filter(|n| n.enabled && n.depends_on.iter().all(|d| completed.contains(d)))
            .collect()
    }
    pub fn bump_revision(&mut self) {
        self.revision += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn n(id: &str, deps: &[&str]) -> FlywheelNode {
        FlywheelNode {
            id: id.into(),
            stage: FlywheelStage::Discover,
            depends_on: deps.iter().map(|x| (*x).into()).collect(),
            enabled: true,
        }
    }
    #[test]
    fn orders_dag() {
        let p =
            FlywheelPlan::new("p", vec![n("discover", &[]), n("create", &["discover"])]).unwrap();
        assert_eq!(p.topological_order().unwrap(), vec!["discover", "create"]);
    }
    #[test]
    fn rejects_cycle() {
        assert!(FlywheelPlan::new("p", vec![n("a", &["b"]), n("b", &["a"])]).is_err());
    }
}

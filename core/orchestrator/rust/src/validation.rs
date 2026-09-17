use std::collections::HashMap;

use uuid::Uuid;

use crate::error::{OrchestratorError, OrchestratorResult};
use crate::model::{StepState, WorkflowDefinition, WorkflowInstance};

/// Validates workflow definitions before they enter the execution plane.
///
/// The validator is intentionally deterministic: the same definition always
/// yields the same result and no wall-clock state is consulted.
pub fn validate_definition(definition: &WorkflowDefinition) -> OrchestratorResult<()> {
    if definition.workflow_type.trim().is_empty() {
        return Err(OrchestratorError::Serialization(
            "workflow_type must not be empty".to_string(),
        ));
    }

    let mut indexes = HashMap::new();
    for (index, step) in definition.steps.iter().enumerate() {
        if step.id.trim().is_empty() {
            return Err(OrchestratorError::UnknownStep {
                step_id: "<empty>".to_string(),
            });
        }
        if indexes.insert(step.id.as_str(), index).is_some() {
            return Err(OrchestratorError::Serialization(format!(
                "duplicate workflow step id {}",
                step.id
            )));
        }
        if step.max_attempts == 0 {
            return Err(OrchestratorError::Serialization(format!(
                "step {} must allow at least one attempt",
                step.id
            )));
        }
    }

    for step in &definition.steps {
        for dependency in &step.dependencies {
            if dependency == &step.id || !indexes.contains_key(dependency.as_str()) {
                return Err(OrchestratorError::UnknownStep {
                    step_id: dependency.clone(),
                });
            }
        }
        if let Some(compensation) = &step.compensation_step {
            if !indexes.contains_key(compensation.as_str()) {
                return Err(OrchestratorError::UnknownStep {
                    step_id: compensation.clone(),
                });
            }
        }
    }

    // Kahn's algorithm. A definition with no remaining zero-indegree node has
    // a dependency cycle and must never be admitted to the scheduler.
    let mut indegree = vec![0usize; definition.steps.len()];
    let mut outgoing = vec![Vec::<usize>::new(); definition.steps.len()];
    for (index, step) in definition.steps.iter().enumerate() {
        indegree[index] = step.dependencies.len();
        for dependency in &step.dependencies {
            let dependency_index = indexes[dependency.as_str()];
            outgoing[dependency_index].push(index);
        }
    }

    let mut ready: Vec<usize> = indegree
        .iter()
        .enumerate()
        .filter_map(|(index, degree)| (*degree == 0).then_some(index))
        .collect();
    let mut visited = 0usize;

    while let Some(index) = ready.pop() {
        visited += 1;
        for next in &outgoing[index] {
            indegree[*next] -= 1;
            if indegree[*next] == 0 {
                ready.push(*next);
            }
        }
    }

    if visited != definition.steps.len() {
        return Err(OrchestratorError::DependencyCycle);
    }

    Ok(())
}

/// Returns a deterministic execution order. Independent steps retain their
/// declaration order, which makes replay and debugging stable.
pub fn topological_order(definition: &WorkflowDefinition) -> OrchestratorResult<Vec<String>> {
    validate_definition(definition)?;

    let mut indexes = HashMap::new();
    for (index, step) in definition.steps.iter().enumerate() {
        indexes.insert(step.id.as_str(), index);
    }

    let mut indegree: Vec<usize> = definition
        .steps
        .iter()
        .map(|step| step.dependencies.len())
        .collect();
    let mut outgoing = vec![Vec::<usize>::new(); definition.steps.len()];

    for (index, step) in definition.steps.iter().enumerate() {
        for dependency in &step.dependencies {
            outgoing[indexes[dependency.as_str()]].push(index);
        }
    }

    let mut ready: Vec<usize> = indegree
        .iter()
        .enumerate()
        .filter_map(|(index, degree)| (*degree == 0).then_some(index))
        .collect();
    ready.sort_unstable_by(|a, b| b.cmp(a));

    let mut result = Vec::with_capacity(definition.steps.len());
    while let Some(index) = ready.pop() {
        result.push(definition.steps[index].id.clone());
        for next in &outgoing[index] {
            indegree[*next] -= 1;
            if indegree[*next] == 0 {
                let position = ready
                    .binary_search_by(|candidate| candidate.cmp(next).reverse())
                    .unwrap_or_else(|position| position);
                ready.insert(position, *next);
            }
        }
    }

    Ok(result)
}

/// Creates an instance only after the definition has passed structural
/// validation. This is the preferred admission path for callers.
pub fn new_validated_instance(
    definition: WorkflowDefinition,
) -> OrchestratorResult<WorkflowInstance> {
    validate_definition(&definition)?;
    Ok(WorkflowInstance::new(definition))
}

/// Returns step identifiers that are executable without mutating workflow
/// state. A step is ready when all dependencies have succeeded or been
/// compensated, and the step itself is Pending or Ready.
pub fn ready_steps(instance: &WorkflowInstance) -> OrchestratorResult<Vec<String>> {
    validate_definition(&instance.definition)?;

    let states: HashMap<&str, StepState> = instance
        .definition
        .steps
        .iter()
        .map(|step| (step.id.as_str(), step.state))
        .collect();

    let mut ready = Vec::new();
    for step in &instance.definition.steps {
        if !matches!(step.state, StepState::Pending | StepState::Ready) {
            continue;
        }

        let dependencies_complete = step.dependencies.iter().all(|dependency| {
            matches!(
                states.get(dependency.as_str()),
                Some(StepState::Succeeded | StepState::Compensated)
            )
        });

        if dependencies_complete {
            ready.push(step.id.clone());
        }
    }

    Ok(ready)
}

pub fn workflow_id(instance: &WorkflowInstance) -> Uuid {
    instance.id
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{WorkflowDefinition, WorkflowStep};

    fn step(id: &str, dependencies: &[&str]) -> WorkflowStep {
        WorkflowStep {
            id: id.to_string(),
            dependencies: dependencies.iter().map(|value| value.to_string()).collect(),
            state: StepState::Pending,
            attempt: 0,
            max_attempts: 3,
            compensation_step: None,
        }
    }

    #[test]
    fn rejects_cycles() {
        let definition = WorkflowDefinition {
            workflow_type: "cycle".to_string(),
            version: 1,
            steps: vec![step("a", &["b"]), step("b", &["a"])],
        };

        assert!(matches!(
            validate_definition(&definition),
            Err(OrchestratorError::DependencyCycle)
        ));
    }

    #[test]
    fn preserves_declaration_order_for_independent_steps() {
        let definition = WorkflowDefinition {
            workflow_type: "linear".to_string(),
            version: 1,
            steps: vec![step("a", &[]), step("b", &[]), step("c", &["a"])],
        };

        assert_eq!(topological_order(&definition).unwrap(), vec!["a", "b", "c"]);
    }

    #[test]
    fn exposes_only_dependency_complete_steps() {
        let definition = WorkflowDefinition {
            workflow_type: "ready".to_string(),
            version: 1,
            steps: vec![step("a", &[]), step("b", &["a"])],
        };
        let instance = new_validated_instance(definition).unwrap();

        assert_eq!(ready_steps(&instance).unwrap(), vec!["a"]);
    }
}

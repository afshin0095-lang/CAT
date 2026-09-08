use crate::error::{OrchestratorError, OrchestratorResult};
use crate::model::{StepState, WorkflowDefinition, WorkflowInstance, WorkflowState};
use crate::validation::topological_order;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaySnapshot {
    pub workflow_id: uuid::Uuid,
    pub revision: u64,
    pub workflow_type: String,
    pub definition_version: u16,
    pub workflow_state: WorkflowState,
    pub ordered_steps: Vec<(String, StepState, u32)>,
    pub graph_identity: Vec<(String, Vec<String>)>,
}

fn graph_identity(definition: &WorkflowDefinition) -> Vec<(String, Vec<String>)> {
    definition
        .steps
        .iter()
        .map(|step| {
            let mut dependencies = step.dependencies.clone();
            dependencies.sort();
            (step.id.clone(), dependencies)
        })
        .collect()
}

pub fn snapshot(instance: &WorkflowInstance) -> OrchestratorResult<ReplaySnapshot> {
    let order = topological_order(&instance.definition)?;
    let ordered_steps = order
        .into_iter()
        .map(|id| {
            let step = instance
                .definition
                .steps
                .iter()
                .find(|step| step.id == id)
                .expect("validated definition must contain every ordered step");
            (step.id.clone(), step.state, step.attempt)
        })
        .collect();

    Ok(ReplaySnapshot {
        workflow_id: instance.id,
        revision: instance.revision,
        workflow_type: instance.definition.workflow_type.clone(),
        definition_version: instance.definition.version,
        workflow_state: instance.state,
        ordered_steps,
        graph_identity: graph_identity(&instance.definition),
    })
}

pub fn verify_replay(
    definition: &WorkflowDefinition,
    expected: &ReplaySnapshot,
) -> OrchestratorResult<()> {
    let actual = topological_order(definition)?;
    let expected_order: Vec<&str> = expected
        .ordered_steps
        .iter()
        .map(|(id, _, _)| id.as_str())
        .collect();

    if actual.len() != expected_order.len()
        || actual.iter().map(String::as_str).ne(expected_order.into_iter())
    {
        return Err(OrchestratorError::Serialization(
            "workflow replay order does not match snapshot".to_string(),
        ));
    }

    if definition.workflow_type != expected.workflow_type
        || definition.version != expected.definition_version
    {
        return Err(OrchestratorError::Serialization(
            "workflow replay definition identity does not match snapshot".to_string(),
        ));
    }

    if graph_identity(definition) != expected.graph_identity {
        return Err(OrchestratorError::Serialization(
            "workflow replay graph does not match snapshot".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::WorkflowStep;

    fn definition() -> WorkflowDefinition {
        WorkflowDefinition {
            workflow_type: "replay".into(),
            version: 1,
            steps: vec![
                WorkflowStep {
                    id: "a".into(),
                    dependencies: vec![],
                    state: StepState::Succeeded,
                    attempt: 1,
                    max_attempts: 3,
                    compensation_step: None,
                },
                WorkflowStep {
                    id: "b".into(),
                    dependencies: vec!["a".into()],
                    state: StepState::Pending,
                    attempt: 0,
                    max_attempts: 3,
                    compensation_step: None,
                },
            ],
        }
    }

    #[test]
    fn snapshot_is_topologically_ordered() {
        let instance = WorkflowInstance::new(definition());
        let replay = snapshot(&instance).unwrap();
        assert_eq!(replay.ordered_steps[0].0, "a");
        assert_eq!(replay.ordered_steps[1].0, "b");
        assert_eq!(replay.workflow_type, "replay");
        assert_eq!(replay.definition_version, 1);
        assert_eq!(replay.graph_identity, vec![("a".into(), vec![]), ("b".into(), vec!["a".into()])]);
    }

    #[test]
    fn replay_verification_rejects_different_graph() {
        let instance = WorkflowInstance::new(definition());
        let replay = snapshot(&instance).unwrap();
        let mut changed = definition();
        changed.steps[1].dependencies.clear();
        assert!(verify_replay(&changed, &replay).is_err());
    }

    #[test]
    fn replay_verification_rejects_different_definition_version() {
        let instance = WorkflowInstance::new(definition());
        let replay = snapshot(&instance).unwrap();
        let mut changed = definition();
        changed.version = 2;
        assert!(verify_replay(&changed, &replay).is_err());
    }

    #[test]
    fn replay_verification_accepts_same_graph_with_dependency_order_difference() {
        let mut original = definition();
        original.steps.push(WorkflowStep {
            id: "c".into(),
            dependencies: vec!["b".into(), "a".into()],
            state: StepState::Pending,
            attempt: 0,
            max_attempts: 3,
            compensation_step: None,
        });
        let instance = WorkflowInstance::new(original.clone());
        let replay = snapshot(&instance).unwrap();
        original.steps[2].dependencies.reverse();
        assert!(verify_replay(&original, &replay).is_ok());
    }
}

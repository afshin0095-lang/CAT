use crate::error::{OrchestratorError, OrchestratorResult};
use crate::model::{StepState, WorkflowDefinition, WorkflowInstance, WorkflowState};
use crate::validation::topological_order;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplaySnapshot {
    pub workflow_id: uuid::Uuid,
    pub revision: u64,
    pub workflow_state: WorkflowState,
    pub ordered_steps: Vec<(String, StepState, u32)>,
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
        workflow_state: instance.state,
        ordered_steps,
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
        || actual
            .iter()
            .map(String::as_str)
            .ne(expected_order.into_iter())
    {
        return Err(OrchestratorError::Serialization(
            "workflow replay order does not match snapshot".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::WorkflowStep;
    use cat_kernel::CapabilityId;

    fn definition() -> WorkflowDefinition {
        WorkflowDefinition {
            workflow_type: "replay".into(),
            version: 1,
            steps: vec![
                WorkflowStep {
                    id: "a".into(),
                    capability_id: CapabilityId::new("cat.capability.test.replay.v1").unwrap(),
                    dependencies: vec![],
                    state: StepState::Succeeded,
                    attempt: 1,
                    max_attempts: 3,
                    compensation_step: None,
                },
                WorkflowStep {
                    id: "b".into(),
                    capability_id: CapabilityId::new("cat.capability.test.replay.v1").unwrap(),
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
    }

    #[test]
    fn replay_verification_rejects_different_graph() {
        let instance = WorkflowInstance::new(definition());
        let replay = snapshot(&instance).unwrap();
        let mut changed = definition();
        changed.steps[1].dependencies.clear();
        assert!(verify_replay(&changed, &replay).is_err());
    }
}

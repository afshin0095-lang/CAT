use std::collections::BTreeMap;

use crate::{CapabilityContract, CapabilityId, CapabilityLifecycle, KernelError, KernelResult};

/// In-memory canonical registry for provider-neutral CAT capability contracts.
///
/// Persistence and distributed synchronization belong to a higher layer. This registry
/// owns deterministic identity, validation, lifecycle transitions, and duplicate rejection.
#[derive(Clone, Debug, Default)]
pub struct CapabilityRegistry {
    entries: BTreeMap<CapabilityId, CapabilityContract>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, contract: CapabilityContract) -> KernelResult<()> {
        contract.validate()?;
        if self.entries.contains_key(&contract.id) {
            return Err(KernelError::InvalidInput(format!(
                "capability {} is already registered",
                contract.id
            )));
        }
        self.entries.insert(contract.id.clone(), contract);
        Ok(())
    }

    pub fn get(&self, id: &CapabilityId) -> Option<&CapabilityContract> {
        self.entries.get(id)
    }

    pub fn get_mut(&mut self, id: &CapabilityId) -> Option<&mut CapabilityContract> {
        self.entries.get_mut(id)
    }

    pub fn list(&self) -> impl Iterator<Item = &CapabilityContract> {
        self.entries.values()
    }

    pub fn transition(
        &mut self,
        id: &CapabilityId,
        target: CapabilityLifecycle,
    ) -> KernelResult<()> {
        let contract = self.entries.get_mut(id).ok_or_else(|| {
            KernelError::InvalidInput(format!("capability {} is not registered", id))
        })?;

        let current = contract.lifecycle;
        if !valid_transition(current, target) {
            return Err(KernelError::InvalidInput(format!(
                "invalid capability lifecycle transition: {current:?} -> {target:?}"
            )));
        }

        if target == CapabilityLifecycle::Active {
            contract.validate()?;
        }

        contract.lifecycle = target;
        Ok(())
    }

    pub fn contains(&self, id: &CapabilityId) -> bool {
        self.entries.contains_key(id)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn valid_transition(current: CapabilityLifecycle, target: CapabilityLifecycle) -> bool {
    matches!(
        (current, target),
        (CapabilityLifecycle::Proposed, CapabilityLifecycle::Specified)
            | (CapabilityLifecycle::Proposed, CapabilityLifecycle::Rejected)
            | (CapabilityLifecycle::Specified, CapabilityLifecycle::Validating)
            | (CapabilityLifecycle::Specified, CapabilityLifecycle::Rejected)
            | (CapabilityLifecycle::Validating, CapabilityLifecycle::Active)
            | (CapabilityLifecycle::Validating, CapabilityLifecycle::Rejected)
            | (CapabilityLifecycle::Rejected, CapabilityLifecycle::Proposed)
            | (CapabilityLifecycle::Active, CapabilityLifecycle::Deprecated)
            | (CapabilityLifecycle::Deprecated, CapabilityLifecycle::Retired)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CapabilityContract, CapabilityId};

    fn contract(name: &str) -> CapabilityContract {
        let mut value = CapabilityContract::new(
            CapabilityId::new(format!("cat.capability.test.{name}.v1")).unwrap(),
            "test",
            "Test capability",
        )
        .unwrap();
        value.inputs.push("input".to_owned());
        value.outputs.push("output".to_owned());
        value.failure_model.push("typed failure".to_owned());
        value.observability.push("test.capability.*".to_owned());
        value.evaluation.push("determinism".to_owned());
        value
    }

    #[test]
    fn registry_rejects_duplicate_identity() {
        let mut registry = CapabilityRegistry::new();
        let first = contract("one");
        let duplicate = first.clone();
        registry.register(first).unwrap();
        assert!(registry.register(duplicate).is_err());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn lifecycle_follows_canonical_state_machine() {
        let mut registry = CapabilityRegistry::new();
        let capability = contract("one");
        let id = capability.id.clone();
        registry.register(capability).unwrap();

        registry.transition(&id, CapabilityLifecycle::Specified).unwrap();
        registry.transition(&id, CapabilityLifecycle::Validating).unwrap();
        registry.transition(&id, CapabilityLifecycle::Active).unwrap();
        registry.transition(&id, CapabilityLifecycle::Deprecated).unwrap();
        registry.transition(&id, CapabilityLifecycle::Retired).unwrap();

        assert_eq!(registry.get(&id).unwrap().lifecycle, CapabilityLifecycle::Retired);
    }

    #[test]
    fn invalid_transition_is_rejected() {
        let mut registry = CapabilityRegistry::new();
        let capability = contract("one");
        let id = capability.id.clone();
        registry.register(capability).unwrap();
        assert!(registry.transition(&id, CapabilityLifecycle::Active).is_err());
    }

    #[test]
    fn registry_iteration_is_deterministic() {
        let mut registry = CapabilityRegistry::new();
        registry.register(contract("zeta")).unwrap();
        registry.register(contract("alpha")).unwrap();
        let ids: Vec<&str> = registry.list().map(|entry| entry.id.as_str()).collect();
        assert_eq!(ids, vec![
            "cat.capability.test.alpha.v1",
            "cat.capability.test.zeta.v1",
        ]);
    }
}

use serde::{Deserialize, Serialize};

use crate::{ContractVersion, EntityId, KernelError, KernelResult, SideEffectClass};

/// Stable identity of a CAT agent.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AgentId(EntityId);

impl AgentId {
    pub fn new() -> Self { Self(EntityId::new()) }
    pub const fn from_entity_id(value: EntityId) -> Self { Self(value) }
    pub const fn as_entity_id(self) -> EntityId { self.0 }
}

impl Default for AgentId {
    fn default() -> Self { Self::new() }
}

/// Canonical v1 contract describing an agent's executable boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentContract {
    pub contract_version: ContractVersion,
    pub agent_id: AgentId,
    pub agent_type: String,
    pub mission: String,
    pub capabilities: Vec<String>,
    pub policy_scope: Vec<String>,
    pub memory_scope: Vec<String>,
    pub knowledge_scope: Vec<String>,
    pub allowed_side_effect: SideEffectClass,
    pub enabled: bool,
}

impl AgentContract {
    pub fn new(
        agent_id: AgentId,
        agent_type: impl Into<String>,
        mission: impl Into<String>,
    ) -> KernelResult<Self> {
        let agent_type = agent_type.into();
        let mission = mission.into();
        if agent_type.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "agent type must not be empty".to_owned(),
            ));
        }
        if mission.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "agent mission must not be empty".to_owned(),
            ));
        }

        Ok(Self {
            contract_version: ContractVersion::V1,
            agent_id,
            agent_type,
            mission,
            capabilities: Vec::new(),
            policy_scope: Vec::new(),
            memory_scope: Vec::new(),
            knowledge_scope: Vec::new(),
            allowed_side_effect: SideEffectClass::S0,
            enabled: false,
        })
    }

    pub fn with_capability(mut self, capability: impl Into<String>) -> KernelResult<Self> {
        self.capabilities = push_non_empty(self.capabilities, capability.into(), "capability")?;
        Ok(self)
    }

    pub fn with_policy_scope(mut self, policy: impl Into<String>) -> KernelResult<Self> {
        self.policy_scope = push_non_empty(self.policy_scope, policy.into(), "policy scope")?;
        Ok(self)
    }

    pub fn with_memory_scope(mut self, scope: impl Into<String>) -> KernelResult<Self> {
        self.memory_scope = push_non_empty(self.memory_scope, scope.into(), "memory scope")?;
        Ok(self)
    }

    pub fn with_knowledge_scope(mut self, scope: impl Into<String>) -> KernelResult<Self> {
        self.knowledge_scope = push_non_empty(self.knowledge_scope, scope.into(), "knowledge scope")?;
        Ok(self)
    }

    pub const fn with_side_effect_limit(mut self, side_effect: SideEffectClass) -> Self {
        self.allowed_side_effect = side_effect;
        self
    }

    pub const fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Validates the minimum invariants required before registration/activation.
    pub fn validate(&self) -> KernelResult<()> {
        if self.contract_version.major == 0 {
            return Err(KernelError::InvalidInput(
                "agent contract major version must be greater than zero".to_owned(),
            ));
        }
        if self.agent_type.trim().is_empty() || self.mission.trim().is_empty() {
            return Err(KernelError::InvalidInput(
                "agent type and mission must not be empty".to_owned(),
            ));
        }
        if self.enabled && self.capabilities.is_empty() {
            return Err(KernelError::InvalidInput(
                "an enabled agent must declare at least one capability".to_owned(),
            ));
        }
        if self.allowed_side_effect != SideEffectClass::S0 && self.policy_scope.is_empty() {
            return Err(KernelError::InvalidInput(
                "side-effectful agents must declare a policy scope".to_owned(),
            ));
        }
        Ok(())
    }
}

fn push_non_empty(mut values: Vec<String>, value: String, field: &str) -> KernelResult<Vec<String>> {
    if value.trim().is_empty() {
        return Err(KernelError::InvalidInput(format!("{field} must not be empty")));
    }
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_agent_is_disabled_and_pure_by_default() {
        let contract = AgentContract::new(AgentId::new(), "affiliate.discovery", "Discover affiliate opportunities").unwrap();
        assert!(!contract.enabled);
        assert_eq!(contract.allowed_side_effect, SideEffectClass::S0);
        assert!(contract.validate().is_ok());
    }

    #[test]
    fn enabled_agent_requires_a_capability() {
        let contract = AgentContract::new(AgentId::new(), "test", "test mission").unwrap().enable();
        assert!(contract.validate().is_err());
    }

    #[test]
    fn side_effectful_agent_requires_policy_scope() {
        let contract = AgentContract::new(AgentId::new(), "test", "test mission")
            .unwrap()
            .with_capability("publish")
            .unwrap()
            .with_side_effect_limit(SideEffectClass::S2)
            .enable();
        assert!(contract.validate().is_err());
    }

    #[test]
    fn duplicate_capabilities_are_not_added() {
        let contract = AgentContract::new(AgentId::new(), "test", "test mission")
            .unwrap()
            .with_capability("search")
            .unwrap()
            .with_capability("search")
            .unwrap();
        assert_eq!(contract.capabilities, vec!["search"]);
    }
}

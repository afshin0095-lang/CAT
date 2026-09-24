use serde::{Deserialize, Serialize};

use crate::{ContractVersion, KernelError, KernelResult, SideEffectClass};

/// Stable globally unique identity of a CAT capability.
///
/// The canonical format is `cat.capability.<domain>.<name>.v<major>`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CapabilityId(String);

impl CapabilityId {
    pub fn new(value: impl Into<String>) -> KernelResult<Self> {
        let value = value.into();
        validate_capability_id(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for CapabilityId {
    type Error = KernelError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl std::fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Lifecycle of a registered capability contract.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum CapabilityLifecycle {
    Proposed,
    Specified,
    Validating,
    Active,
    Rejected,
    Deprecated,
    Retired,
}

impl CapabilityLifecycle {
    pub const fn is_runtime_usable(self) -> bool {
        matches!(self, Self::Active)
    }
}

/// Declares how duplicate durable invocations are handled.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum IdempotencyPolicy {
    Required,
    ExplicitDeduplication { strategy: String },
    NotApplicable,
}

/// Canonical provider-neutral contract for a CAT capability.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilityContract {
    pub contract_version: ContractVersion,
    pub id: CapabilityId,
    pub domain: String,
    pub purpose: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub side_effects: SideEffectClass,
    pub required_policy: Vec<String>,
    pub dependencies: Vec<CapabilityId>,
    pub evidence: Vec<String>,
    pub failure_model: Vec<String>,
    pub idempotency: IdempotencyPolicy,
    pub observability: Vec<String>,
    pub evaluation: Vec<String>,
    pub economics: Vec<String>,
    pub lifecycle: CapabilityLifecycle,
}

impl CapabilityContract {
    pub fn new(
        id: CapabilityId,
        domain: impl Into<String>,
        purpose: impl Into<String>,
    ) -> KernelResult<Self> {
        let domain = domain.into();
        let purpose = purpose.into();
        require_non_empty(&domain, "capability domain")?;
        require_non_empty(&purpose, "capability purpose")?;

        Ok(Self {
            contract_version: ContractVersion::V1,
            id,
            domain,
            purpose,
            inputs: Vec::new(),
            outputs: Vec::new(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            side_effects: SideEffectClass::S0,
            required_policy: Vec::new(),
            dependencies: Vec::new(),
            evidence: Vec::new(),
            failure_model: Vec::new(),
            idempotency: IdempotencyPolicy::NotApplicable,
            observability: Vec::new(),
            evaluation: Vec::new(),
            economics: Vec::new(),
            lifecycle: CapabilityLifecycle::Proposed,
        })
    }

    pub fn validate(&self) -> KernelResult<()> {
        if self.contract_version.major == 0 {
            return Err(KernelError::InvalidInput(
                "capability contract major version must be greater than zero".to_owned(),
            ));
        }
        validate_capability_id(self.id.as_str())?;
        require_non_empty(&self.domain, "capability domain")?;
        require_non_empty(&self.purpose, "capability purpose")?;
        require_non_empty_collection(&self.inputs, "capability inputs")?;
        require_non_empty_collection(&self.outputs, "capability outputs")?;
        require_non_empty_collection(&self.failure_model, "capability failure model")?;
        require_non_empty_collection(&self.observability, "capability observability")?;
        require_non_empty_collection(&self.evaluation, "capability evaluation")?;

        if self.side_effects.requires_authorization() && self.required_policy.is_empty() {
            return Err(KernelError::InvalidInput(
                "side-effecting capabilities must declare a required policy".to_owned(),
            ));
        }

        if self.side_effects != SideEffectClass::S0
            && matches!(self.idempotency, IdempotencyPolicy::NotApplicable)
        {
            return Err(KernelError::InvalidInput(
                "durable side-effecting capabilities must declare an idempotency policy".to_owned(),
            ));
        }

        if self.side_effects != SideEffectClass::S0 && self.evidence.is_empty() {
            return Err(KernelError::InvalidInput(
                "side-effecting capabilities must declare evidence requirements".to_owned(),
            ));
        }

        Ok(())
    }
}

fn validate_capability_id(value: &str) -> KernelResult<()> {
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() != 5
        || parts[0] != "cat"
        || parts[1] != "capability"
        || parts[2].is_empty()
        || parts[3].is_empty()
        || !parts[4].starts_with('v')
        || parts[4][1..].parse::<u16>().is_err()
        || parts[4][1..] == "0"
    {
        return Err(KernelError::InvalidInput(
            "capability id must match cat.capability.<domain>.<name>.v<major>".to_owned(),
        ));
    }

    if value.chars().any(|character| character.is_whitespace() || character.is_ascii_uppercase()) {
        return Err(KernelError::InvalidInput(
            "capability id must be lowercase and contain no whitespace".to_owned(),
        ));
    }

    Ok(())
}

fn require_non_empty(value: &str, field: &str) -> KernelResult<()> {
    if value.trim().is_empty() {
        return Err(KernelError::InvalidInput(format!("{field} must not be empty")));
    }
    Ok(())
}

fn require_non_empty_collection(values: &[String], field: &str) -> KernelResult<()> {
    if values.is_empty() || values.iter().any(|value| value.trim().is_empty()) {
        return Err(KernelError::InvalidInput(format!(
            "{field} must contain at least one non-empty value"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_contract() -> CapabilityContract {
        let mut contract = CapabilityContract::new(
            CapabilityId::new("cat.capability.affiliate.discover.v1").unwrap(),
            "affiliate",
            "Discover affiliate opportunities",
        )
        .unwrap();
        contract.inputs.push("DiscoveryRequest".to_owned());
        contract.outputs.push("DiscoveryResult".to_owned());
        contract.failure_model.push("typed retryable failure".to_owned());
        contract.observability.push("affiliate.discovery.*".to_owned());
        contract.evaluation.push("discovery precision".to_owned());
        contract
    }

    #[test]
    fn capability_id_enforces_canonical_shape() {
        assert!(CapabilityId::new("cat.capability.affiliate.discover.v1").is_ok());
        assert!(CapabilityId::new("cat.capability.Affiliate.discover.v1").is_err());
        assert!(CapabilityId::new("cat.tool.affiliate.discover.v1").is_err());
        assert!(CapabilityId::new("cat.capability.affiliate.discover.v0").is_err());
    }

    #[test]
    fn pure_capability_can_validate_without_policy() {
        assert!(valid_contract().validate().is_ok());
    }

    #[test]
    fn side_effecting_capability_requires_policy_evidence_and_idempotency() {
        let mut contract = valid_contract();
        contract.side_effects = SideEffectClass::S2;
        assert!(contract.validate().is_err());

        contract.required_policy.push("affiliate.publish".to_owned());
        contract.evidence.push("audit.record".to_owned());
        contract.idempotency = IdempotencyPolicy::Required;
        assert!(contract.validate().is_ok());
    }

    #[test]
    fn capability_contract_round_trips_through_json() {
        let contract = valid_contract();
        let encoded = serde_json::to_string(&contract).unwrap();
        let decoded: CapabilityContract = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, contract);
    }
}

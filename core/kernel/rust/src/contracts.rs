use serde::{Deserialize, Serialize};

/// Semantic version for a CAT contract family.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ContractVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl ContractVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub const V1: Self = Self::new(1, 0, 0);
}

impl Default for ContractVersion {
    fn default() -> Self {
        Self::V1
    }
}

/// Declares the maximum side-effect boundary an invocation may cross.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum SideEffectClass {
    /// Pure computation; no externally observable mutation.
    S0,
    /// Local or reversible state mutation.
    S1,
    /// External or durable business mutation requiring policy authorization.
    S2,
    /// Financial, security-sensitive, or otherwise high-impact mutation.
    S3,
}

impl SideEffectClass {
    pub const fn requires_authorization(self) -> bool {
        !matches!(self, Self::S0)
    }

    pub const fn is_high_impact(self) -> bool {
        matches!(self, Self::S3)
    }
}

/// Durable lifecycle state of a contract invocation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum InvocationStatus {
    Pending,
    Running,
    WaitingForApproval,
    Succeeded,
    Failed,
    Cancelled,
    Unknown,
}

impl InvocationStatus {
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Failed | Self::Cancelled | Self::Unknown
        )
    }

    pub const fn is_success(self) -> bool {
        matches!(self, Self::Succeeded)
    }
}

/// Outcome status intentionally distinguishes an unresolved external result from failure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum OutcomeStatus {
    Succeeded,
    Failed,
    Unknown,
}

impl OutcomeStatus {
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_version_defaults_to_v1() {
        assert_eq!(ContractVersion::default(), ContractVersion::V1);
    }

    #[test]
    fn side_effect_authorization_boundary_is_explicit() {
        assert!(!SideEffectClass::S0.requires_authorization());
        assert!(SideEffectClass::S1.requires_authorization());
        assert!(SideEffectClass::S2.requires_authorization());
        assert!(SideEffectClass::S3.requires_authorization());
        assert!(SideEffectClass::S3.is_high_impact());
        assert!(!SideEffectClass::S2.is_high_impact());
    }

    #[test]
    fn unknown_invocation_is_terminal_but_not_success() {
        assert!(InvocationStatus::Unknown.is_terminal());
        assert!(!InvocationStatus::Unknown.is_success());
    }

    #[test]
    fn unknown_outcome_is_not_resolved() {
        assert!(!OutcomeStatus::Unknown.is_resolved());
        assert!(OutcomeStatus::Succeeded.is_resolved());
        assert!(OutcomeStatus::Failed.is_resolved());
    }

    #[test]
    fn contract_primitives_round_trip_through_json() {
        let version = ContractVersion::new(2, 3, 1);
        let json = serde_json::to_string(&version).expect("serialize version");
        let decoded: ContractVersion = serde_json::from_str(&json).expect("deserialize version");
        assert_eq!(decoded, version);
    }
}

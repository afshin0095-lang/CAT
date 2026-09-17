use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ComplianceIssue {
    MissingDisclosure,
    InsufficientProminence,
    UnsubstantiatedClaim,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub id: Uuid,
    pub content: String,
    pub issues: Vec<ComplianceIssue>,
    pub is_compliant: bool,
    pub recommendations: Vec<String>,
}

impl ComplianceCheck {
    pub fn new(content: String) -> Self {
        let mut check = Self {
            id: Uuid::now_v7(),
            content,
            issues: Vec::new(),
            is_compliant: true,
            recommendations: Vec::new(),
        };
        check.validate();
        check
    }
    fn validate(&mut self) {
        let lower = self.content.to_lowercase();
        let disclosure = ["affiliate", "sponsored", "commission", "disclosure", "#ad"]
            .iter()
            .any(|x| lower.contains(x));
        if !disclosure {
            self.issues.push(ComplianceIssue::MissingDisclosure);
            self.recommendations
                .push("Add a clear affiliate disclosure near the beginning".into());
        } else if lower
            .find("affiliate")
            .or_else(|| lower.find("sponsored"))
            .or_else(|| lower.find("#ad"))
            .unwrap_or(0)
            > 100
        {
            self.issues.push(ComplianceIssue::InsufficientProminence);
            self.recommendations
                .push("Move the disclosure into the first 100 characters".into());
        }
        if [
            "guaranteed",
            "100% sure",
            "never fails",
            "always works",
            "earn money fast",
        ]
        .iter()
        .any(|x| lower.contains(x))
        {
            self.issues.push(ComplianceIssue::UnsubstantiatedClaim);
            self.recommendations
                .push("Remove or substantiate absolute claims".into());
        }
        self.is_compliant = self.issues.is_empty();
    }
    pub fn remediate(&mut self, content: String) -> bool {
        self.content = content;
        self.issues.clear();
        self.recommendations.clear();
        self.validate();
        self.is_compliant
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompliancePolicy {
    pub require_disclosure: bool,
    pub disclosure_position_chars: usize,
    pub forbidden_claims: Vec<String>,
}
impl Default for CompliancePolicy {
    fn default() -> Self {
        Self {
            require_disclosure: true,
            disclosure_position_chars: 100,
            forbidden_claims: vec!["guaranteed".into(), "100% sure".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disclosure_is_required() {
        assert!(!ComplianceCheck::new("Buy this now".into()).is_compliant);
    }
    #[test]
    fn compliant_disclosure_passes() {
        assert!(ComplianceCheck::new("#ad Affiliate link: product review".into()).is_compliant);
    }
}

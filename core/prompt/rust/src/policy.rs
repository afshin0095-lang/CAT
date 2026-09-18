use std::collections::BTreeSet;

use crate::{PromptDocument, PromptVariable};

#[derive(Clone, Debug, Default)]
pub struct PromptPolicy {
    allowed_variables: BTreeSet<String>,
    forbidden_sensitive_variables: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptPolicyDecision {
    pub allowed: bool,
    pub blocked_variables: Vec<String>,
}

impl PromptPolicy {
    pub fn new<I, S>(allowed_variables: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        Self {
            allowed_variables: allowed_variables.into_iter().map(Into::into).collect(),
            forbidden_sensitive_variables: true,
        }
    }
    pub fn allow_sensitive_variables(mut self, allowed: bool) -> Self {
        self.forbidden_sensitive_variables = !allowed;
        self
    }
    pub fn evaluate(&self, document: &PromptDocument) -> PromptPolicyDecision {
        let blocked_variables: Vec<String> = document.variables.values()
            .filter(|variable| !self.allowed_variables.contains(&variable.name)
                || (self.forbidden_sensitive_variables && variable.sensitive))
            .map(|variable| variable.name.clone()).collect();
        PromptPolicyDecision { allowed: blocked_variables.is_empty(), blocked_variables }
    }
    pub fn permits(&self, variable: &PromptVariable) -> bool {
        self.allowed_variables.contains(&variable.name)
            && !(self.forbidden_sensitive_variables && variable.sensitive)
    }
}

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::{LlmError, SafetyClass};

/// Stable identifier for a prompt contract.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct PromptId(pub String);

impl PromptId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

/// Immutable prompt revision. Prompt behavior must be reproducible from this
/// identifier plus the versioned template body.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct PromptVersion(pub u32);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub id: PromptId,
    pub version: PromptVersion,
    pub body: String,
    pub required_variables: BTreeSet<String>,
    pub safety: SafetyClass,
}

impl PromptTemplate {
    pub fn new(
        id: PromptId,
        version: PromptVersion,
        body: impl Into<String>,
        required_variables: impl IntoIterator<Item = String>,
        safety: SafetyClass,
    ) -> Result<Self, LlmError> {
        let body = body.into();
        if body.trim().is_empty() {
            return Err(LlmError::InvalidPrompt(
                "prompt body must not be empty".to_owned(),
            ));
        }

        let required_variables: BTreeSet<String> = required_variables.into_iter().collect();
        if required_variables.iter().any(|name| name.trim().is_empty()) {
            return Err(LlmError::InvalidPrompt(
                "prompt variable names must not be empty".to_owned(),
            ));
        }

        Ok(Self {
            id,
            version,
            body,
            required_variables,
            safety,
        })
    }

    /// Deterministic template rendering. Variables are explicit and unresolved
    /// placeholders are rejected instead of silently entering model input.
    pub fn render(&self, variables: &BTreeMap<String, String>) -> Result<RenderedPrompt, LlmError> {
        for variable in &self.required_variables {
            if !variables.contains_key(variable) {
                return Err(LlmError::InvalidPrompt(format!(
                    "missing required prompt variable: {variable}"
                )));
            }
        }

        let mut rendered = self.body.clone();
        for (key, value) in variables {
            let token = format!("{{{{{key}}}}}");
            rendered = rendered.replace(&token, value);
        }

        if contains_placeholder(&rendered) {
            return Err(LlmError::InvalidPrompt(
                "rendered prompt contains unresolved placeholders".to_owned(),
            ));
        }

        Ok(RenderedPrompt {
            id: self.id.clone(),
            version: self.version,
            content: rendered,
            safety: self.safety,
            variables: variables.clone(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RenderedPrompt {
    pub id: PromptId,
    pub version: PromptVersion,
    pub content: String,
    pub safety: SafetyClass,
    /// Captured input variables support reproducibility and evaluation without
    /// allowing prompt rendering itself to mutate canonical domain state.
    pub variables: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default)]
pub struct PromptRegistry {
    templates: BTreeMap<(PromptId, PromptVersion), PromptTemplate>,
}

impl PromptRegistry {
    pub fn register(&mut self, template: PromptTemplate) -> Result<(), LlmError> {
        let key = (template.id.clone(), template.version);
        if self.templates.contains_key(&key) {
            return Err(LlmError::InvalidPrompt(format!(
                "prompt {} version {} is already registered",
                key.0.0, key.1.0
            )));
        }
        self.templates.insert(key, template);
        Ok(())
    }

    pub fn get(&self, id: &PromptId, version: PromptVersion) -> Option<&PromptTemplate> {
        self.templates.get(&(id.clone(), version))
    }

    pub fn latest(&self, id: &PromptId) -> Option<&PromptTemplate> {
        self.templates
            .iter()
            .filter(|((candidate, _), _)| candidate == id)
            .max_by_key(|((_, version), _)| *version)
            .map(|(_, template)| template)
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

fn contains_placeholder(value: &str) -> bool {
    value.contains("{{") || value.contains("}}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template(version: u32) -> PromptTemplate {
        PromptTemplate::new(
            PromptId::new("cat.reasoning.summary"),
            PromptVersion(version),
            "Summarize {{topic}} for {{audience}}.",
            ["topic".to_owned(), "audience".to_owned()],
            SafetyClass::Standard,
        )
        .unwrap()
    }

    #[test]
    fn registry_versions_prompts_without_overwriting_history() {
        let mut registry = PromptRegistry::default();
        registry.register(template(1)).unwrap();
        registry.register(template(2)).unwrap();

        assert_eq!(registry.len(), 2);
        assert_eq!(
            registry
                .latest(&PromptId::new("cat.reasoning.summary"))
                .unwrap()
                .version,
            PromptVersion(2)
        );
    }

    #[test]
    fn rendering_rejects_missing_or_unresolved_variables() {
        let template = template(1);
        let variables = BTreeMap::from([("topic".to_owned(), "CAT".to_owned())]);
        assert!(matches!(
            template.render(&variables),
            Err(LlmError::InvalidPrompt(_))
        ));
    }

    #[test]
    fn rendering_is_deterministic_and_captures_provenance() {
        let template = template(1);
        let variables = BTreeMap::from([
            ("topic".to_owned(), "CAT".to_owned()),
            ("audience".to_owned(), "operators".to_owned()),
        ]);

        let rendered = template.render(&variables).unwrap();
        assert_eq!(rendered.content, "Summarize CAT for operators.");
        assert_eq!(rendered.id, PromptId::new("cat.reasoning.summary"));
        assert_eq!(rendered.version, PromptVersion(1));
        assert_eq!(rendered.variables, variables);
    }
}

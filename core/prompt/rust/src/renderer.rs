use crate::{PromptDocument, PromptError, PromptPolicy, PromptResult};

#[derive(Clone, Debug, Default)]
pub struct PromptRenderer;

impl PromptRenderer {
    pub fn render(document: &PromptDocument, policy: &PromptPolicy) -> PromptResult<String> {
        if document.blocks.is_empty() {
            return Err(PromptError::EmptyDocument);
        }

        let decision = policy.evaluate(document);
        if let Some(variable) = decision.blocked_variables.first() {
            if document.variables[variable].sensitive {
                return Err(PromptError::ForbiddenVariable(variable.clone()));
            }
            return Err(PromptError::ForbiddenVariable(variable.clone()));
        }

        let mut output = String::new();
        for block in &document.blocks {
            if block.content.trim().is_empty() {
                return Err(PromptError::EmptyBlock(block.name.clone()));
            }

            let rendered = Self::substitute(&block.content, document)?;
            output.push_str(&format!("[{}:{}]\n{}\n", role_name(block.role), block.name, rendered));
        }

        Ok(output)
    }

    fn substitute(template: &str, document: &PromptDocument) -> PromptResult<String> {
        let mut result = template.to_owned();
        let mut cursor = 0;
        while let Some(relative) = result[cursor..].find("{{") {
            let start = cursor + relative;
            let Some(end_relative) = result[start + 2..].find("}}") else {
                return Err(PromptError::UnresolvedToken(result[start..].to_owned()));
            };
            let end = start + 2 + end_relative;
            let name = result[start + 2..end].trim();
            let Some(variable) = document.variables.get(name) else {
                return Err(PromptError::MissingVariable(name.to_owned()));
            };
            result.replace_range(start..end + 2, &variable.value);
            cursor = start + variable.value.len();
        }
        Ok(result)
    }
}

fn role_name(role: crate::PromptRole) -> &'static str {
    match role {
        crate::PromptRole::System => "system",
        crate::PromptRole::Developer => "developer",
        crate::PromptRole::User => "user",
        crate::PromptRole::Tool => "tool",
    }
}

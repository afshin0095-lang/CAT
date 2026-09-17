use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptRole { System, Developer, User, Tool }

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromptBlock { pub role: PromptRole, pub name: String, pub content: String }

impl PromptBlock {
    pub fn new(role: PromptRole, name: impl Into<String>, content: impl Into<String>) -> Self {
        Self { role, name: name.into(), content: content.into() }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromptVariable { pub name: String, pub value: String, pub sensitive: bool }

impl PromptVariable {
    pub fn public(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into(), sensitive: false }
    }
    pub fn sensitive(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self { name: name.into(), value: value.into(), sensitive: true }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct PromptDocument {
    pub version: u16,
    pub blocks: Vec<PromptBlock>,
    pub variables: BTreeMap<String, PromptVariable>,
}

impl PromptDocument {
    pub fn new(version: u16) -> Self { Self { version, ..Default::default() } }
    pub fn push_block(&mut self, block: PromptBlock) { self.blocks.push(block); }
    pub fn insert_variable(&mut self, variable: PromptVariable) {
        self.variables.insert(variable.name.clone(), variable);
    }
}

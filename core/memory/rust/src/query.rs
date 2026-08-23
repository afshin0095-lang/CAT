use crate::{Classification, LifecycleState, MemoryKind, MemoryObject};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryQuery<'a> {
    pub namespace: Option<&'a str>,
    pub kind: Option<MemoryKind>,
    pub state: Option<LifecycleState>,
    pub minimum_classification: Option<Classification>,
}

impl<'a> MemoryQuery<'a> {
    pub const fn all() -> Self {
        Self { namespace: None, kind: None, state: None, minimum_classification: None }
    }

    pub const fn namespace(namespace: &'a str) -> Self {
        Self { namespace: Some(namespace), ..Self::all() }
    }

    pub const fn with_kind(mut self, kind: MemoryKind) -> Self { self.kind = Some(kind); self }
    pub const fn with_state(mut self, state: LifecycleState) -> Self { self.state = Some(state); self }
    pub const fn with_minimum_classification(mut self, classification: Classification) -> Self {
        self.minimum_classification = Some(classification); self
    }

    pub fn matches(&self, object: &MemoryObject) -> bool {
        if let Some(namespace) = self.namespace && object.namespace != namespace { return false; }
        if let Some(kind) = self.kind && object.kind != kind { return false; }
        if let Some(state) = self.state && object.state != state { return false; }
        if let Some(minimum) = self.minimum_classification && object.classification < minimum { return false; }
        true
    }
}

pub fn filter<'a, I>(objects: I, query: MemoryQuery<'_>) -> Vec<&'a MemoryObject>
where
    I: IntoIterator<Item = &'a MemoryObject>,
{
    objects.into_iter().filter(|object| query.matches(object)).collect()
}

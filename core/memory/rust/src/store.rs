use std::collections::BTreeMap;

use crate::{MemoryId, MemoryObject, MemoryOperation, MemoryPolicy};

pub trait MemoryStore {
    fn insert(&mut self, object: MemoryObject) -> Result<(), MemoryStoreError>;
    fn get(&self, id: MemoryId) -> Option<&MemoryObject>;
    fn remove(&mut self, id: MemoryId) -> Result<MemoryObject, MemoryStoreError>;
    fn list_namespace(&self, namespace: &str) -> Vec<&MemoryObject>;
}

#[derive(Default)]
pub struct InMemoryMemoryStore {
    objects: BTreeMap<MemoryId, MemoryObject>,
    policy: MemoryPolicy,
    now_ms: u64,
}

impl InMemoryMemoryStore {
    pub fn new(policy: MemoryPolicy, now_ms: u64) -> Self {
        Self {
            objects: BTreeMap::new(),
            policy,
            now_ms,
        }
    }
    pub fn policy(&self) -> MemoryPolicy {
        self.policy
    }
    pub fn set_now_ms(&mut self, now_ms: u64) {
        self.now_ms = now_ms;
    }
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}

impl MemoryStore for InMemoryMemoryStore {
    fn insert(&mut self, object: MemoryObject) -> Result<(), MemoryStoreError> {
        object.validate(self.now_ms)?;
        let operation = if self.objects.contains_key(&object.id) {
            MemoryOperation::Update
        } else {
            MemoryOperation::Create
        };
        self.policy.authorize(&object, operation)?;
        if self.objects.contains_key(&object.id) {
            return Err(MemoryStoreError::AlreadyExists(object.id));
        }
        self.objects.insert(object.id, object);
        Ok(())
    }
    fn get(&self, id: MemoryId) -> Option<&MemoryObject> {
        self.objects.get(&id)
    }
    fn remove(&mut self, id: MemoryId) -> Result<MemoryObject, MemoryStoreError> {
        self.objects
            .remove(&id)
            .ok_or(MemoryStoreError::NotFound(id))
    }
    fn list_namespace(&self, namespace: &str) -> Vec<&MemoryObject> {
        self.objects
            .values()
            .filter(|item| item.namespace == namespace)
            .collect()
    }
}

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
pub enum MemoryStoreError {
    #[error("memory object {0:?} already exists")]
    AlreadyExists(MemoryId),
    #[error("memory object {0:?} was not found")]
    NotFound(MemoryId),
    #[error(transparent)]
    Validation(#[from] crate::MemoryValidationError),
}

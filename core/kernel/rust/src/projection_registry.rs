use std::collections::BTreeMap;

/// Static metadata for a projection known to the runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionDescriptor {
    pub projection_id: String,
    pub version: u16,
    pub description: String,
}

impl ProjectionDescriptor {
    pub fn new(
        projection_id: impl Into<String>,
        version: u16,
        description: impl Into<String>,
    ) -> Self {
        Self {
            projection_id: projection_id.into(),
            version,
            description: description.into(),
        }
    }
}

/// Deterministic registry used by bootstrapping, health checks and migration tooling.
#[derive(Clone, Debug, Default)]
pub struct ProjectionRegistry {
    descriptors: BTreeMap<String, ProjectionDescriptor>,
}

impl ProjectionRegistry {
    pub fn register(&mut self, descriptor: ProjectionDescriptor) -> Result<(), String> {
        if descriptor.projection_id.trim().is_empty() {
            return Err("projection_id must not be empty".into());
        }
        if self.descriptors.contains_key(&descriptor.projection_id) {
            return Err(format!(
                "projection already registered: {}",
                descriptor.projection_id
            ));
        }
        self.descriptors
            .insert(descriptor.projection_id.clone(), descriptor);
        Ok(())
    }

    pub fn get(&self, projection_id: &str) -> Option<&ProjectionDescriptor> {
        self.descriptors.get(projection_id)
    }

    pub fn all(&self) -> impl Iterator<Item = &ProjectionDescriptor> {
        self.descriptors.values()
    }

    pub fn len(&self) -> usize {
        self.descriptors.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_deterministic_and_rejects_duplicates() {
        let mut registry = ProjectionRegistry::default();
        registry
            .register(ProjectionDescriptor::new(
                "affiliate-summary",
                1,
                "Affiliate summary projection",
            ))
            .unwrap();
        assert!(
            registry
                .register(ProjectionDescriptor::new(
                    "affiliate-summary",
                    2,
                    "replacement"
                ))
                .is_err()
        );
        assert_eq!(registry.get("affiliate-summary").unwrap().version, 1);
        assert_eq!(registry.len(), 1);
    }
}

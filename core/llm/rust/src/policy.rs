use crate::{ModelId, ProviderId, SafetyClass};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRoute {
    pub provider: ProviderId,
    pub model: ModelId,
    pub allowed_safety: SafetyClass,
    pub enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutingPolicy {
    pub routes: Vec<ModelRoute>,
    pub default_provider: ProviderId,
    pub default_model: ModelId,
}

impl RoutingPolicy {
    pub fn resolve(&self, requested: Option<&ModelId>, safety: SafetyClass) -> Option<&ModelRoute> {
        let model = requested.unwrap_or(&self.default_model);
        self.routes.iter().find(|route| {
            route.enabled && &route.model == model && safety_allowed(route.allowed_safety, safety)
        })
    }
}

fn safety_allowed(route: SafetyClass, requested: SafetyClass) -> bool {
    let rank = |value| match value {
        SafetyClass::Unclassified => 0,
        SafetyClass::Standard => 1,
        SafetyClass::Sensitive => 2,
        SafetyClass::Restricted => 3,
    };
    rank(requested) <= rank(route)
}

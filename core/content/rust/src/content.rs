use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TemplateType { EmailSequence, BlogPost, ProductReview, VideoScript, SocialPost, Newsletter }

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TemplateMetrics { pub uses: u32, pub conversions: u32, pub avg_ctr: f64, pub avg_conversion_rate: f64 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentTemplate { pub id: String, pub name: String, pub template_type: TemplateType, pub category: String, pub content: String, pub variables: Vec<String>, pub max_uses: Option<u32>, pub performance_metrics: TemplateMetrics }

impl ContentTemplate {
    pub fn new(id: String, name: String, template_type: TemplateType, category: String, content: String, variables: Vec<String>) -> Self { Self { id, name, template_type, category, content, variables, max_uses: None, performance_metrics: TemplateMetrics::default() } }
    pub fn render(&self, vars: &HashMap<String, String>) -> Result<String, String> { let mut out = self.content.clone(); for key in &self.variables { let value = vars.get(key).ok_or_else(|| format!("Missing variable: {key}"))?; out = out.replace(&format!("{{{{{key}}}}}"), value); } Ok(out) }
    pub fn can_use(&self) -> bool { self.max_uses.map(|max| self.performance_metrics.uses < max).unwrap_or(true) }
    pub fn record_use(&mut self, converted: bool, ctr: f64) { self.performance_metrics.uses += 1; if converted { self.performance_metrics.conversions += 1; } let n = self.performance_metrics.uses as f64; self.performance_metrics.avg_ctr = (self.performance_metrics.avg_ctr * (n - 1.0) + ctr) / n; self.performance_metrics.avg_conversion_rate = self.performance_metrics.conversions as f64 / n; }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TemplateLibrary { templates: HashMap<String, ContentTemplate> }
impl TemplateLibrary { pub fn new() -> Self { Self::default() } pub fn add_template(&mut self, template: ContentTemplate) { self.templates.insert(template.id.clone(), template); } pub fn get(&self, id: &str) -> Option<&ContentTemplate> { self.templates.get(id) } pub fn get_mut(&mut self, id: &str) -> Option<&mut ContentTemplate> { self.templates.get_mut(id) } pub fn list_by_type(&self, kind: &TemplateType) -> Vec<&ContentTemplate> { self.templates.values().filter(|t| &t.template_type == kind).collect() } pub fn best_performer(&self, kind: &TemplateType) -> Option<&ContentTemplate> { self.list_by_type(kind).into_iter().max_by(|a,b| a.performance_metrics.avg_conversion_rate.partial_cmp(&b.performance_metrics.avg_conversion_rate).unwrap_or(std::cmp::Ordering::Equal)) } }

#[cfg(test)]
mod tests { use super::*; #[test] fn renders_variables() { let t = ContentTemplate::new("1".into(), "Welcome".into(), TemplateType::EmailSequence, "onboarding".into(), "Hi {{name}}".into(), vec!["name".into()]); let mut vars = HashMap::new(); vars.insert("name".into(), "Afshin".into()); assert_eq!(t.render(&vars).unwrap(), "Hi Afshin"); } #[test] fn rejects_missing_variables() { let t = ContentTemplate::new("1".into(), "Welcome".into(), TemplateType::EmailSequence, "onboarding".into(), "Hi {{name}}".into(), vec!["name".into()]); assert!(t.render(&HashMap::new()).is_err()); } }

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ValidationLevel { Critical, Warning, Info }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValidationResult { pub level: ValidationLevel, pub message: String, pub field: String, pub suggests_fix: Option<String> }
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ValidationGate { pub passed: bool, pub results: Vec<ValidationResult> }
impl ValidationGate { pub fn new() -> Self { Self { passed: true, results: Vec::new() } } pub fn add_result(&mut self, result: ValidationResult) { if result.level == ValidationLevel::Critical { self.passed = false; } self.results.push(result); } pub fn critical_failures(&self) -> Vec<&ValidationResult> { self.results.iter().filter(|r| r.level == ValidationLevel::Critical).collect() } pub fn warnings(&self) -> Vec<&ValidationResult> { self.results.iter().filter(|r| r.level == ValidationLevel::Warning).collect() } }

pub struct ContentValidator;
impl ContentValidator {
    pub fn validate_affiliate_content(content: &str) -> ValidationGate { let mut g = ValidationGate::new(); if content.len() < 50 { g.add_result(ValidationResult { level: ValidationLevel::Critical, message: "Content is too short".into(), field: "content_length".into(), suggests_fix: Some("Write at least 50 characters".into()) }); } let links = content.matches("http").count(); if links == 0 { g.add_result(ValidationResult { level: ValidationLevel::Critical, message: "No affiliate link found".into(), field: "affiliate_link".into(), suggests_fix: Some("Add an affiliate link".into()) }); } if links > 5 { g.add_result(ValidationResult { level: ValidationLevel::Warning, message: "Too many links".into(), field: "link_density".into(), suggests_fix: Some("Prefer 1-3 relevant links".into()) }); } g }
    pub fn validate_email_campaign(subject: &str, body: &str) -> ValidationGate { let mut g = ValidationGate::new(); if subject.is_empty() { g.add_result(ValidationResult { level: ValidationLevel::Critical, message: "Subject is missing".into(), field: "subject".into(), suggests_fix: None }); } if body.len() < 100 { g.add_result(ValidationResult { level: ValidationLevel::Critical, message: "Body is too short".into(), field: "body_length".into(), suggests_fix: None }); } if !["click", "buy", "shop", "learn"].iter().any(|x| body.to_lowercase().contains(x)) { g.add_result(ValidationResult { level: ValidationLevel::Warning, message: "No clear call to action".into(), field: "cta".into(), suggests_fix: Some("Add a clear CTA".into()) }); } g }
    pub fn validate_web_content(content: &str) -> ValidationGate { let mut g = ValidationGate::new(); if !content.contains("<title>") { g.add_result(ValidationResult { level: ValidationLevel::Critical, message: "Missing title tag".into(), field: "seo_title".into(), suggests_fix: None }); } if !content.contains("<h1>") { g.add_result(ValidationResult { level: ValidationLevel::Warning, message: "Missing H1".into(), field: "h1_tag".into(), suggests_fix: None }); } g }
}

#[cfg(test)]
mod tests { use super::*; #[test] fn blocks_short_unlinked_content() { let g = ContentValidator::validate_affiliate_content("Buy now"); assert!(!g.passed); assert_eq!(g.critical_failures().len(), 2); } #[test] fn accepts_valid_web_content() { let g = ContentValidator::validate_web_content("<title>Review</title><h1>Product</h1>"); assert!(g.passed); } }

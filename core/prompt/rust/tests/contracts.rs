use cat_prompt::{
    PromptBlock, PromptDocument, PromptPolicy, PromptRenderer, PromptRole, PromptVariable,
};

fn document() -> PromptDocument {
    let mut document = PromptDocument::new(1);
    document.push_block(PromptBlock::new(
        PromptRole::System,
        "bootstrap",
        "You are CAT. Mission={{mission}}.",
    ));
    document.push_block(PromptBlock::new(
        PromptRole::User,
        "request",
        "Execute task {{task}}.",
    ));
    document.insert_variable(PromptVariable::public(
        "mission",
        "safe autonomous commerce",
    ));
    document.insert_variable(PromptVariable::public("task", "inspect affiliate evidence"));
    document
}

#[test]
fn renderer_is_deterministic_and_preserves_role_order() {
    let document = document();
    let policy = PromptPolicy::new(["mission", "task"]);
    let first = PromptRenderer::render(&document, &policy).unwrap();
    let second = PromptRenderer::render(&document, &policy).unwrap();
    assert_eq!(first, second);
    assert!(first.starts_with("[system:bootstrap]"));
    assert!(first.contains("[user:request]"));
    assert!(first.contains("Mission=safe autonomous commerce"));
}

#[test]
fn forbidden_variable_stops_rendering_before_model_execution() {
    let mut document = document();
    document.insert_variable(PromptVariable::public("secret", "should-not-leak"));
    assert!(PromptRenderer::render(&document, &PromptPolicy::new(["mission", "task"])).is_err());
}

#[test]
fn sensitive_variable_is_denied_by_default() {
    let mut document = document();
    document.insert_variable(PromptVariable::sensitive("api_key", "secret"));
    assert!(
        PromptRenderer::render(
            &document,
            &PromptPolicy::new(["mission", "task", "api_key"])
        )
        .is_err()
    );
}

#[test]
fn missing_variable_is_explicit() {
    let mut document = PromptDocument::new(1);
    document.push_block(PromptBlock::new(PromptRole::User, "request", "{{missing}}"));
    assert!(
        matches!(PromptRenderer::render(&document, &PromptPolicy::new(std::iter::empty::<&str>())),
        Err(cat_prompt::PromptError::ForbiddenVariable(name)) if name == "missing")
    );
}

#[test]
fn sensitive_variables_can_be_explicitly_enabled() {
    let mut document = PromptDocument::new(1);
    document.push_block(PromptBlock::new(
        PromptRole::Tool,
        "credential",
        "{{token}}",
    ));
    document.insert_variable(PromptVariable::sensitive("token", "explicitly-approved"));
    let policy = PromptPolicy::new(["token"]).allow_sensitive_variables(true);
    assert_eq!(
        PromptRenderer::render(&document, &policy).unwrap(),
        "[tool:credential]\nexplicitly-approved\n"
    );
}

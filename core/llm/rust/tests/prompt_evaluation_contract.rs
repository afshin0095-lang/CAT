use std::collections::BTreeMap;

use cat_llm::{
    NonEmptyOutputEvaluator, PromptEvaluator, PromptId, PromptRegistry, PromptTemplate,
    PromptVersion, SafetyClass,
};

#[tokio::test]
async fn prompt_registry_and_evaluation_boundary_preserve_versioned_provenance() {
    let mut registry = PromptRegistry::default();
    let template = PromptTemplate::new(
        PromptId::new("cat.integration.answer"),
        PromptVersion(1),
        "Answer {{question}}.",
        ["question".to_owned()],
        SafetyClass::Standard,
    )
    .unwrap();

    registry.register(template).unwrap();
    let prompt = registry
        .get(&PromptId::new("cat.integration.answer"), PromptVersion(1))
        .unwrap()
        .render(&BTreeMap::from([("question".to_owned(), "What is CAT?".to_owned())]))
        .unwrap();

    let evaluation = NonEmptyOutputEvaluator
        .evaluate(&prompt, "CAT is Commerce AI Trinity.")
        .await
        .unwrap();

    assert!(evaluation.passed);
    assert_eq!(evaluation.prompt_id, prompt.id);
    assert_eq!(evaluation.prompt_version, prompt.version);
    assert_eq!(registry.len(), 1);
}

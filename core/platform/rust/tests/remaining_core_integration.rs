use cat_decision::Alternative;
use cat_llm::{GenerationRequest, Message, ModelId};
use cat_rag::{DocumentChunk, Embedding, RetrievalQuery};
use cat_reasoning::{Evidence, ReasoningMode, ReasoningRequest};
use cat_platform::{CoreCommand, IntegrationContext, IntegrationTarget, RemainingCoreRuntime, TypedCoreCommand};
use serde_json::json;
use uuid::Uuid;

#[test]
fn all_remaining_core_targets_execute_through_one_platform_boundary() {
    let runtime = RemainingCoreRuntime::new();
    let context = IntegrationContext::new("cross-core-test");

    let generation = GenerationRequest::new(
        ModelId::new("local.deterministic"),
        vec![Message::user("integration smoke test")],
    );
    let llm = runtime.execute(TypedCoreCommand::Llm(CoreCommand::new(
        Uuid::now_v7(), "generate", context.clone(), serde_json::to_value(generation).unwrap(),
    ))).unwrap();
    assert_eq!(llm.target, IntegrationTarget::Llm);
    assert!(llm.accepted);

    let reasoning = ReasoningRequest {
        request_id: Uuid::now_v7(),
        objective: "choose supported evidence".into(),
        context: json!({"source":"integration"}),
        evidence: vec![Evidence {
            evidence_id: Uuid::now_v7(),
            source: "integration".into(),
            statement: "supported".into(),
            confidence: 0.9,
            authoritative: true,
            metadata: json!({}),
        }],
        mode: ReasoningMode::Deterministic,
        max_steps: 4,
    };
    let reasoning_response = runtime.execute(TypedCoreCommand::Reasoning(CoreCommand::new(
        Uuid::now_v7(), "reason", context.clone(), serde_json::to_value(reasoning).unwrap(),
    ))).unwrap();
    assert_eq!(reasoning_response.target, IntegrationTarget::Reasoning);
    assert_eq!(reasoning_response.payload["advisory_only"], true);

    let decision = cat_decision::DecisionRequest {
        decision_id: Uuid::now_v7(),
        objective: "select safe alternative".into(),
        alternatives: vec![Alternative {
            id: "safe".into(),
            label: "Safe".into(),
            rationale: "highest confidence".into(),
            expected_value: 0.8,
            confidence: 0.9,
            constraints_satisfied: true,
            metadata: json!({}),
        }],
        required_confidence: 0.7,
        require_human_approval: false,
        context: json!({}),
    };
    let decision_response = runtime.execute(TypedCoreCommand::Decision(CoreCommand::new(
        Uuid::now_v7(), "decide", context.clone(), serde_json::to_value(decision).unwrap(),
    ))).unwrap();
    assert_eq!(decision_response.target, IntegrationTarget::Decision);
    assert_eq!(decision_response.payload["advisory_only"], true);

    let tenant_id = Uuid::now_v7();
    let chunk = DocumentChunk {
        id: Uuid::now_v7(),
        document_id: Uuid::now_v7(),
        tenant_id,
        ordinal: 0,
        text: "affiliate conversion evidence".into(),
        content_hash: "integration-hash".into(),
        metadata: json!({"domain":"affiliate"}),
        embedding: Some(Embedding::new("integration", vec![1.0, 0.0])),
    };
    runtime.execute(TypedCoreCommand::Retrieval(CoreCommand::new(
        Uuid::now_v7(), "upsert", context.clone(), serde_json::to_value(chunk).unwrap(),
    ))).unwrap();

    let query = RetrievalQuery::new(tenant_id, "affiliate")
        .with_embedding(Embedding::new("integration", vec![1.0, 0.0]))
        .with_limit(5);
    let retrieval_response = runtime.execute(TypedCoreCommand::Retrieval(CoreCommand::new(
        Uuid::now_v7(), "retrieve", context, serde_json::to_value(query).unwrap(),
    ))).unwrap();
    assert_eq!(retrieval_response.target, IntegrationTarget::Retrieval);
    assert_eq!(retrieval_response.payload["count"], 1);
}

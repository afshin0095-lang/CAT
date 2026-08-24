use std::sync::{Arc, Mutex};

use cat_decision::{DecisionRequest, DeterministicDecisionEngine};
use cat_llm::{DeterministicProvider, GenerationRequest, LlmProvider};
use cat_rag::{DocumentChunk, InMemoryIndex, RetrievalQuery, Retriever};
use cat_reasoning::{DeterministicReasoningEngine, ReasoningRequest};
use serde_json::{json, Value};
use tokio::runtime::Builder;
use uuid::Uuid;

use crate::{CoreCommand, IntegrationTarget, PlatformError, PlatformResult, TypedCoreCommand, TypedCoreResponse};

/// Concrete composition boundary for the remaining stateless/stateful AI cores.
///
/// Domain semantics remain owned by the target crate. This runtime only converts the platform
/// command envelope into the target core's public request model and converts the result back into
/// a transport-neutral platform response.
#[derive(Clone)]
pub struct RemainingCoreRuntime {
    retrieval: Arc<Mutex<InMemoryIndex>>,
    llm: DeterministicProvider,
    reasoning: DeterministicReasoningEngine,
    decision: DeterministicDecisionEngine,
}

impl Default for RemainingCoreRuntime {
    fn default() -> Self {
        Self {
            retrieval: Arc::new(Mutex::new(InMemoryIndex::default())),
            llm: DeterministicProvider,
            reasoning: DeterministicReasoningEngine::default(),
            decision: DeterministicDecisionEngine::default(),
        }
    }
}

impl RemainingCoreRuntime {
    pub fn new() -> Self { Self::default() }

    pub fn execute(&self, command: TypedCoreCommand) -> PlatformResult<TypedCoreResponse> {
        match command {
            TypedCoreCommand::Llm(command) => self.execute_llm(command),
            TypedCoreCommand::Reasoning(command) => self.execute_reasoning(command),
            TypedCoreCommand::Decision(command) => self.execute_decision(command),
            TypedCoreCommand::Retrieval(command) => self.execute_retrieval(command),
            other => Err(PlatformError::AdapterNotFound(format!(
                "remaining-core runtime does not own {:?}", other.target()
            ))),
        }
    }

    fn execute_llm(&self, command: CoreCommand) -> PlatformResult<TypedCoreResponse> {
        match command.operation.as_str() {
            "generate" => {
                let request: GenerationRequest = serde_json::from_value(command.payload)
                    .map_err(|error| invalid(format!("invalid generation request: {error}")))?;
                let provider = self.llm.clone();
                let response = run_llm(provider, request)
                    .map_err(|error| invalid(format!("LLM provider failed: {error}")))?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Llm,
                    "generate",
                    json!({
                        "request_id": response.request_id,
                        "provider": response.provider,
                        "model": response.model,
                        "content": response.content,
                        "usage": response.usage,
                        "finish_reason": response.finish_reason,
                        "correlation_id": command.context.correlation_id,
                        "causation_id": command.context.causation_id,
                    }),
                ))
            }
            "health" => Ok(response(
                command.context.request_id,
                IntegrationTarget::Llm,
                "health",
                json!({"provider": self.llm.id(), "ready": true, "mode": "deterministic"}),
            )),
            _ => Err(invalid(format!("unsupported LLM operation: {}", command.operation))),
        }
    }

    fn execute_reasoning(&self, command: CoreCommand) -> PlatformResult<TypedCoreResponse> {
        if command.operation != "reason" && command.operation != "explain" {
            return Err(invalid(format!("unsupported reasoning operation: {}", command.operation)));
        }
        let request: ReasoningRequest = serde_json::from_value(command.payload)
            .map_err(|error| invalid(format!("invalid reasoning request: {error}")))?;
        let result = self.reasoning.reason(&request)
            .map_err(|error| invalid(format!("reasoning failed: {error}")))?;
        Ok(response(
            command.context.request_id,
            IntegrationTarget::Reasoning,
            &command.operation,
            json!({
                "reasoning_id": result.reasoning_id,
                "request_id": result.request_id,
                "conclusion": result.conclusion,
                "confidence": result.confidence,
                "hypotheses": result.hypotheses,
                "steps": result.steps,
                "evidence_used": result.evidence_used,
                "assumptions": result.assumptions,
                "advisory_only": true,
                "correlation_id": command.context.correlation_id,
                "causation_id": command.context.causation_id,
            }),
        ))
    }

    fn execute_decision(&self, command: CoreCommand) -> PlatformResult<TypedCoreResponse> {
        if command.operation != "decide" && command.operation != "evaluate_policy" {
            return Err(invalid(format!("unsupported decision operation: {}", command.operation)));
        }
        let request: DecisionRequest = serde_json::from_value(command.payload)
            .map_err(|error| invalid(format!("invalid decision request: {error}")))?;
        let result = self.decision.decide(&request)
            .map_err(|error| invalid(format!("decision failed: {error}")))?;
        Ok(response(
            command.context.request_id,
            IntegrationTarget::Decision,
            &command.operation,
            json!({
                "decision_id": result.decision_id,
                "selected": result.selected,
                "status": result.status,
                "confidence": result.confidence,
                "policy_reasons": result.policy_reasons,
                "advisory_only": true,
                "trace_id": result.trace_id,
                "correlation_id": command.context.correlation_id,
                "causation_id": command.context.causation_id,
            }),
        ))
    }

    fn execute_retrieval(&self, command: CoreCommand) -> PlatformResult<TypedCoreResponse> {
        match command.operation.as_str() {
            "retrieve" | "rank" => {
                let query: RetrievalQuery = serde_json::from_value(command.payload)
                    .map_err(|error| invalid(format!("invalid retrieval query: {error}")))?;
                let index = self.retrieval.lock()
                    .map_err(|_| invalid("retrieval index lock poisoned"))?;
                let hits = index.retrieve(&query)
                    .map_err(|error| invalid(format!("retrieval failed: {error}")))?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Retrieval,
                    &command.operation,
                    json!({
                        "tenant_id": query.tenant_id,
                        "query": query.query,
                        "hits": hits,
                        "count": hits.len(),
                        "correlation_id": command.context.correlation_id,
                        "causation_id": command.context.causation_id,
                    }),
                ))
            }
            "upsert" => {
                let chunk: DocumentChunk = serde_json::from_value(command.payload)
                    .map_err(|error| invalid(format!("invalid document chunk: {error}")))?;
                let chunk_id = chunk.id;
                let mut index = self.retrieval.lock()
                    .map_err(|_| invalid("retrieval index lock poisoned"))?;
                index.upsert(chunk).map_err(|error| invalid(format!("retrieval upsert failed: {error}")))?;
                Ok(response(
                    command.context.request_id,
                    IntegrationTarget::Retrieval,
                    "upsert",
                    json!({"chunk_id": chunk_id, "index_size": index.len()}),
                ))
            }
            _ => Err(invalid(format!("unsupported retrieval operation: {}", command.operation))),
        }
    }
}

fn run_llm(
    provider: DeterministicProvider,
    request: GenerationRequest,
) -> Result<cat_llm::GenerationResponse, cat_llm::LlmError> {
    std::thread::spawn(move || {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| cat_llm::LlmError::ProviderFailure(error.to_string()))?
            .block_on(provider.generate(request))
    })
    .join()
    .map_err(|_| cat_llm::LlmError::ProviderFailure("LLM execution thread panicked".into()))?
}

fn invalid(message: impl Into<String>) -> PlatformError {
    PlatformError::InvalidCommand(message.into())
}

fn response(request_id: Uuid, target: IntegrationTarget, operation: &str, payload: Value) -> TypedCoreResponse {
    TypedCoreResponse {
        request_id,
        target,
        operation: operation.to_owned(),
        accepted: true,
        payload,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cat_decision::Alternative;
    use cat_llm::{GenerationRequest, Message, ModelId};
    use cat_rag::{DocumentChunk, Embedding, RetrievalQuery};
    use cat_reasoning::{Evidence, ReasoningMode};
    use serde_json::json;

    #[test]
    fn llm_generation_is_concretely_wired() {
        let runtime = RemainingCoreRuntime::new();
        let request = GenerationRequest::new(ModelId::new("local.deterministic"), vec![Message::user("hello CAT")]);
        let response = runtime.execute(TypedCoreCommand::Llm(CoreCommand::new(
            Uuid::now_v7(), "generate", IntegrationContext::new("test"), serde_json::to_value(request).unwrap(),
        ))).unwrap();
        assert_eq!(response.target, IntegrationTarget::Llm);
        assert!(response.payload["content"].as_str().unwrap().contains("hello CAT"));
    }

    #[test]
    fn reasoning_is_concretely_wired_and_remains_advisory() {
        let evidence = Evidence { evidence_id: Uuid::now_v7(), source: "test".into(), statement: "candidate-a is supported".into(), confidence: 0.9, authoritative: true, metadata: json!({}) };
        let request = ReasoningRequest { request_id: Uuid::now_v7(), objective: "choose".into(), context: json!({}), evidence: vec![evidence], mode: ReasoningMode::Deterministic, max_steps: 4 };
        let response = RemainingCoreRuntime::new().execute(TypedCoreCommand::Reasoning(CoreCommand::new(
            Uuid::now_v7(), "reason", IntegrationContext::new("test"), serde_json::to_value(request).unwrap(),
        ))).unwrap();
        assert_eq!(response.payload["advisory_only"], true);
    }

    #[test]
    fn decision_is_concretely_wired_and_remains_advisory() {
        let alternative = Alternative { id: "a".into(), label: "A".into(), rationale: "best".into(), expected_value: 0.9, confidence: 0.9, constraints_satisfied: true, metadata: json!({}) };
        let request = DecisionRequest { decision_id: Uuid::now_v7(), objective: "choose".into(), alternatives: vec![alternative], required_confidence: 0.7, require_human_approval: false, context: json!({}) };
        let response = RemainingCoreRuntime::new().execute(TypedCoreCommand::Decision(CoreCommand::new(
            Uuid::now_v7(), "decide", IntegrationContext::new("test"), serde_json::to_value(request).unwrap(),
        ))).unwrap();
        assert_eq!(response.payload["advisory_only"], true);
    }

    #[test]
    fn retrieval_is_concretely_wired_and_tenant_scoped() {
        let tenant = Uuid::now_v7();
        let chunk = DocumentChunk { id: Uuid::now_v7(), document_id: Uuid::now_v7(), tenant_id: tenant, ordinal: 0, text: "affiliate truth".into(), content_hash: "h".into(), metadata: json!({}), embedding: Some(Embedding::new("test", vec![1.0, 0.0])) };
        let runtime = RemainingCoreRuntime::new();
        runtime.execute(TypedCoreCommand::Retrieval(CoreCommand::new(Uuid::now_v7(), "upsert", IntegrationContext::new("test"), serde_json::to_value(chunk).unwrap()))).unwrap();
        let query = RetrievalQuery::new(tenant, "affiliate").with_embedding(Embedding::new("test", vec![1.0, 0.0])).with_limit(1);
        let response = runtime.execute(TypedCoreCommand::Retrieval(CoreCommand::new(Uuid::now_v7(), "retrieve", IntegrationContext::new("test"), serde_json::to_value(query).unwrap()))).unwrap();
        assert_eq!(response.payload["count"], 1);
    }
}

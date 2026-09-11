use std::pin::Pin;

use futures_core::Stream;

use crate::{GenerationChunk, LlmError};

/// Provider-neutral stream returned by an LLM provider.
///
/// The stream carries derived output only. It does not establish canonical
/// business truth and must be validated by the consuming engine before use.
pub type LlmGenerationStream =
    Pin<Box<dyn Stream<Item = Result<GenerationChunk, LlmError>> + Send>>;

/// Collects a stream into the canonical response text while preserving the
/// first terminal metadata and usage values supplied by the provider.
pub async fn collect_stream(mut stream: LlmGenerationStream) -> Result<String, LlmError> {
    use futures_util::StreamExt;

    let mut content = String::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        content.push_str(&chunk.delta);
    }
    Ok(content)
}

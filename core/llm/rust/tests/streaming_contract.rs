use cat_llm::{
    DeterministicProvider, GenerationRequest, LlmProvider, Message, ModelId, collect_stream,
};

#[tokio::test]
async fn deterministic_provider_emits_non_empty_delta_chunks_and_one_terminal_chunk() {
    let provider = DeterministicProvider;
    let request = GenerationRequest::new(
        ModelId::new("deterministic-v1"),
        vec![Message::user("stream contract")],
    );

    let mut stream = provider.generate_stream(request.clone()).await.unwrap();
    let mut chunks = Vec::new();
    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        chunks.push(chunk.unwrap());
    }

    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|chunk| !chunk.delta.is_empty()));
    assert_eq!(
        chunks
            .iter()
            .filter(|chunk| chunk.finish_reason.is_some())
            .count(),
        1
    );
    assert!(chunks.last().unwrap().usage.is_some());
    assert!(
        chunks
            .iter()
            .all(|chunk| chunk.request_id == request.request_id)
    );
}

#[tokio::test]
async fn collected_stream_matches_normal_generation() {
    let provider = DeterministicProvider;
    let request = GenerationRequest::new(
        ModelId::new("deterministic-v1"),
        vec![Message::user("same output")],
    );

    let expected = provider.generate(request.clone()).await.unwrap().content;
    let stream = provider.generate_stream(request).await.unwrap();
    let actual = collect_stream(stream).await.unwrap();

    assert_eq!(actual, expected);
}

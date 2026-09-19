use reqwest::Client;
use serde_json::Value;

use crate::LlmError;

#[derive(Clone)]
pub(crate) struct HttpLlmClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl HttpLlmClient {
    pub(crate) fn new(
        base_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Result<Self, LlmError> {
        let base_url = base_url.into().trim_end_matches('/').to_owned();
        let api_key = api_key.into();
        if base_url.is_empty() {
            return Err(LlmError::InvalidRequest(
                "LLM base URL cannot be empty".to_owned(),
            ));
        }
        if api_key.is_empty() {
            return Err(LlmError::InvalidRequest(
                "LLM API key cannot be empty".to_owned(),
            ));
        }
        Ok(Self {
            client: Client::new(),
            base_url,
            api_key,
        })
    }

    pub(crate) fn url(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    pub(crate) async fn post_json(&self, path: &str, body: Value) -> Result<Value, LlmError> {
        self.post_json_with(|request| request.bearer_auth(&self.api_key), path, body)
            .await
    }

    pub(crate) async fn post_anthropic(&self, path: &str, body: Value) -> Result<Value, LlmError> {
        self.post_json_with(
            |request| {
                request
                    .header("x-api-key", &self.api_key)
                    .header("anthropic-version", "2023-06-01")
            },
            path,
            body,
        )
        .await
    }

    async fn post_json_with<F>(
        &self,
        apply_auth: F,
        path: &str,
        body: Value,
    ) -> Result<Value, LlmError>
    where
        F: FnOnce(reqwest::RequestBuilder) -> reqwest::RequestBuilder,
    {
        let response = apply_auth(self.client.post(self.url(path)))
            .json(&body)
            .send()
            .await
            .map_err(|error| LlmError::ProviderFailure(error.to_string()))?;

        let status = response.status();
        let body = response
            .json::<Value>()
            .await
            .map_err(|error| LlmError::ProviderFailure(error.to_string()))?;

        if !status.is_success() {
            return Err(LlmError::ProviderRejected(format!(
                "HTTP {}: {}",
                status, body
            )));
        }
        Ok(body)
    }
}

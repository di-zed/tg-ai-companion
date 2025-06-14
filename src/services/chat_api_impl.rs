use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use serde_json::{json, Value};
use std::env;

use crate::services::chat_api::ChatApi;

/// `RealChatApi` is a concrete implementation of the [`ChatApi`] trait that uses
/// an OpenAI-compatible REST API (e.g., OpenAI, LocalAI).
///
/// It builds requests using `reqwest`, formats the body according to
/// the Chat Completions API, and parses the returned assistant message.
///
/// Environment variables used:
/// - `OPEN_AI_URL` — Base URL of the API (e.g. `http://localhost:8080` or `https://api.openai.com`)
/// - `OPEN_AI_MODEL` — Model name (e.g. `gpt-3.5-turbo`, `mistral`)
/// - `OPEN_AI_API_KEY` — Optional API key (required for OpenAI)
pub struct RealChatApi {
    client: Client,
    base_url: String,
    model: String,
    api_key: Option<String>,
}

impl RealChatApi {
    /// Constructs a new [`RealChatApi`] instance using environment variables.
    ///
    /// # Returns
    /// - `Ok(Self)` — if required environment variables are present
    /// - `Err(_)` — if `OPEN_AI_URL` or `OPEN_AI_MODEL` is missing
    ///
    /// # Errors
    /// Returns an error if `OPEN_AI_URL` or `OPEN_AI_MODEL` environment variables are not set.
    pub fn new_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let base_url: String = env::var("OPEN_AI_URL")?;
        let model: String = env::var("OPEN_AI_MODEL")?;
        let api_key: Option<String> = env::var("OPEN_AI_API_KEY").ok();

        Ok(Self {
            client: Client::new(),
            base_url,
            model,
            api_key,
        })
    }
}

#[async_trait]
impl ChatApi for RealChatApi {
    /// Sends a chat completion request to an OpenAI-compatible API endpoint.
    ///
    /// # Arguments
    ///
    /// * `prompt` - A user-provided string to be sent as input to the assistant.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` — the assistant's response
    /// * `Err(Box<dyn Error>)` — if request fails or response format is unexpected
    ///
    /// # Example
    ///
    /// ```
    /// let api = RealChatApi::new_from_env()?;
    /// let reply = api.call_chat_api("Hello!").await?;
    /// println!("{}", reply);
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// - The HTTP request fails (e.g., timeout, connection error)
    /// - The response is not in the expected format
    /// - `"choices[0].message.content"` is missing or not a string
    async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let body: Value = json!({
            "model": self.model,
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let url: String = format!(
            "{}/v1/chat/completions",
            self.base_url.trim_end_matches('/')
        );

        let mut request: RequestBuilder = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body);

        if let Some(key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }

        let response: Response = request.send().await?;
        let json: Value = response.json().await?;

        let content = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Missing content in the response!")?
            .to_string();

        Ok(content)
    }
}

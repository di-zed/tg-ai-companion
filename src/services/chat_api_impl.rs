use crate::services::chat_api::ChatApi;
use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use serde_json::{json, Value};
use std::env;
use std::error::Error;

/// `RealChatApi` is a concrete implementation of the [`ChatApi`] trait
/// that uses an OpenAI-compatible REST API (e.g., OpenAI, LocalAI).
///
/// It constructs requests using `reqwest`, formats the request body
/// according to the Chat Completions API spec, and parses the returned
/// assistant message.
///
/// Environment variables used:
/// - `OPEN_AI_URL` — base URL of the API (e.g. `http://localhost:8080` or `https://api.openai.com`)
/// - `OPEN_AI_MODEL` — model name (e.g. `gpt-3.5-turbo`, `mistral`)
/// - `OPEN_AI_API_KEY` — optional API key (required for OpenAI)
pub struct RealChatApi {
    client: Client,
    base_url: String,
    model: String,
    api_key: Option<String>,
}

impl RealChatApi {
    /// Creates a new instance of [`RealChatApi`] from environment variables.
    ///
    /// Requires the following environment variables to be set and non-empty:
    /// - `OPEN_AI_URL` — the base URL of the API
    /// - `OPEN_AI_MODEL` — the model name to use
    /// - `OPEN_AI_API_KEY` — (optional) API key for authorization
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if all required variables are set correctly.
    /// - `Err` if any required environment variable is missing or empty.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tg_ai_companion::services::chat_api_impl::RealChatApi;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let api = RealChatApi::new_from_env()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new_from_env() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let base_url = env::var("OPEN_AI_URL")
            .map_err(|_| "Environment variable OPEN_AI_URL is not set or empty")?;
        if base_url.trim().is_empty() {
            return Err("Environment variable OPEN_AI_URL cannot be empty".into());
        }

        let model = env::var("OPEN_AI_MODEL")
            .map_err(|_| "Environment variable OPEN_AI_MODEL is not set or empty")?;
        if model.trim().is_empty() {
            return Err("Environment variable OPEN_AI_MODEL cannot be empty".into());
        }

        let api_key = env::var("OPEN_AI_API_KEY").ok();

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
    /// * `prompt` — user input string to be sent to the assistant.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` — assistant’s response.
    /// * `Err(Box<dyn Error + Send + Sync>)` — if request fails or response format is invalid.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tg_ai_companion::services::chat_api_impl::RealChatApi;
    /// use tg_ai_companion::services::chat_api::ChatApi;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    ///     let api = RealChatApi::new_from_env()?;
    ///     let reply = api.call_chat_api("Hello!").await?;
    ///     println!("{}", reply);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The HTTP request fails (e.g., timeout, connection error).
    /// - The response does not contain expected fields.
    /// - `"choices[0].message.content"` is missing or not a string.
    async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let body: Value = json!({
            "model": self.model,
            "messages": [
                { "role": "user", "content": prompt }
            ]
        });

        let url = format!(
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

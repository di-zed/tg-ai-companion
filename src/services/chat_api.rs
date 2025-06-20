use async_trait::async_trait;
use std::error::Error;

/// Defines the interface for a chat-based language model API (e.g., OpenAI, LocalAI).
///
/// This trait allows consumers to abstract over different backend implementations
/// (e.g., real HTTP clients, mocks for testing).
///
/// Any implementation must be thread-safe (`Send + Sync`) and provide an asynchronous
/// method for sending prompts and receiving model-generated responses.
#[async_trait]
pub trait ChatApi: Send + Sync {
    /// Sends a prompt to a chat API and returns the assistant's response.
    ///
    /// # Arguments
    ///
    /// * `prompt` - The user's input message or question to be sent to the model.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` — The model's response as a plain string.
    /// * `Err(Box<dyn std::error::Error + Send + Sync>)` — If the API call or response parsing fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_trait::async_trait;
    /// use std::error::Error;
    ///
    /// #[async_trait]
    /// trait ChatApi: Send + Sync {
    ///     async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
    /// }
    ///
    /// struct DummyApi;
    ///
    /// #[async_trait]
    /// impl ChatApi for DummyApi {
    ///     async fn call_chat_api(&self, _prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    ///         Ok("Dummy response".to_string())
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    ///     let chat_api = DummyApi;
    ///     let response = chat_api.call_chat_api("What's the weather today?").await?;
    ///     println!("Model response: {}", response);
    ///     Ok(())
    /// }
    /// ```
    async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
}

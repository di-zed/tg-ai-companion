use async_trait::async_trait;

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
    /// * `Err(Box<dyn std::error::Error>)` — If the API call or response parsing fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use async_trait::async_trait;
    ///
    /// #[async_trait]
    /// trait ChatApi {
    ///     async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    /// }
    ///
    /// struct DummyApi;
    ///
    /// #[async_trait]
    /// impl ChatApi for DummyApi {
    ///     async fn call_chat_api(&self, _prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    ///         Ok("Dummy response".to_string())
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let chat_api = DummyApi;
    ///     let response = chat_api.call_chat_api("What's the weather today?").await?;
    ///     println!("Model response: {}", response);
    ///     Ok(())
    /// }
    /// ```
    async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}

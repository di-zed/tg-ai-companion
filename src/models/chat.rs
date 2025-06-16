use serde::Deserialize;

/// Represents a request payload for the chat API endpoint.
///
/// This struct is used to deserialize incoming JSON data containing
/// the user's prompt message for chat completion.
///
/// # Fields
///
/// * `prompt` – The user-provided input that will be sent to the chat model.
///
/// # Example
///
/// ```json
/// {
///   "prompt": "Tell me a joke."
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub prompt: String,
}

use tg_ai_companion::services::chat_api::ChatApi;
use tg_ai_companion::services::chat_api_impl::RealChatApi;

/// Integration test for the `RealChatApi::call_chat_api` method.
///
/// This test sends a sample prompt to the real API endpoint configured via environment variables
/// and asserts that a non-empty response is received.
///
/// **Note:**
/// - Requires environment variables `OPEN_AI_URL` and `OPEN_AI_MODEL` to be set in the `.env` file or environment.
/// - Optionally, `OPEN_AI_API_KEY` should be set if the API requires authentication.
/// - This test performs a real network request and is therefore an integration test, not a unit test.
///
/// # Environment variables used:
/// - `OPEN_AI_URL` — Base URL of the OpenAI-compatible API (e.g., `https://api.openai.com`)
/// - `OPEN_AI_MODEL` — Model name (e.g., `gpt-3.5-turbo`)
/// - `OPEN_AI_API_KEY` — Optional API key for authorization
///
/// # Example
/// ```ignore
/// cargo test -- --nocapture
/// ```
///
/// # Errors
/// Return an error if:
/// - Environment variables are missing or invalid
/// - Network request fails
/// - A Response JSON format is unexpected
#[tokio::test]
async fn test_call_chat_api() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let api = RealChatApi::new_from_env()?;

    let prompt = "Integration test: say hello";
    let response = api.call_chat_api(prompt).await?;

    println!("API response: {}", response);
    assert!(!response.trim().is_empty());

    Ok(())
}

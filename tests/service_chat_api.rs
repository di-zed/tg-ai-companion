use std::error::Error;

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
///
/// # Example
/// ```ignore
/// cargo test -- --nocapture
/// ```
#[tokio::test]
async fn test_call_chat_api() -> Result<(), Box<dyn Error + Send + Sync>> {
    match dotenv::dotenv() {
        Ok(_) => println!(".env loaded"),
        Err(_) => println!("No .env file found, using environment"),
    }

    let api = RealChatApi::new_from_env()?;

    let prompt = "Integration test: say hello";
    let response = api.call_chat_api(prompt).await?;

    println!("API response: {}", response);

    assert!(
        !response.trim().is_empty(),
        "API response should not be empty"
    );
    assert!(
        response.to_lowercase().contains("hello"),
        "API response should contain 'hello'"
    );

    Ok(())
}

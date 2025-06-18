use reqwest::Client;
use std::env;

use crate::models::telegram::SendMessageRequest;

/// Sends a message to a Telegram chat using a custom base URL and bot token.
///
/// This function is useful for integration testing, allowing you to pass
/// a mock server's base URL instead of the real Telegram API URL. It constructs
/// the full API endpoint using the provided `base_url` and `token`, then sends
/// a POST request with the given `chat_id` and `text`.
///
/// # Arguments
///
/// * `base_url` - The base URL of the Telegram API (e.g., `"https://api.telegram.org"` or mock URL).
/// * `token` - The bot token for authentication.
/// * `chat_id` - The unique identifier of the target Telegram chat.
/// * `text` - The message content to send.
///
/// # Returns
///
/// * `Ok(())` if the message was sent successfully.
/// * `Err(String)` with an error message if the HTTP request failed or the response was not successful.
pub async fn send_telegram_message_custom(
    base_url: &str,
    token: &str,
    chat_id: i64,
    text: String,
) -> Result<(), String> {
    let url = format!("{}/bot{}/sendMessage", base_url, token);

    let message = SendMessageRequest { chat_id, text };

    let client = Client::new();
    let response = client
        .post(&url)
        .json(&message)
        .send()
        .await
        .map_err(|e| format!("HTTP error: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        Err(format!("Telegram API error {}: {}", status, body))
    }
}

/// Sends a message to a Telegram chat using the Telegram Bot API.
///
/// # Arguments
///
/// * `chat_id` – Telegram chat ID
/// * `text` – Text of the message to send
///
/// # Returns
///
/// * `Ok(())` – if the message was sent successfully
/// * `Err(String)` – if there was an error
pub async fn send_telegram_message(chat_id: i64, text: String) -> Result<(), String> {
    let base_url = env::var("TELEGRAM_API_BASE_URL")
        .map_err(|_| "Environment variable TELEGRAM_API_BASE_URL is not set or empty")?;
    if base_url.trim().is_empty() {
        return Err("Environment variable TELEGRAM_API_BASE_URL cannot be empty".into());
    }

    let token = env::var("TELEGRAM_BOT_TOKEN")
        .map_err(|_| "Environment variable TELEGRAM_BOT_TOKEN is not set or empty")?;
    if token.trim().is_empty() {
        return Err("Environment variable TELEGRAM_BOT_TOKEN cannot be empty".into());
    }

    send_telegram_message_custom(&base_url, &token, chat_id, text).await
}

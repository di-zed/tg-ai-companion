use actix_web::{web, HttpResponse, Responder};

use crate::models::telegram::TelegramUpdate;
use crate::services::chat_api::ChatApi;
use crate::services::telegram_api::TelegramApi;

/// Handles incoming Telegram webhook updates.
///
/// This function processes an incoming Telegram update, extracts the chat ID and message text,
/// sends the prompt to the AI chat API, and responds with the AI-generated text via the Telegram Bot API.
///
/// # Arguments
///
/// * `update` - The deserialized Telegram update received via webhook.
/// * `chat_api` - An implementation of the `ChatApi` trait used to get the AI-generated response.
/// * `telegram_api` - An implementation of the `TelegramApi` trait used to send the message back to Telegram.
///
/// # Returns
///
/// An `impl Responder` representing the HTTP response:
/// - `200 OK` with `"Message sent"` on success.
/// - `400 Bad Request` if the message text is missing or empty.
/// - `500 Internal Server Error` if the chat API or Telegram API calls fail.
///
/// # Example
///
/// ```json
/// {
///   "update_id": 123456789,
///   "message": {
///     "message_id": 1,
///     "chat": {
///       "id": 987654321,
///       "type": "private"
///     },
///     "text": "Hello bot"
///   }
/// }
/// ```
pub async fn telegram_webhook(
    update: web::Json<TelegramUpdate>,
    chat_api: web::Data<dyn ChatApi>,
    telegram_api: web::Data<dyn TelegramApi>,
) -> impl Responder {
    let (chat_id, prompt) = match update
        .message
        .as_ref()
        .and_then(|m| Some((m.chat.id, m.text.as_ref()?)))
    {
        Some((chat_id, text)) if !text.trim().is_empty() => (chat_id, text.clone()),
        _ => return HttpResponse::BadRequest().body("No Message Text"),
    };

    let response_text = match chat_api.call_chat_api(&prompt).await {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error calling chat API: {}", e);
            return HttpResponse::InternalServerError().body("Error calling chat API");
        }
    };

    match telegram_api
        .send_telegram_message(chat_id, response_text)
        .await
    {
        Ok(()) => HttpResponse::Ok().body("Message sent"),
        Err(e) => {
            eprintln!("Error sending to Telegram: {}", e);
            HttpResponse::InternalServerError().body("Failed to send message to Telegram")
        }
    }
}

use actix_web::{web, HttpResponse, Responder};

use crate::models::telegram::TelegramUpdate;
use crate::services::chat_api::ChatApi;
use crate::services::telegram::send_telegram_message;

/// Handles incoming Telegram webhook updates.
///
/// This endpoint expects a JSON body containing a Telegram update with a message.
/// It extracts the text from the message, passes it to a chat API (e.g., LocalAI or OpenAI),
/// and returns the generated response.
///
/// # Arguments
///
/// * `update` - JSON body containing a Telegram update (typically from a bot webhook).
/// * `chat_api` - A shared, thread-safe instance of an object that implements the `ChatApi` trait,
/// injected via Actix Web's data mechanism.
///
/// # Returns
///
/// * `200 OK` with the model-generated response if successful.
/// * `400 Bad Request` if there is no message text in the update.
/// * `500 Internal Server Error` if the chat API call fails.
///
/// # Telegram docs
///
/// https://core.telegram.org/bots/api#update
pub async fn telegram_webhook(
    update: web::Json<TelegramUpdate>,
    chat_api: web::Data<dyn ChatApi>,
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

    match send_telegram_message(chat_id, response_text).await {
        Ok(()) => HttpResponse::Ok().body("Message sent"),
        Err(e) => {
            eprintln!("Error sending to Telegram: {}", e);
            HttpResponse::InternalServerError().body("Failed to send message to Telegram")
        }
    }
}

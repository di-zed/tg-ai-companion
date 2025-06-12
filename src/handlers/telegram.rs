use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

/// Represents a message from Telegram.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#message
#[derive(Debug, Deserialize)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub text: Option<String>,
}

/// Represents an incoming update from Telegram.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#update
#[derive(Debug, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: Option<TelegramMessage>,
}

/// Handles incoming webhook updates from Telegram.
///
/// This endpoint receives `POST` requests from Telegram containing update data,
/// such as new messages. It logs the content of the incoming update and optionally
/// processes message text if present.
///
/// # Arguments
///
/// * `update` - A JSON body containing the Telegram update (`TelegramUpdate`),
///   deserialized automatically by Actix.
///
/// # Returns
///
/// * `200 OK` response to acknowledge successful receipt to Telegram.
///
/// # Telegram docs
///
/// https://core.telegram.org/bots/api#update
pub async fn telegram_webhook(update: web::Json<TelegramUpdate>) -> impl Responder {
    println!("Received an update from Telegram: {:?}", update);

    if let Some(message) = &update.message {
        if let Some(text) = &message.text {
            println!("User wrote: {}", text);
        }
    }

    HttpResponse::Ok()
}

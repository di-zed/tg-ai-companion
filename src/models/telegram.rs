use serde::Deserialize;

/// Represents a message from Telegram.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#message
#[derive(Debug, Deserialize)]
pub struct TelegramMessage {
    pub message_id: Option<i64>,
    pub text: Option<String>,
}

/// Represents an incoming update from Telegram.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#update
#[derive(Debug, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: Option<i64>,
    pub message: Option<TelegramMessage>,
}

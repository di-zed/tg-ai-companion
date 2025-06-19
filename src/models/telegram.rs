use serde::{Deserialize, Serialize};

/// Represents a Telegram chat.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#chat
#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramChat {
    pub id: i64,
}

/// Represents a message from Telegram.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#message
#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub chat: TelegramChat,
    pub text: Option<String>,
}

/// Represents an incoming update from Telegram.
///
/// Details in the Telegram API documentation:
/// https://core.telegram.org/bots/api#update
#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramUpdate {
    pub update_id: i64,
    pub message: Option<TelegramMessage>,
}

/// Represents a request to send a message via the Telegram Bot API.
///
/// This struct is serialized into JSON and sent in a POST request
/// to the Telegram `sendMessage` endpoint.
///
/// # Fields
/// - `chat_id`: Unique identifier for the target chat. This ID is provided in each incoming Telegram update.
/// - `text`: The message text to be sent to the specified chat.
///
/// # Example
/// ```rust
/// use tg_ai_companion::models::telegram::SendMessageRequest;
///
/// let request = SendMessageRequest {
///     chat_id: 123456789,
///     text: "Hello, Telegram!".to_string(),
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub chat_id: i64,
    pub text: String,
}

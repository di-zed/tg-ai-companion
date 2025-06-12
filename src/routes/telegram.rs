use actix_web::{web, Scope};

use crate::handlers::telegram::telegram_webhook;

/// Initializes all Telegram-related routes.
pub fn init_telegram_routes() -> Scope {
    web::scope("/telegram").route("/webhook", web::post().to(telegram_webhook))
}

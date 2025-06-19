use actix_web::{web, Scope};
use std::sync::Arc;

use crate::handlers::telegram::telegram_webhook;
use crate::services::chat_api::ChatApi;
use crate::services::chat_api_impl::RealChatApi;
use crate::services::telegram_api::TelegramApi;
use crate::services::telegram_api_impl::RealTelegramApi;

/// Initializes all Telegram-related routes.
pub fn init_telegram_routes() -> Scope {
    let real_chat_api: RealChatApi =
        RealChatApi::new_from_env().expect("Failed to initialize Chat API");
    let chat_api: Arc<dyn ChatApi> = Arc::new(real_chat_api);
    let chat_api_data: web::Data<dyn ChatApi> = web::Data::from(chat_api);

    let real_telegram_api: RealTelegramApi =
        RealTelegramApi::new_from_env().expect("Failed to initialize Telegram API");
    let telegram_api: Arc<dyn TelegramApi> = Arc::new(real_telegram_api);
    let telegram_api_data: web::Data<dyn TelegramApi> = web::Data::from(telegram_api);

    web::scope("/telegram")
        .app_data(chat_api_data)
        .app_data(telegram_api_data)
        .route("/webhook", web::post().to(telegram_webhook))
}

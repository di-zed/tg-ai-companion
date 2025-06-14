use actix_web::{web, Scope};
use std::sync::Arc;

use crate::handlers::telegram::telegram_webhook;
use crate::services::chat_api::ChatApi;
use crate::services::chat_api_impl::RealChatApi;

/// Initializes all Telegram-related routes.
pub fn init_telegram_routes() -> Scope {
    let real_api: RealChatApi =
        RealChatApi::new_from_env().expect("Failed to initialize Telegram API");

    let chat_api: Arc<dyn ChatApi> = Arc::new(real_api);
    let chat_api_data: web::Data<dyn ChatApi> = web::Data::from(chat_api);

    web::scope("/telegram")
        .app_data(chat_api_data)
        .route("/webhook", web::post().to(telegram_webhook))
}

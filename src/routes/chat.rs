use actix_web::{web, Scope};
use std::sync::Arc;

use crate::handlers::chat::chat_endpoint;
use crate::services::chat_api::ChatApi;
use crate::services::chat_api_impl::RealChatApi;

/// Initializes all Chat-related routes.
pub fn init_chat_routes() -> Scope {
    let real_api: RealChatApi = RealChatApi::new_from_env().expect("Failed to initialize Chat API");

    let chat_api: Arc<dyn ChatApi> = Arc::new(real_api);
    let chat_api_data: web::Data<dyn ChatApi> = web::Data::from(chat_api);

    web::scope("/chat")
        .app_data(chat_api_data)
        .route("", web::post().to(chat_endpoint))
}

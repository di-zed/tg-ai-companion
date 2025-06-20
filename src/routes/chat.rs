use crate::handlers::chat::chat_endpoint;
use crate::middleware::auth::validator;
use crate::services::chat_api::ChatApi;
use crate::services::chat_api_impl::RealChatApi;
use actix_web::dev;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::sync::Arc;

/// Initializes all Chat-related routes.
pub fn init_chat_routes() -> impl dev::HttpServiceFactory {
    let auth = HttpAuthentication::with_fn(validator);

    let real_api: RealChatApi = RealChatApi::new_from_env().expect("Failed to initialize Chat API");

    let chat_api: Arc<dyn ChatApi> = Arc::new(real_api);
    let chat_api_data: web::Data<dyn ChatApi> = web::Data::from(chat_api);

    web::scope("/chat")
        .wrap(auth)
        .app_data(chat_api_data)
        .route("", web::post().to(chat_endpoint))
}

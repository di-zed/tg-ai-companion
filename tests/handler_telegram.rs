use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

use tg_ai_companion::handlers::telegram::telegram_webhook;
use tg_ai_companion::models::telegram::{TelegramChat, TelegramMessage, TelegramUpdate};
use tg_ai_companion::services::chat_api::ChatApi;
use tg_ai_companion::services::telegram_api::TelegramApi;

/// Mock implementation of ChatApi for testing.
/// Simply echoes back the prompt prefixed with "Echo:".
struct MockChatApi;

#[async_trait]
impl ChatApi for MockChatApi {
    async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(format!("Echo: {}", prompt))
    }
}

/// Mock implementation of TelegramApi for testing.
/// Asserts that the message sent matches expected chat ID and text.
struct MockTelegramApi;

#[async_trait]
impl TelegramApi for MockTelegramApi {
    async fn send_telegram_message(&self, chat_id: i64, text: String) -> Result<(), String> {
        assert_eq!(chat_id, 987654321);
        assert_eq!(text, "Echo: Hello bot");
        Ok(())
    }
}

/// Integration test for the Telegram webhook handler.
///
/// This test verifies that:
/// - The handler accepts a valid Telegram update JSON payload,
/// - Returns HTTP 200 OK with body "Processing",
/// - Internally calls the mocked Chat API and Telegram API (asserted inside mocks).
#[actix_web::test]
async fn test_telegram_webhook_success() {
    // Wrap mocks in Arc and web::Data for dependency injection
    let chat_api: web::Data<dyn ChatApi> =
        web::Data::from(Arc::new(MockChatApi) as Arc<dyn ChatApi>);
    let telegram_api: web::Data<dyn TelegramApi> =
        web::Data::from(Arc::new(MockTelegramApi) as Arc<dyn TelegramApi>);

    // Initialize Actix app with injected dependencies and route
    let app = test::init_service(
        App::new()
            .app_data(chat_api.clone())
            .app_data(telegram_api.clone())
            .route("/webhook", web::post().to(telegram_webhook)),
    )
    .await;

    // Prepare a sample Telegram update with message text
    let update = TelegramUpdate {
        update_id: 123456789,
        message: Some(TelegramMessage {
            message_id: 1,
            chat: TelegramChat { id: 987654321 },
            text: Some("Hello bot".to_string()),
        }),
    };

    // Build POST request with JSON body
    let req = test::TestRequest::post()
        .uri("/webhook")
        .set_json(&update)
        .to_request();

    // Send request and get response
    let resp = test::call_service(&app, req).await;

    // Assert HTTP status is 200 OK
    assert_eq!(resp.status(), StatusCode::OK);

    // Read the response body and assert it equals "Processing"
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert_eq!(body_str, "Processing");
}

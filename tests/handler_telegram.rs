use actix_web::{http::StatusCode, test};
use actix_web::{web, App};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

use tg_ai_companion::handlers::telegram::telegram_webhook;
use tg_ai_companion::models::telegram::{TelegramChat, TelegramMessage, TelegramUpdate};
use tg_ai_companion::services::chat_api::ChatApi;
use tg_ai_companion::services::telegram_api::TelegramApi;

/// Mock implementation of the ChatApi trait for unit testing.
///
/// This mock simulates a simple chat API that echoes the input prompt.
struct MockChatApi;

#[async_trait]
impl ChatApi for MockChatApi {
    async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        Ok(format!("Echo: {}", prompt))
    }
}

/// Mock implementation of the TelegramApi trait for unit testing.
///
/// This mock simulates sending a Telegram message and asserts
/// that the input matches expected values.
struct MockTelegramApi;

#[async_trait]
impl TelegramApi for MockTelegramApi {
    async fn send_telegram_message(&self, chat_id: i64, text: String) -> Result<(), String> {
        assert_eq!(chat_id, 987654321);
        assert_eq!(text, "Echo: Hello bot");
        Ok(())
    }
}

/// Integration test for the `telegram_webhook` handler.
///
/// This test verifies the complete flow:
/// - Receives a simulated Telegram update.
/// - Calls the mocked Chat API.
/// - Sends a response using the mocked Telegram API.
/// - Asserts that the HTTP response is correct.
#[actix_web::test]
async fn test_telegram_webhook_success() {
    // Wrap mock implementations in Arc and web::Data
    let chat_api: web::Data<dyn ChatApi> =
        web::Data::from(Arc::new(MockChatApi {}) as Arc<dyn ChatApi>);
    let telegram_api: web::Data<dyn TelegramApi> =
        web::Data::from(Arc::new(MockTelegramApi {}) as Arc<dyn TelegramApi>);

    // Initialize Actix app with injected mocks
    let app = test::init_service(
        App::new()
            .app_data(chat_api.clone())
            .app_data(telegram_api.clone())
            .route("/webhook", web::post().to(telegram_webhook)),
    )
    .await;

    // Simulate incoming Telegram update
    let update = TelegramUpdate {
        update_id: 123456789,
        message: Some(TelegramMessage {
            message_id: 1,
            chat: TelegramChat { id: 987654321 },
            text: Some("Hello bot".to_string()),
        }),
    };

    // Send POST request to webhook endpoint
    let req = test::TestRequest::post()
        .uri("/webhook")
        .set_json(&update)
        .to_request();

    let resp = test::call_service(&app, req).await;

    // Assert HTTP 200 OK and correct body
    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    assert_eq!(body, "Message sent");
}

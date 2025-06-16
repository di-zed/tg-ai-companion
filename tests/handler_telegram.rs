use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use serde_json::json;
use std::sync::Arc;

use tg_ai_companion::handlers::telegram::telegram_webhook;
use tg_ai_companion::services::chat_api::ChatApi;

mock! {
    /// Mock implementation of the `ChatApi` trait for testing purposes.
    ///
    /// This mock allows simulating the behavior of the asynchronous `call_chat_api` method,
    /// enabling tests to verify how the webhook handler interacts with the chat API
    /// without making real external calls.
    pub ChatApi {}

    #[async_trait]
    impl ChatApi for ChatApi {
        /// Mock async method to simulate sending a prompt to the chat API and receiving a response.
        async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    }
}

/// Tests that the `/telegram/webhook` endpoint correctly handles
/// a Telegram message with valid text and returns the expected chat API response.
///
/// The test:
/// - Mocks `call_chat_api` to return "Hi back!" when the prompt is "Hello".
/// - Sends a POST request containing a Telegram message with `"text": "Hello"`.
/// - Asserts that the HTTP response status is 200 OK and the body matches the mocked response.
#[actix_web::test]
async fn test_telegram_webhook_ok() {
    let mut mock_api = MockChatApi::new();

    mock_api
        .expect_call_chat_api()
        .with(eq("Hello"))
        .times(1)
        .returning(|_| Ok("Hi back!".to_string()));

    let chat_api_data = web::Data::from(Arc::new(mock_api) as Arc<dyn ChatApi>);

    let app = test::init_service(
        App::new()
            .app_data(chat_api_data)
            .route("/telegram/webhook", web::post().to(telegram_webhook)),
    )
    .await;

    let req_body = json!({ "message": { "text": "Hello" } });

    let req = test::TestRequest::post()
        .uri("/telegram/webhook")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp_body = test::read_body(resp).await;
    let resp_str = std::str::from_utf8(&resp_body).unwrap();
    assert_eq!(resp_str, "Hi back!");
}

/// Tests that the `/telegram/webhook` endpoint returns a 400 Bad Request
/// when the Telegram message does not contain any text.
///
/// The test:
/// - Sends a POST request with an empty `"message": {}` JSON object.
/// - Asserts that the HTTP response status is 400 Bad Request.
/// - Checks that the response body contains the error message "No Message Text".
#[actix_web::test]
async fn test_telegram_webhook_no_text() {
    let mock_api = MockChatApi::new();
    let chat_api_data = web::Data::from(Arc::new(mock_api) as Arc<dyn ChatApi>);

    let app = test::init_service(
        App::new()
            .app_data(chat_api_data)
            .route("/telegram/webhook", web::post().to(telegram_webhook)),
    )
    .await;

    let req_body = json!({ "message": {} });

    let req = test::TestRequest::post()
        .uri("/telegram/webhook")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let resp_body = test::read_body(resp).await;
    let resp_str = std::str::from_utf8(&resp_body).unwrap();
    assert_eq!(resp_str, "No Message Text");
}

/// Tests the `/telegram/webhook` endpoint's behavior when the chat API call fails.
///
/// The test:
/// - Mocks `call_chat_api` to always return an error.
/// - Sends a POST request with a valid Telegram message containing `"text": "Hello"`.
/// - Asserts that the HTTP response status is 500 Internal Server Error.
/// - Verifies that the response body contains the error message "Error calling chat API".
#[actix_web::test]
async fn test_telegram_webhook_api_error() {
    let mut mock_api = MockChatApi::new();

    mock_api
        .expect_call_chat_api()
        .returning(|_| Err("API failure".into()));

    let chat_api_data = web::Data::from(Arc::new(mock_api) as Arc<dyn ChatApi>);

    let app = test::init_service(
        App::new()
            .app_data(chat_api_data)
            .route("/telegram/webhook", web::post().to(telegram_webhook)),
    )
    .await;

    let req_body = json!({ "message": { "text": "Hello" } });

    let req = test::TestRequest::post()
        .uri("/telegram/webhook")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let resp_body = test::read_body(resp).await;
    let resp_str = std::str::from_utf8(&resp_body).unwrap();
    assert_eq!(resp_str, "Error calling chat API");
}

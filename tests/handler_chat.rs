//! Integration tests for the `/chat` endpoint using a mock `ChatApi`.
//!
//! These tests simulate interaction with the chat API without making real HTTP calls,
//! ensuring the endpoint behaves correctly under various conditions.

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use serde_json::json;
use std::sync::Arc;

use tg_ai_companion::handlers::chat::chat_endpoint;
use tg_ai_companion::services::chat_api::ChatApi;

mock! {
    /// A mock implementation of the `ChatApi` trait for testing.
    ///
    /// This mock is used to simulate the behavior of the `call_chat_api` method
    /// without performing actual external API calls.
    pub ChatApi {}

    #[async_trait]
    impl ChatApi for ChatApi {
        /// Simulates sending a prompt to the chat API and returning a response.
        async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
    }
}

/// Verifies that the `/chat` endpoint successfully returns the expected response
/// when a valid message with text is sent.
///
/// Setup:
/// - Mocks `call_chat_api` to return `"Hi back!"` when prompt is `"Hello"`.
/// - Sends a valid request with a Telegram-like message.
///
/// Asserts:
/// - Status is `200 OK`
/// - Response body is `"Hi back!"`
#[actix_web::test]
async fn test_chat_endpoint_ok() {
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
            .route("/chat", web::post().to(chat_endpoint)),
    )
    .await;

    let req_body = json!({ "prompt": "Hello" });

    let req = test::TestRequest::post()
        .uri("/chat")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let resp_body = test::read_body(resp).await;
    let resp_str = std::str::from_utf8(&resp_body).unwrap();
    assert_eq!(resp_str, "Hi back!");
}

/// Verifies that the `/chat` endpoint returns a `400 Bad Request`
/// when the incoming message lacks a `text` field.
///
/// Sends:
/// - A request with an empty `"prompt": ""` JSON object.
///
/// Asserts:
/// - Status is `400 Bad Request`
/// - Body is `"No Message Text"`
#[actix_web::test]
async fn test_chat_endpoint_no_text() {
    let mock_api = MockChatApi::new();
    let chat_api_data = web::Data::from(Arc::new(mock_api) as Arc<dyn ChatApi>);

    let app = test::init_service(
        App::new()
            .app_data(chat_api_data)
            .route("/chat", web::post().to(chat_endpoint)),
    )
    .await;

    let req_body = json!({ "prompt": "" });

    let req = test::TestRequest::post()
        .uri("/chat")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let resp_body = test::read_body(resp).await;
    let resp_str = std::str::from_utf8(&resp_body).unwrap();
    assert_eq!(resp_str, "Prompt cannot be empty");
}

/// Ensures that the `/chat` endpoint handles internal API errors gracefully
/// and responds with a `500 Internal Server Error`.
///
/// Setup:
/// - Mocks `call_chat_api` to always return an error.
///
/// Sends:
/// - A valid request with `"text": "Hello"`
///
/// Asserts:
/// - Status is `500 Internal Server Error`
/// - Body is `"Error calling chat API"`
#[actix_web::test]
async fn test_chat_endpoint_api_error() {
    let mut mock_api = MockChatApi::new();

    mock_api
        .expect_call_chat_api()
        .returning(|_| Err("API failure".into()));

    let chat_api_data = web::Data::from(Arc::new(mock_api) as Arc<dyn ChatApi>);

    let app = test::init_service(
        App::new()
            .app_data(chat_api_data)
            .route("/chat", web::post().to(chat_endpoint)),
    )
    .await;

    let req_body = json!({ "prompt": "Hello" });

    let req = test::TestRequest::post()
        .uri("/chat")
        .set_json(&req_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let resp_body = test::read_body(resp).await;
    let resp_str = std::str::from_utf8(&resp_body).unwrap();
    assert_eq!(resp_str, "Error calling chat API");
}

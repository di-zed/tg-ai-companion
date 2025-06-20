//! Integration tests for the `/chat` endpoint using a mock `ChatApi`.
//!
//! These tests simulate interaction with the chat API without making real HTTP calls,
//! ensuring the endpoint behaves correctly under various conditions.

use actix_web::{http::StatusCode, test, web, App};
use async_trait::async_trait;
use mockall::predicate::*;
use mockall::*;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

use tg_ai_companion::handlers::chat::chat_endpoint;
use tg_ai_companion::services::chat_api::ChatApi;

mock! {
    /// A mock implementation of the `ChatApi` trait for testing.
    ///
    /// This mock simulates `call_chat_api` without real HTTP calls.
    pub ChatApi {}

    #[async_trait]
    impl ChatApi for ChatApi {
        async fn call_chat_api(&self, prompt: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
    }
}

/// Tests successful response from `/chat` endpoint.
///
/// Mocks `call_chat_api` to return "Hi back!" when prompt is "Hello".
#[actix_web::test]
async fn test_chat_endpoint_ok() {
    let mut mock_api = MockChatApi::new();

    mock_api
        .expect_call_chat_api()
        .with(eq("Hello"))
        .times(1)
        .returning(|_| Ok("Hi back!".to_string()));

    // Wrap the mock in Arc and then into web::Data to inject into the app state.
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
    assert_eq!(resp_str.trim(), "Hi back!");
}

/// Tests `/chat` endpoint returns 400 Bad Request for empty prompt.
#[actix_web::test]
async fn test_chat_endpoint_empty_prompt() {
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
    assert_eq!(resp_str.trim(), "Prompt cannot be empty");
}

/// Tests `/chat` endpoint returns 500 Internal Server Error when ChatApi fails.
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
    assert_eq!(resp_str.trim(), "Error calling chat API");
}

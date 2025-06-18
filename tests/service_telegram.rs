use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;

use tg_ai_companion::services::telegram::send_telegram_message_custom;

/// Integration test for `send_telegram_message_custom` function.
///
/// This test verifies that when the Telegram API responds with a successful
/// HTTP 200 status and an OK JSON response, the function returns `Ok(())`.
///
/// It uses `httpmock` to mock the Telegram API endpoint, expecting a POST request
/// with the correct `chat_id` and `text` JSON body, and responds with a successful
/// Telegram API response.
///
/// Assertions:
/// - The result of the function call is `Ok`.
/// - The mock server received exactly one request matching the expectations.
#[tokio::test]
async fn test_send_telegram_message_custom_success() {
    let server = MockServer::start();

    let fake_token = "TEST_TOKEN";
    let chat_id = 123456789;
    let text = "Hello from test!".to_string();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/bot{}/sendMessage", fake_token))
            .json_body(json!({
                "chat_id": chat_id,
                "text": text
            }));
        then.status(200).body(r#"{"ok":true,"result":{}}"#);
    });

    let result = send_telegram_message_custom(&server.base_url(), fake_token, chat_id, text).await;

    assert!(result.is_ok());
    mock.assert_hits(1);
}

/// Integration test for `send_telegram_message_custom` function covering error responses.
///
/// This test verifies that when the Telegram API responds with an error HTTP status code (e.g., 400),
/// the function returns an `Err` containing the status code and error message from the API.
///
/// It uses `httpmock` to mock the Telegram API endpoint, expecting a POST request
/// with the correct `chat_id` and `text` JSON body, and responds with a Telegram API error response.
///
/// Assertions:
/// - The result of the function call is an error (`Err`).
/// - The error message contains the HTTP status code returned by the mock server.
/// - The error message contains the error description from the Telegram API response.
/// - The mock server received exactly one request matching the expectations.
#[tokio::test]
async fn test_send_telegram_message_custom_error() {
    let server = MockServer::start();

    let fake_token = "TEST_TOKEN";
    let chat_id = 123456789;
    let text = "Hello error test!".to_string();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(format!("/bot{}/sendMessage", fake_token))
            .json_body(json!({
                "chat_id": chat_id,
                "text": text
            }));
        then.status(400)
            .body(r#"{"ok":false,"description":"Bad Request: chat not found"}"#);
    });

    let result = send_telegram_message_custom(&server.base_url(), fake_token, chat_id, text).await;

    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(err.contains("Telegram API error 400"));
    assert!(err.contains("Bad Request: chat not found"));

    mock.assert_hits(1);
}

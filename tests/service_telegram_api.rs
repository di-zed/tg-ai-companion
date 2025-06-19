use httpmock::{Method::POST, MockServer};

use tg_ai_companion::models::telegram::SendMessageRequest;
use tg_ai_companion::services::telegram_api::TelegramApi;
use tg_ai_companion::services::telegram_api_impl::RealTelegramApi;

/// A fake token used for mocking Telegram Bot API requests in tests.
const FAKE_TOKEN: &str = "FAKE_TOKEN";

/// Tests that `send_telegram_message` returns `Ok(())` on a successful API response.
#[tokio::test]
async fn test_send_telegram_message_success() {
    let server = MockServer::start();

    let chat_id = 123456;
    let text = "Hello, world!".to_string();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(&format!("/bot{}/sendMessage", FAKE_TOKEN))
            .json_body_obj(&SendMessageRequest {
                chat_id,
                text: text.clone(),
            });

        then.status(200)
            .header("Content-Type", "application/json")
            .body(r#"{"ok":true,"result":{}}"#);
    });

    let api = RealTelegramApi {
        client: reqwest::Client::new(),
        base_url: server.base_url(),
        token: FAKE_TOKEN.to_string(),
    };

    let result = api.send_telegram_message(chat_id, text).await;
    assert!(result.is_ok(), "Expected success, got: {:?}", result);

    mock.assert();
}

/// Tests that `send_telegram_message` returns an error on Telegram API-level failure (e.g. 400).
#[tokio::test]
async fn test_send_telegram_message_api_error() {
    let server = MockServer::start();

    let chat_id = 42;
    let text = "fail this".to_string();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path(&format!("/bot{}/sendMessage", FAKE_TOKEN))
            .json_body_obj(&SendMessageRequest {
                chat_id,
                text: text.clone(),
            });

        then.status(400)
            .body(r#"{"ok":false,"description":"Bad Request"}"#);
    });

    let api = RealTelegramApi {
        client: reqwest::Client::new(),
        base_url: server.base_url(),
        token: FAKE_TOKEN.to_string(),
    };

    let result = api.send_telegram_message(chat_id, text).await;
    assert!(
        result.is_err(),
        "Expected Telegram API error, got: {:?}",
        result
    );

    mock.assert();
}

/// Tests that `send_telegram_message` returns an error when the network request fails (e.g. unreachable host).
#[tokio::test]
async fn test_send_telegram_message_network_failure() {
    let api = RealTelegramApi {
        client: reqwest::Client::new(),
        base_url: "http://127.0.0.1:12345".to_string(), // unreachable port
        token: FAKE_TOKEN.to_string(),
    };

    let result = api.send_telegram_message(1, "test".to_string()).await;

    assert!(result.is_err(), "Expected network error, got: {:?}", result);
}

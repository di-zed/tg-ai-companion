use actix_web::{web, HttpResponse, Responder};

use crate::models::chat::ChatRequest;
use crate::services::chat_api::ChatApi;

/// Handles incoming chat requests by forwarding the prompt to the chat API service.
///
/// # Arguments
///
/// * `payload` - A JSON payload containing the `ChatRequest` with the user's prompt.
/// * `chat_api` - Shared reference to an implementation of the `ChatApi` trait, used to process the prompt.
///
/// # Behavior
///
/// - Validates that the `prompt` field in the request is not empty or whitespace only.
/// - If the prompt is empty, returns `400 Bad Request` with an appropriate error message.
/// - Calls the asynchronous chat API to get a response for the prompt.
/// - On success, returns `200 OK` with the chat API's response as the body.
/// - On failure, logs the error and returns `500 Internal Server Error`.
///
/// # Returns
///
/// An `impl Responder` that corresponds to the HTTP response with either the chat response or an error message.
pub async fn chat_endpoint(
    payload: web::Json<ChatRequest>,
    chat_api: web::Data<dyn ChatApi>,
) -> impl Responder {
    if payload.prompt.trim().is_empty() {
        return HttpResponse::BadRequest().body("Prompt cannot be empty");
    }

    match chat_api.call_chat_api(&payload.prompt).await {
        Ok(response_text) => HttpResponse::Ok().body(response_text),
        Err(e) => {
            eprintln!("Error calling chat API: {}", e);
            HttpResponse::InternalServerError().body("Error calling chat API")
        }
    }
}

use actix_web::{dev::ServiceRequest, error, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use std::env;

/// Validates Bearer token from the `Authorization` header.
///
/// This function is used with `HttpAuthentication::bearer(...)` middleware.
/// It checks whether the provided token matches the `API_TOKEN` environment variable.
///
/// # Arguments
/// - `req`: Incoming request
/// - `credentials`: Optional Bearer token extracted from the request
///
/// # Returns
/// - `Ok(req)` if the token is valid
/// - `Err((Error, req))` if the token is missing or incorrect
///
/// # Environment
/// - `API_TOKEN`: expected token value
pub async fn validator(
    req: ServiceRequest,
    credentials: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credentials) = credentials else {
        return Err((error::ErrorBadRequest("No Bearer Header"), req));
    };

    let expected_token: String = env::var("API_TOKEN").unwrap_or_default();

    if credentials.token() != expected_token {
        return Err((error::ErrorBadRequest("Authentication Error"), req));
    }

    Ok(req)
}

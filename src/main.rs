use actix_cors::Cors;
use actix_web::{http::header, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use std::env;

use tg_ai_companion::middleware::auth::validator;
use tg_ai_companion::routes::telegram::init_telegram_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from `.env` file into std::env
    dotenv().ok();

    // Read host and port from environment variables.
    let host: String = env::var("SERVER_HOST_NAME").unwrap_or_else(|_| "127.0.0.1".into());
    let port: String = env::var("SERVER_HOST_PORT").unwrap_or_else(|_| "80".into());
    let bind_address: String = format!("{}:{}", host, port);

    println!("🚀 Server running at {}", bind_address);

    HttpServer::new(move || {
        let auth = HttpAuthentication::with_fn(validator);

        App::new()
            .service(init_telegram_routes())
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(auth)
    })
    .bind(bind_address)?
    .run()
    .await
}

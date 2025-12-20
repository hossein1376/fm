use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use std::sync::Arc;
use log::info;

mod api;
mod auth;
mod db;
mod hosts;
mod models;
mod metrics;
mod ws;

use rust_embed::RustEmbed;
use actix_web::{HttpRequest, HttpResponse};

#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct Assets;

async fn serve_spa(req: HttpRequest) -> HttpResponse {
    let path = req.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    match Assets::get(path) {
        Some(content) => {
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime_type.as_ref())
                .body(content.data.into_owned())
        }
        None => {
            // Fallback to index.html for SPA routing
            match Assets::get("index.html") {
                Some(content) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(content.data.into_owned()),
                None => HttpResponse::NotFound().body("Not found"),
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    info!("Starting File Manager server...");

    // Initialize database
    let db = Arc::new(db::Database::new().await.expect("Failed to initialize database"));
    db.initialize().await.expect("Failed to initialize database schema");

    // Initialize metrics
    let metrics = Arc::new(metrics::Metrics::new());

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    info!("Server listening on http://{}", bind_addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(metrics.clone()))
            .service(
                web::scope("/api")
                    .configure(api::configure)
            )
            .service(web::resource("/ws").to(ws::ws_handler))
            .service(web::resource("/metrics").to(metrics::metrics_handler))
            .default_service(web::get().to(serve_spa))
    })
    .bind(&bind_addr)?
    .run()
    .await
}

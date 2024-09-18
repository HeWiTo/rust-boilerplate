pub mod config;
pub mod routes;
pub mod handlers;
pub mod middleware;
pub mod errors;
pub mod validation;
pub mod docs;
pub mod websocket;

use actix_web::{web, App, HttpServer};
use crate::config::AppConfig;
use crate::routes::configure_routes;
use crate::middleware::tenant::TenantMiddleware;
use crate::docs::configure_swagger;

pub async fn run_server(config: AppConfig) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TenantMiddleware)
            .configure(configure_routes)
            .configure(configure_swagger)
            // Add any other middleware or configurations here
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run();

    println!("Server running at http://{}:{}/", config.host, config.port);
    server.await
}

// Re-export important structs and functions for easier use
pub use crate::config::AppConfig;
pub use crate::errors::ApiError;
use actix_web::{web, App, HttpServer};
use crate::middleware::tenant::TenantMiddleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... other initialization code ...

    HttpServer::new(|| {
        App::new()
            .wrap(TenantMiddleware)
            // ... other middleware and services ...
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
use actix_web::web;
use crate::handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // ... other routes ...
    );
    
    // Add this line for the Askama-rendered index page
    cfg.route("/", web::get().to(handlers::index));
}
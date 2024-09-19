use actix_web::{web, HttpResponse, Responder};
use askama_actix::TemplateIntoResponse;
use crate::templates::{IndexTemplate, User};

// ... other handlers ...

pub async fn index() -> impl Responder {
    let template = IndexTemplate {
        title: "Welcome to Rust SaaS",
        user: Some(User { name: "John Doe" }),
    };
    template.into_response()
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 - Not Found")
}
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Authentication error: {0}")]
    AuthError(String),
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::AuthError(_) => HttpResponse::Unauthorized().json(self.to_string()),
            ApiError::DbError(_) => HttpResponse::InternalServerError().json("Database error occured"),
            ApiError::NotFound(msg) => HttpResponse::NotFound().json(msg),
            ApiError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            ApiError::InternalServerError => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}

// Helper function to map any error into an ApiError
pub fn map_error(err: impl std::error::Error) -> ApiError {
    log::error!("Error: {:?}", err);
    ApiError::InternalServerError
}
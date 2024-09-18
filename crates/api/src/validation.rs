use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterInput {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginInput {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

// Add more input structs as needed

use actix_web::{web, HttpResponse, ResponseError};
use validator::ValidationErrors;

impl ResponseError for ValidationErrors {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::BasRequest().json(self.to_string())
    }
}

pub fn validate<T>(input: &web::Json<T>) -> Result<(), ValidationErrors>
where
    T: Validate,
{
    input.validate()
}
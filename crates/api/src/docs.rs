use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use actix_web::web;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::login,
        crate::handlers::auth::register,
        crate::handlers::auth::google_auth,
        crate::handlers::auth::google_callback,
        // Add more paths here as you implement them
    ),
    components(
        schemas(crate::validation::LoginInput, crate::validation::RegisterInput)
    ),
    tags(
        (name = "auth", description = "Authentication endpoints.")
    )
)]
struct ApiDoc;

pub fn configure_swagger(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-docs/openapi.json", openapi)
        .config(cfg);
}
use actix_web::{web, App, HttpServer};
use actix_files as fs;
use api::{
    config::AppConfig,
    middleware::tenant::TenantMiddleware,
    routes::configure_routes,
    docs::configure_swagger,
};
use infrastructure::{
    init_infrastructure,
    InfrastructureConfig,
    telemetry::init_telemetry,
    PaymentProviderConfig,
    EmailProviderConfig,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
    dotenv::dotenv().ok();
    let config = AppConfig::from_env().expect("Failed to load configuration");

    // Initialize telemetry
    init_telemetry();

    // Initialize infrastructure
    let infra_config = InfrastructureConfig {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        mongodb_url: std::env::var("MONGODB_URI").expect("MONGODB_URI must be set"),
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        google_client_id: std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
        google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
        payment_provider: PaymentProviderConfig::Stripe(
            std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set")
        ),
        email_provider: EmailProviderConfig::SendGrid {
            api_key: std::env::var("SENDGRID_API_KEY").expect("SENDGRID_API_KEY must be set")
        },
    };
    let infrastructure = init_infrastructure(infra_config).await.expect("Failed to initialize infrastructure");

    log::info!("Starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(TenantMiddleware)
            .app_data(web::Data::new(infrastructure.clone()))
            .configure(configure_routes)
            .configure(configure_swagger)
            .service(fs::Files::new("/", "./frontend/dist").index_file("index.html"))
            .default_service(web::route().to(api::handlers::not_found))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}
pub mod database;
pub mod auth;
pub mod email;
pub mod payment;
pub mod telemetry;

use anyhow::Result;
use async_trait::async_trait;

// Re-export important types and traits
pub use database::{DatabaseManager, PgUserRepository, MongoUserRepository};
pub use auth::{AuthService, JwtAuth, GoogleOAuth};
pub use email::EmailService;
pub use payment::PaymentService;
pub use telemetry::init_telemetry;

pub enum EmailProviderConfig {
    Mailgun { api_key: String, domain: String },
    SendGrid { api_key: String },
}

pub enum PaymentProviderConfig {
    Stripe(String), // API Key
    LemonSqueezy(String), // API Key
}

// Infrastructure configuration
pub struct InfrastructureConfig {
    pub database_url: String,
    pub mongodb_url: String,
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub email_provider: EmailProviderConfig,
    pub payment_provider: PaymentProviderConfig,
    // Add more configuration fields as needed
}

// Main infrastructure service that holds all components
pub struct Infrastructure {
    pub db: DatabaseManager,
    pub auth: AuthService,
    pub email: EmailService,
    pub payment: PaymentService,
}

impl Infrastructure {
    pub async fn new(config: InfrastructureConfig) -> Result<Self> {
        let db = DatabaseManager::new(&config.database_url, &config.mongodb_url).await?;
        let auth = AuthService::new(
            config.jwt_secret,
            config.google_client_id,
            config.google_client_secret,
        );
        let email = match config.email_provider {
            EmailProviderConfig::Mailgun { api_key, domain } => EmailService::new_mailgun(api_key, domain),
            EmailProviderConfig::SendGrid { api_key } => EmailService::new_sendgrid(api_key),
        };
        let payment = match config.payment_provider {
            PaymentProviderConfig::Stripe(api_key) => PaymentService::new_stripe(api_key),
            PaymentProviderConfig::LemonSqueezy(api_key) => PaymentService::new_lemon_squeezy(api_key),
        };

        Ok(Self {
            db,
            auth,
            email,
            payment,
        })
    }
}

// Implement core repository traits for our infrastructure implementations
#[async_trait]
impl core::repositories::UserRepository for PgUserRepository {
    async fn create_user(&self, user: &core::entities::User) -> core::Result<()> {
        // Implementation here
        todo!()
    }

    async fn get_user_by_email(&self, email: &str) -> core::Result<Option<core::entities::User>> {
        // Implementation here
        todo!()
    }

    // Implement other methods...
}

// Similar implementations for other repositories and services...

// Helper function to initialize all infrastructure components
pub async fn init_infrastructure(config: InfrastructureConfig) -> Result<Infrastructure> {
    init_telemetry();
    Infrastructure::new(config).await
}
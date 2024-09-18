mod jwt;
mod google_oauth;

use anyhow::Result;
use async_trait::async_trait;
use core::repositories::AuthRepository;

use self::jwt::JwtAuth;
use self::google_oauth::GoogleOAuth;

pub struct AuthService {
    jwt_auth: JwtAuth,
    google_oauth: GoogleOAuth,
}

impl AuthService {
    pub fn new(
        jwt_secret: String,
        google_client_id: String,
        google_client_secret: String,
        google_redirect_url: String
    ) -> Self {
        Self {
            jwt_auth: JwtAuth::new(jwt_secret),
            google_oauth: GoogleOAuth::new(
                google_client_id,
                google_client_secret,
                google_redirect_url,
            ),
        }
    }
}

#[async_trait]
impl AuthRepository for AuthService {
    async fn login(&self, email: &str, password: &str) -> Result<String> {
        // Implement login logic here
        // For example, verify credentials against the database
        // If successful, generate and return a JWT token
        self.jwt_auth.generate_token(email)
    }

    async fn register(&self, email: &str, password: &str) -> Result<String> {
        // Implement registration logic here
        // For example, create a new user in the database
        // If successful, generate and return a JWT token
        self.jwt_auth.generate_token(email)
    }

    async fn google_auth_url(&self) -> (String, String) {
        let (url, csrf_token) = self.google_oauth.get_auth_url();
        (url, csrf_token.secret().clone())
    }

    async fn google_callback(&self, code: &str) -> Result<String> {
        let access_token = self.google_oauth.exchange_code(code.to_string()).await?;
        // Here you would typically use the access token to fetch the user's info from Google
        // Then create or update the user in your database
        // For this example, we'll just return a JWT token
        self.jwt_auth.generate_token(&access_token)
    }
}
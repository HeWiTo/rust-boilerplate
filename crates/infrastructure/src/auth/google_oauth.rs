use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl,
    TokenUrl, AuthorizationCode, CsrfToken, Scope, TokenResponse,
};
use anyhow::Result;

pub struct GoogleOAuth {
    client: BasicClient,
}

impl GoogleOAuth {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String,
    ) -> Self {
        let google_client_id = ClientId::new(client_id);
        let google_client_secret = ClientSecret::new(client_secret);
        let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .expect("Invalid authorization endpoint URL");
        let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
            .expect("Invalid token endpoint URL");

        let client = BasicClient::new(
            google_client_id,
            Some(google_client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Invalid redirect URL"));

        Self { client }
    }

    pub fn get_auth_url(&self) -> (String, CsrfToken) {
        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();

        (auth_url.to_string(), csrf_token)
    }

    pub async fn exchange_code(&self, code: String) -> Result<String> {
        let token = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        Ok(token.access_token().secret().clone())
    }
}
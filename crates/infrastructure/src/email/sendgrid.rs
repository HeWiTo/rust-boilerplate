use super::{EmailProvider, Email};
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub struct SendGridService {
    client: Client,
    api_key: String,
}

impl SendGridService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl EmailProvider for SendGridService {
    async fn send_email(&self, email: &Email) -> Result<()> {
        let url = "https://api.sendgrid.com/v3/mail/send";
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "personalizations": [{
                    "to": [{"email": &email.to}]
                }],
                "from": {"email": &email.from},
                "subject": &email.subject,
                "content": [{
                    "type": "text/plain",
                    "value": &email.body
                }]
            }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to send email via SendGrid"))
        }
    }
}
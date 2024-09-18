use super::{EmailProvider, Email};
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub struct MailgunService {
    client: Client,
    api_key: String,
    domain: String,
}

impl MailgunService {
    pub fn new(api_key: String, domain: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            domain,
        }
    }
}

#[async_trait]
impl EmailProvider for MailgunService {
    async fn send_email(&self, email: &Email) -> Result<()> {
        let url = format!("https://api.mailgun.net/v3/{}/messages", self.domain);
        
        let response = self.client
            .post(&url)
            .basic_auth("api", Some(&self.api_key))
            .form(&json!({
                "from": &email.from,
                "to": &email.to,
                "subject": &email.subject,
                "text": &email.body,
            }))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to send email via Mailgun"))
        }
    }
}
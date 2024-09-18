mod mailgun;
mod sendgrid;

use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub body: String,
}

#[async_trait]
pub trait EmailProvider {
    async fn send_email(&self, email: &Email) -> Result<()>;
}

pub enum EmailService {
    Mailgun(mailgun::MailgunService),
    SendGrid(sendgrid::SendGridService),
}

impl EmailService {
    pub fn new_mailgun(api_key: String, domain: String) -> Self {
        EmailService::Mailgun(mailgun::MailgunService::new(api_key, domain))
    }

    pub fn new_sendgrid(api_key: String) -> Self {
        EmailService::SendGrid(sendgrid::SendGridService::new(api_key))
    }

    pub async fn send_email(&self, email: &Email) -> Result<()> {
        match self {
            EmailService::Mailgun(service) => service.send_email(email).await,
            EmailService::SendGrid(service) => service.send_email(email).await,
        }
    }
}
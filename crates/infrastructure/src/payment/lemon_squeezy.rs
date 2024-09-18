use super::{PaymentProvider, PaymentIntent};
use async_trait::async_trait;
use anyhow::Result;
use reqwest::Client;
use serde_json::json;

pub struct LemonSqueezyService {
    client: Client,
    api_key: String,
}

impl LemonSqueezyService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl PaymentProvider for LemonSqueezyService {
    async fn create_payment_intent(&self, amount: u64, currency: &str) -> Result<PaymentIntent> {
        // Note: This is a placeholder implementation. You'll need to adjust this
        // to match LemonSqueezy's actual API for creating payments.
        let response = self.client
            .post("https://api.lemonsqueezy.com/v1/payments")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "amount": amount,
                "currency": currency,
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // Parse the response and create a PaymentIntent
        // This is just an example, adjust according to actual LemonSqueezy response
        Ok(PaymentIntent {
            id: response["id"].as_str().unwrap_or("").to_string(),
            amount,
            currency: currency.to_string(),
            status: response["status"].as_str().unwrap_or("").to_string(),
        })
    }

    async fn confirm_payment_intent(&self, payment_intent_id: &str) -> Result<PaymentIntent> {
        // Note: This is a placeholder implementation. You'll need to adjust this
        // to match LemonSqueezy's actual API for confirming payments.
        let response = self.client
            .post(&format!("https://api.lemonsqueezy.com/v1/payments/{}/confirm", payment_intent_id))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // Parse the response and create a PaymentIntent
        // This is just an example, adjust according to actual LemonSqueezy response
        Ok(PaymentIntent {
            id: payment_intent_id.to_string(),
            amount: response["amount"].as_u64().unwrap_or(0),
            currency: response["currency"].as_str().unwrap_or("").to_string(),
            status: response["status"].as_str().unwrap_or("").to_string(),
        })
    }
}
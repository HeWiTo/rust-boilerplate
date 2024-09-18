mod stripe;
mod lemon_squeezy;

use async_trait::async_trait;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentIntent {
    pub id: String,
    pub amount: u64,
    pub currency: String,
    pub status: String,
}

#[async_trait]
pub trait PaymentProvider {
    async fn create_payment_intent(&self, amount: u64, currency: &str) -> Result<PaymentIntent>;
    async fn confirm_payment_intent(&self, payment_intent_id: &str) -> Result<PaymentIntent>;
}

pub enum PaymentService {
    Stripe(stripe::StripeService),
    LemonSqueezy(lemon_squeezy::LemonSqueezyService),
}

impl PaymentService {
    pub fn new_stripe(api_key: String) -> Self {
        PaymentService::Stripe(stripe::StripeService::new(api_key))
    }

    pub fn new_lemon_squeezy(api_key: String) -> Self {
        PaymentService::LemonSqueezy(lemon_squeezy::LemonSqueezyService::new(api_key))
    }

    pub async fn create_payment_intent(&self, amount: u64, currency: &str) -> Result<PaymentIntent> {
        match self {
            PaymentService::Stripe(service) => service.create_payment_intent(amount, currency).await,
            PaymentService::LemonSqueezy(service) => service.create_payment_intent(amount, currency).await,
        }
    }

    pub async fn confirm_payment_intent(&self, payment_intent_id: &str) -> Result<PaymentIntent> {
        match self {
            PaymentService::Stripe(service) => service.confirm_payment_intent(payment_intent_id).await,
            PaymentService::LemonSqueezy(service) => service.confirm_payment_intent(payment_intent_id).await,
        }
    }
}
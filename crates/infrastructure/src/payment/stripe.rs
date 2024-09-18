use super::{PaymentProvider, PaymentIntent};
use async_trait::async_trait;
use anyhow::Result;
use stripe::{Client, CreatePaymentIntent, ConfirmPaymentIntent};

pub struct StripeService {
    client: Client,
}

impl StripeService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(api_key),
        }
    }
}

#[async_trait]
impl PaymentProvider for StripeService {
    async fn create_payment_intent(&self, amount: u64, currency: &str) -> Result<PaymentIntent> {
        let mut create_intent = CreatePaymentIntent::new(amount, currency.parse()?);
        create_intent.confirm = Some(false);

        let payment_intent = self.client.payment_intents().create(&create_intent).await?;

        Ok(PaymentIntent {
            id: payment_intent.id.to_string(),
            amount: payment_intent.amount,
            currency: payment_intent.currency.to_string(),
            status: payment_intent.status.to_string(),
        })
    }

    async fn confirm_payment_intent(&self, payment_intent_id: &str) -> Result<PaymentIntent> {
        let confirm_intent = ConfirmPaymentIntent::new();
        let payment_intent = self.client.payment_intents().confirm(payment_intent_id, &confirm_intent).await?;

        Ok(PaymentIntent {
            id: payment_intent.id.to_string(),
            amount: payment_intent.amount,
            currency: payment_intent.currency.to_string(),
            status: payment_intent.status.to_string(),
        })
    }
}
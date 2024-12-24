use stripe::{Client, CreatePaymentIntent, PaymentIntent, CreateCustomer, Customer, Currency};
use async_trait::async_trait;
use crate::application::ports::StripePort;

pub struct StripeClient {
    client: Client,
}

impl StripeClient {
    pub fn new(secret_key: String) -> Self {
        Self {
            client: Client::new(secret_key),
        }
    }
}

#[async_trait]
impl StripePort for StripeClient {
    async fn create_payment_intent(&self, amount: i64, currency: Currency) -> Result<PaymentIntent, String> {
        let mut params = CreatePaymentIntent::new(amount, currency);
        params.payment_method_types = Some(vec!["card".to_string()]);
        
        PaymentIntent::create(&self.client, params)
            .await
            .map_err(|err| format!("Stripe API error: {}", err))
    }

    async fn create_customer(&self, email: String, name: Option<String>) -> Result<Customer, String> {
        let mut params = CreateCustomer::new();
        params.email = Some(email);
        if let Some(name) = name {
            params.name = Some(name);
        }

        Customer::create(&self.client, params)
            .await
            .map_err(|err| format!("Stripe API error: {}", err))
    }
}

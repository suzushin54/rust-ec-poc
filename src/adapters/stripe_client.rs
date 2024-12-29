use async_trait::async_trait;
use std::str::FromStr;
use stripe::{Client, CreatePaymentIntent, PaymentIntent, CreateCustomer, Customer, Currency};
use crate::application::ports::stripe::StripePort;

#[derive(Clone)]
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

    async fn create_customer(&self, email: String, token: Option<String>, name: Option<String>) -> Result<Customer, String> {
        let mut params = CreateCustomer::new();
        params.email = Some(&email);
        let description = format!("Customer for {}", email);
        params.description = Some(&description);
        if let Some(name) = name {
            params.name = Some(Box::leak(name.into_boxed_str()));
        }
 
        if let Some(ref token_str) = token {
            match stripe::CardTokenId::from_str(token_str) {
                Ok(card_token_id) => {
                    let token_id = stripe::TokenId::Card(card_token_id);
                    params.source = Some(stripe::PaymentSourceParams::Token(token_id));
                }
                Err(_) => {
                    return Err("Invalid card token ID".to_string());
                }
            }
        }

        Customer::create(&self.client, params)
            .await
            .map_err(|err| format!("Stripe API error: {}", err))
    }
}

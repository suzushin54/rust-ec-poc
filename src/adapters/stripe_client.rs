use async_trait::async_trait;
use std::str::FromStr;
use stripe::{
    Client, CreatePaymentIntent, PaymentIntent, CreateCustomer, Customer, CustomerId, Currency,
    CreatePaymentIntentPaymentMethodData,
    CreatePaymentIntentPaymentMethodDataType,
    CreatePaymentIntentPaymentMethodOptions,
    CreatePaymentIntentPaymentMethodOptionsCustomerBalance,
    CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer,
    CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType,
    CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType,
};
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
    async fn create_payment_intent_card(&self, amount: i64) -> Result<PaymentIntent, String> {
        let mut params = CreatePaymentIntent::new(amount, Currency::JPY);
        params.payment_method_types = Some(vec!["card".to_string()]);
        
        PaymentIntent::create(&self.client, params)
            .await
            .map_err(|err| format!("Stripe API error: {}", err))
    }
    async fn create_payment_intent_bank_transfer(&self, amount: i64, customer_id: String) -> Result<PaymentIntent, String> {
        let mut params = CreatePaymentIntent::new(amount, Currency::JPY);

        match CustomerId::from_str(&customer_id) {
            Ok(customer_id) => params.customer = Some(customer_id),
            Err(e) => return Err(format!("Invalid Customer ID: {}", e)),
        }

        params.payment_method_types = Some(vec!["customer_balance".to_string()]); 
        params.payment_method_options = Some(CreatePaymentIntentPaymentMethodOptions {
            customer_balance: Some(CreatePaymentIntentPaymentMethodOptionsCustomerBalance {
                bank_transfer: Some(CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransfer {
                    type_: CreatePaymentIntentPaymentMethodOptionsCustomerBalanceBankTransferType::JpBankTransfer,
                    eu_bank_transfer: None,
                    requested_address_types: None,
                }),
                funding_type: Some(CreatePaymentIntentPaymentMethodOptionsCustomerBalanceFundingType::BankTransfer),
                setup_future_usage: None,
            }),
            ..Default::default()
        });
        params.payment_method_data = Some(CreatePaymentIntentPaymentMethodData {
            type_: CreatePaymentIntentPaymentMethodDataType::CustomerBalance,
            ..Default::default()
        });
        
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

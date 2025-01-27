use async_trait::async_trait;
use stripe::{PaymentIntent, Customer};

#[async_trait]
pub trait StripePort {
    async fn create_payment_intent_card(&self, amount: i64) -> Result<PaymentIntent, String>;
    async fn create_payment_intent_bank_transfer(&self, amount: i64, customer_id: String) -> Result<PaymentIntent, String>;
    async fn create_customer(&self, email: String, token: Option<String>, name: Option<String>) -> Result<Customer, String>;
}

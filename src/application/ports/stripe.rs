use stripe::{Currency, PaymentIntent, Customer};

#[async_trait::async_trait]
pub trait StripePort {
    async fn create_payment_intent(&self, amount: i64, currency: Currency) -> Result<PaymentIntent, String>;
    async fn create_customer(&self, email: String, name: Option<String>) -> Result<Customer, String>;
}
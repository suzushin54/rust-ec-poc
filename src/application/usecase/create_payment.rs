use crate::application::ports::stripe::StripePort;
use stripe::Currency;

/// PaymentUseCase is a generic service that depends on a StripePort implementation.
#[derive(Clone)]
pub struct PaymentUseCase<P: StripePort> {
    stripe_port: P,
}

/// Implementation of PaymentUseCase with methods to init and payment.
impl<P: StripePort> PaymentUseCase<P> {
    pub fn new(stripe_port: P) -> Self {
        Self { stripe_port }
    }

    pub async fn execute(&self, amount: i64) -> Result<String, String> {
        let payment = self
            .stripe_port
            .create_payment_intent(amount, Currency::JPY)
            .await
            .map_err(|e| format!("Failed to create payment intent: {}", e))?;

        Ok(payment.id.to_string())
    }

}

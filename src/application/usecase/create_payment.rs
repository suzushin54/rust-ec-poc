use crate::application::ports::stripe::StripePort;

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

    pub async fn execute(&self, amount: i64, customer_id: String) -> Result<String, String> {
        // TODO: card, bank_transfer などで処理を分岐
        let payment_card = self
            .stripe_port
            .create_payment_intent_card(amount)
            .await
            .map_err(|e| format!("Failed to create payment intent card: {}", e))?;
        
        let payment = self
            .stripe_port
            .create_payment_intent_bank_transfer(amount, customer_id)
            .await
            .map_err(|e| format!("Failed to create payment intent bank transfer: {}", e))?;

        Ok(payment.id.to_string())
    }

}

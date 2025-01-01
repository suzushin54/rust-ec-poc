use crate::application::ports::stripe::StripePort;

/// PaymentMethod defines different types of payment methods.
pub enum PaymentMethod {
    Card,
    BankTransfer,
    ConvenienceStore,
}

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

    pub async fn execute(
        &self, amount: i64,
        customer_id: String,
        payment_method: PaymentMethod,
    ) -> Result<String, String> {
        match payment_method {
            PaymentMethod::Card => {
                let payment = self
                    .stripe_port
                    .create_payment_intent_card(amount)
                    .await
                    .map_err(|e| format!("Failed to create payment intent card: {}", e))?;
                Ok(payment.id.to_string())

            }
            PaymentMethod::BankTransfer => {
                let payment = self
                    .stripe_port
                    .create_payment_intent_bank_transfer(amount, customer_id)
                    .await
                    .map_err(|e| format!("Failed to create payment intent bank transfer: {}", e))?;

                Ok(payment.id.to_string())
            }
            PaymentMethod::ConvenienceStore => {
                Ok("dummy".to_string()) 
            }
        }
    }
}

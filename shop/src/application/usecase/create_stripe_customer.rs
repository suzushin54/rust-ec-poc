use crate::application::ports::stripe::StripePort;

#[derive(Clone)]
pub struct CreateStripeCustomerUseCase<P: StripePort> {
    stripe_port: P,
}

impl<P: StripePort> CreateStripeCustomerUseCase<P> {
    pub fn new(stripe_port: P) -> Self {
        Self { stripe_port }
    }

    pub async fn execute(
        &self,
        email: String,
        token: Option<String>,
        name: Option<String>,
    ) -> Result<String, String> {
        let customer = self
            .stripe_port
            .create_customer(email, token, name)
            .await
            .map_err(|e| format!("Failed to create customer: {}", e))?;
        Ok(customer.id.to_string())
    }
}

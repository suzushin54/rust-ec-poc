use crate::adapters::stripe_client::StripeClient;

#[derive(Clone)]
pub struct AppState {
    pub stripe_client: StripeClient,
}

impl AppState {
    pub fn new(stripe_client: StripeClient) -> Self {
        Self { stripe_client }
    }
} 
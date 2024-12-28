use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use crate::application::usecase::create_payment::PaymentUseCase;
use crate::adapters::stripe_client::StripeClient;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub amount: i64,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    pub success: bool,
    pub message: String,
    pub payment_id: Option<String>,
}
#[axum::debug_handler]
pub async fn handle_payment(Json(payload): Json<PaymentRequest>) -> impl IntoResponse {
    dotenv().ok();
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    let stripe_client = StripeClient::new(secret_key);
    let payment_usecase = PaymentUseCase::new(stripe_client);

    match payment_usecase
        .execute(payload.amount)
        .await
    {
        Ok(payment_id) => Json(PaymentResponse {
            success: true,
            message: "Payment successfully".to_string(),
            payment_id: Some(payment_id),
        }),
        Err(err) => Json(PaymentResponse {
            success: false,
            message: err,
            payment_id: None,
        }),
    }
}

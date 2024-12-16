use axum::{Router, Json, routing::post};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaymentRequest {
    amount: u64,
    currency: String,
}

async fn handle_payment(Json(payload): Json<PaymentRequest>) -> &'static str {
    println!("Processing payment: {} {}", payload.amount, payload.currency);
    "Payment processed"
}

pub fn router() -> Router {
    Router::new().route("/payment", post(handle_payment))
}

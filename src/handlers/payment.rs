use axum::{Router, Json, routing::post};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use stripe::{Client, CreatePaymentIntent, Currency, PaymentIntent, PaymentMethodId};

#[derive(Deserialize)]
pub struct PaymentRequest {
    amount: i64,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    success: bool,
    message: String,
    payment_intent_id: Option<String>,
}

async fn handle_payment(Json(payload): Json<PaymentRequest>) -> Json<PaymentResponse> {
    dotenv().ok();
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    let client = Client::new(secret_key);

    let mut params = CreatePaymentIntent::new(payload.amount, Currency::JPY);
    params.payment_method_types = Some(vec!["card".to_string()]);
    params.payment_method = Some("pm_card_visa".parse::<PaymentMethodId>().expect("Invalid PaymentMethodId"));
    params.confirm = Some(true);
    params.off_session = None; // Some(PaymentIntentOffSession::Exists(true));

    match PaymentIntent::create(&client, params).await {
        Ok(payment_intent) => Json(PaymentResponse {
            success: true,
            message: "PaymentIntent created successfully".to_string(),
            payment_intent_id: Some(payment_intent.id.to_string()),
        }),
        Err(err) => Json(PaymentResponse {
            success: false,
            message: format!("Failed to create PaymentIntent: {}", err),
            payment_intent_id: None,
        }),
    }
}

pub fn router() -> Router {
    Router::new().route("/payment", post(handle_payment))
}

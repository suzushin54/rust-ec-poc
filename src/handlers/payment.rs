use axum::{Router, Json, routing::post};
use serde::{Deserialize, Serialize};
use stripe::{Client, CreatePaymentIntent, Currency, PaymentIntent};

#[derive(Deserialize)]
pub struct PaymentRequest {
    amount: u64,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    success: bool,
    message: String,
    payment_intent_id: Option<String>,
}

async fn handle_payment(Json(payload): Json<PaymentRequest>) -> Json<PaymentResponse> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    let client = Client::new(secret_key);
    
    let create_payment = CreatePaymentIntent {
        amount: 200,
        currency: Currency::JPY,
        payment_method_types: Some(vec!["card".to_string()]),
        application_fee_amount: None,
        automatic_payment_methods: None,
        capture_method: None,
        confirm: None,
        confirmation_method: None,
        customer: None,
        description: None,
        error_on_requires_action: None,
        expand: &[],
        mandate: None,
        mandate_data: None,
        metadata: None,
        off_session: None,
        on_behalf_of: None,
        payment_method: None,
        payment_method_configuration: None,
        payment_method_data: None,
        payment_method_options: None,
        radar_options: None,
        receipt_email: None,
        return_url: None,
        setup_future_usage: None,
        shipping: None,
        statement_descriptor: None,
        statement_descriptor_suffix: None,
        transfer_data: None,
        transfer_group: None,
        use_stripe_sdk: None,
    };

    match PaymentIntent::create(&client, create_payment).await {
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

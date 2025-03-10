use axum::{Json, response::IntoResponse, extract::State};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::application::usecase::create_payment::{PaymentUseCase, PaymentMethod};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub amount: i64,
    pub customer_id: String,
    pub payment_method: String,
}

#[derive(Serialize)]
pub struct PaymentResponse {
    pub success: bool,
    pub message: String,
    pub payment_id: Option<String>,
}

impl FromStr for PaymentMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "card" => Ok(PaymentMethod::Card),
            "bank_transfer" => Ok(PaymentMethod::BankTransfer),
            "convenience_store" => Ok(PaymentMethod::ConvenienceStore),
            _ => Err(format!("Invalid payment method: {}", s)),
        }
    }
}

pub async fn handle_payment(
    State(state): State<AppState>,
    Json(payload): Json<PaymentRequest>,
) -> impl IntoResponse {
    let payment_usecase = PaymentUseCase::new(state.stripe_client);

    // Convert payment_method to PaymentMethod enum
    let payment_method = match PaymentMethod::from_str(&payload.payment_method) {
        Ok(method) => method,
        Err(err) => {
            return Json(PaymentResponse {
                success: false,
                message: err,
                payment_id: None,
            });
        }
    };

    match payment_usecase
        .execute(payload.amount, payload.customer_id, payment_method)
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

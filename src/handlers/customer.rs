use axum::{Json, response::IntoResponse, extract::State};
use serde::{Deserialize, Serialize};
use crate::application::usecase::create_stripe_customer::CreateStripeCustomerUseCase;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CustomerRequest {
    pub email: String,
    pub token: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    pub success: bool,
    pub message: String,
    pub customer_id: Option<String>,
}

pub async fn handle_create_customer(
    State(state): State<AppState>,
    Json(payload): Json<CustomerRequest>,
) -> impl IntoResponse {
    let customer_usecase = CreateStripeCustomerUseCase::new(state.stripe_client);

    match customer_usecase
        .execute(payload.email.clone(), payload.token.clone(), payload.name.clone())
        .await
    {
        Ok(customer_id) => Json(CustomerResponse {
            success: true,
            message: "Customer created successfully".to_string(),
            customer_id: Some(customer_id),
        }),
        Err(err) => Json(CustomerResponse {
            success: false,
            message: err,
            customer_id: None,
        }),
    }
}

use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use crate::application::usecase::create_stripe_customer::CreateStripeCustomerUseCase;
use crate::adapters::stripe_client::StripeClient;

#[derive(Deserialize)]
pub struct CustomerRequest {
    pub email: String,
    pub token: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    pub success: bool,
    pub message: String,
    pub customer_id: Option<String>,
}

pub async fn handle_create_customer(Json(payload): Json<CustomerRequest>) -> impl IntoResponse {
    // NOTE: stateで受け取っても良いが、サブルーターにしないと無関係の処理でも受け取ってしまうため検討
    dotenv().ok();
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    let stripe_client = StripeClient::new(secret_key);
    let customer_usecase = CreateStripeCustomerUseCase::new(stripe_client);

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

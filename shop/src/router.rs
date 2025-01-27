use axum::{Router, routing::post, routing::get};
use crate::handlers::{customer::handle_create_customer, payment::handle_payment};
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new() 
        .route("/", get(|| async { "Hello, World!" }))
        .route("/payment", post(handle_payment))
        .route("/customer", post(handle_create_customer))
        .with_state(state)
}

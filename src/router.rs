use axum::{Router, routing::post, routing::get};
use crate::handlers::{customer::handle_create_customer, payment::handle_payment};

pub fn create_router() -> Router {
    Router::new() 
        .route("/", get(|| async { "Hello, World!" }))
        .route("/payment", post(handle_payment))
        .route("/customer", post(handle_create_customer))
}

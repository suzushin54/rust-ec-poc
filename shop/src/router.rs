use axum::{Router, routing::post, routing::get};
use crate::handlers::{customer::handle_create_customer, payment::handle_payment};
use crate::state::AppState;
use management;

async fn handle_management_console() -> &'static str {
    // managementクレートの機能を呼び出し
    management::init();
    "Management console initialized"
}

pub fn create_router(state: AppState) -> Router {
    Router::new() 
        .route("/", get(|| async { "Hello, World!" }))
        .route("/payment", post(handle_payment))
        .route("/customer", post(handle_create_customer))
        .route("/management/console", get(handle_management_console))
        .with_state(state)
}

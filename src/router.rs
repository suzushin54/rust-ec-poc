use axum::{Router, routing::get};
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(handlers::payment::router())
}

mod router;
mod handlers;
mod adapters;
mod application;
mod state;

use dotenv::dotenv;
use std::env;
use crate::state::AppState;
use crate::adapters::stripe_client::StripeClient;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

    info!("Starting the server...");

    dotenv().ok();
    let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY not set");
    let stripe_client = StripeClient::new(secret_key);
    
    let state = AppState::new(stripe_client);
    let app = router::create_router(state);

    let address = "0.0.0.0:8080";
    info!("Listening on {}", address);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

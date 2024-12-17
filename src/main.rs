mod router;
mod handlers;

use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .init();

    info!("Starting the server...");

    // build our application with a single route
    let app = router::create_router();

    let address = "0.0.0.0:8080";
    info!("Listening on {}", address);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

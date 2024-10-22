use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Set up logging
    tracing_subscriber::fmt::init();

    // TODO: Define the API routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/calculate", get(calculate_handler));

    // Read the port from environment variables or default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));

    tracing::info!("Server running at http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Welcome to the Data Processor API"
}

async fn calculate_handler() -> &'static str {
    "Calculation endpoint"
}

use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use tokio::task;
mod data_stream;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Set up logging
    tracing_subscriber::fmt::init();

    // Initialize the data stream simulation
    task::spawn(async {
        data_stream::simulate_data_stream().await;
    });

    // API routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/top-consumers", get(top_consumers_handler))
        .route(
            "/thirty-percent-consumers",
            get(thirty_percent_consumers_handler),
        )
        .route("/forecast", get(forecast_handler));

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

// TODO: Handlers for the endpoints
async fn root_handler() -> &'static str {
    "Welcome to the Data Processor API"
}

async fn top_consumers_handler() -> &'static str {
    "Top consumers data"
}

async fn thirty_percent_consumers_handler() -> &'static str {
    "Thirty percent consumers data"
}

async fn forecast_handler() -> &'static str {
    "Forecast data"
}

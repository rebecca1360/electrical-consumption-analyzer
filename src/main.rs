use axum::extract::State;
use axum::{routing::get, Router};
use dotenvy::dotenv;
use models::consumer::ConsumerData;
use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::task;
mod data_stream;
mod models;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv().ok();

    // Set up logging
    tracing_subscriber::fmt::init();

    // Create shared state for consumer aggregation
    let consumer_aggregation = Arc::new(Mutex::new(HashMap::<String, ConsumerData>::new()));

    // Clone the shared state and pass it to the simulation
    let simulation_state = consumer_aggregation.clone();
    task::spawn(async move {
        data_stream::simulate_data_stream(simulation_state).await;
    });

    // Set up the app with routes and shared state
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/top-consumers", get(top_consumers_handler))
        .route(
            "/thirty-percent-consumers",
            get(thirty_percent_consumers_handler),
        )
        .route("/forecast", get(forecast_handler))
        .with_state(consumer_aggregation); // Use `.with_state` for shared state

    // Read the port from environment variables or default to 3000
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));

    tracing::info!("Server running at http://{}", addr);

    // Start the server with the app
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> &'static str {
    "Welcome to the Data Processor API"
}

pub async fn top_consumers_handler(
    State(consumer_aggregation): State<Arc<Mutex<HashMap<String, ConsumerData>>>>,
) -> String {
    let aggregation = consumer_aggregation.lock().await;

    // Convert the HashMap to a vector and sort by total consumption (descending)
    let mut consumer_list: Vec<(&String, &ConsumerData)> = aggregation.iter().collect();
    consumer_list.sort_by(|a, b| {
        b.1.total_consumption
            .partial_cmp(&a.1.total_consumption)
            .unwrap()
    });

    // Collect top consumers (e.g., top 5)
    let top_consumers: Vec<String> = consumer_list
        .iter()
        .take(5)
        .map(|(id, data)| {
            format!(
                "Consumer ID: {}, Total Consumption: {:.2} kWh",
                id, data.total_consumption
            )
        })
        .collect();

    // Create a response to display
    if top_consumers.is_empty() {
        "No consumers data available.".to_string()
    } else {
        format!("Top Consumers:\n{}", top_consumers.join("\n"))
    }
}

// TODO: Add functionality for the endpoints
async fn thirty_percent_consumers_handler() -> &'static str {
    "Thirty percent consumers data"
}

async fn forecast_handler() -> &'static str {
    "Forecast data"
}

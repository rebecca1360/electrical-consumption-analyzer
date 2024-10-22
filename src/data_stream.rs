use std::{collections::HashMap, sync::Arc};

use chrono::Utc;
use rand::{distributions::Uniform, Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;
use serde_json::{json, Value};
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};
use tracing::{error, info};

use crate::models::{
    consumer::{aggregate_data, ConsumerData},
    meter_reading::{MeterReading, MeterReadingError},
};

// Data stream simulation
pub async fn simulate_data_stream(consumer_aggregation: Arc<Mutex<HashMap<String, ConsumerData>>>) {
    let meter_ids = vec!["MTR-001", "MTR-002", "MTR-003"];
    let consumer_ids = vec!["CNS-001", "CNS-002", "CNS-003"];

    // Thread-safe RNG
    let mut rng = ChaCha12Rng::from_entropy();
    let interval_dist = Uniform::new(500, 3000);
    let value_dist = Uniform::new(0.0, 100.0);

    loop {
        let anomaly_chance: f64 = rng.gen();
        // Generate reading that could be abnormal
        let reading = if anomaly_chance < 0.1 {
            generate_anomalous_reading(&meter_ids, &consumer_ids, &mut rng, &value_dist)
        } else {
            generate_normal_reading(&meter_ids, &consumer_ids, &mut rng, &value_dist)
        };

        match parse_meter_reading(&reading) {
            Ok(parsed) => {
                info!("Successfully parsed reading: {:?}", parsed);
                aggregate_data(consumer_aggregation.clone(), parsed).await;
            }
            Err(err) => error!("Failed to parse reading: {}", err),
        }

        // Sleep for a random interval to simulate irregular intervals
        let interval_ms = rng.sample(&interval_dist);
        sleep(Duration::from_millis(interval_ms)).await;
    }
}

// Generate a normal reading
fn generate_normal_reading(
    meter_ids: &[&str],
    consumer_ids: &[&str],
    rng: &mut impl Rng,
    value_dist: &Uniform<f64>,
) -> serde_json::Value {
    let meter_id = meter_ids[rng.gen_range(0..meter_ids.len())].to_string();
    let consumer_id = consumer_ids[rng.gen_range(0..consumer_ids.len())].to_string();
    let consumption = rng.sample(value_dist);

    json!({
        "timestamp": Utc::now().to_string(),
        "meter_id": meter_id,
        "consumer_id": consumer_id,
        "consumption": consumption
    })
}

// Generate anomalous reading
fn generate_anomalous_reading(
    meter_ids: &[&str],
    consumer_ids: &[&str],
    rng: &mut impl Rng,
    value_dist: &Uniform<f64>,
) -> serde_json::Value {
    let anomaly_type = rng.gen_range(0..4);
    let meter_id = meter_ids[rng.gen_range(0..meter_ids.len())].to_string();
    let consumer_id = consumer_ids[rng.gen_range(0..consumer_ids.len())].to_string();
    let consumption = rng.sample(value_dist);

    match anomaly_type {
        0 => {
            // Missing meter_id
            json!({
                "timestamp": Utc::now().to_string(),
                "consumer_id": consumer_id,
                "consumption": consumption
            })
        }
        1 => {
            // Wrong data type for meter_id
            json!({
                "timestamp": Utc::now().to_string(),
                "meter_id": 12345, // Wrong type
                "consumer_id": consumer_id,
                "consumption": consumption
            })
        }
        2 => {
            // Missing consumer_id
            json!({
                "timestamp": Utc::now().to_string(),
                "meter_id": meter_id,
                "consumption": consumption
            })
        }
        3 => {
            // Wrong data type for consumption
            json!({
                "timestamp": Utc::now().to_string(),
                "meter_id": meter_id,
                "consumer_id": consumer_id,
                "consumption": "InvalidType"
            })
        }
        _ => {
            // Missing consumption
            json!({
                "timestamp": Utc::now().to_string(),
                "meter_id": meter_id,
                "consumer_id": consumer_id
            })
        }
    }
}

pub fn parse_meter_reading(json_value: &Value) -> Result<MeterReading, MeterReadingError> {
    let max_consumption = 1000.0;

    // Parse json
    let parsed: MeterReading =
        serde_json::from_value(json_value.clone()).map_err(MeterReadingError::JsonError)?;

    // Additional validation
    if parsed.consumption < 0.0 {
        return Err(MeterReadingError::ValidationError(
            "Consumption cannot be negative".to_string(),
        ));
    }

    if parsed.consumption > max_consumption {
        return Err(MeterReadingError::ValidationError(
            "Consumption is too high".to_string(),
        ));
    }

    if parsed.meter_id.is_empty() {
        return Err(MeterReadingError::ValidationError(
            "Meter ID cannot be empty".to_string(),
        ));
    }
    if parsed.consumer_id.is_empty() {
        return Err(MeterReadingError::ValidationError(
            "Consumer ID cannot be empty".to_string(),
        ));
    }

    Ok(parsed)
}

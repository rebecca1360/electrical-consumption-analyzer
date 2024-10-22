use super::meter_reading::MeterReading;

use chrono::{DateTime, Utc};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct ConsumerData {
    pub total_consumption: f64,
    pub last_reading_time: Option<DateTime<Utc>>,
}

impl ConsumerData {
    fn new() -> Self {
        Self {
            total_consumption: 0.0,
            last_reading_time: None,
        }
    }

    fn add_reading(&mut self, reading: f64, timestamp: DateTime<Utc>) {
        self.total_consumption += reading;
        self.last_reading_time = Some(timestamp);
    }
}

pub async fn aggregate_data(
    consumer_aggregation: Arc<Mutex<HashMap<String, ConsumerData>>>,
    reading: MeterReading,
) {
    let mut aggregation = consumer_aggregation.lock().await;
    let consumer_entry = aggregation
        .entry(reading.consumer_id.clone())
        .or_insert_with(ConsumerData::new);

    consumer_entry.add_reading(reading.consumption, reading.timestamp);
}

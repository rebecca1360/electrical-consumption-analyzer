use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct MeterReading {
    pub timestamp: DateTime<Utc>,
    pub meter_id: String,
    pub consumer_id: String,
    pub consumption: f64,
}

impl MeterReading {}

#[derive(Debug, Error)]
pub enum MeterReadingError {
    #[error("Invalid JSON format: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Missing or invalid field: {0}")]
    ValidationError(String),
}


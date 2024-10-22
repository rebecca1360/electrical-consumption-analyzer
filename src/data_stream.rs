use chrono::{DateTime, Utc};
use rand::{distributions::Uniform, Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct MeterReading {
    pub timestamp: DateTime<Utc>,
    pub meter_id: String,
    pub consumer_id: String,
    pub consumption: f64,
}

// Data stream simulation 
pub async fn simulate_data_stream() {
    let meter_ids = vec!["MTR-001", "MTR-002", "MTR-003"];
    let consumer_ids = vec!["CNS-001", "CNS-002", "CNS-003"];

    // Thread-safe RNG
    let mut rng = ChaCha12Rng::from_entropy(); 
    let interval_dist = Uniform::new(500, 3000); 
    let value_dist = Uniform::new(0.0, 100.0); 

    loop {
        // Generate a normal reading
        let reading = generate_normal_reading(&meter_ids, &consumer_ids, &mut rng, &value_dist);

        // Print reading
        println!("{:?}", reading);

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
) -> MeterReading {
    let meter_id = meter_ids[rng.gen_range(0..meter_ids.len())].to_string();
    let consumer_id = consumer_ids[rng.gen_range(0..consumer_ids.len())].to_string();
    let consumption = rng.sample(value_dist);

    MeterReading {
        timestamp: Utc::now(),
        meter_id,
        consumer_id,
        consumption,
    }
}

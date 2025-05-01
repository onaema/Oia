use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn calculate_profit(entry: f64, exit: f64) -> f64 {
    (exit - entry) / entry * 100.0
}
//! Helpers: Time Functions, Math Utilities, Format Helpers

pub fn now_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn format_amount(amount: f64) -> String {
    format!("{:.4}", amount)
}

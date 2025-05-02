//! Logger: Error Logging, Trade Logging, Performance Logging

pub fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}

pub fn log_trade(msg: &str) {
    println!("[TRADE] {}", msg);
}

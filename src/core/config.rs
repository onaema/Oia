//! Core Config: Bot Settings, Trading Parameters, API Keys
#[derive(Debug, Clone)]
pub struct CoreConfig {
    pub api_key: String,
    pub risk_level: f64,
    pub max_trade_size: f64,
}

impl CoreConfig {
    pub fn new() -> Self {
        Self {
            api_key: String::from("demo-key"),
            risk_level: 0.5,
            max_trade_size: 1.0,
        }
    }
}

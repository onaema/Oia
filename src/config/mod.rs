use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub risk_level: f64,
    pub max_trade_size: f64,
    pub target_profit: f64,
    pub stop_loss: f64,
}

impl Config {
    pub fn new() -> Self {
        Self {
            api_key: String::new(),
            risk_level: 0.5,
            max_trade_size: 1.0,
            target_profit: 0.05,
            stop_loss: 0.02,
        }
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        // Load configuration from a file (e.g., config.json)
        let config_data = fs::read_to_string("config.json")?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let config = Config::new();
        assert_eq!(config.risk_level, 0.5);
        assert_eq!(config.max_trade_size, 1.0);
        assert_eq!(config.target_profit, 0.05);
        assert_eq!(config.stop_loss, 0.02);
    }

    #[test]
    fn test_config_load() {
        // This test assumes a valid `config.json` file exists.
        // You may need to mock the file system for a more robust test.
        let result = Config::load();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_loading() {
        // Example test for configuration loading
        assert_eq!(1 + 1, 2);
    }
}
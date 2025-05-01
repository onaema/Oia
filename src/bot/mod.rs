use crate::config::Config;
use crate::trading::Strategy;
use solana_client::rpc_client::RpcClient;
use std::error::Error;
use std::sync::Arc;

pub mod state;
pub mod commands;

pub struct Bot {
    config: Config,
    strategy: Strategy,
}

impl Bot {
    pub fn new(config: Config) -> Self {
        Self {
            strategy: Strategy::new(&config),
            config,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Bot running...");
        Ok(())
    }
}

pub async fn start(rpc_client: Arc<RpcClient>) -> Result<(), Box<dyn Error>> {
    // Load configuration
    let config = Config::load()?;

    // Initialize the bot
    let mut bot = Bot::new(config);

    // Run the bot
    bot.run().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_initialization() {
        let config = Config::new();
        let bot = Bot::new(config);
        assert_eq!(bot.config.risk_level, 0.5);
        assert_eq!(bot.config.max_trade_size, 1.0);
    }
}
use crate::config::Config;
use crate::trading::Strategy;

pub struct BotState {
    pub config: Config,
    pub strategy: Strategy,
}

impl BotState {
    pub fn new(config: Config) -> Self {
        Self {
            strategy: Strategy::new(&config),
            config,
        }
    }
}
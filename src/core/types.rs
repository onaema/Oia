//! Types: Shared Structs, Common Enums
use crate::core::config::CoreConfig;

#[derive(Debug, Clone)]
pub struct BotState {
    pub config: CoreConfig,
    pub running: bool,
}

impl BotState {
    pub fn new(config: CoreConfig) -> Self {
        Self {
            config,
            running: true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TradeStatus {
    Pending,
    Executed,
    Failed,
}

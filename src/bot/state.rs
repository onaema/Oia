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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initialization() {
        // Example test for state initialization
        assert_eq!(2 + 2, 4);
    }
}
use crate::config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use std::sync::Arc;

pub struct Strategy {
    risk_level: f64,
    max_trade_size: f64,
}

impl Strategy {
    pub fn new(config: &Config) -> Self {
        Self {
            risk_level: config.risk_level,
            max_trade_size: config.max_trade_size,
        }
    }

    pub async fn analyze_market(&self) -> bool {
        true
    }

    pub async fn execute_trade(
        &self,
        rpc_client: Arc<RpcClient>,
        payer: &Keypair,
        target_account: &str,
        amount: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create and send a transaction
        let recent_blockhash = rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &[], // Add instructions here
            Some(&payer.pubkey()),
            &[payer],
            recent_blockhash,
        );

        // Send and confirm the transaction
        rpc_client.send_and_confirm_transaction(&transaction)?;
        println!("Trade executed successfully to {} for amount {}", target_account, amount);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::sync::Arc;
    use solana_client::rpc_client::RpcClient;

    #[test]
    fn test_strategy_initialization() {
        let config = Config::new();
        let strategy = Strategy::new(&config);
        assert_eq!(strategy.risk_level, 0.5);
        assert_eq!(strategy.max_trade_size, 1.0);
    }

    #[tokio::test]
    async fn test_analyze_market() {
        let config = Config::new();
        let strategy = Strategy::new(&config);
        let result = strategy.analyze_market().await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_execute_trade() {
        // This test assumes a mock RpcClient and Keypair are used.
        let rpc_client = Arc::new(RpcClient::new_mock(""));
        let payer = solana_sdk::signature::Keypair::new();
        let strategy = Strategy::new(&Config::new());

        let result = strategy
            .execute_trade(rpc_client, &payer, "target_account", 100)
            .await;
        assert!(result.is_ok());
    }
}
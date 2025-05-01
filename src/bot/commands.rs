use std::error::Error;
use std::sync::Arc;
use solana_client::rpc_client::RpcClient;
use crate::bot::state::BotState;

pub async fn start(rpc_client: Arc<RpcClient>) -> Result<(), Box<dyn Error>> {
    // Load configuration
    let config = crate::config::Config::load()?;

    // Initialize the bot state
    let mut bot_state = BotState::new(config);

    // Run the bot
    println!("Bot running...");
    Ok(())
}
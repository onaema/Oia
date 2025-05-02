//! Sniper: Meme Token Sniping Engine
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::telegram::alerts::send_trade_alert;
use crate::integration::dex::{PumpFun, Birdeye, Dexscreener, Raydium, Jupiter};
use crate::integration::solana::execute_real_trade;
use solana_sdk::signature::Keypair;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashSet;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnipeConfig {
    pub min_liquidity: f64,
    pub max_buy: f64,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub auto_buy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnipeEvent {
    pub token: String,
    pub time: DateTime<Utc>,
    pub tx_hash: Option<String>,
    pub status: String,
    pub price: Option<f64>,
}

pub struct Sniper {
    pub config: SnipeConfig,
    pub history: Vec<SnipeEvent>,
}

impl Sniper {
    pub fn new(config: SnipeConfig) -> Self {
        Self { config, history: vec![] }
    }
    pub async fn monitor_and_snipe(&mut self) {
        let mut seen = HashSet::new();
        loop {
            // 1. Pantau token baru dari Pump.fun
            if let Ok(resp) = reqwest::get("https://api.pump.fun/api/tokens").await {
                if let Ok(list) = resp.json::<serde_json::Value>().await {
                    if let Some(tokens) = list.as_array() {
                        for token in tokens {
                            let addr = token.get("address").and_then(|v| v.as_str()).unwrap_or("");
                            if !seen.contains(addr)
                                && (self.config.whitelist.is_empty() || self.config.whitelist.contains(&addr.to_string()))
                                && !self.config.blacklist.contains(&addr.to_string())
                            {
                                let liq = token.get("liquidityUsd").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                if liq >= self.config.min_liquidity {
                                    let price = token.get("priceUsd").and_then(|v| v.as_f64());
                                    let event = SnipeEvent {
                                        token: addr.to_string(),
                                        time: Utc::now(),
                                        tx_hash: None,
                                        status: "DETECTED (Pump.fun)".to_string(),
                                        price,
                                    };
                                    self.log_event(event);
                                    seen.insert(addr.to_string());
                                    if self.config.auto_buy {
                                        // Eksekusi pembelian nyata via Solana SDK
                                        let mut tx_hash = None;
                                        let mut status = "BOUGHT (Pump.fun)".to_string();
                                        if let (Ok(pk), Ok(target)) = (
                                            std::env::var("SOLANA_PRIVATE_KEY").map(|k| Keypair::from_base58_string(&k)),
                                            std::env::var("SOLANA_TARGET").map(|a| Pubkey::from_str(&a).unwrap_or(Pubkey::new_unique()))
                                        ) {
                                            let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
                                            match execute_real_trade(&rpc, &pk, &target, self.config.max_buy as u64) {
                                                Ok(sig) => tx_hash = Some(sig),
                                                Err(e) => { status = format!("FAILED: {}", e); }
                                            }
                                        }
                                        let buy_event = SnipeEvent {
                                            token: addr.to_string(),
                                            time: Utc::now(),
                                            tx_hash,
                                            status,
                                            price,
                                        };
                                        self.log_event(buy_event);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // 2. Pantau token baru dari Birdeye
            if let Ok(resp) = reqwest::get("https://public-api.birdeye.so/public/tokenlist?chain=solana").await {
                if let Ok(list) = resp.json::<serde_json::Value>().await {
                    if let Some(tokens) = list.get("data").and_then(|d| d.as_array()) {
                        for token in tokens {
                            let addr = token.get("address").and_then(|v| v.as_str()).unwrap_or("");
                            if !seen.contains(addr)
                                && (self.config.whitelist.is_empty() || self.config.whitelist.contains(&addr.to_string()))
                                && !self.config.blacklist.contains(&addr.to_string())
                            {
                                let price = None;
                                let event = SnipeEvent {
                                    token: addr.to_string(),
                                    time: Utc::now(),
                                    tx_hash: None,
                                    status: "DETECTED (Birdeye)".to_string(),
                                    price,
                                };
                                self.log_event(event);
                                seen.insert(addr.to_string());
                                if self.config.auto_buy {
                                    // Eksekusi pembelian nyata via Solana SDK
                                    let mut tx_hash = None;
                                    let mut status = "BOUGHT (Birdeye)".to_string();
                                    if let (Ok(pk), Ok(target)) = (
                                        std::env::var("SOLANA_PRIVATE_KEY").map(|k| Keypair::from_base58_string(&k)),
                                        std::env::var("SOLANA_TARGET").map(|a| Pubkey::from_str(&a).unwrap_or(Pubkey::new_unique()))
                                    ) {
                                        let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
                                        match execute_real_trade(&rpc, &pk, &target, self.config.max_buy as u64) {
                                            Ok(sig) => tx_hash = Some(sig),
                                            Err(e) => { status = format!("FAILED: {}", e); }
                                        }
                                    }
                                    let buy_event = SnipeEvent {
                                        token: addr.to_string(),
                                        time: Utc::now(),
                                        tx_hash,
                                        status,
                                        price,
                                    };
                                    self.log_event(buy_event);
                                }
                            }
                        }
                    }
                }
            }
            // 3. Pantau token baru dari Dexscreener
            if let Ok(resp) = reqwest::get("https://api.dexscreener.com/latest/dex/tokens/solana").await {
                if let Ok(list) = resp.json::<serde_json::Value>().await {
                    if let Some(pairs) = list.get("pairs").and_then(|d| d.as_array()) {
                        for pair in pairs {
                            let addr = pair.get("address").and_then(|v| v.as_str()).unwrap_or("");
                            if !seen.contains(addr)
                                && (self.config.whitelist.is_empty() || self.config.whitelist.contains(&addr.to_string()))
                                && !self.config.blacklist.contains(&addr.to_string())
                            {
                                let price = pair.get("priceUsd").and_then(|v| v.as_str()).and_then(|s| s.parse().ok());
                                let event = SnipeEvent {
                                    token: addr.to_string(),
                                    time: Utc::now(),
                                    tx_hash: None,
                                    status: "DETECTED (Dexscreener)".to_string(),
                                    price,
                                };
                                self.log_event(event);
                                seen.insert(addr.to_string());
                                if self.config.auto_buy {
                                    // Eksekusi pembelian nyata via Solana SDK
                                    let mut tx_hash = None;
                                    let mut status = "BOUGHT (Dexscreener)".to_string();
                                    if let (Ok(pk), Ok(target)) = (
                                        std::env::var("SOLANA_PRIVATE_KEY").map(|k| Keypair::from_base58_string(&k)),
                                        std::env::var("SOLANA_TARGET").map(|a| Pubkey::from_str(&a).unwrap_or(Pubkey::new_unique()))
                                    ) {
                                        let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
                                        match execute_real_trade(&rpc, &pk, &target, self.config.max_buy as u64) {
                                            Ok(sig) => tx_hash = Some(sig),
                                            Err(e) => { status = format!("FAILED: {}", e); }
                                        }
                                    }
                                    let buy_event = SnipeEvent {
                                        token: addr.to_string(),
                                        time: Utc::now(),
                                        tx_hash,
                                        status,
                                        price,
                                    };
                                    self.log_event(buy_event);
                                }
                            }
                        }
                    }
                }
            }
            // 4. Pantau token baru dari Raydium (pool list)
            if let Ok(resp) = reqwest::get("https://api.raydium.io/v2/sdk/liquidity/mainnet.json").await {
                if let Ok(list) = resp.json::<serde_json::Value>().await {
                    if let Some(pools) = list.get("official").and_then(|d| d.as_array()) {
                        for pool in pools {
                            let addr = pool.get("lpMint").and_then(|v| v.as_str()).unwrap_or("");
                            if !seen.contains(addr)
                                && (self.config.whitelist.is_empty() || self.config.whitelist.contains(&addr.to_string()))
                                && !self.config.blacklist.contains(&addr.to_string())
                            {
                                let price = None;
                                let event = SnipeEvent {
                                    token: addr.to_string(),
                                    time: Utc::now(),
                                    tx_hash: None,
                                    status: "DETECTED (Raydium)".to_string(),
                                    price,
                                };
                                self.log_event(event);
                                seen.insert(addr.to_string());
                                if self.config.auto_buy {
                                    // Eksekusi pembelian nyata via Solana SDK
                                    let mut tx_hash = None;
                                    let mut status = "BOUGHT (Raydium)".to_string();
                                    if let (Ok(pk), Ok(target)) = (
                                        std::env::var("SOLANA_PRIVATE_KEY").map(|k| Keypair::from_base58_string(&k)),
                                        std::env::var("SOLANA_TARGET").map(|a| Pubkey::from_str(&a).unwrap_or(Pubkey::new_unique()))
                                    ) {
                                        let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
                                        match execute_real_trade(&rpc, &pk, &target, self.config.max_buy as u64) {
                                            Ok(sig) => tx_hash = Some(sig),
                                            Err(e) => { status = format!("FAILED: {}", e); }
                                        }
                                    }
                                    let buy_event = SnipeEvent {
                                        token: addr.to_string(),
                                        time: Utc::now(),
                                        tx_hash,
                                        status,
                                        price,
                                    };
                                    self.log_event(buy_event);
                                }
                            }
                        }
                    }
                }
            }
            // 5. Pantau token baru dari Jupiter (token list)
            if let Ok(resp) = reqwest::get("https://token.jup.ag/all").await {
                if let Ok(list) = resp.json::<serde_json::Value>().await {
                    if let Some(tokens) = list.as_array() {
                        for token in tokens {
                            let addr = token.get("address").and_then(|v| v.as_str()).unwrap_or("");
                            if !seen.contains(addr)
                                && (self.config.whitelist.is_empty() || self.config.whitelist.contains(&addr.to_string()))
                                && !self.config.blacklist.contains(&addr.to_string())
                            {
                                let price = None;
                                let event = SnipeEvent {
                                    token: addr.to_string(),
                                    time: Utc::now(),
                                    tx_hash: None,
                                    status: "DETECTED (Jupiter)".to_string(),
                                    price,
                                };
                                self.log_event(event);
                                seen.insert(addr.to_string());
                                if self.config.auto_buy {
                                    // Eksekusi pembelian nyata via Solana SDK
                                    let mut tx_hash = None;
                                    let mut status = "BOUGHT (Jupiter)".to_string();
                                    if let (Ok(pk), Ok(target)) = (
                                        std::env::var("SOLANA_PRIVATE_KEY").map(|k| Keypair::from_base58_string(&k)),
                                        std::env::var("SOLANA_TARGET").map(|a| Pubkey::from_str(&a).unwrap_or(Pubkey::new_unique()))
                                    ) {
                                        let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
                                        match execute_real_trade(&rpc, &pk, &target, self.config.max_buy as u64) {
                                            Ok(sig) => tx_hash = Some(sig),
                                            Err(e) => { status = format!("FAILED: {}", e); }
                                        }
                                    }
                                    let buy_event = SnipeEvent {
                                        token: addr.to_string(),
                                        time: Utc::now(),
                                        tx_hash,
                                        status,
                                        price,
                                    };
                                    self.log_event(buy_event);
                                }
                            }
                        }
                    }
                }
            }
            sleep(Duration::from_secs(10)).await;
        }
    }
    pub fn log_event(&mut self, event: SnipeEvent) {
        self.history.push(event.clone());
        let msg = format!("[SNIPER] {} | status: {} | tx: {:?}", event.token, event.status, event.tx_hash);
        send_trade_alert(&msg);
    }
    pub fn get_history(&self) -> &[SnipeEvent] {
        &self.history
    }
    pub fn update_config(&mut self, config: SnipeConfig) {
        self.config = config;
    }
}

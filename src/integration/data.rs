//! Data Integration: Market Data, Historical Data, Price Feeds

use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use lazy_static::lazy_static;

// Struktur cache harga sederhana (in-memory, TTL 30 detik)
pub struct PriceCacheEntry {
    pub price: f64,
    pub timestamp: Instant,
}

lazy_static! {
    static ref PRICE_CACHE: Arc<Mutex<HashMap<(String, String), PriceCacheEntry>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn get_cached_price(source: &str, symbol: &str, fetch_fn: impl Fn(&str) -> std::pin::Pin<Box<dyn std::future::Future<Output=Option<f64>> + Send>>) -> Option<f64> {
    let key = (source.to_string(), symbol.to_string());
    let mut cache = PRICE_CACHE.lock().unwrap();
    if let Some(entry) = cache.get(&key) {
        if entry.timestamp.elapsed() < Duration::from_secs(30) {
            return Some(entry.price);
        }
    }
    drop(cache);
    let price = fetch_fn(symbol).await?;
    let mut cache = PRICE_CACHE.lock().unwrap();
    cache.insert(key, PriceCacheEntry { price, timestamp: Instant::now() });
    Some(price)
}

pub struct MarketDataProvider;

impl MarketDataProvider {
    pub fn new() -> Self {
        Self
    }

    pub fn get_latest_price(&self, _symbol: &str) -> f64 {
        // Placeholder: return dummy price
        0.99
    }
}

// --- CoinGecko ---
pub struct CoinGecko;
impl CoinGecko {
    pub async fn get_token_price(symbol: &str) -> Option<f64> {
        let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", symbol);
        let resp = Client::new().get(&url).send().await.ok()?;
        let json: Value = resp.json().await.ok()?;
        json.get(symbol)?.get("usd")?.as_f64()
    }
}

// --- Dexscreener ---
pub struct Dexscreener;
impl Dexscreener {
    pub async fn get_token_price(symbol: &str) -> Option<f64> {
        let url = format!("https://api.dexscreener.com/latest/dex/tokens/{}", symbol);
        let resp = reqwest::Client::new().get(&url).send().await.ok()?;
        let json: serde_json::Value = resp.json().await.ok()?;
        // Dexscreener response: { "pairs": [ { "priceUsd": ... } ] }
        json.get("pairs")?.get(0)?.get("priceUsd")?.as_str()?.parse().ok()
    }
}

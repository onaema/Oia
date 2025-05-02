//! Web: REST API, WebSocket Server, Frontend Assets
use warp::Filter;
use crate::ui::dashboard::{get_dashboard_stats, DashboardStats};
use crate::ui::notifications::send_notification;
use warp::reply::Json;
use crate::integration::solana::{SolanaWallet, execute_real_trade};
use crate::db::repository::Repository;
use crate::db::models::TradeHistory;
use crate::telegram::alerts::send_trade_alert;
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::str::FromStr;
use crate::integration::data::get_cached_price;
use warp::filters::BoxedFilter;
use std::sync::Arc;
use tokio::sync::Mutex as TokioMutex;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::sniper::{Sniper, SnipeConfig, SnipeEvent};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static WS_CLIENTS: once_cell::sync::Lazy<Arc<AtomicUsize>> = once_cell::sync::Lazy::new(|| Arc::new(AtomicUsize::new(0)));
static SNIPER: Lazy<Mutex<Sniper>> = Lazy::new(|| Mutex::new(Sniper::new(SnipeConfig {
    min_liquidity: 1.0,
    max_buy: 0.1,
    whitelist: vec![],
    blacklist: vec![],
    auto_buy: false,
})));

/// Logging middleware for request details
pub fn with_logging() -> impl Filter<Extract = ((),), Error = std::convert::Infallible> + Clone {
    warp::log::custom(|info| {
        log::info!(
            "{} {} {} {} {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.remote_addr().map(|a| a.to_string()).unwrap_or_default(),
            info.elapsed()
        );
    })
}

#[derive(Deserialize)]
pub struct TradeRequest {
    pub symbol: String,
    pub amount: f64,
    pub side: String, // "buy" or "sell"
}

#[derive(Serialize)]
pub struct TradeResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct CancelTradeRequest {
    pub id: u64,
}

#[derive(Deserialize)]
pub struct UpdateTradeRequest {
    pub id: u64,
    pub status: String,
}

// Rate limiting: in-memory, per-IP, 30 req/menit
lazy_static::lazy_static! {
    static ref RATE_LIMIT: Arc<TokioMutex<HashMap<String, (u32, Instant)>>> = Arc::new(TokioMutex::new(HashMap::new()));
}

async fn check_rate_limit(ip: String) -> bool {
    let mut map = RATE_LIMIT.lock().await;
    let now = Instant::now();
    let entry = map.entry(ip).or_insert((0, now));
    if now.duration_since(entry.1) > Duration::from_secs(60) {
        *entry = (1, now);
        true
    } else if entry.0 < 30 {
        entry.0 += 1;
        true
    } else {
        false
    }
}

fn with_rate_limit() -> BoxedFilter<((),)> {
    warp::addr::remote()
        .and_then(|addr: Option<std::net::SocketAddr>| async move {
            let ip = addr.map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
            if check_rate_limit(ip).await {
                Ok(())
            } else {
                Err(warp::reject::custom(RateLimitRejection))
            }
        })
        .untuple_one()
        .boxed()
}

#[derive(Debug)]
struct RateLimitRejection;
impl warp::reject::Reject for RateLimitRejection {}

fn rate_limit_recover(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.find::<RateLimitRejection>().is_some() {
        let json = warp::reply::json(&serde_json::json!({"error": "Rate limit exceeded"}));
        Ok(warp::reply::with_status(json, warp::http::StatusCode::TOO_MANY_REQUESTS))
    } else {
        Err(err)
    }
}

async fn ws_handler(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();
    WS_CLIENTS.fetch_add(1, Ordering::SeqCst);
    // Kirim pesan harga dummy setiap 2 detik (bisa diganti harga DEX nyata)
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = interval.tick() => {
                // TODO: Ambil harga nyata dari DEX/cache
                let msg = serde_json::json!({"symbol": "SOL/USDC", "price": 123.45, "ts": chrono::Utc::now()});
                if tx.send(Message::text(msg.to_string())).await.is_err() {
                    break;
                }
            }
            Some(Ok(_msg)) = rx.next() => {
                // Bisa handle pesan dari client jika perlu
            }
            else => break,
        }
    }
    WS_CLIENTS.fetch_sub(1, Ordering::SeqCst);
}

/// Returns all API routes for the backend
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let hello = warp::path!("api" / "hello")
        .map(|| warp::reply::json(&{"message": "Hello from backend!"}));

    let status = warp::path!("api" / "status")
        .map(|| warp::reply::json(&{"status": "ok", "time": chrono::Utc::now().to_rfc3339()}));

    let dashboard = warp::path!("api" / "dashboard")
        .map(|| {
            let stats: DashboardStats = get_dashboard_stats();
            warp::reply::json(&stats)
        });

    let notify = warp::path!("api" / "notify" / String)
        .map(|msg: String| {
            send_notification(&msg);
            warp::reply::json(&{"notified": true, "message": msg})
        });

    let trade = warp::path!("api" / "trade")
        .and(warp::post())
        .and(warp::header::optional::<String>("x-api-key"))
        .and(warp::body::json())
        .and_then(|api_key: Option<String>, req: TradeRequest| async move {
            let valid_key = std::env::var("API_KEY").unwrap_or_else(|_| "secret".to_string());
            if api_key.as_deref() != Some(&valid_key) {
                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                    warp::reply::json(&TradeResponse {
                        success: false,
                        message: "Unauthorized".to_string(),
                    }),
                    warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
            if req.amount <= 0.0 || req.symbol.trim().is_empty() || !(req.side == "buy" || req.side == "sell") {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&TradeResponse {
                        success: false,
                        message: "Invalid input".to_string(),
                    }),
                    warp::http::StatusCode::BAD_REQUEST,
                ));
            }
            // --- Trading nyata ke Solana ---
            let mut tx_result = None;
            if std::env::var("REAL_TRADE").unwrap_or_default() == "1" {
                let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
                // Ganti dengan private key base58 Anda
                let payer = Keypair::from_base58_string(&std::env::var("SOLANA_PRIVATE_KEY").unwrap_or_default());
                let target = Pubkey::from_str(&std::env::var("SOLANA_TARGET").unwrap_or_default()).unwrap_or(Pubkey::new_unique());
                match execute_real_trade(&rpc, &payer, &target, req.amount as u64) {
                    Ok(sig) => tx_result = Some(sig),
                    Err(e) => {
                        return Ok(warp::reply::with_status(
                            warp::reply::json(&TradeResponse {
                                success: false,
                                message: format!("Solana tx error: {}", e),
                            }),
                            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                }
            }
            // --- Simpan trade ke DB ---
            let trade = TradeHistory {
                id: crate::utils::helpers::now_timestamp(),
                symbol: req.symbol.clone(),
                amount: req.amount,
                status: format!("{} executed", req.side),
            };
            let repo = Repository::new();
            if let Err(e) = std::panic::catch_unwind(|| repo.save_trade(&trade)) {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&TradeResponse {
                        success: false,
                        message: format!("Failed to save trade: {:?}", e),
                    }),
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
            send_notification(&format!("Trade {} {} {}", req.side, req.amount, req.symbol));
            send_trade_alert(&format!("Trade {} {} {}", req.side, req.amount, req.symbol));
            Ok(warp::reply::json(&TradeResponse {
                success: true,
                message: format!("Trade {} {} {} success{}", req.side, req.amount, req.symbol, tx_result.map(|sig| format!(" (tx: {})", sig)).unwrap_or_default()),
            }))
        });

    let cancel_trade = warp::path!("api" / "trade" / "cancel")
        .and(warp::post())
        .and(warp::header::optional::<String>("x-api-key"))
        .and(warp::body::json())
        .and_then(|api_key: Option<String>, req: CancelTradeRequest| async move {
            let valid_key = std::env::var("API_KEY").unwrap_or_else(|_| "secret".to_string());
            if api_key.as_deref() != Some(&valid_key) {
                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                    warp::reply::json(&{"success": false, "message": "Unauthorized"}),
                    warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
            let repo = Repository::new();
            let result = repo.cancel_trade(req.id);
            if result {
                Ok(warp::reply::json(&{"success": true, "message": "Trade cancelled"}))
            } else {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&{"success": false, "message": "Trade not found"}),
                    warp::http::StatusCode::NOT_FOUND,
                ));
            }
        });

    let update_trade = warp::path!("api" / "trade" / "update")
        .and(warp::post())
        .and(warp::header::optional::<String>("x-api-key"))
        .and(warp::body::json())
        .and_then(|api_key: Option<String>, req: UpdateTradeRequest| async move {
            let valid_key = std::env::var("API_KEY").unwrap_or_else(|_| "secret".to_string());
            if api_key.as_deref() != Some(&valid_key) {
                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                    warp::reply::json(&{"success": false, "message": "Unauthorized"}),
                    warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
            let repo = Repository::new();
            let result = repo.update_trade_status(req.id, &req.status);
            if result {
                Ok(warp::reply::json(&{"success": true, "message": "Trade updated"}))
            } else {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&{"success": false, "message": "Trade not found"}),
                    warp::http::StatusCode::NOT_FOUND,
                ));
            }
        });

    let wallet_balance = warp::path!("api" / "wallet" / "balance")
        .and(warp::header::optional::<String>("x-api-key"))
        .and_then(|api_key: Option<String>| async move {
            let valid_key = std::env::var("API_KEY").unwrap_or_else(|_| "secret".to_string());
            if api_key.as_deref() != Some(&valid_key) {
                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                    warp::reply::json(&{"success": false, "message": "Unauthorized"}),
                    warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
            let wallet = SolanaWallet::new(Pubkey::new_unique());
            let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
            let balance = wallet.get_balance(&rpc).unwrap_or(0);
            Ok(warp::reply::json(&{"balance": balance}))
        });

    let trades = warp::path!("api" / "trades")
        .and(warp::header::optional::<String>("x-api-key"))
        .and_then(|api_key: Option<String>| async move {
            let valid_key = std::env::var("API_KEY").unwrap_or_else(|_| "secret".to_string());
            if api_key.as_deref() != Some(&valid_key) {
                return Ok::<_, warp::Rejection>(warp::reply::with_status(
                    warp::reply::json(&{"success": false, "message": "Unauthorized"}),
                    warp::http::StatusCode::UNAUTHORIZED,
                ));
            }
            let repo = Repository::new();
            let list = repo.load_trades();
            Ok(warp::reply::json(&list))
        });

    let stats_summary = warp::path!("api" / "stats" / "summary")
        .map(|| {
            let repo = Repository::new();
            let trades = repo.load_trades();
            let total = trades.len();
            let profit: f64 = trades.iter().filter(|t| t.status.contains("buy")).map(|t| t.amount).sum();
            let loss: f64 = trades.iter().filter(|t| t.status.contains("sell")).map(|t| t.amount).sum();
            warp::reply::json(&serde_json::json!({
                "total_trades": total,
                "total_buy": profit,
                "total_sell": loss
            }))
        });

    let price = warp::path!("api" / "price" / String / String)
        .and(with_rate_limit())
        .and_then(|source: String, symbol: String| async move {
            let price = match source.as_str() {
                "pumpfun" => get_cached_price("pumpfun", &symbol, |s| Box::pin(crate::integration::dex::PumpFun::get_price(s))).await,
                "raydium" => get_cached_price("raydium", &symbol, |s| Box::pin(crate::integration::dex::Raydium::get_price(s))).await,
                "jupiter" => get_cached_price("jupiter", &symbol, |s| Box::pin(crate::integration::dex::Jupiter::get_price(s))).await,
                "orca" => get_cached_price("orca", &symbol, |s| Box::pin(crate::integration::dex::Orca::get_price(s))).await,
                "meteora" => get_cached_price("meteora", &symbol, |s| Box::pin(crate::integration::dex::Meteora::get_price(s))).await,
                "lifinity" => get_cached_price("lifinity", &symbol, |s| Box::pin(crate::integration::dex::Lifinity::get_price(s))).await,
                "birdeye" => get_cached_price("birdeye", &symbol, |s| Box::pin(crate::integration::dex::Birdeye::get_price(s))).await,
                "stepn" => get_cached_price("stepn", &symbol, |s| Box::pin(crate::integration::dex::Stepn::get_price(s))).await,
                "coingecko" => get_cached_price("coingecko", &symbol, |s| Box::pin(crate::integration::data::CoinGecko::get_token_price(s))).await,
                "dexscreener" => get_cached_price("dexscreener", &symbol, |s| Box::pin(crate::integration::data::Dexscreener::get_token_price(s))).await,
                // Serum tetap sync karena belum ada logic nyata
                "serum" => crate::integration::dex::SerumDex::get_price(&symbol),
                _ => None,
            };
            Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({"source": source, "symbol": symbol, "price": price })))
        })
        .recover(rate_limit_recover);

    let market = warp::path!("api" / "market" / String)
        .and_then(|symbol: String| async move {
            let info = crate::integration::data::CoinGeckoTerminal::get_market_info(&symbol).await;
            Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({"symbol": symbol, "info": info })))
        });

    let health = warp::path!("api" / "health")
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok", "time": chrono::Utc::now().to_rfc3339()})));

    let docs = warp::path!("api" / "docs")
        .map(|| warp::reply::json(&serde_json::json!({
            "endpoints": [
                {"method": "GET", "path": "/api/hello", "desc": "Cek koneksi backend"},
                {"method": "GET", "path": "/api/status", "desc": "Status backend & waktu server"},
                {"method": "GET", "path": "/api/dashboard", "desc": "Statistik trading (dummy/statik)"},
                {"method": "GET", "path": "/api/trades", "desc": "Riwayat trade (X-API-KEY)"},
                {"method": "POST", "path": "/api/trade", "desc": "Eksekusi trade (X-API-KEY, body: symbol, amount, side)"},
                {"method": "POST", "path": "/api/trade/cancel", "desc": "Cancel trade by id (X-API-KEY, body: id)"},
                {"method": "POST", "path": "/api/trade/update", "desc": "Update status trade by id (X-API-KEY, body: id, status)"},
                {"method": "GET", "path": "/api/wallet/balance", "desc": "Cek saldo wallet (X-API-KEY)"},
                {"method": "GET", "path": "/api/notify/{msg}", "desc": "Kirim notifikasi (log)"},
                {"method": "GET", "path": "/api/stats/summary", "desc": "Statistik lanjutan (total trade, buy, sell)"},
                {"method": "GET", "path": "/api/price/{source}/{symbol}", "desc": "Cek harga token dari DEX atau CoinGecko"},
                {"method": "GET", "path": "/api/market/{symbol}", "desc": "Cek info market token dari CoinGecko"},
                {"method": "GET", "path": "/api/health", "desc": "Healthcheck backend"},
                {"method": "GET", "path": "/api/ws/price", "desc": "WebSocket harga token realtime"},
                {"method": "GET", "path": "/api/sniper/config", "desc": "Lihat config sniping"},
                {"method": "POST", "path": "/api/sniper/config", "desc": "Update config sniping (body: config)"},
                {"method": "GET", "path": "/api/sniper/history", "desc": "Lihat history snipes"}
            ],
            "auth": "Gunakan header X-API-KEY: secret (atau sesuai .env/API_KEY)",
            "note": "Semua endpoint trade, wallet, dan trades membutuhkan autentikasi."
        })));

    let ws_route = warp::path!("api" / "ws" / "price")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(ws_handler));

    let sniper_config = warp::path!("api" / "sniper" / "config")
        .and(warp::get())
        .map(|| {
            let sniper = SNIPER.lock().unwrap();
            warp::reply::json(&sniper.config)
        })
        .or(warp::path!("api" / "sniper" / "config")
            .and(warp::post())
            .and(warp::body::json())
            .map(|cfg: SnipeConfig| {
                let mut sniper = SNIPER.lock().unwrap();
                sniper.update_config(cfg);
                warp::reply::json(&{"success": true})
            })
        );

    let sniper_history = warp::path!("api" / "sniper" / "history")
        .map(|| {
            let sniper = SNIPER.lock().unwrap();
            warp::reply::json(&sniper.get_history())
        });

    hello
        .or(status)
        .or(dashboard)
        .or(notify)
        .or(trade)
        .or(cancel_trade)
        .or(update_trade)
        .or(wallet_balance)
        .or(trades)
        .or(docs)
        .or(stats_summary)
        .or(price)
        .or(market)
        .or(health)
        .or(ws_route)
        .or(sniper_config)
        .or(sniper_history)
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use serde_json::Value;

    #[tokio::test]
    async fn test_hello_endpoint() {
        let api = routes();
        let resp = request().method("GET").path("/api/hello").reply(&api).await;
        assert_eq!(resp.status(), 200);
        let v: Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(v["message"], "Hello from backend!");
    }

    #[tokio::test]
    async fn test_status_endpoint() {
        let api = routes();
        let resp = request().method("GET").path("/api/status").reply(&api).await;
        assert_eq!(resp.status(), 200);
        let v: Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(v["status"], "ok");
    }

    #[tokio::test]
    async fn test_wallet_balance_unauthorized() {
        let api = routes();
        let resp = request().method("GET").path("/api/wallet/balance").reply(&api).await;
        assert_eq!(resp.status(), 401);
    }

    #[tokio::test]
    async fn test_trades_unauthorized() {
        let api = routes();
        let resp = request().method("GET").path("/api/trades").reply(&api).await;
        assert_eq!(resp.status(), 401);
    }

    #[tokio::test]
    async fn test_trade_invalid_input() {
        let api = routes();
        let resp = request()
            .method("POST")
            .path("/api/trade")
            .header("x-api-key", "secret")
            .json(&serde_json::json!({"symbol":"","amount":0,"side":"buy"}))
            .reply(&api)
            .await;
        assert_eq!(resp.status(), 400);
    }

    #[tokio::test]
    async fn test_trade_unauthorized() {
        let api = routes();
        let resp = request()
            .method("POST")
            .path("/api/trade")
            .json(&serde_json::json!({"symbol":"SOL/USDC","amount":1,"side":"buy"}))
            .reply(&api)
            .await;
        assert_eq!(resp.status(), 401);
    }
}

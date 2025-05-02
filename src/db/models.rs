//! Database Models: Trade History, Performance Metrics, System Logs

#[derive(Debug, Clone)]
pub struct TradeHistory {
    pub id: u64,
    pub symbol: String,
    pub amount: f64,
    pub status: String,
}

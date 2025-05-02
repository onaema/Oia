//! Database Repository: Data Access, Query Builder, Cache Layer
use crate::db::models::TradeHistory;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write};
use serde_json;

pub struct Repository;

impl Repository {
    pub fn new() -> Self {
        Self
    }

    pub fn save_trade(&self, trade: &TradeHistory) {
        let mut trades = self.load_trades();
        trades.push(trade.clone());
        let json = serde_json::to_string_pretty(&trades).unwrap();
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("trades.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn load_trades(&self) -> Vec<TradeHistory> {
        let mut file = match File::open("trades.json") {
            Ok(f) => f,
            Err(_) => return vec![],
        };
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        }
    }

    pub fn cancel_trade(&self, trade_id: u64) -> bool {
        let mut trades = self.load_trades();
        let original_len = trades.len();
        trades.retain(|t| t.id != trade_id);
        let json = serde_json::to_string_pretty(&trades).unwrap();
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("trades.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        trades.len() < original_len
    }

    pub fn update_trade_status(&self, trade_id: u64, new_status: &str) -> bool {
        let mut trades = self.load_trades();
        let mut updated = false;
        for t in &mut trades {
            if t.id == trade_id {
                t.status = new_status.to_string();
                updated = true;
            }
        }
        let json = serde_json::to_string_pretty(&trades).unwrap();
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open("trades.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();
        updated
    }
}

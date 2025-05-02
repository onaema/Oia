//! Dashboard: Trade Overview, Performance Charts, Settings Panel
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct DashboardStats {
    pub total_trades: u64,
    pub profit: f64,
    pub loss: f64,
}

pub fn get_dashboard_stats() -> DashboardStats {
    DashboardStats {
        total_trades: 42,
        profit: 123.45,
        loss: 12.34,
    }
}

pub struct Dashboard;

impl Dashboard {
    pub fn new() -> Self {
        Self
    }
    pub fn get_overview(&self) -> String {
        "Trade overview: ...".to_string()
    }
}

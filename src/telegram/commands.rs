//! Telegram Commands: Start/Stop, Status/Info, Settings

pub fn handle_command(cmd: &str) -> String {
    match cmd {
        "/start" => "Bot started!".to_string(),
        "/status" => "Bot is running.".to_string(),
        _ => "Unknown command.".to_string(),
    }
}

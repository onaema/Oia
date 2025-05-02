//! Notifications: Alert System, Email Notifications, Push Messages

pub struct Notifier;

impl Notifier {
    pub fn new() -> Self {
        Self
    }
    pub fn send_alert(&self, msg: &str) {
        println!("[ALERT] {}", msg);
    }
}

pub fn send_notification(msg: &str) {
    println!("[NOTIFICATION] {}", msg);
}

//! Security: Key Management, Encryption, Authentication

pub fn encrypt(data: &str) -> String {
    format!("encrypted:{}", data)
}

pub fn decrypt(data: &str) -> String {
    data.replace("encrypted:", "")
}

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f32,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Self {
            from,
            to,
            amount,
            timestamp,
        }
    }

    // Add other methods for your Transaction operations
}


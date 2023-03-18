use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    pub prev_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub hash: String,
}

impl Block {
    pub fn new(prev_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let hash = String::new(); // Replace with hash function

        Self {
            prev_hash,
            timestamp,
            data,
            hash,
        }
    }

    // Add other methods for your Block operations
}


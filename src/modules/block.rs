use std::time::{SystemTime, UNIX_EPOCH};
use sha3::{Digest, Sha3_256};
use std::io::Write;
//use bincode::*;
//use hex::*;

pub struct Block {
    pub id: u64,
    pub prev_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub hash: String,
    pub nonce: u64,
    pub parents: Option<Vec<u64>>,
}

impl Block {
    pub fn new(prev_hash: String, data: String) -> Self {
        let id = 50; // TODO: Make real id system

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let hash = String::new(); // Replace with hash function

        let nonce = 0;

        let parents = None;
        //let parents = vec![51]; // TODO: Make sure we get the right parent

        Self {
            id,
            prev_hash,
            timestamp,
            data,
            hash,
            nonce,
            parents,
        }
    }

    pub fn get_hash(&self) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(self.id.to_be_bytes());
        hasher.update(self.prev_hash.as_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(self.data.as_bytes());
        hasher.update(self.nonce.to_be_bytes());
        match &self.parents {
            Some(parents) => {
                let serialized_parents = bincode::serialize(&parents).expect("Failed to serialize parents");
                hasher.update(&serialized_parents);
            }
            None => (),
        }
        let result = hasher.finalize();
        hex::encode(result)

    }
    
    // Add other methods for your Block operations


}


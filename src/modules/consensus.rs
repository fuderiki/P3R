use crate::modules::block::Block;
use crate::modules::transaction::Transaction;
use sha3::{Digest, Sha3_256};

const DIFFICULTY: usize = 4;

pub fn pow(block: &mut Block, transactions: &[Transaction]) {
    let mut nonce: u64 = 0;
    loop {
        let hash = calculate_hash(block, transactions, nonce);
        if hash_starts_with_difficulty(&hash, DIFFICULTY) {
            block.hash = hash;
            block.nonce = nonce;
            break;
        }
        nonce += 1;
    }
}

fn calculate_hash(block: &Block, transactions: &[Transaction], nonce: u64) -> String {
    let mut hasher = sha3::Sha3_256::new();
    let serialized_transactions = bincode::serialize(&transactions).expect("Failed to serialize transactions");
    hasher.update(block.id.to_be_bytes());
    hasher.update(block.prev_hash.as_bytes());
    hasher.update(block.timestamp.to_be_bytes());
    hasher.update(block.data.as_bytes());
    hasher.update(nonce.to_be_bytes());
    hasher.update(&serialized_transactions);
    if let Some(parents) = &block.parents {
        let serialized_parents = bincode::serialize(&parents).expect("Failed to serialize parents");
        hasher.update(&serialized_parents);
    }
    let result = hasher.finalize();
    hex::encode(result)
}

fn hash_starts_with_difficulty(hash: &str, difficulty: usize) -> bool {
    let prefix = "0".repeat(difficulty);
    hash.starts_with(&prefix)
}


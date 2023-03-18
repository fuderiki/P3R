use crate::modules::block::Block;
use crate::modules::transaction::Transaction;
use sha3::{Digest, Sha3_256};

const DIFFICULTY: usize = 4;

// Function to perform Proof of Work
pub fn pow(block: &mut Block, transactions: &[Transaction]) {
    let mut nonce: u64 = 0; // Start nonce at 0
    loop {
        let hash = calculate_hash(block, transactions, nonce); // Calculate hash with current nonce
        if hash_starts_with_difficulty(&hash, DIFFICULTY) { // Check if hash meets the required difficulty
            block.hash = hash; // Set hash value in block
            block.nonce = nonce; // Set nonce value in block
            break; // Exit loop if hash meets requirements
        }
        nonce += 1; // Increment nonce if hash does not meet requirements
    }
}

// Function to calculate hash value
fn calculate_hash(block: &Block, transactions: &[Transaction], nonce: u64) -> String {
    let mut hasher = Sha3_256::new(); // Create new SHA-3 hasher
    let serialized_transactions = bincode::serialize(&transactions).expect("Failed to serialize transactions"); // Serialize transactions
    hasher.update(block.id.to_be_bytes()); // Hash block ID
    hasher.update(block.prev_hash.as_bytes()); // Hash previous block hash
    hasher.update(block.timestamp.to_be_bytes()); // Hash block timestamp
    hasher.update(block.data.as_bytes()); // Hash block data
    hasher.update(nonce.to_be_bytes()); // Hash nonce
    hasher.update(&serialized_transactions); // Hash serialized transactions
    if let Some(parents) = &block.parents { // Check if block has parent blocks
        let serialized_parents = bincode::serialize(&parents).expect("Failed to serialize parents"); // Serialize parent block IDs
        hasher.update(&serialized_parents); // Hash serialized parent block IDs
    }
    let result = hasher.finalize(); // Finalize hash
    hex::encode(result) // Return hex-encoded hash value
}

// Function to check if hash value meets required difficulty
fn hash_starts_with_difficulty(hash: &str, difficulty: usize) -> bool {
    let prefix = "0".repeat(difficulty); // Create prefix string with required number of zeros
    hash.starts_with(&prefix) // Check if hash value starts with prefix
}

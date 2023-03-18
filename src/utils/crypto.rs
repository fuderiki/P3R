use sha3::{Digest, Sha3_256};

pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

// Add other cryptography functions and utilities as needed

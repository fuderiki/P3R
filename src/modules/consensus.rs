use crate::modules::block::Block;
use crate::modules::transaction::Transaction;
use crate::utils::crypto::sha3_256;
/*    Just a basic PoW for now 
      The goal is to replace it with something more elegant later 
*/

    
const DIFFICULTY: usize = 4;

pub fn pow(block: &mut Block, transactions: &[Transaction]) {
    let mut nonce = 0;
    loop {
        let data = format!(
            "{:?}{:?}{:?}{:?}",
            block.timestamp, block.prev_hash, transactions, nonce
        );
        let hash = sha3_256(data.as_bytes());
        if hash.starts_with(&[0; DIFFICULTY]) {
            block.hash = hash;
            block.nonce = nonce;
            break;
        }
        nonce += 1;
    }
}

use super::*;
use crate::modules::block::Block;
use crate::modules::transaction::{Transaction, TxInput, TxOutput};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_pow() {
    let mut block = Block::new(vec![], [0; 32]);
    let txin = TxInput::new([0; 32], 0, vec![], vec![]);
    let txout = TxOutput::new(10, vec![0; 20]);
    let transaction = Transaction::new(vec![txin], vec![txout]);
    let transactions = vec![transaction];
    let start = SystemTime::now();
    pow(&mut block, &transactions);
    let end = SystemTime::now();
    println!("Hash: {:?}", block.hash);
    println!("Nonce: {:?}", block.nonce);
    println!("Elapsed time: {:?}", end.duration_since(start).unwrap());
    assert!(block.hash.starts_with(&[0; DIFFICULTY]));
}


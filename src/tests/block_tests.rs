use crate::modules::block::Block;

#[test]
fn test_block_creation() {
    let prev_hash = String::from("previous block hash");
    let data = String::from("data");

    let block = Block::new(prev_hash, data);

    assert_eq!(block.prev_hash, "previous block hash");
    assert_eq!(block.data, "data");
    // Add assertions to test other fields of the Block struct
}


use crate::modules::block::Block;
use crate::modules::dag::DAG;

#[test]
fn test_dag_creation() {
    let dag = DAG::new();

    assert_eq!(dag.nodes.len(), 0);
}

#[test]
fn test_add_block_to_dag() {
    let mut dag = DAG::new();

    let block = Block::new("previous block hash".to_string(), "data".to_string());
    dag.add_block(block, vec![]);

    assert_eq!(dag.nodes.len(), 1);
    // Add assertions to test the values of the Block and parent node hashes
}

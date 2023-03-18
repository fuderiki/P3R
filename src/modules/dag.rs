pub struct DAG {
    pub nodes: Vec<Node>,
}

pub struct Node {
    pub block: Block,
    pub parents: Vec<String>,
}

impl DAG {
    pub fn new() -> Self {
        let nodes = vec![];

        Self { nodes }
    }

    pub fn add_block(&mut self, block: Block, parents: Vec<String>) {
        let node = Node { block, parents };
        self.nodes.push(node);
    }

    // Add other methods for your DAG operations
}

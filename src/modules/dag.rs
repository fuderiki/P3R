use crate::modules::block::Block;
use std::collections::{HashMap, HashSet};

pub struct DAG {
    pub blocks: HashMap<u64, Block>,
    pub tip_ids: HashSet<u64>,
    pub tips: Vec<String>,
}

impl DAG {
    pub fn new() -> Self {
        DAG {
            blocks: HashMap::new(),
            tip_ids: HashSet::new(),
            tips: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) -> bool {
        // Check if block already exists
        if self.blocks.contains_key(&block.id) {
            return false;
        }

        // Check if block points to non-existent parent(s)
        if let Some(parents) = &block.parents {
            if !parents.iter().all(|parent_id| self.blocks.contains_key(parent_id)) {
                return false;
            }
        }

        // Add block to DAG
        self.blocks.insert(block.id, block);

        // Update tips
        self.update_tips();

        true
    }

    pub fn get_block(&self, id: &u64) -> Option<&Block> {
        self.blocks.get(id)
    }

    pub fn get_tips(&self) -> &Vec<String> {
        &self.tips
    }

    fn update_tips(&mut self) {
        // TODO: Implement update_tips function
        self.tips = vec![];
    }
}

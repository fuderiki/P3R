use crate::modules::block::Block;
use std::collections::{HashMap, HashSet};

pub struct DAG {
    pub blocks: HashMap<String, Block>,
    pub tip_ids: HashSet<String>,
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
        // Check if block is already in DAG
        if self.blocks.contains_key(&block.hash) {
            return false;
        }

        // Check if block points to non-existent parent(s)
        if let Some(parents) = &block.parents {
            for parent_id in parents {
                if !self.blocks.contains_key(&parent_id) {
                    return false;
                }
            }
        }
        else {
            return false;
        }

        // Add block to DAG
        self.blocks.insert(block.id.clone(), block);
        self.tip_ids.insert(block.id.clone());

        // Update tips
        if let Some(parents) = self.blocks[&block.id].parents {
            for parent_id in parents {
                if self.tip_ids.remove(&parent_id) {
                    self.tips.retain(|tip_id| *tip_id != parent_id);
                }
            }
        }
        self.tips.push(block.id);

        true
    }

    pub fn get_tips(&self) -> Vec<&Block> {
        self.tips.iter().map(|tip_id| &self.blocks[tip_id]).collect()
    }

    pub fn get_block(&self, id: &str) -> Option<&Block> {
        self.blocks.get(id)
    }
}

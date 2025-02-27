use crate::block::Block;

// Blockchain in simple terms = DB composed of linked-list of blocks. But, with information stored in distributed manner.
// That's why we need a consensus mechanism to sync the info.
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

// simple implementation without 1) searching hash and 2) storing block. 
// Only feature = storing a block.
impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blocks = Vec::new();
        blocks.push(Blockchain::new_genesis_block());
        Blockchain { blocks: blocks }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block_hash = &self.blocks.last().unwrap().hash();
        let new_block = Block::new(data, &prev_block_hash);
        self.blocks.push(new_block);
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    fn new_genesis_block() -> Block {
        let block = Block::new("Genesis Block", &vec![]);
        block
    }
}
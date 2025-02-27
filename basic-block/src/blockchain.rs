use crate::block::Block;
use rocksdb::{DB, Options, WriteBatch};
use std::sync::Arc;

// Blockchain in simple terms = DB composed of linked-list of blocks. But, with information stored in distributed manner.
// That's why we need a consensus mechanism to sync the info.
pub struct Blockchain {
    tip: Vec<u8>,
    database: Arc<DB>,
}

// simple implementation without 1) searching hash and 2) storing block. 
// Only feature = storing a block.
impl Blockchain {
    pub fn new() -> Blockchain {
        let tip: Vec<u8>;
        let path = "./blockchain.db";
        let mut options = Options::default();
        options.create_if_missing(true);
        let database = Arc::new(DB::open(&options, path).unwrap());
        let last_hash = database.get(b"l").unwrap();

        if last_hash.is_none() {
            let genesis_block = Blockchain::new_genesis_block();
            let genesis_hash = genesis_block.hash().to_vec();
            database.put(&genesis_hash, &genesis_block.serialize()).unwrap();
            database.put(b"l", &genesis_hash).unwrap();
            tip = genesis_hash;
        } else {
            tip = last_hash.unwrap().to_vec();
        }

        Blockchain { tip: tip, database: database, }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block_hash: Vec<u8> = self.database.get(b"l").unwrap().unwrap().to_vec();
        let new_block = Block::new(data, &prev_block_hash);

        let mut batch = WriteBatch::default();
        batch.put(&new_block.hash(), &new_block.serialize());
        batch.put(b"l", &new_block.hash());
        self.database.write(batch).unwrap();
        self.tip = new_block.hash().to_vec();
    }

    fn new_genesis_block() -> Block {
        let block = Block::new("Genesis Block", &vec![]);
        block
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(Arc::clone(&self.database), self.tip.clone())
    }
}

// Blockchain iterator for a new search logic.
pub struct BlockchainIterator {
    current_hash: Vec<u8>,
    database: Arc<DB>,
}

impl BlockchainIterator {
    pub fn new(database: Arc<DB>, current_hash: Vec<u8>) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: current_hash,
            database: database,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let block_data = self.database.get(&self.current_hash).unwrap();
        if block_data.is_none() {
            return None;
        }

        let block = Block::deserialize(&block_data.unwrap());
        self.current_hash = block.prev_block_hash().to_vec();
        Some(block)
    }
}
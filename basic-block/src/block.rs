use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp: i64, // info about block generation time
    pub data: Vec<u8>, // info about block Tx data
    prev_block_hash: Vec<u8>, // hash of previous block
    hash: Vec<u8>, // hash of current block
}

impl Block {
    // method for generating a block
    pub fn new(data: &str, prev_block_hash: &[u8]) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            data: data.as_bytes().to_vec(),
            prev_block_hash: prev_block_hash.to_vec(),
            hash: vec![],
        };

        block.set_hash();
        block
    }

    // method for getting the hash of the block
    pub fn set_hash(&mut self) {
        let timestamp = self.timestamp.to_string();
        let headers = [
            self.prev_block_hash.clone(),
            self.data.clone(),
            timestamp.as_bytes().to_vec(),
        ]
        .concat();

        self.hash = Sha256::digest(&headers).to_vec();
    }   

    // method for accessing the hash of the block (getter)
    pub fn hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn prev_block_hash(&self) -> &[u8] {
        &self.prev_block_hash
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
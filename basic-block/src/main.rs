pub mod block; 
pub mod blockchain;
pub mod pow;

use hex;

// Test script to see a simple block producing step.
fn main() {
    let mut bc = blockchain::Blockchain::new();

    bc.add_block("Send 1 Hype to Alice");
    bc.add_block("Send 2 more HYPE to Alice");

    for block in bc.blocks() {
        println!("Prev. hash: {:?}", hex::encode(&block.prev_block_hash()));
        println!("Data: {}", String::from_utf8(block.data().to_vec()).unwrap());
        println!("Hash: {:?}", hex::encode(&block.hash()));

        let pow = pow::ProofOfWork::new(&block, 24);
        println!("PoW: {}", pow.validate());
        println!();
    }
}

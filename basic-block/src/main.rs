pub mod block; 
pub mod blockchain;

use hex;

fn main() {
    let mut bc = blockchain::Blockchain::new();

    bc.add_block("Send 1 Hype to Alice");
    bc.add_block("Send 2 more HYPE to Alice");

    for block in bc.blocks() {
        println!("Prev. hash: {:?}", hex::encode(&block.prev_block_hash()));
        println!("Data: {}", String::from_utf8(block.data().to_vec()).unwrap());
        println!("Hash: {:?}", hex::encode(&block.hash()));
        println!();
    }
}

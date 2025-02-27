pub mod block; 
pub mod blockchain;
pub mod pow;

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

// expected output:
//
// Prev. hash: ""
// Data: Genesis Block
// Hash: "757b4e1de185a4b578c9aa8f09e5ce41f08f4d16bc3b79acac9b487f7b963720"

// Prev. hash: "757b4e1de185a4b578c9aa8f09e5ce41f08f4d16bc3b79acac9b487f7b963720"
// Data: Send 1 Hype to Alice
// Hash: "f5cd705de484cf854e02421c123a8ae60f9681d2ad2fd3855f9f699a9e4bfe8a"

// Prev. hash: "f5cd705de484cf854e02421c123a8ae60f9681d2ad2fd3855f9f699a9e4bfe8a"
// Data: Send 2 more HYPE to Alice
// Hash: "43bc6d740754231447c79c97967ee50364d46c445f45cc03ee0d35f5c049cee7"
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

        let pow = pow::ProofOfWork::new(&block, 8);
        println!("PoW: {}", pow.validate());
        println!();
    }
}

// expected output:
//
// Mining the block containing "Genesis Block"
// Mining the block containing "Send 1 Hype to Alice"
// Mining the block containing "Send 2 more HYPE to Alice"
// Prev. hash: ""
// Data: Genesis Block
// Hash: "00fb786fe64b55bc66af676cc66066b7f56469d7f33910421e9f807ec43519d6"
// PoW: true

// Prev. hash: "00fb786fe64b55bc66af676cc66066b7f56469d7f33910421e9f807ec43519d6"
// Data: Send 1 Hype to Alice
// Hash: "00cc461156a0ce74d40c30cb4c349791e4eb09f59daea9c5770d0c430d84c311"
// PoW: true

// Prev. hash: "00cc461156a0ce74d40c30cb4c349791e4eb09f59daea9c5770d0c430d84c311"
// Data: Send 2 more HYPE to Alice
// Hash: "002d5a208bc0b36a096b7c1f0f8aaa2d9d28692df55ffeba403764ef3763bf81"
// PoW: true

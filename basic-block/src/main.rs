pub mod block; 
pub mod blockchain;
pub mod pow;

use hex;

fn main() {
    let mut bc = blockchain::Blockchain::new();

    bc.add_block("Send 1 Hype to Alice");
    bc.add_block("Send 2 more HYPE to Alice");

    let mut iterator = bc.iterator();
    loop {
        match iterator.next() {
            Some(block) => {
                println!("Prev. hash: \"{}\"", hex::encode(block.prev_block_hash()));
                println!("Data: {}", String::from_utf8(block.data().to_vec()).unwrap());
                println!("Hash: \"{}\"\n", hex::encode(block.hash()));

                let pow = pow::ProofOfWork::new(&block, 8);
                print!("PoW: {}", pow.validate());
                println!("");
            },
            None => break,
        }
    }
}

// expected output:
//
// Mining the block containing "Genesis Block"
// Mining the block containing "Send 1 Hype to Alice"
// Mining the block containing "Send 2 more HYPE to Alice"
// Prev. hash: "00049bfcb20678e0a84ed579f43dbc990b89edd6204d48abe5861f8c47be3285"
// Data: Send 2 more HYPE to Alice
// Hash: "005d42bfc6e08d02381f6161a2a7c2067b1a442612fbafb601054899414a6321"

// PoW: true
// Prev. hash: "00abe2f12d9cb807b663d0d3d16565c7fd30748c33245df71a23b048baabe1da"
// Data: Send 1 Hype to Alice
// Hash: "00049bfcb20678e0a84ed579f43dbc990b89edd6204d48abe5861f8c47be3285"

// PoW: true
// Prev. hash: ""
// Data: Genesis Block
// Hash: "00abe2f12d9cb807b663d0d3d16565c7fd30748c33245df71a23b048baabe1da"

// PoW: true
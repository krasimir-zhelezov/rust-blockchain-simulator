use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Block {
    index: u128,
    data: String,
    hash: String,
    previous: String,
    nonce: u64,
    timestamp: DateTime<Utc>
}

// #[derive(Serialize, Deserialize)]
// struct MiningState {
//     block_index: u64,
//     last_nonce: u64,
// }

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    let genesis_block: Block = create_genesis_block();
    println!("Genesis Block: {:#?}", genesis_block);
    blockchain.push(genesis_block);
}

fn create_genesis_block() -> Block {
    let mut genesis_block = Block {
        index: 0,
        data: String::from("Genesis Block"),
        hash: String::new(),
        previous: "0".repeat(64),
        nonce: 0,
        timestamp: Utc::now()
    };

    let nonce_hash = find_nonce(&genesis_block);
    genesis_block.nonce = nonce_hash.0;
    genesis_block.hash = nonce_hash.1;

    genesis_block
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn find_nonce(block: &Block) -> (u64, String) {
    let mut nonce = 0;
    let data = format!("{}{}", nonce, block.data);
    let mut hash_string = hash(&data);
    
    while !hash_string.starts_with("000") {
        nonce += 1;
        let data = format!("{}{}", nonce, block.data);
        hash_string = hash(&data);
        
        if nonce % 1_000_000 == 0 {
            println!("Nonce: {} / Hash: {}", nonce, hash_string);
        }
    }

    (nonce, hash_string)
}
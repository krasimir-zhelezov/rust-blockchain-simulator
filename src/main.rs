use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs;

#[derive(Debug)]
struct Block {
    index: u128,
    data: String,
    hash: String,
    previous: String,
    nonce: u64,
    timestamp: DateTime<Utc>
}

#[derive(Serialize, Deserialize)]
struct MiningState {
    block_index: u64,
    last_nonce: u64,
}

fn main() {
    // let mut blockchain: Vec<Block> = Vec::new();

    let genesis_block: Block = create_genesis_block();

    println!("Genesis Block: {:#?}", genesis_block);
}

fn create_genesis_block() -> Block {
    let mut genesis_block = Block {
        index: 0,
        data: String::from("Genesis Block"),
        hash: String::new(),
        previous: "0".repeat(16),
        nonce: 0,
        timestamp: Utc::now()
    };

    genesis_block.nonce = find_nonce(&genesis_block, 0);

    genesis_block
}

fn hash(input: &str) -> String {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn find_nonce(block: &Block, block_index: u64) -> u64 {
    let mut nonce = 0;
    let data = format!("{}{}", nonce, block.data);
    let mut hash_string = hash(&data);
    
    while !hash_string.starts_with("0") {
        nonce += 1;
        let data = format!("{}{}", nonce, block.data);
        hash_string = hash(&data);
        
        if nonce % 100_000 == 0 {
            println!("Trying nonce: {}", nonce);
        }
    }

    nonce
}
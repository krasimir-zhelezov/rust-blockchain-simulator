// src/blockchain/service.rs
use super::model::Block;
use sha2::{Sha256, Digest};
use chrono::Utc;

static BLOCKCHAIN: once_cell::sync::Lazy<std::sync::Mutex<Vec<Block>>> = 
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(Vec::new()));

pub fn get_blockchain() -> Vec<Block> {
    BLOCKCHAIN.lock().unwrap().clone()
}

pub fn add_block(data: String) -> Block {
    let mut blockchain = BLOCKCHAIN.lock().unwrap();
    
    let previous_hash = if blockchain.is_empty() {
        "0".repeat(64)
    } else {
        blockchain.last().unwrap().hash.clone()
    };

    let new_block = initialize_block(
        blockchain.len() as u128,
        data,
        previous_hash
    );

    blockchain.push(new_block.clone());
    new_block
}

fn initialize_block(index: u128, data: String, previous: String) -> Block {
    let mut block = Block {
        index,
        data,
        hash: String::new(),
        previous,
        nonce: 0,
        timestamp: Utc::now().to_string()
    };

    let nonce_hash = find_nonce(&block);
    block.nonce = nonce_hash.0;
    block.hash = nonce_hash.1;

    block
}

fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn find_nonce(block: &Block) -> (u64, String) {
    let mut nonce = 0;
    let mut hash_string; 
    
    loop {
        hash_string = hash(&format!("{}{}{}{}{}", nonce, block.index, block.data, block.timestamp, block.previous));
        
        if hash_string.starts_with("000") {
            break;
        }
        
        nonce += 1;
        
        if nonce % 1_000_000 == 0 {
            println!("Nonce: {} / Hash: {}", nonce, hash_string);
        }
    }

    (nonce, hash_string)
}

pub fn is_blockchain_valid(blockchain: &[Block]) -> bool {
    let mut previous_block_hash: String = String::new();

    for block in blockchain {
        // println!("{:#?}", block);
        if block.index == 0 {
            previous_block_hash = block.hash.clone();
            continue;
        }

        if block.previous != previous_block_hash {
            println!("Block {} is invalid", block.index);
            return false;
        }

        println!("Block {} is valid", block.index);
        previous_block_hash = block.hash.clone();
    }

    true
}
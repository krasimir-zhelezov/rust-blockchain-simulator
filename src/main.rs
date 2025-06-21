use chrono::{DateTime, Utc};
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

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    blockchain.push(create_block(&blockchain, "Genesis Block".to_string()));
    blockchain.push(create_block(&blockchain, "Transaction Info".to_string()));
    blockchain.push(create_block(&blockchain, "Hello World".to_string()));
    blockchain.push(create_block(&blockchain, "Cool data".to_string()));
    blockchain.push(create_block(&blockchain, "Rust the best".to_string()));
    blockchain.push(create_block(&blockchain, "blockchain-simulator".to_string()));

    println!("Valid blockchain: {}", is_blockchain_valid(&blockchain));

    println!("{:#?}", blockchain);
}

fn create_block(blockchain: &[Block], data: String) -> Block {    
    let previous_hash = if blockchain.is_empty() {
        "0".repeat(64)
    } else {
        blockchain[blockchain.len() - 1].hash.clone()
    };

    initialize_block(
        blockchain.len() as u128,
        data,
        previous_hash
    )
}

fn initialize_block(index: u128, data: String, previous: String) -> Block {
    let mut block = Block {
        index,
        data,
        hash: String::new(),
        previous,
        nonce: 0,
        timestamp: Utc::now()
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

fn is_blockchain_valid(blockchain: &[Block]) -> bool {
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